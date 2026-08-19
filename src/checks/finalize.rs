//! Phase-6 Slice 6D — final result assembly (PBA-0145).
//!
//! Slice 6D owns exactly one thing: turning Slice 6C's raw, internal
//! [`Phase6ExecutionState`] into the crate's actual public result type,
//! [`WellFormednessResult`] (Slice Contract §6.2).
//!
//! # What Slice 6D does
//!
//! - Wraps each of 6C's [`DefectCore`] values as an unqualified
//!   [`DefectIdentity`] (Slice-1 produces no revision qualification — §6.3
//!   (1)(2)(3) only) and applies the existing [`canonical_sort`] exactly
//!   once, using the crate's own §6.4 total order — no new sort or
//!   comparison logic is written here.
//! - Constructs [`WellFormednessResult::well_formed`] when the canonically
//!   sorted defect set is empty, or [`WellFormednessResult::unusable`]
//!   (first defect + remaining defects, in canonical order) when it is not.
//! - Carries 6C's `not_evaluated: Vec<NotEvaluatedRecord>` through unchanged
//!   — no reordering, no filtering, no reinterpretation. Ruling B is pure
//!   set-membership: the presence of not-evaluated records never by itself
//!   forces `Unusable`, only actual defects do.
//!
//! # What Slice 6D does **not** do
//!
//! - It does not run or re-run any check — every defect and every
//!   not-evaluated record already came from Slice 6C.
//! - It does not touch 6A/6B/6C's own logic.
//! - It does not invent a new order for `not_evaluated` — 6C's own push
//!   order is preserved verbatim.
//! - It does not serialize, log, or otherwise report the result.

use crate::checks::orchestration::Phase6ExecutionState;
use crate::result::{DefectIdentity, NotEvaluatedRecord, WellFormednessResult, canonical_sort};

/// Assemble Slice 6C's raw [`Phase6ExecutionState`] into the crate's public
/// [`WellFormednessResult`] (Slice Contract §6.2).
///
/// Applies the existing [`canonical_sort`] to the defect set exactly once
/// (Slice-1 defects are always unqualified — §6.3). An empty canonically
/// sorted defect set yields `WellFormed`; a non-empty one yields `Unusable`
/// with the first defect and the remaining defects, in canonical order.
/// `not_evaluated` is carried through unchanged in both branches.
#[must_use]
pub fn finalize_phase6(state: Phase6ExecutionState) -> WellFormednessResult {
    let Phase6ExecutionState {
        defects,
        not_evaluated,
    } = state;

    let mut identities: Vec<DefectIdentity> = defects
        .into_iter()
        .map(DefectIdentity::unqualified)
        .collect();

    canonical_sort(&mut identities)
        .expect("Slice-1 defects are always unqualified, so the set is always homogeneous");

    assemble(identities, not_evaluated)
}

