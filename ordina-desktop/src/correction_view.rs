//! egui rendering + control for the correction loop — RFC-0004 §17/§19.
//!
//! Intentionally thin: **all** correction logic lives in the pure, egui-free
//! `draft`/`hydrate`/`translate`/`correction` modules (unit-tested). This
//! module only lays the committed diagnosis out on screen and wires the bounded
//! Trader controls to [`CorrectionSession`] methods — it reimplements no
//! translation, hydration, or checking (RFC-0004 §13: one shared translation
//! implementation).
//!
//! The bounded surface (PBA-0168): begin a correction, set the semantic-model
//! version to the supported token, add/confirm/remove a Blocking condition,
//! confirm the submission, and submit (full fresh re-check). Everything else in
//! the Strategy is carried through translation verbatim.

use std::path::Path;

use eframe::egui;
use ordina::model::condition::ConditionClassification;
use ordina::model::guard::Guard;
use ordina::result::Verdict;

use crate::correction::{CorrectionSession, Stage, SubmitError};
use crate::persist;
use crate::translate::TranslationGateError;
use crate::view_model::DiagnosisViewModel;

/// Render the whole correction surface for one session, mutating it in response
/// to the Trader's bounded controls. `envelope_path` is the caller-supplied
/// local persistence path (PBA-0182); when `Some`, a human-reachable Save
/// action persists the whole envelope atomically.
pub fn show(
    ui: &mut egui::Ui,
    session: &mut CorrectionSession,
    envelope_path: Option<&Path>,
    last_message: &mut String,
) {
    // The committed diagnosis, reusing N-22-A's diagnosis view unchanged.
    let vm = DiagnosisViewModel::from_result(session.result());
    crate::diagnosis_view::show(ui, &vm);

    // The Well-Formed → revision-identity endpoint (§16, via N-22-B-1).
    if let Some(identity) = session.revision_identity() {
        ui.separator();
        match identity {
            Ok(_) => {
                ui.label(
                    egui::RichText::new(
                        "Revision identity computed for this Well-Formed Strategy.",
                    )
                    .strong(),
                );
            }
            Err(_) => {
                ui.label("Revision identity unavailable.");
            }
        }
    }

    // Dogfood-0 persistence — a human-reachable Save action (PBA-0182 §5),
    // shown only when a local envelope path is configured. Saves the whole
    // envelope (Strategy + CheckerContext + current mint high-water) atomically.
    if let Some(path) = envelope_path {
        ui.separator();
        if ui.button("Save to local envelope").clicked() {
            match persist::save_session(session, path) {
                Ok(()) => *last_message = format!("Saved to {}.", path.display()),
                Err(error) => *last_message = format!("Save failed: {error}"),
            }
        }
    }

    ui.separator();
    match session.stage() {
        Stage::Diagnosing => {
            // Exhausted-corpus gate (PBA-0182 §7): editing leads to minting new
            // OccurrenceId/LineageId on submit; if the mint domain is exhausted,
            // refuse to open an editing session so the minter's fail-closed
            // exhaustion path is never reached. Diagnosis remains available.
            if persist::can_mint(session.high_water()) {
                if ui.button("Begin correction").clicked() {
                    session.begin_edit();
                    *last_message = "Editing — the Strategy content is now editable.".to_owned();
                }
            } else {
                ui.label(
                    egui::RichText::new(
                        "This corpus's identity mint domain is exhausted; \
                         editing that mints new constructs is disabled.",
                    )
                    .strong(),
                );
            }
        }
        Stage::Editing => show_editing_controls(ui, session, last_message),
    }

    if !last_message.is_empty() {
        ui.separator();
        ui.label(last_message.as_str());
    }
}

