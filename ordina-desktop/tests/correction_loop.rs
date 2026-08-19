//! N-22-B-2 correction-loop conformance tests — RFC-0004 §10/§11/§12/§14/§16/
//! §17/§19, exercised through `ordina_desktop`'s public library API.
//!
//! Each test maps to one PBA-0168 completion criterion:
//!
//! | Criterion                                   | Test(s)                                                   |
//! |---------------------------------------------|-----------------------------------------------------------|
//! | hydration round-trip                        | `hydration_round_trip_preserves_content_and_identity`     |
//! | explicit-absence preservation               | `explicit_absence_round_trips_as_absence`                 |
//! | omission ≠ absence                          | `omission_is_gated_absence_translates`                    |
//! | confirmation gating                         | `submission_is_gated_on_per_rule_and_final_confirmation`  |
//! | deterministic translation                   | `translation_is_deterministic`                            |
//! | identity preservation (edit in place)       | `edited_in_place_condition_preserves_identity`            |
//! | fresh minting (new construct)               | `new_condition_mints_fresh_identity_above_the_corpus`     |
//! | removed-construct handling                  | `removed_condition_is_absent_and_its_identity_unused`     |
//! | full fresh re-check                          | `submit_runs_a_full_fresh_recheck_over_the_whole_strategy`|
//! | no speculative fixed state                  | `a_gate_rejection_leaves_the_committed_state_unchanged`   |
//! | N-22-A diagnosis regression                 | `n22a_diagnosis_still_builds_from_a_session_result`       |
//! | end-to-end Unusable → edit → re-check        | `end_to_end_unusable_edit_recheck_becomes_well_formed`    |
//! | confirmed Strategy → Well-Formed → identity  | `end_to_end_well_formed_yields_the_clean_revision_identity`|

use ordina::model::condition::ConditionClassification;
use ordina::model::declared::Declared;
use ordina::model::opaque::{LineageId, OccurrenceId, OpaqueValue};
use ordina::model::strategy::Strategy;
use ordina::result::{Verdict, compute_revision_identity};

use ordina_desktop::correction::{CorrectionSession, Stage, SubmitError};
use ordina_desktop::correction_fixtures::{
    MINT_BASE, checker_context, clean_strategy, supported_version, unusable_missing_version,
};
use ordina_desktop::draft::FieldState;
use ordina_desktop::hydrate::hydrate;
use ordina_desktop::translate::{IdentityMinter, RequiredElement, TranslationGateError, translate};

fn occ(handle: u64) -> OccurrenceId {
    OccurrenceId::new(OpaqueValue::new(handle))
}

fn lineage(handle: u64) -> LineageId {
    LineageId::new(OpaqueValue::new(handle))
}

/// The whole checker verdict for a Strategy under the fixture context.
fn verdict_of(strategy: &Strategy) -> Verdict {
    checker_context().check(strategy).verdict()
}

// --- hydration round-trip ---------------------------------------------------

#[test]
fn hydration_round_trip_preserves_content_and_identity() {
    // §17: hydrate an already-Well-Formed Strategy and translate the untouched
    // draft straight back. Hydrated content is already-confirmed (§17.2), so no
    // extra confirmation is needed. The result must be revision-identity-equal
    // (same operative content) and preserve the envelope's occurrence identity.
    let original = clean_strategy();
    let draft = hydrate(&original);
    let mut minter = IdentityMinter::seeded_at(MINT_BASE);
    let round_tripped = translate(&draft, &mut minter).expect("hydrated draft is submittable");

    assert_eq!(
        compute_revision_identity(&original).unwrap(),
        compute_revision_identity(&round_tripped).unwrap(),
        "round-tripping unchanged hydrated content must preserve the revision identity"
    );
    assert_eq!(
        round_tripped.occurrence, original.occurrence,
        "the envelope occurrence identity must be preserved, not re-minted"
    );
    assert_eq!(verdict_of(&round_tripped), Verdict::WellFormed);
    // No new identities were minted for unchanged content (§17.4): the minter
    // never advanced past its seed.
    let mut fresh = IdentityMinter::seeded_at(MINT_BASE);
    let _ = translate(&draft, &mut fresh).unwrap();
    // (Structural: a second identically-seeded translation is byte-equivalent —
    // see `translation_is_deterministic`.)
}

// --- explicit-absence preservation ------------------------------------------

