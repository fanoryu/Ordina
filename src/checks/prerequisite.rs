//! Phase-6 Slice 6B — prerequisite evaluation-state & root-blocker
//! correlation (PBA-0137 / ER-0112).
//!
//! Slice 6B owns exactly the §5.2 prerequisite-satisfaction *decision* for one
//! `(check, construct)` unit: given a prerequisite check's already-produced
//! defect set (Slice 5 checks are pure functions over `&Strategy`, unchanged
//! here), did that prerequisite pass for the specific occurrence a downstream
//! check depends on — and if not, which existing upstream [`DefectIdentity`]
//! is the **root** blocker (PBA-0134 / ER-0109 Ruling A)?
//!
//! # What Slice 6B does **not** do
//!
//! - It does **not** run or re-implement any check's own defect-producing
//!   logic — every `PrerequisiteOutcome` is derived from a caller-supplied
//!   `&[DefectCore]` already produced by the owning check (§5.1's checks 1,
//!   2, 6, 13A, 13B, 14A, 14B, unchanged).
//! - It does **not** re-walk guards or derivation expressions to rediscover
//!   dependencies — [`crate::checks::dependency_enumeration::guard_dependencies`]
//!   / [`derivation_dependencies`](crate::checks::dependency_enumeration::derivation_dependencies)
//!   supply that, and the lineage→declaration-occurrence resolvers
//!   ([`evidence_occurrence`](crate::checks::dependency_enumeration::evidence_occurrence)
//!   / [`configuration_occurrence`](crate::checks::dependency_enumeration::configuration_occurrence))
//!   supply the Check-6 correlation anchor.
//! - It does **not** aggregate a final defect set, sort it canonically, build
//!   a [`crate::result::WellFormednessResult`], or decide a
//!   [`crate::result::Verdict`] — all Slice 6C/6D.
//! - It does **not** modify [`NotEvaluatedRecord`] — Ruling A's singular
//!   `blocking: DefectIdentity` field is used exactly as declared.
//! - It introduces **no** canonical order for not-evaluated records — Ruling
//!   B: their collection is set-membership, never ordered here.
//!
//! # Root-blocker propagation (Ruling A)
//!
//! A prerequisite unit's status as input to a downstream conjunctive gate is
//! captured by [`PrerequisiteOutcome`]: `Satisfied`, or `Failed` carrying one
//! `DefectIdentity`. That identity is never invented — it is either (a) the
//! §6.4 canonical-minimum among the prerequisite check's own defects anchored
//! on the relevant occurrence ([`outcome_for_occurrence`]), or (b), where the
//! prerequisite unit was **itself** already determined not-evaluated, the
//! **inherited** root blocker from that upstream determination
//! ([`combined_outcome`]) — never a freshly synthesized identity. Where a
//! downstream check has several independent prerequisite edges (14B: 13B plus
//! 14A on three roles), [`conjunction`] collects every `Failed` candidate and
//! selects the canonical-minimum via [`DefectIdentity::try_cmp`]
//! ([`select_canonical_minimum`]) — the same §6.4 total order `canonical_sort`
//! uses, never a new ordering.

use core::cmp::Ordering;

use crate::checks::dependency_enumeration::{
    configuration_occurrence, evidence_occurrence, guard_dependencies,
};
use crate::model::guard::Guard;
use crate::model::opaque::{LineageId, OccurrenceId};
use crate::model::strategy::Strategy;
use crate::result::identity::{DefectCore, DefectIdentity};
use crate::result::{CheckNumber, NotEvaluatedRecord};

/// Whether a prerequisite check passed cleanly for one unit, or failed —
/// carrying the failing defect's identity as the candidate blocker.
///
/// `Failed` always carries an **existing** upstream [`DefectIdentity`] — this
/// module never constructs one from scratch.
#[derive(Debug, Clone)]
pub enum PrerequisiteOutcome {
    Satisfied,
    Failed(DefectIdentity),
}

/// Whether a downstream check actually ran for one `(check, construct)` unit
/// — the Phase-6 explicit projection of the Phase-5 "local non-computability"
/// silence into a Ruling-A-shaped decision.
#[derive(Debug, Clone)]
pub enum EvaluationState {
    Evaluated,
    NotEvaluated(DefectIdentity),
}