fn show_editing_controls(
    ui: &mut egui::Ui,
    session: &mut CorrectionSession,
    last_message: &mut String,
) {
    ui.heading("Correcting");
    let supported = session.context().supported;

    if let Some(draft) = session.draft_mut() {
        // Current field state (§9) and submission-confirmation status (§11).
        ui.label(format!(
            "Semantic-model version: {} · conditions {} · submission {}",
            field_state_label(draft.semantic_model_version()),
            if draft.conditions_declared_absent() {
                "declared absent"
            } else {
                "present"
            },
            if draft.is_submission_confirmed() {
                "confirmed"
            } else {
                "not confirmed"
            },
        ));

        ui.horizontal(|ui| {
            if ui
                .button("Set semantic-model version to supported")
                .clicked()
            {
                draft.set_semantic_model_version(supported);
                *last_message =
                    "Semantic-model version set. Confirm submission, then submit.".to_owned();
            }
            if ui.button("Declare version absent").clicked() {
                draft.declare_semantic_model_version_absent();
                *last_message =
                    "Version declared absent — the re-check will stay Unusable.".to_owned();
            }
            if ui.button("Clear version").clicked() {
                draft.clear_semantic_model_version();
                *last_message =
                    "Version not yet addressed — submission is blocked until you address it."
                        .to_owned();
            }
        });

        ui.horizontal(|ui| {
            if ui.button("Add Blocking condition").clicked() {
                let index = draft.add_condition(
                    ConditionClassification::Blocking,
                    Guard::always_applicable(),
                );
                *last_message = format!("Added condition #{index} — confirm it before submitting.");
            }
        });

        // Per-condition controls: confirm the unconfirmed, remove any (§11/§17.6).
        let condition_count = draft.conditions().len();
        for index in 0..condition_count {
            let confirmed = draft.conditions()[index].is_confirmed();
            let hydrated = draft.conditions()[index].is_hydrated();
            ui.horizontal(|ui| {
                ui.label(format!(
                    "Condition #{index} ({}, {})",
                    if hydrated { "existing" } else { "new" },
                    if confirmed {
                        "confirmed"
                    } else {
                        "unconfirmed"
                    }
                ));
                if !confirmed
                    && ui.button("Confirm").clicked()
                    && let Some(condition) = draft.condition_mut(index)
                {
                    condition.confirm();
                    *last_message = format!("Condition #{index} confirmed.");
                }
                // In-place classification edit (§14.2.3 — identity preserved).
                if ui.button("Set Temporary").clicked()
                    && let Some(condition) = draft.condition_mut(index)
                {
                    condition.set_classification(ConditionClassification::Temporary);
                    *last_message = format!("Condition #{index} set Temporary — re-confirm it.");
                }
                if ui.button("Remove").clicked() {
                    draft.remove_condition(index);
                    *last_message = format!("Condition #{index} removed.");
                }
            });
        }

        ui.horizontal(|ui| {
            if ui.button("Confirm submission").clicked() {
                draft.confirm_submission();
                *last_message = "Submission confirmed — you can now submit.".to_owned();
            }
        });
    }

    ui.horizontal(|ui| {
        if ui.button("Submit (full re-check)").clicked() {
            match session.submit() {
                Ok(Verdict::WellFormed) => {
                    *last_message = "Submitted — the Strategy is now Well-Formed.".to_owned();
                }
                Ok(Verdict::Unusable) => {
                    *last_message =
                        "Submitted and re-checked — the Strategy is still Unusable.".to_owned();
                }
                Err(error) => *last_message = gate_message(&error),
            }
        }
        if ui.button("Abandon edit").clicked() {
            session.abandon_edit();
            *last_message = "Edit abandoned — the prior diagnosis is unchanged.".to_owned();
        }
    });
}

/// A short label for a §9 field state, for the status line.
fn field_state_label(
    state: &crate::draft::FieldState<ordina::model::opaque::VersionToken>,
) -> &'static str {
    match state {
        crate::draft::FieldState::Authored(_) => "authored",
        crate::draft::FieldState::DeclaredAbsent => "declared absent",
        crate::draft::FieldState::NotYetAddressed => "not yet addressed",
    }
}

/// Trader-facing copy for a submission that did not translate (§10/§11). A
/// translation-gate rejection is deliberately phrased as an incomplete
/// authoring act, never as a checker defect (§10).
fn gate_message(error: &SubmitError) -> String {
    match error {
        SubmitError::NoDraft => "No draft to submit.".to_owned(),
        SubmitError::Gate(TranslationGateError::RequiredElementNotAddressed(_)) => {
            "You have not yet addressed a required element (the semantic-model version).".to_owned()
        }
        SubmitError::Gate(TranslationGateError::UnconfirmedContent { condition_index }) => {
            format!("Confirm condition #{condition_index} before submitting.")
        }
        SubmitError::Gate(TranslationGateError::SubmissionNotConfirmed) => {
            "Confirm the submission before submitting.".to_owned()
        }
    }
}
