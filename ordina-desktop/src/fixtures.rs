//! Developer-only sample `WellFormednessResult` data.
//!
//! **Not a Trader authoring path.** There is no accepted authoring-to-
//! canonical translation contract yet (N-25, OPEN), so nothing in this crate
//! can construct a `WellFormednessResult` from real Trader input. This
//! module exists solely so `main.rs` has something concrete to show while
//! N-22-B (which would need N-25) remains blocked — it mirrors the same
//! developer-fixture pattern `tests/conformance/main.rs` already uses in the
//! `ordina` crate itself.

use ordina::model::opaque::{OccurrenceId, OpaqueValue};
use ordina::result::Discriminator;
use ordina::result::{
    CheckNumber, DefectCategory, DefectCore, DefectIdentity, NotEvaluatedRecord,
    WellFormednessResult,
};

fn occ(handle: u64) -> OccurrenceId {
    OccurrenceId::new(OpaqueValue::new(handle))
}

fn defect(handle: u64, category: DefectCategory) -> DefectIdentity {
    DefectIdentity::unqualified(DefectCore::new(occ(handle), category, Discriminator::None))
}

/// A representative `Unusable` result: two independent defects, plus one
/// suppressed check blocked by the first — enough to exercise every
/// diagnosis-view requirement (§9.2, §9.5, §9.6, §9.11) at once.
#[must_use]
pub fn sample_unusable() -> WellFormednessResult {
    let missing_property = defect(1, DefectCategory::MissingRequiredProperty);
    let unresolved_reference = defect(2, DefectCategory::UnresolvedReference);
    let suppressed = NotEvaluatedRecord::new(CheckNumber::C13A, occ(3), missing_property.clone());

    WellFormednessResult::unusable(
        missing_property,
        vec![unresolved_reference],
        vec![suppressed],
    )
}