/// The direct outcome of one prerequisite check against one occurrence: does
/// its already-produced defect set contain a defect anchored on `occurrence`?
///
/// If more than one defect shares that anchor (a real but rare shape — e.g.
/// two distinct discriminators on one occurrence), the §6.4 canonical-minimum
/// is selected via [`DefectCore`]'s own total order — never insertion order,
/// never first/last match. Independent of `defects`' storage order.
#[must_use]
pub fn outcome_for_occurrence(
    defects: &[DefectCore],
    occurrence: OccurrenceId,
) -> PrerequisiteOutcome {
    defects
        .iter()
        .filter(|defect| defect.offending() == occurrence)
        .min()
        .cloned()
        .map_or(PrerequisiteOutcome::Satisfied, |core| {
            PrerequisiteOutcome::Failed(DefectIdentity::unqualified(core))
        })
}

/// The outcome of one prerequisite **unit** as input to a downstream gate,
/// accounting for both possibilities: the prerequisite check ran for this
/// construct and either did or did not produce a defect on `occurrence`
/// ([`outcome_for_occurrence`]), or the prerequisite unit was **itself**
/// already not-evaluated — in which case its inherited root blocker is
/// reused directly (Ruling A transitive propagation), never re-derived or
/// replaced with a new identity.
#[must_use]
pub fn combined_outcome(
    upstream: &EvaluationState,
    defects_if_evaluated: &[DefectCore],
    occurrence: OccurrenceId,
) -> PrerequisiteOutcome {
    match upstream {
        EvaluationState::NotEvaluated(blocker) => PrerequisiteOutcome::Failed(blocker.clone()),
        EvaluationState::Evaluated => outcome_for_occurrence(defects_if_evaluated, occurrence),
    }
}

/// Combine multiple prerequisite outcomes under §5.2 conjunctive gating: the
/// downstream check is evaluable only if **every** outcome is `Satisfied`.
/// Where one or more failed, the conjunction fails with the canonical-minimum
/// candidate among the failures ([`select_canonical_minimum`]) — Ruling A's
/// tie-break, never an arbitrary or first-encountered pick beyond that order.
#[must_use]
pub fn conjunction(outcomes: impl IntoIterator<Item = PrerequisiteOutcome>) -> PrerequisiteOutcome {
    let candidates: Vec<DefectIdentity> = outcomes
        .into_iter()
        .filter_map(|outcome| match outcome {
            PrerequisiteOutcome::Failed(id) => Some(id),
            PrerequisiteOutcome::Satisfied => None,
        })
        .collect();
    select_canonical_minimum(candidates)
        .map_or(PrerequisiteOutcome::Satisfied, PrerequisiteOutcome::Failed)
}

/// The §6.4 canonical-minimum among a set of candidate blocking identities —
/// Ruling A's tie-break when a downstream unit has multiple independent
/// root-blocker candidates. Every Slice-1 check-layer defect identity is
/// `Unqualified` (no revision identity is applicable at the check layer), so
/// [`DefectIdentity::try_cmp`] is always defined between them here; the fold
/// keeps the first candidate over one that does not compare strictly less,
/// which — for the reachable `Unqualified`-only case — is exactly the total
/// canonical-minimum, independent of input order.
#[must_use]
fn select_canonical_minimum(candidates: Vec<DefectIdentity>) -> Option<DefectIdentity> {
    let mut candidates = candidates.into_iter();
    let mut minimum = candidates.next()?;
    for candidate in candidates {
        if matches!(candidate.try_cmp(&minimum), Some(Ordering::Less)) {
            minimum = candidate;
        }
    }
    Some(minimum)
}

/// Project a resolved [`PrerequisiteOutcome`] into the downstream check's own
/// [`EvaluationState`] for this unit.
#[must_use]
pub fn evaluation_state(outcome: PrerequisiteOutcome) -> EvaluationState {
    match outcome {
        PrerequisiteOutcome::Satisfied => EvaluationState::Evaluated,
        PrerequisiteOutcome::Failed(id) => EvaluationState::NotEvaluated(id),
    }
}