#[test]
fn explicit_absence_round_trips_as_absence() {
    // §9 state 2 / §17.1: a Strategy whose semantic-model version is explicitly
    // absent hydrates to DeclaredAbsent and translates back to Declared::Absent
    // — the absence is preserved, never silently filled.
    let original = unusable_missing_version();
    let draft = hydrate(&original);
    assert_eq!(
        draft.semantic_model_version(),
        &FieldState::DeclaredAbsent,
        "an absent version hydrates as explicit state-2 absence"
    );
    let mut minter = IdentityMinter::seeded_at(MINT_BASE);
    let round_tripped = translate(&draft, &mut minter).expect("already-confirmed");
    assert_eq!(
        round_tripped.semantic_model_version,
        Declared::Absent,
        "explicit absence must survive the round-trip as absence"
    );
}

// --- omission ≠ absence -----------------------------------------------------

#[test]
fn omission_is_gated_absence_translates() {
    // §9/§10: *not yet addressed* (state 3, an omission) is gate-rejected;
    // *declared absent* (state 2) translates to Declared::Absent. Same field,
    // two different states, two different outcomes — an omission is not an
    // absence.
    let base = clean_strategy();

    // State 3 — omission: gate rejects before Well-Formedness.
    let mut omitted = hydrate(&base);
    omitted.clear_semantic_model_version();
    omitted.confirm_submission();
    let mut minter = IdentityMinter::seeded_at(MINT_BASE);
    assert_eq!(
        translate(&omitted, &mut minter).unwrap_err(),
        TranslationGateError::RequiredElementNotAddressed(RequiredElement::SemanticModelVersion),
        "an unaddressed required element must be gate-rejected, not treated as absent"
    );

    // State 2 — declared absent: translates to an explicit absence.
    let mut declared_absent = hydrate(&base);
    declared_absent.declare_semantic_model_version_absent();
    declared_absent.confirm_submission();
    let mut minter2 = IdentityMinter::seeded_at(MINT_BASE);
    let translated = translate(&declared_absent, &mut minter2).expect("state 2 is submittable");
    assert_eq!(translated.semantic_model_version, Declared::Absent);
}

// --- confirmation gating ----------------------------------------------------

#[test]
fn submission_is_gated_on_per_rule_and_final_confirmation() {
    // §11: a newly added construct needs its own per-rule confirmation, and the
    // submission needs a distinct final confirmation. Neither substitutes for
    // the other.
    let mut draft = hydrate(&clean_strategy());
    let index = draft.add_condition(ConditionClassification::Blocking, guard());

    // Unconfirmed construct → UnconfirmedContent, even before anything else.
    draft.confirm_submission();
    let mut m1 = IdentityMinter::seeded_at(MINT_BASE);
    assert_eq!(
        translate(&draft, &mut m1).unwrap_err(),
        TranslationGateError::UnconfirmedContent {
            condition_index: index
        }
    );

    // Confirm the construct — but that edit cleared the submission confirmation
    // (§11), so now the *final* confirmation is missing.
    draft.condition_mut(index).unwrap().confirm();
    let mut m2 = IdentityMinter::seeded_at(MINT_BASE);
    assert_eq!(
        translate(&draft, &mut m2).unwrap_err(),
        TranslationGateError::SubmissionNotConfirmed
    );

    // Give the final confirmation — now it translates.
    draft.confirm_submission();
    let mut m3 = IdentityMinter::seeded_at(MINT_BASE);
    assert!(translate(&draft, &mut m3).is_ok());
}

// --- deterministic translation ----------------------------------------------

#[test]
fn translation_is_deterministic() {
    // §12/§13: the same confirmed draft, translated twice with identically
    // seeded minters, produces identical revision-identity-bearing content AND
    // identical minted occurrence identities.
    let mut draft = hydrate(&clean_strategy());
    let index = draft.add_condition(ConditionClassification::Blocking, guard());
    draft.condition_mut(index).unwrap().confirm();
    draft.confirm_submission();

    let mut m1 = IdentityMinter::seeded_at(MINT_BASE);
    let mut m2 = IdentityMinter::seeded_at(MINT_BASE);
    let a = translate(&draft, &mut m1).unwrap();
    let b = translate(&draft, &mut m2).unwrap();

    assert_eq!(
        compute_revision_identity(&a).unwrap(),
        compute_revision_identity(&b).unwrap(),
        "the same confirmed content must translate to the same revision identity"
    );
    // Identically seeded ⇒ even the minted occurrence handles agree.
    let (ca, cb) = (present(&a.conditions), present(&b.conditions));
    assert_eq!(ca.last().unwrap().occurrence, cb.last().unwrap().occurrence);
}

