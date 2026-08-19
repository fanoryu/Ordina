//! egui rendering of a [`DiagnosisViewModel`] — RFC-0003 §9.1-9.6, §9.11.
//!
//! This module is intentionally thin: all translation logic lives in
//! `view_model` (pure, egui-free, unit-testable). This module only lays the
//! already-decided content out on screen.

use eframe::egui;
use ordina::result::Verdict;

use crate::view_model::DiagnosisViewModel;

/// Renders the complete diagnosis surface for one `WellFormednessResult`.
pub fn show(ui: &mut egui::Ui, vm: &DiagnosisViewModel) {
    show_verdict(ui, vm.verdict);

    if vm.verdict == Verdict::WellFormed {
        return;
    }

    ui.separator();
    ui.heading(format!("Issues ({})", vm.defects.len()));
    for defect in &vm.defects {
        ui.group(|ui| {
            ui.label(egui::RichText::new(&defect.headline).strong());
            if !defect.detail.is_empty() {
                ui.label(&defect.detail);
            }
            let fan_out = vm.blocked_by(&defect.identity);
            if !fan_out.is_empty() {
                ui.label(format!(
                    "resolving this may allow {} other check(s) to run",
                    fan_out.len()
                ));
            }
        });
    }

    // §9.5/§9.6: not-evaluated is a separate section, distinct styling, never
    // added to the defect count above.
    if !vm.not_evaluated.is_empty() {
        ui.separator();
        ui.heading(format!("Not yet checked ({})", vm.not_evaluated.len()));
        for entry in &vm.not_evaluated {
            ui.group(|ui| {
                ui.label(&entry.message);
                let blocking_headline = vm
                    .defects
                    .iter()
                    .find(|d| d.identity.try_eq(&entry.blocking) == Some(true))
                    .map_or_else(|| "another issue".to_owned(), |d| d.headline.clone());
                ui.label(format!("blocked by: {blocking_headline}"));
            });
        }
    }
}

/// §9.1/§9.10: `UNUSABLE` gets its own distinct, minimal primary
/// presentation — never a shared label/color role with READY/WAIT/DO_NOT_TRADE
/// (which this crate does not render at all; N-22-A has no trade-readiness
/// surface to conflate with).
fn show_verdict(ui: &mut egui::Ui, verdict: Verdict) {
    match verdict {
        Verdict::WellFormed => {
            ui.heading("Well-Formed");
            ui.label("No issues found.");
        }
        Verdict::Unusable => {
            ui.heading(egui::RichText::new("Unusable").color(egui::Color32::from_rgb(178, 34, 34)));
            ui.label("This Strategy cannot be evaluated yet — see the issues below.");
        }
    }
}