/// Check-6 prerequisite correlation for one target lineage named by a guard
/// or derivation dependency (§5.2: "check 6 passes for every evidence
/// declaration that guard references").
///
/// Resolves `lineage` to its unique declaration occurrence via the Slice-6A
/// resolver ([`evidence_occurrence`] or [`configuration_occurrence`]) and
/// checks `check6_defects` against that occurrence. If the lineage resolves
/// to **no unique** declaration (absent collection, zero, or ambiguous
/// matches), this draws no suppression conclusion of its own — matching Slice
/// 6A's module docs, this function returns `Satisfied`: there is no *known*
/// Check-6 defect to attribute, and 6A itself makes no defect/validity
/// judgement in that case either.
#[must_use]
pub fn check6_outcome_for_lineage(
    strategy: &Strategy,
    lineage: LineageId,
    is_evidence: bool,
    check6_defects: &[DefectCore],
) -> PrerequisiteOutcome {
    let resolved = if is_evidence {
        evidence_occurrence(strategy, lineage)
    } else {
        configuration_occurrence(strategy, lineage)
    };
    match resolved {
        Some(occurrence) => outcome_for_occurrence(check6_defects, occurrence),
        None => PrerequisiteOutcome::Satisfied,
    }
}

/// Check-6 prerequisite correlation for **every** evidence declaration a
/// guard structurally references (§5.2, checks 9, 11, 12's edge) —
/// conjunction of [`check6_outcome_for_lineage`] over
/// [`guard_dependencies`], which already deduplicates repeated references to
/// one lineage into a single dependency (Slice 6A: membership, not
/// multiplicity).
#[must_use]
pub fn guard_check6_outcome(
    strategy: &Strategy,
    guard: &Guard,
    check6_defects: &[DefectCore],
) -> PrerequisiteOutcome {
    conjunction(
        guard_dependencies(guard)
            .into_iter()
            .map(|lineage| check6_outcome_for_lineage(strategy, lineage, true, check6_defects)),
    )
}

/// Build the Phase-6 [`NotEvaluatedRecord`] for one single-unit
/// `(check, construct)` suppressed evaluation from its resolved
/// [`EvaluationState`] — `None` if the unit was evaluated (nothing to
/// record). `construct` is that unit's own occurrence identity (First
/// Runnable Slice Contract Revision 5 §A, PBR-0017 — occurrence-based, never
/// lineage-based). A small per-unit constructor bounded to Slice 6B;
/// aggregating these into a `WellFormednessResult`'s not-evaluated set
/// remains Slice 6C/6D.
#[must_use]
pub fn not_evaluated_record(
    check: CheckNumber,
    construct: OccurrenceId,
    state: &EvaluationState,
) -> Option<NotEvaluatedRecord> {
    match state {
        EvaluationState::Evaluated => None,
        EvaluationState::NotEvaluated(blocker) => {
            Some(NotEvaluatedRecord::new(check, construct, blocker.clone()))
        }
    }
}