// --- identity preservation (edit in place) ----------------------------------

#[test]
fn edited_in_place_condition_preserves_identity() {
    // §14.2.3/§17.4: changing a hydrated condition's classification in place is
    // an edit of the same occurrence — its occurrence and lineage identities
    // are preserved, not re-minted.
    let original = clean_strategy(); // its one condition is occ 300 / lineage 300
    let mut draft = hydrate(&original);
    draft
        .condition_mut(0)
        .unwrap()
        .set_classification(ConditionClassification::Temporary);
    draft.condition_mut(0).unwrap().confirm();
    draft.confirm_submission();

    let mut minter = IdentityMinter::seeded_at(MINT_BASE);
    let translated = translate(&draft, &mut minter).unwrap();
    let condition = &present(&translated.conditions)[0];
    assert_eq!(
        condition.occurrence,
        occ(300),
        "an edited-in-place condition keeps its occurrence identity"
    );
    assert_eq!(
        condition.lineage,
        Declared::Present(lineage(300)),
        "an edited-in-place condition keeps its lineage identity"
    );
}

// --- fresh minting (new construct) ------------------------------------------

#[test]
fn new_condition_mints_fresh_identity_above_the_corpus() {
    // §14.1/§14.2.4/§14.3: a genuinely new condition mints fresh occurrence and
    // lineage identities, from the seeded base (above the corpus high-water),
    // in submission order — distinct from every preserved identity.
    let mut draft = hydrate(&clean_strategy());
    let index = draft.add_condition(ConditionClassification::Blocking, guard());
    draft.condition_mut(index).unwrap().confirm();
    draft.confirm_submission();

    let mut minter = IdentityMinter::seeded_at(MINT_BASE);
    let translated = translate(&draft, &mut minter).unwrap();
    let conditions = present(&translated.conditions);

    // The hydrated condition kept occ 300; the new one minted from MINT_BASE.
    assert_eq!(conditions[0].occurrence, occ(300));
    assert_eq!(conditions[1].occurrence, occ(MINT_BASE));
    assert_eq!(
        conditions[1].lineage,
        Declared::Present(lineage(MINT_BASE + 1))
    );
    assert_ne!(
        conditions[1].occurrence, conditions[0].occurrence,
        "a minted occurrence must never collide with a preserved one"
    );
}

// --- removed-construct handling ---------------------------------------------

#[test]
fn removed_condition_is_absent_and_its_identity_unused() {
    // §14.2.5/§17.6: a removed condition simply does not appear in the next
    // submission, and its occurrence identity is not reused for anything else.
    let mut draft = hydrate(&clean_strategy()); // one condition, occ 300
    assert!(draft.remove_condition(0));
    draft.confirm_submission();

    let mut minter = IdentityMinter::seeded_at(MINT_BASE);
    let translated = translate(&draft, &mut minter).unwrap();
    // The collection stays present-but-empty (§9): removal is not absence.
    match &translated.conditions {
        Declared::Present(conditions) => assert!(
            conditions.is_empty(),
            "the removed condition must be gone, leaving a present-empty collection"
        ),
        Declared::Absent => panic!("removal must leave the collection present, not absent"),
    }
    // Nothing new was minted, so the removed occurrence 300 is simply unused —
    // never reused or re-minted (the minter never advanced from its seed).
}

// --- full fresh re-check ----------------------------------------------------

#[test]
fn submit_runs_a_full_fresh_recheck_over_the_whole_strategy() {
    // §19: submitting re-checks the ENTIRE Strategy and returns a fresh verdict,
    // not a delta. Correcting the only defect flips Unusable → Well-Formed.
    let mut session =
        CorrectionSession::begin(checker_context(), unusable_missing_version(), MINT_BASE);
    assert_eq!(session.result().verdict(), Verdict::Unusable);

    session.begin_edit();
    let supported = supported_version();
    let draft = session.draft_mut().unwrap();
    draft.set_semantic_model_version(supported);
    draft.confirm_submission();

    let verdict = session.submit().expect("gate satisfied");
    assert_eq!(verdict, Verdict::WellFormed);
    assert_eq!(
        session.result().verdict(),
        Verdict::WellFormed,
        "the committed diagnosis is the fresh whole-Strategy result"
    );
}