/// Build the `WellFormed`/`Unusable` result from an already canonically
/// sorted defect set and its accompanying not-evaluated records.
fn assemble(
    mut sorted_defects: Vec<DefectIdentity>,
    not_evaluated: Vec<NotEvaluatedRecord>,
) -> WellFormednessResult {
    if sorted_defects.is_empty() {
        return WellFormednessResult::well_formed(not_evaluated);
    }
    let first_defect = sorted_defects.remove(0);
    WellFormednessResult::unusable(first_defect, sorted_defects, not_evaluated)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::opaque::{OccurrenceId, OpaqueValue};
    use crate::result::category::{CheckNumber, DefectCategory};
    use crate::result::discriminator::Discriminator;
    use crate::result::identity::DefectCore;
    use crate::result::verdict::Verdict;

    fn occ(handle: u64) -> OccurrenceId {
        OccurrenceId::new(OpaqueValue::new(handle))
    }

    fn defect(handle: u64, category: DefectCategory) -> DefectCore {
        DefectCore::new(occ(handle), category, Discriminator::None)
    }

    fn record(check: CheckNumber, construct: u64, blocker: DefectCore) -> NotEvaluatedRecord {
        NotEvaluatedRecord::new(check, occ(construct), DefectIdentity::unqualified(blocker))
    }

    fn state(
        defects: Vec<DefectCore>,
        not_evaluated: Vec<NotEvaluatedRecord>,
    ) -> Phase6ExecutionState {
        Phase6ExecutionState {
            defects,
            not_evaluated,
        }
    }

    // -- WellFormed / Unusable branch -----------------------------------

    #[test]
    fn zero_defects_yields_well_formed() {
        let result = finalize_phase6(state(Vec::new(), Vec::new()));
        assert_eq!(result.verdict(), Verdict::WellFormed);
        assert!(result.defects().is_empty());
    }

    #[test]
    fn exactly_one_defect_yields_unusable_with_that_defect() {
        let d = defect(1, DefectCategory::TypeMismatch);
        let result = finalize_phase6(state(vec![d.clone()], Vec::new()));
        assert_eq!(result.verdict(), Verdict::Unusable);
        assert_eq!(result.defects().len(), 1);
        assert_eq!(
            result.defects()[0].core(),
            &d,
            "the single defect must be present"
        );
    }

    #[test]
    fn multiple_defects_are_ordered_canonically_not_by_insertion() {
        // Insert check-16 (UnknownConstruct) before check-1
        // (MissingRequiredProperty): canonical order must reorder them.
        let insertion_order = vec![
            defect(2, DefectCategory::UnknownConstruct),
            defect(1, DefectCategory::MissingRequiredProperty),
            defect(1, DefectCategory::TypeMismatch),
        ];
        let result = finalize_phase6(state(insertion_order, Vec::new()));
        assert_eq!(result.verdict(), Verdict::Unusable);
        let categories: Vec<_> = result
            .defects()
            .iter()
            .map(|d| match d {
                DefectIdentity::Unqualified(core) => core.category(),
                DefectIdentity::Qualified { core, .. } => core.category(),
            })
            .collect();
        assert_eq!(
            categories,
            vec![
                DefectCategory::MissingRequiredProperty,
                DefectCategory::TypeMismatch,
                DefectCategory::UnknownConstruct,
            ]
        );
    }

    // -- not_evaluated carry-through --------------------------------------

    #[test]
    fn not_evaluated_is_preserved_unchanged_when_well_formed() {
        let blocker = defect(9, DefectCategory::InvalidEvidenceDeclaration);
        let r = record(CheckNumber::C12, 300, blocker);
        let result = finalize_phase6(state(Vec::new(), vec![r.clone()]));
        assert_eq!(result.verdict(), Verdict::WellFormed);
        assert_eq!(result.not_evaluated().len(), 1);
        assert_eq!(result.not_evaluated()[0].check(), r.check());
        assert_eq!(result.not_evaluated()[0].construct(), r.construct());
        assert_eq!(result.not_evaluated()[0].counterpart(), r.counterpart());
    }

    #[test]
    fn not_evaluated_is_preserved_unchanged_when_unusable() {
        let d = defect(1, DefectCategory::TypeMismatch);
        let blocker = defect(9, DefectCategory::InvalidEvidenceDeclaration);
        let r = record(CheckNumber::C12, 300, blocker);
        let result = finalize_phase6(state(vec![d], vec![r.clone()]));
        assert_eq!(result.verdict(), Verdict::Unusable);
        assert_eq!(result.not_evaluated().len(), 1);
        assert_eq!(result.not_evaluated()[0].construct(), r.construct());
    }

    #[test]
    fn not_evaluated_records_alone_do_not_force_unusable() {
        let blocker = defect(9, DefectCategory::InvalidEvidenceDeclaration);
        let r = record(CheckNumber::C14B, 1, blocker);
        let result = finalize_phase6(state(Vec::new(), vec![r]));
        assert_eq!(result.verdict(), Verdict::WellFormed);
        assert_eq!(result.not_evaluated().len(), 1);
    }

    #[test]
    fn same_semantic_defects_in_different_insertion_orders_produce_identical_final_order() {
        let a = state(
            vec![
                defect(1, DefectCategory::TypeMismatch),
                defect(2, DefectCategory::UnknownConstruct),
                defect(1, DefectCategory::MissingRequiredProperty),
            ],
            Vec::new(),
        );
        let b = state(
            vec![
                defect(1, DefectCategory::MissingRequiredProperty),
                defect(2, DefectCategory::UnknownConstruct),
                defect(1, DefectCategory::TypeMismatch),
            ],
            Vec::new(),
        );
        let ra = finalize_phase6(a);
        let rb = finalize_phase6(b);
        let cats = |r: &WellFormednessResult| -> Vec<_> {
            r.defects()
                .iter()
                .map(|d| match d {
                    DefectIdentity::Unqualified(core) => core.category(),
                    DefectIdentity::Qualified { core, .. } => core.category(),
                })
                .collect()
        };
        assert_eq!(cats(&ra), cats(&rb));
    }

    #[test]
    fn r05_single_unit_not_evaluated_record_is_preserved_unchanged() {
        let blocker = defect(9, DefectCategory::InvalidEvidenceDeclaration);
        let r = record(CheckNumber::C13A, 500, blocker);
        let result = finalize_phase6(state(Vec::new(), vec![r]));
        assert_eq!(result.not_evaluated().len(), 1);
        assert_eq!(result.not_evaluated()[0].counterpart(), None);
    }

    #[test]
    fn r05_pairwise_not_evaluated_record_is_preserved_unchanged_with_counterpart() {
        let blocker = defect(9, DefectCategory::InvalidEvidenceDeclaration);
        let r = NotEvaluatedRecord::pairwise(
            CheckNumber::C9,
            occ(400),
            occ(401),
            DefectIdentity::unqualified(blocker),
        );
        let result = finalize_phase6(state(Vec::new(), vec![r]));
        assert_eq!(result.not_evaluated().len(), 1);
        assert_eq!(result.not_evaluated()[0].construct(), occ(400));
        assert_eq!(result.not_evaluated()[0].counterpart(), Some(occ(401)));
    }

    // -- no check-function calls -------------------------------------------
    // (structural: this file never imports or calls any of `check_1` through
    // `check_16` — grep-audited in the implementation report, not
    // re-asserted at runtime since there is nothing to construct here.)
}