/// Build the Phase-6 [`NotEvaluatedRecord`] for one pairwise `(check, {a, b})`
/// suppressed evaluation (checks 9, 11-subsumption) from its resolved
/// [`EvaluationState`] — `None` if the unit was evaluated. Canonicalizes the
/// pair via [`NotEvaluatedRecord::pairwise`] (Revision 4 §C.7's existing
/// orientation, reused verbatim) — identical regardless of `(a, b)` argument
/// order.
#[must_use]
pub fn not_evaluated_record_pairwise(
    check: CheckNumber,
    a: OccurrenceId,
    b: OccurrenceId,
    state: &EvaluationState,
) -> Option<NotEvaluatedRecord> {
    match state {
        EvaluationState::Evaluated => None,
        EvaluationState::NotEvaluated(blocker) => {
            Some(NotEvaluatedRecord::pairwise(check, a, b, blocker.clone()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::declared::Declared;
    use crate::model::evidence::EvidenceDeclaration;
    use crate::model::guard::{Atom, ComparisonOperator, Guard};
    use crate::model::literal::Literal;
    use crate::model::opaque::{DimensionSymbol, LineageId, OccurrenceId, OpaqueValue};
    use crate::numeric::ExactNumber;
    use crate::result::category::DefectCategory;
    use crate::result::discriminator::Discriminator;

    fn occ(handle: u64) -> OccurrenceId {
        OccurrenceId::new(OpaqueValue::new(handle))
    }

    fn lineage(handle: u64) -> LineageId {
        LineageId::new(OpaqueValue::new(handle))
    }

    fn dim(handle: u64) -> DimensionSymbol {
        DimensionSymbol::new(OpaqueValue::new(handle))
    }

    fn defect(occurrence: OccurrenceId, category: DefectCategory) -> DefectCore {
        DefectCore::new(occurrence, category, Discriminator::None)
    }

    fn evidence_decl(occurrence: u64, lin: u64) -> EvidenceDeclaration {
        EvidenceDeclaration::numeric(occ(occurrence), lineage(lin), dim(1))
    }

    fn strategy_with_evidence(items: Vec<EvidenceDeclaration>) -> Strategy {
        Strategy {
            evidence_declarations: Declared::Present(items),
            ..Strategy::wholly_undeclared(occ(1))
        }
    }

    // -- outcome_for_occurrence ----------------------------------------

    #[test]
    fn no_defect_on_occurrence_is_satisfied() {
        let defects = vec![defect(occ(2), DefectCategory::UnresolvedReference)];
        assert!(matches!(
            outcome_for_occurrence(&defects, occ(1)),
            PrerequisiteOutcome::Satisfied
        ));
    }

    #[test]
    fn one_defect_on_occurrence_is_the_direct_blocker() {
        let d = defect(occ(1), DefectCategory::UnresolvedReference);
        let defects = vec![d.clone()];
        match outcome_for_occurrence(&defects, occ(1)) {
            PrerequisiteOutcome::Failed(id) => {
                assert!(id.try_eq(&DefectIdentity::unqualified(d)) == Some(true));
            }
            PrerequisiteOutcome::Satisfied => panic!("expected Failed"),
        }
    }

    #[test]
    fn multiple_defects_on_one_occurrence_select_the_canonical_minimum() {
        let low = defect(occ(1), DefectCategory::UnresolvedReference);
        let high = defect(occ(1), DefectCategory::TypeMismatch);
        assert!(low.cmp(&high) == Ordering::Less);
        let forward = vec![low.clone(), high.clone()];
        let backward = vec![high, low.clone()];
        for defects in [forward, backward] {
            match outcome_for_occurrence(&defects, occ(1)) {
                PrerequisiteOutcome::Failed(id) => {
                    assert_eq!(
                        id.try_eq(&DefectIdentity::unqualified(low.clone())),
                        Some(true)
                    );
                }
                PrerequisiteOutcome::Satisfied => panic!("expected Failed"),
            }
        }
    }

    // -- combined_outcome / transitive propagation ----------------------

    #[test]
    fn evaluated_upstream_defers_to_direct_defect_lookup() {
        let d = defect(occ(5), DefectCategory::TypeMismatch);
        let defects = vec![d];
        let upstream = EvaluationState::Evaluated;
        assert!(matches!(
            combined_outcome(&upstream, &defects, occ(1)),
            PrerequisiteOutcome::Satisfied
        ));
        assert!(matches!(
            combined_outcome(&upstream, &defects, occ(5)),
            PrerequisiteOutcome::Failed(_)
        ));
    }

    #[test]
    fn not_evaluated_upstream_propagates_its_own_root_blocker_unchanged() {
        let root = DefectIdentity::unqualified(defect(occ(9), DefectCategory::UnresolvedReference));
        let upstream = EvaluationState::NotEvaluated(root.clone());
        // Defects here are irrelevant — the inherited blocker must be reused
        // verbatim, never re-derived from `defects_if_evaluated`.
        let unrelated = vec![defect(occ(1), DefectCategory::TypeMismatch)];
        match combined_outcome(&upstream, &unrelated, occ(1)) {
            PrerequisiteOutcome::Failed(id) => {
                assert_eq!(id.try_eq(&root), Some(true));
            }
            PrerequisiteOutcome::Satisfied => panic!("expected inherited Failed"),
        }
    }

    // -- conjunction / canonical-minimum among independent blockers -----

    #[test]
    fn conjunction_of_all_satisfied_is_satisfied() {
        let outcomes = vec![
            PrerequisiteOutcome::Satisfied,
            PrerequisiteOutcome::Satisfied,
        ];
        assert!(matches!(
            conjunction(outcomes),
            PrerequisiteOutcome::Satisfied
        ));
    }

    #[test]
    fn conjunction_selects_canonical_minimum_among_independent_blockers() {
        // ENTRY ultimately blocked by a Check-2 defect (lower check number),
        // STOP_LOSS carries a real Check-14A defect (higher check number) —
        // §6.4 key 1 (check number) decides.
        let entry_blocker =
            DefectIdentity::unqualified(defect(occ(10), DefectCategory::UnresolvedReference));
        let stop_loss_blocker =
            DefectIdentity::unqualified(defect(occ(11), DefectCategory::DimensionMismatch));
        assert_eq!(
            entry_blocker.try_cmp(&stop_loss_blocker),
            Some(Ordering::Less)
        );

        let forward = vec![
            PrerequisiteOutcome::Failed(entry_blocker.clone()),
            PrerequisiteOutcome::Failed(stop_loss_blocker.clone()),
            PrerequisiteOutcome::Satisfied,
        ];
        let backward = vec![
            PrerequisiteOutcome::Satisfied,
            PrerequisiteOutcome::Failed(stop_loss_blocker),
            PrerequisiteOutcome::Failed(entry_blocker.clone()),
        ];
        for outcomes in [forward, backward] {
            match conjunction(outcomes) {
                PrerequisiteOutcome::Failed(id) => {
                    assert_eq!(id.try_eq(&entry_blocker), Some(true));
                }
                PrerequisiteOutcome::Satisfied => panic!("expected Failed"),
            }
        }
    }

    // -- evaluation_state / not_evaluated_record -------------------------

    #[test]
    fn satisfied_outcome_yields_evaluated_state_and_no_record() {
        let state = evaluation_state(PrerequisiteOutcome::Satisfied);
        assert!(matches!(state, EvaluationState::Evaluated));
        assert!(not_evaluated_record(CheckNumber::C14B, occ(1), &state).is_none());
    }

    #[test]
    fn failed_outcome_yields_not_evaluated_state_and_a_record_naming_the_root_blocker() {
        let root = DefectIdentity::unqualified(defect(occ(3), DefectCategory::UnresolvedReference));
        let state = evaluation_state(PrerequisiteOutcome::Failed(root.clone()));
        let record = not_evaluated_record(CheckNumber::C14B, occ(1), &state)
            .expect("Failed outcome must yield a record");
        assert!(matches!(record.check(), CheckNumber::C14B));
        assert_eq!(record.construct(), occ(1));
        assert_eq!(record.counterpart(), None);
        assert_eq!(record.blocking().try_eq(&root), Some(true));
    }

    #[test]
    fn not_evaluated_record_pairwise_uses_canonical_orientation_and_absent_lineage_is_irrelevant() {
        // The unit identity is occurrence-based; no LineageId is consulted or
        // required, so a construct whose lineage would be absent is
        // structurally irrelevant here.
        let root = DefectIdentity::unqualified(defect(occ(9), DefectCategory::UnresolvedReference));
        let state = evaluation_state(PrerequisiteOutcome::Failed(root.clone()));
        let forward = not_evaluated_record_pairwise(CheckNumber::C9, occ(5), occ(2), &state)
            .expect("Failed outcome must yield a record");
        let backward = not_evaluated_record_pairwise(CheckNumber::C9, occ(2), occ(5), &state)
            .expect("Failed outcome must yield a record");
        assert_eq!(forward.construct(), occ(2));
        assert_eq!(forward.counterpart(), Some(occ(5)));
        assert_eq!(backward.construct(), forward.construct());
        assert_eq!(backward.counterpart(), forward.counterpart());
    }

    // -- Check-6 lineage correlation (Slice 6A integration) --------------

    #[test]
    fn guard_dependency_lineage_resolves_and_a_matching_check6_defect_blocks() {
        let strategy = strategy_with_evidence(vec![evidence_decl(200, 900)]);
        let check6_defects = vec![defect(occ(200), DefectCategory::InvalidEvidenceDeclaration)];
        let outcome = check6_outcome_for_lineage(&strategy, lineage(900), true, &check6_defects);
        assert!(matches!(outcome, PrerequisiteOutcome::Failed(_)));
    }

    #[test]
    fn unrelated_declaration_defect_does_not_block() {
        let strategy =
            strategy_with_evidence(vec![evidence_decl(200, 900), evidence_decl(201, 901)]);
        let check6_defects = vec![defect(occ(201), DefectCategory::InvalidEvidenceDeclaration)];
        let outcome = check6_outcome_for_lineage(&strategy, lineage(900), true, &check6_defects);
        assert!(matches!(outcome, PrerequisiteOutcome::Satisfied));
    }

    #[test]
    fn ambiguous_lineage_resolution_is_treated_as_satisfied_not_a_synthesized_blocker() {
        let strategy =
            strategy_with_evidence(vec![evidence_decl(200, 900), evidence_decl(201, 900)]);
        let check6_defects = vec![defect(occ(200), DefectCategory::InvalidEvidenceDeclaration)];
        let outcome = check6_outcome_for_lineage(&strategy, lineage(900), true, &check6_defects);
        assert!(matches!(outcome, PrerequisiteOutcome::Satisfied));
    }

    fn atom(lineage_handle: u64) -> Atom {
        Atom {
            evidence: lineage(lineage_handle),
            operator: ComparisonOperator::Equal,
            literal: Literal::Numeric {
                value: ExactNumber::from_integer(1),
                dimension: dim(1),
            },
        }
    }

    #[test]
    fn guard_referencing_a_blocked_declaration_is_blocked_and_duplicate_atoms_stay_one_dependency()
    {
        let strategy = strategy_with_evidence(vec![evidence_decl(200, 900)]);
        let check6_defects = vec![defect(occ(200), DefectCategory::InvalidEvidenceDeclaration)];
        // Two atoms referencing the same lineage — 6A dedups to one dependency.
        let guard = Guard {
            atoms: vec![atom(900), atom(900)],
        };
        assert_eq!(guard_dependencies(&guard).len(), 1);
        let outcome = guard_check6_outcome(&strategy, &guard, &check6_defects);
        assert!(matches!(outcome, PrerequisiteOutcome::Failed(_)));
    }

    #[test]
    fn guard_referencing_a_clean_declaration_is_satisfied() {
        let strategy = strategy_with_evidence(vec![evidence_decl(200, 900)]);
        let check6_defects: Vec<DefectCore> = Vec::new();
        let guard = Guard {
            atoms: vec![atom(900)],
        };
        let outcome = guard_check6_outcome(&strategy, &guard, &check6_defects);
        assert!(matches!(outcome, PrerequisiteOutcome::Satisfied));
    }

    // -- determinism -------------------------------------------------------

    #[test]
    fn candidate_permutation_does_not_change_the_canonical_minimum() {
        let a = DefectIdentity::unqualified(defect(occ(1), DefectCategory::UnresolvedReference));
        let b = DefectIdentity::unqualified(defect(occ(2), DefectCategory::UnresolvedReference));
        let c = DefectIdentity::unqualified(defect(occ(1), DefectCategory::TypeMismatch));
        let orderings = [
            vec![a.clone(), b.clone(), c.clone()],
            vec![c.clone(), b.clone(), a.clone()],
            vec![b, c, a.clone()],
        ];
        for candidates in orderings {
            let minimum = select_canonical_minimum(candidates).expect("non-empty");
            assert_eq!(minimum.try_eq(&a), Some(true));
        }
    }

    #[test]
    fn repeated_invocation_is_stable() {
        let strategy = strategy_with_evidence(vec![evidence_decl(200, 900)]);
        let check6_defects = vec![defect(occ(200), DefectCategory::InvalidEvidenceDeclaration)];
        let first = check6_outcome_for_lineage(&strategy, lineage(900), true, &check6_defects);
        let second = check6_outcome_for_lineage(&strategy, lineage(900), true, &check6_defects);
        assert_eq!(
            matches!(first, PrerequisiteOutcome::Failed(_)),
            matches!(second, PrerequisiteOutcome::Failed(_))
        );
    }
}