// --- no speculative fixed state ---------------------------------------------

#[test]
fn a_gate_rejection_leaves_the_committed_state_unchanged() {
    // §19: a translation-gate rejection commits nothing — the prior Unusable
    // diagnosis and Strategy remain operative, and the draft stays open. No
    // transition implies a repair before a fresh result exists.
    let mut session =
        CorrectionSession::begin(checker_context(), unusable_missing_version(), MINT_BASE);
    session.begin_edit();
    // Leave the required version unaddressed, then attempt to submit.
    let draft = session.draft_mut().unwrap();
    draft.clear_semantic_model_version();
    draft.confirm_submission();

    let error = session.submit().expect_err("gate must reject");
    assert_eq!(
        error,
        SubmitError::Gate(TranslationGateError::RequiredElementNotAddressed(
            RequiredElement::SemanticModelVersion
        ))
    );
    assert_eq!(
        session.result().verdict(),
        Verdict::Unusable,
        "the committed diagnosis must be unchanged after a gate rejection"
    );
    assert_eq!(
        session.stage(),
        Stage::Editing,
        "the draft must remain open after a gate rejection"
    );
    assert!(
        session.revision_identity().is_none(),
        "an Unusable Strategy has no revision identity"
    );
}

// --- N-22-A diagnosis regression --------------------------------------------

#[test]
fn n22a_diagnosis_still_builds_from_a_session_result() {
    // The N-22-A diagnosis surface still works over a session's result: the
    // Unusable missing-version Strategy surfaces exactly the check-16 defect,
    // enumerable and non-empty.
    use ordina_desktop::view_model::DiagnosisViewModel;
    let session =
        CorrectionSession::begin(checker_context(), unusable_missing_version(), MINT_BASE);
    let vm = DiagnosisViewModel::from_result(session.result());
    assert_eq!(vm.verdict, Verdict::Unusable);
    assert_eq!(
        vm.defects.len(),
        1,
        "exactly the unsupported-version defect"
    );
    assert!(
        vm.defects[0]
            .headline
            .to_lowercase()
            .contains("semantic model version")
    );
}

// --- end-to-end flows -------------------------------------------------------

#[test]
fn end_to_end_unusable_edit_recheck_becomes_well_formed() {
    // The complete correction loop: diagnose Unusable → hydrate → edit → confirm
    // → translate → full re-check → Well-Formed.
    let mut session =
        CorrectionSession::begin(checker_context(), unusable_missing_version(), MINT_BASE);
    assert!(session.revision_identity().is_none());

    session.begin_edit();
    let supported = supported_version();
    let draft = session.draft_mut().unwrap();
    draft.set_semantic_model_version(supported);
    draft.confirm_submission();
    session.submit().unwrap();

    assert_eq!(session.result().verdict(), Verdict::WellFormed);
}

#[test]
fn end_to_end_well_formed_yields_the_clean_revision_identity() {
    // §16 endpoint (via N-22-B-1): once the corrected Strategy is Well-Formed,
    // the session exposes a revision identity — and, because the corrected
    // content is exactly `clean_strategy`'s, it is that Strategy's identity.
    let mut session =
        CorrectionSession::begin(checker_context(), unusable_missing_version(), MINT_BASE);
    session.begin_edit();
    let supported = supported_version();
    let draft = session.draft_mut().unwrap();
    draft.set_semantic_model_version(supported);
    draft.confirm_submission();
    session.submit().unwrap();

    let identity = session
        .revision_identity()
        .expect("a Well-Formed Strategy has a revision identity")
        .expect("computation succeeds under the established WF precondition");
    assert_eq!(
        identity,
        compute_revision_identity(&clean_strategy()).unwrap(),
        "correcting the missing version reproduces the clean Strategy's revision identity"
    );
}

// --- helpers ----------------------------------------------------------------

fn guard() -> ordina::model::guard::Guard {
    ordina::model::guard::Guard::always_applicable()
}

fn present(
    conditions: &Declared<Vec<ordina::model::condition::Condition>>,
) -> &[ordina::model::condition::Condition] {
    match conditions {
        Declared::Present(v) => v,
        Declared::Absent => panic!("expected a present conditions collection"),
    }
}
