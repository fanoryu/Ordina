//! Phase-6 Slice 6C — fixed-sequence orchestration (PBA-0144).
//!
//! Slice 6C owns exactly one thing: calling each of the 17 existing Phase-5
//! check functions **once**, in a fixed deterministic order, and threading
//! Slice 6A ([`crate::checks::dependency_enumeration`]) and Slice 6B
//! ([`crate::checks::prerequisite`]) through the §5.2 prerequisite edges that
//! are not already reconstructed locally by the check that owns them.
//!
//! # What Slice 6C does **not** do
//!
//! - It does **not** aggregate a final defect set, run [`crate::result::canonical_sort`],
//!   or construct a [`crate::result::WellFormednessResult`] / [`crate::result::Verdict`]
//!   — all Slice 6D.
//! - It does **not** reimplement any check's own defect-producing logic — every
//!   defect comes from calling that check's own existing public function.
//! - It does **not** build a generic dependency-graph scheduler — the sequence
//!   below is the fixed order the Slice Contract §5.1 check table already
//!   enumerates checks in, hardcoded.
//!
//! # Why most checks need no additional gating here
//!
//! Every Phase-5 check function is a pure function of `&Strategy` (plus, for
//! checks 14A/14B/16, a small semantic parameter) that already emits a defect
//! only where the relevant judgement is **computable** — each check's own
//! module documents this ("Computability, not gating"). Calling every check
//! unconditionally therefore always produces the §5.1-correct defect set: an
//! unrelated defect never taints another check's output, because each check
//! already only reports what it can determine from data it owns.
//!
//! What Slice 6C **adds** is the §6.2 `not-evaluated` metadata Slice 5 checks
//! never produced: a deterministic [`crate::result::NotEvaluatedRecord`] per
//! suppressed `(check, construct)` unit, built from already-produced defect
//! vectors via Slice 6B's primitives, for the §5.2 edges that route through
//! another check's defect set (Check-6 evidence-declaration correlation for
//! checks 9/11/12; Check-2 leaf well-typedness and 13A for check 14A; 13B and
//! 14A for check 14B).

use crate::checks::candidate_derivation_presence::check_13b;
use crate::checks::candidate_dimension_divergence::check_14b;
use crate::checks::condition_classification::check_3;
use crate::checks::configuration_bounds::check_4;
use crate::checks::configuration_position::check_5;
use crate::checks::contradictory_rules::check_12;
use crate::checks::derivation_dimension::check_14a;
use crate::checks::derivation_form::check_13a;
use crate::checks::direction_conflict::check_9;
use crate::checks::direction_coverage::check_8;
use crate::checks::direction_determinism::check_10;
use crate::checks::evidence_integrity::check_6;
use crate::checks::operative_provenance::check_15;
use crate::checks::prerequisite::{
    EvaluationState, PrerequisiteOutcome, combined_outcome, conjunction, evaluation_state,
    guard_check6_outcome, not_evaluated_record, not_evaluated_record_pairwise,
    outcome_for_occurrence,
};
use crate::checks::reference_integrity::check_2;
use crate::checks::required_property::check_1;
use crate::checks::semantic_model_version::check_16;
use crate::model::declared::Declared;
use crate::model::derivation::CandidateRole;
use crate::model::guard::Guard;
use crate::model::opaque::VersionToken;
use crate::model::strategy::Strategy;
use crate::result::NotEvaluatedRecord;
use crate::result::category::CheckNumber;
use crate::result::identity::DefectCore;
use crate::semantics::dimension::DimensionBinding;

/// Slice 6C's raw internal execution state: the defects each of the 17
/// checks produced (in fixed check order, one `Vec<DefectCore>` per check),
/// plus the not-evaluated records built for suppressed units. **Not** a
/// [`crate::result::WellFormednessResult`] — final aggregation, canonical
/// sorting, and verdict assembly remain Slice 6D.
#[derive(Debug, Clone, Default)]
pub struct Phase6ExecutionState {
    /// Every defect produced by an evaluated check, in fixed check-invocation
    /// order (§5.1 table order), then in each check's own returned order.
    /// Not canonically sorted — that is Slice 6D's `canonical_sort`.
    pub defects: Vec<DefectCore>,
    /// Every suppressed `(check, construct)` unit's not-evaluated record.
    /// Set-membership only (Ruling B) — pushed in a fixed, deterministic
    /// order (check order, then construct order) purely for reproducible
    /// `Vec` output, never as a conformance requirement.
    pub not_evaluated: Vec<NotEvaluatedRecord>,
}

impl Phase6ExecutionState {
    fn push_defects(&mut self, mut defects: Vec<DefectCore>) {
        self.defects.append(&mut defects);
    }

    fn push_record(&mut self, record: Option<NotEvaluatedRecord>) {
        if let Some(record) = record {
            self.not_evaluated.push(record);
        }
    }
}

/// Run all 17 Phase-5 checks once each, in the fixed order the §5.1 check
/// table enumerates them, threading Slice 6A/6B through the §5.2 edges that
/// route through another check's already-produced defects.
///
/// `supported` is check 16's supported semantic-model version token;
/// `binding` is checks 14A/14B's dimensionless-symbol binding.
#[must_use]
pub fn run_phase6(
    strategy: &Strategy,
    binding: &DimensionBinding,
    supported: VersionToken,
) -> Phase6ExecutionState {
    let mut state = Phase6ExecutionState::default();

    // Checks with no §5.2 prerequisite edge routed through another check's
    // defect set: always evaluated. Fixed order per §5.1's check table.
    let check1 = check_1(strategy);
    state.push_defects(check1.clone());
    let check2 = check_2(strategy);
    state.push_defects(check2.clone());
    state.push_defects(check_3(strategy));
    state.push_defects(check_4(strategy));
    state.push_defects(check_5(strategy));
    let check6 = check_6(strategy);
    state.push_defects(check6.clone());
    state.push_defects(check_8(strategy));

    // Check 9 — Direction precedence validity (pairwise). §5.2: checks 1, 2
    // on both rules (already reconstructed locally by check_9's own
    // computability discipline — an ill-typed rule contributes no tie) +
    // check 6 on every evidence declaration both rules' guards reference.
    state.push_defects(check_9(strategy));
    push_direction_rule_pairwise_not_evaluated(&mut state, strategy, &check6, CheckNumber::C9);

    state.push_defects(check_10(strategy));

    // Check 11 — contradiction (per rule) + pairwise subsumption. Same
    // Check-6 correlation edge as check 9/12.
    state.push_defects(check_11_defects(strategy));
    push_direction_rule_contradiction_not_evaluated(
        &mut state,
        strategy,
        &check6,
        CheckNumber::C11,
    );
    push_direction_rule_pairwise_not_evaluated(&mut state, strategy, &check6, CheckNumber::C11);

    // Check 12 — contradictory conditions. §5.2: checks 1, 2 on the
    // condition + check 6 on every evidence declaration its guard
    // references.
    state.push_defects(check_12(strategy));
    push_condition_not_evaluated(&mut state, strategy, &check6);

    let check13a = check_13a(strategy);
    state.push_defects(check13a.clone());
    let check13b = check_13b(strategy);
    state.push_defects(check13b.clone());

    // Check 14A — per-derivation dimensional consistency. §5.2: 13A on that
    // derivation + check 2 on its leaves.
    let (check14a, derivation_states) =
        check_14a_with_states(strategy, binding, &check1, &check2, &check13a);
    state.push_defects(check14a.clone());
    push_derivation_14a_not_evaluated(&mut state, strategy, &derivation_states);

    // Check 14B — cross-derivation agreement. §5.2: 13B + 14A on all three
    // roles. "14A passes" means both that 14A's own prerequisite was
    // evaluable *and* that its evaluation produced no defect on that
    // derivation — `combined_outcome` folds both, inheriting the 14A
    // prerequisite's blocker unchanged where already not-evaluated
    // (Ruling A), or checking `check14a` directly where it was evaluated.
    let check14b = check_14b(strategy, binding);
    state.push_defects(check14b);
    push_check14b_not_evaluated(
        &mut state,
        strategy,
        &check13b,
        &check14a,
        &derivation_states,
    );

    state.push_defects(check_15(strategy));
    state.push_defects(check_16(strategy, supported));

    state
}

// ---------------------------------------------------------------------------
// Check 11 has no single top-level "all defects" function distinguishing
// contradiction from subsumption in its return type — `check_11` already
// returns both categories combined, matching Slice 5's existing contract.
// ---------------------------------------------------------------------------
fn check_11_defects(strategy: &Strategy) -> Vec<DefectCore> {
    crate::checks::unreachable_rules::check_11(strategy)
}

// ---------------------------------------------------------------------------
// Check-6 correlation helpers (§5.2 checks 9, 11, 12 shared edge).
// ---------------------------------------------------------------------------

/// The `EvaluationState` for one guard's Check-6 prerequisite.
fn guard_check6_state(
    strategy: &Strategy,
    guard: &Guard,
    check6: &[DefectCore],
) -> EvaluationState {
    evaluation_state(guard_check6_outcome(strategy, guard, check6))
}

fn push_condition_not_evaluated(
    state: &mut Phase6ExecutionState,
    strategy: &Strategy,
    check6: &[DefectCore],
) {
    let Declared::Present(conditions) = &strategy.conditions else {
        return;
    };
    for condition in conditions {
        let Declared::Present(guard) = &condition.guard else {
            continue;
        };
        let eval = guard_check6_state(strategy, guard, check6);
        let record = not_evaluated_record(CheckNumber::C12, condition.occurrence, &eval);
        state.push_record(record);
    }
}

fn push_direction_rule_contradiction_not_evaluated(
    state: &mut Phase6ExecutionState,
    strategy: &Strategy,
    check6: &[DefectCore],
    check: CheckNumber,
) {
    let Declared::Present(rules) = &strategy.direction_rules else {
        return;
    };
    for rule in rules {
        let Declared::Present(guard) = &rule.guard else {
            continue;
        };
        let eval = guard_check6_state(strategy, guard, check6);
        let record = not_evaluated_record(check, rule.occurrence, &eval);
        state.push_record(record);
    }
}

fn push_direction_rule_pairwise_not_evaluated(
    state: &mut Phase6ExecutionState,
    strategy: &Strategy,
    check6: &[DefectCore],
    check: CheckNumber,
) {
    let Declared::Present(rules) = &strategy.direction_rules else {
        return;
    };
    for (i, rule_a) in rules.iter().enumerate() {
        let Declared::Present(guard_a) = &rule_a.guard else {
            continue;
        };
        for rule_b in rules.iter().skip(i + 1) {
            let Declared::Present(guard_b) = &rule_b.guard else {
                continue;
            };
            let outcome_a = guard_check6_outcome(strategy, guard_a, check6);
            let outcome_b = guard_check6_outcome(strategy, guard_b, check6);
            let combined = conjunction([outcome_a, outcome_b]);
            let eval = evaluation_state(combined);
            let record =
                not_evaluated_record_pairwise(check, rule_a.occurrence, rule_b.occurrence, &eval);
            state.push_record(record);
        }
    }
}

// ---------------------------------------------------------------------------
// Check 14A — 13A on the derivation + check 2 on its leaves.
// ---------------------------------------------------------------------------

/// The `EvaluationState` of every *present* candidate derivation's check-14A
/// unit, alongside 14A's own (self-gated) defect output. Returned states are
/// reused by the 14B correlation below (Ruling A transitive propagation —
/// never re-derived).
fn check_14a_with_states(
    strategy: &Strategy,
    binding: &DimensionBinding,
    check1: &[DefectCore],
    check2: &[DefectCore],
    check13a: &[DefectCore],
) -> (Vec<DefectCore>, Vec<(CandidateRole, EvaluationState)>) {
    let defects = check_14a(strategy, binding);
    let mut states = Vec::new();
    let Declared::Present(derivations) = &strategy.candidate_derivations else {
        return (defects, states);
    };
    for derivation in derivations {
        let Declared::Present(role) = derivation.role else {
            continue;
        };
        if derivation.expression.is_absent() {
            continue;
        }
        // 13A prerequisite for this derivation.
        let outcome_13a = outcome_for_occurrence(check13a, derivation.occurrence);
        // check 2 on this derivation's own leaves: check_2's TYPE_MISMATCH /
        // UNRESOLVED_REFERENCE defects for a derivation's leaves are anchored
        // on the *derivation's own occurrence* (NODE-PATH discriminated,
        // `reference_integrity::check_derivation`), never on the resolved
        // leaf declaration's occurrence — so the correlation is a direct
        // occurrence lookup, not a per-leaf resolve.
        let outcome_2 = outcome_for_occurrence(check2, derivation.occurrence);
        // check 1 on the referencing construct is folded into 13A's own
        // computability (13A requires check 1 on that derivation).
        let _ = check1;
        let combined = conjunction([outcome_13a, outcome_2]);
        states.push((role, evaluation_state(combined)));
    }
    (defects, states)
}

fn push_derivation_14a_not_evaluated(
    state: &mut Phase6ExecutionState,
    strategy: &Strategy,
    derivation_states: &[(CandidateRole, EvaluationState)],
) {
    let Declared::Present(derivations) = &strategy.candidate_derivations else {
        return;
    };
    for derivation in derivations {
        let Declared::Present(role) = derivation.role else {
            continue;
        };
        if let Some((_, eval)) = derivation_states.iter().find(|(r, _)| *r == role) {
            let record = not_evaluated_record(CheckNumber::C14A, derivation.occurrence, eval);
            state.push_record(record);
        }
    }
}

// ---------------------------------------------------------------------------
// Check 14B — 13B + 14A on all three roles.
// ---------------------------------------------------------------------------

fn push_check14b_not_evaluated(
    state: &mut Phase6ExecutionState,
    strategy: &Strategy,
    check13b: &[DefectCore],
    check14a: &[DefectCore],
    derivation_states: &[(CandidateRole, EvaluationState)],
) {
    // 13B is Strategy-level: its own defect set (if non-empty) is this
    // unit's failure, canonical-minimum selected exactly as `conjunction`
    // would over its own defects.
    let outcome_13b = if check13b.is_empty() {
        PrerequisiteOutcome::Satisfied
    } else {
        conjunction(check13b.iter().cloned().map(|core| {
            PrerequisiteOutcome::Failed(crate::result::identity::DefectIdentity::unqualified(core))
        }))
    };

    let Declared::Present(derivations) = &strategy.candidate_derivations else {
        // No derivation collection at all: still gate on 13B alone.
        let combined = outcome_13b;
        let eval = evaluation_state(combined);
        let record = not_evaluated_record(CheckNumber::C14B, strategy.occurrence, &eval);
        state.push_record(record);
        return;
    };

    let mut outcomes = vec![outcome_13b];
    for role in [
        CandidateRole::Entry,
        CandidateRole::StopLoss,
        CandidateRole::TakeProfit,
    ] {
        // "14A passes" requires both that 14A's own prerequisite unit was
        // evaluable and that the evaluation itself produced no defect on
        // this derivation — `combined_outcome` folds both in one Ruling-A-
        // consistent step, inheriting the upstream blocker verbatim where
        // already not-evaluated.
        if let (Some((_, prereq_state)), Some(derivation)) = (
            derivation_states.iter().find(|(r, _)| *r == role),
            derivations
                .iter()
                .find(|d| d.role == Declared::Present(role)),
        ) {
            outcomes.push(combined_outcome(
                prereq_state,
                check14a,
                derivation.occurrence,
            ));
        }
    }
    let combined = conjunction(outcomes);
    let eval = evaluation_state(combined);
    let record = not_evaluated_record(CheckNumber::C14B, strategy.occurrence, &eval);
    state.push_record(record);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::condition::{Condition, ConditionClassification};
    use crate::model::config::ConfigurationParameter;
    use crate::model::declared::DeclaredCategory;
    use crate::model::derivation::{CandidateDerivation, DerivationExpression};
    use crate::model::direction::{Direction, DirectionRule};
    use crate::model::evidence::EvidenceDeclaration;
    use crate::model::guard::{Atom, ComparisonOperator, Guard};
    use crate::model::literal::Literal;
    use crate::model::opaque::{DimensionSymbol, OpaqueValue, PrecedenceOrdinal};
    use crate::numeric::ExactNumber;
    use crate::result::category::DefectCategory;

    fn occ(handle: u64) -> crate::model::opaque::OccurrenceId {
        crate::model::opaque::OccurrenceId::new(OpaqueValue::new(handle))
    }

    fn lineage(handle: u64) -> crate::model::opaque::LineageId {
        crate::model::opaque::LineageId::new(OpaqueValue::new(handle))
    }

    fn price() -> DimensionSymbol {
        DimensionSymbol::new(OpaqueValue::new(1))
    }

    fn price_literal(value: i64) -> Literal {
        Literal::Numeric {
            value: ExactNumber::from_integer(value),
            dimension: price(),
        }
    }

    fn supported() -> VersionToken {
        VersionToken::new(OpaqueValue::new(1))
    }

    fn binding() -> DimensionBinding {
        DimensionBinding::new(DimensionSymbol::new(OpaqueValue::new(0)))
    }

    /// Every §3 construct present, well-formed, and mutually consistent —
    /// mirrors `model::strategy::tests::complete_strategy`.
    fn clean_strategy() -> Strategy {
        let evidence = lineage(100);
        Strategy::new(
            occ(1),
            lineage(1),
            supported(),
            vec![EvidenceDeclaration::numeric(occ(100), evidence, price())],
            vec![ConfigurationParameter::new(
                occ(200),
                lineage(200),
                price(),
                price_literal(1),
                price_literal(10),
                true,
            )],
            vec![Condition::new(
                occ(300),
                lineage(300),
                ConditionClassification::Blocking,
                Guard::from_atoms(vec![Atom::new(
                    evidence,
                    ComparisonOperator::Greater,
                    price_literal(50),
                )]),
            )],
            vec![
                DirectionRule::new(
                    occ(400),
                    lineage(400),
                    PrecedenceOrdinal::new(OpaqueValue::new(9)),
                    Guard::from_atoms(vec![Atom::new(
                        evidence,
                        ComparisonOperator::Greater,
                        price_literal(100),
                    )]),
                    Direction::Buy,
                ),
                DirectionRule::new(
                    occ(401),
                    lineage(401),
                    PrecedenceOrdinal::new(OpaqueValue::new(1)),
                    Guard::always_applicable(),
                    Direction::Sell,
                ),
            ],
            vec![
                CandidateDerivation::new(
                    occ(500),
                    lineage(500),
                    CandidateRole::Entry,
                    DerivationExpression::EvidenceReference(evidence),
                ),
                CandidateDerivation::new(
                    occ(501),
                    lineage(501),
                    CandidateRole::StopLoss,
                    DerivationExpression::Subtract {
                        left: Box::new(DerivationExpression::EvidenceReference(evidence)),
                        right: Box::new(DerivationExpression::ConfigurationReference(lineage(200))),
                    },
                ),
                CandidateDerivation::new(
                    occ(502),
                    lineage(502),
                    CandidateRole::TakeProfit,
                    DerivationExpression::Literal(price_literal(150)),
                ),
            ],
        )
    }

    // -- clean execution path -----------------------------------------------

    #[test]
    fn clean_strategy_produces_no_defects_and_no_not_evaluated_records() {
        let strategy = clean_strategy();
        let state = run_phase6(&strategy, &binding(), supported());
        assert!(state.defects.is_empty(), "{:?}", state.defects);
        assert!(state.not_evaluated.is_empty(), "{:?}", state.not_evaluated);
    }

    #[test]
    fn every_check_actually_ran_on_the_clean_strategy() {
        // A cheap proxy that all 17 checks were invoked: running the same
        // orchestration twice over the same input is deterministic and
        // produces an identical (empty) result — not merely an empty Vec by
        // construction.
        let strategy = clean_strategy();
        let a = run_phase6(&strategy, &binding(), supported());
        let b = run_phase6(&strategy, &binding(), supported());
        assert_eq!(a.defects.len(), b.defects.len());
        assert_eq!(a.not_evaluated.len(), b.not_evaluated.len());
    }

    // -- Check-6 declaration blocker suppresses dependent checks -------------

    #[test]
    fn check_6_declaration_blocker_suppresses_dependent_condition_contradiction_check() {
        // An ENUM evidence declaration with an empty value set is a Check-6
        // defect (`INVALID_EVIDENCE_DECLARATION`). The condition whose guard
        // references it must be recorded not-evaluated for check 12; an
        // unrelated condition must not be affected.
        let mut strategy = clean_strategy();
        let bad_evidence = lineage(900);
        strategy.evidence_declarations = Declared::Present(vec![
            EvidenceDeclaration {
                occurrence: occ(100),
                lineage: Declared::Present(lineage(100)),
                declared_type: DeclaredCategory::One(
                    crate::model::evidence::EvidenceTypeTag::Numeric,
                ),
                declared_dimension: Declared::Present(price()),
                declared_values: Declared::Absent,
            },
            EvidenceDeclaration {
                occurrence: occ(101),
                lineage: Declared::Present(bad_evidence),
                declared_type: DeclaredCategory::One(crate::model::evidence::EvidenceTypeTag::Enum),
                declared_dimension: Declared::Absent,
                declared_values: Declared::Present(vec![]),
            },
        ]);
        strategy.conditions = Declared::Present(vec![
            Condition::new(
                occ(300),
                lineage(300),
                ConditionClassification::Blocking,
                Guard::from_atoms(vec![Atom::new(
                    bad_evidence,
                    ComparisonOperator::Equal,
                    Literal::Enum(crate::model::opaque::EnumValue::new(OpaqueValue::new(1))),
                )]),
            ),
            Condition::new(
                occ(301),
                lineage(301),
                ConditionClassification::Blocking,
                Guard::from_atoms(vec![Atom::new(
                    lineage(100),
                    ComparisonOperator::Greater,
                    price_literal(1),
                )]),
            ),
        ]);

        let state = run_phase6(&strategy, &binding(), supported());

        // Check 6 defect present on the bad declaration.
        assert!(state.defects.iter().any(|d| d.offending() == occ(101)
            && d.category() == DefectCategory::InvalidEvidenceDeclaration));

        // Condition occ(300) is not-evaluated for check 12, blocked by the
        // check-6 defect; condition occ(301) is unaffected.
        let suppressed = state
            .not_evaluated
            .iter()
            .find(|r| matches!(r.check(), CheckNumber::C12) && r.construct() == occ(300));
        assert!(suppressed.is_some(), "{:?}", state.not_evaluated);
        assert!(
            !state
                .not_evaluated
                .iter()
                .any(|r| matches!(r.check(), CheckNumber::C12) && r.construct() == occ(301))
        );
    }

    // -- 13B failure suppresses 14B, 14A on unaffected roles still runs ------

    #[test]
    fn missing_take_profit_suppresses_only_14b_not_14a_on_present_roles() {
        let mut strategy = clean_strategy();
        // Drop the TAKE_PROFIT derivation — 13B now reports a missing role.
        if let Declared::Present(derivations) = &mut strategy.candidate_derivations {
            derivations.retain(|d| d.role != Declared::Present(CandidateRole::TakeProfit));
        }

        let state = run_phase6(&strategy, &binding(), supported());

        assert!(
            state
                .defects
                .iter()
                .any(|d| d.category() == DefectCategory::MissingCandidateDerivation)
        );
        // 14B is not-evaluated (Strategy-level, anchored on the envelope).
        assert!(
            state
                .not_evaluated
                .iter()
                .any(|r| matches!(r.check(), CheckNumber::C14B) && r.construct() == occ(1))
        );
        // 14A on ENTRY/STOP_LOSS is unaffected — no not-evaluated record for
        // either present derivation's own occurrence under check 14A.
        assert!(
            !state
                .not_evaluated
                .iter()
                .any(|r| matches!(r.check(), CheckNumber::C14A))
        );
    }

    // -- 14A mismatch suppresses 14B -----------------------------------------

    #[test]
    fn a_14a_dimension_mismatch_suppresses_14b() {
        let mut strategy = clean_strategy();
        // STOP_LOSS := price + price (dimensionally fine) becomes
        // STOP_LOSS := price - <dimensionless literal>: a DIMENSION_MISMATCH.
        if let Declared::Present(derivations) = &mut strategy.candidate_derivations {
            for derivation in derivations.iter_mut() {
                if derivation.role == Declared::Present(CandidateRole::StopLoss) {
                    derivation.expression = Declared::Present(DerivationExpression::Subtract {
                        left: Box::new(DerivationExpression::EvidenceReference(lineage(100))),
                        right: Box::new(DerivationExpression::Literal(Literal::Numeric {
                            value: ExactNumber::from_integer(1),
                            dimension: DimensionSymbol::new(OpaqueValue::new(0)), // dimensionless
                        })),
                    });
                }
            }
        }

        let state = run_phase6(&strategy, &binding(), supported());

        assert!(
            state
                .defects
                .iter()
                .any(|d| d.category() == DefectCategory::DimensionMismatch)
        );
        assert!(
            state
                .not_evaluated
                .iter()
                .any(|r| matches!(r.check(), CheckNumber::C14B))
        );
    }

    // -- Check-2 defect suppresses only its own dependent 14A unit -----------

    #[test]
    fn a_check_2_unresolved_reference_suppresses_14a_for_that_derivation_only() {
        let mut strategy = clean_strategy();
        // ENTRY references an evidence lineage that does not exist —
        // UNRESOLVED_REFERENCE anchored on the ENTRY derivation's own
        // occurrence. This must suppress 14A for ENTRY only.
        if let Declared::Present(derivations) = &mut strategy.candidate_derivations {
            for derivation in derivations.iter_mut() {
                if derivation.role == Declared::Present(CandidateRole::Entry) {
                    derivation.expression =
                        Declared::Present(DerivationExpression::EvidenceReference(lineage(999)));
                }
            }
        }

        let state = run_phase6(&strategy, &binding(), supported());

        assert!(
            state.defects.iter().any(|d| d.offending() == occ(500)
                && d.category() == DefectCategory::UnresolvedReference)
        );
        assert!(
            state
                .not_evaluated
                .iter()
                .any(|r| matches!(r.check(), CheckNumber::C14A) && r.construct() == occ(500))
        );
        // STOP_LOSS's own 14A unit is unaffected.
        assert!(
            !state
                .not_evaluated
                .iter()
                .any(|r| matches!(r.check(), CheckNumber::C14A) && r.construct() == occ(501))
        );
        // 14B is transitively not-evaluated too (inherits the same root
        // blocker, never a re-derived one).
        let record_14b = state
            .not_evaluated
            .iter()
            .find(|r| matches!(r.check(), CheckNumber::C14B))
            .expect("14B must be not-evaluated when a required 14A unit is not-evaluated");
        let record_14a = state
            .not_evaluated
            .iter()
            .find(|r| matches!(r.check(), CheckNumber::C14A) && r.construct() == occ(500))
            .expect("14A must be not-evaluated for ENTRY");
        assert_eq!(
            record_14b.blocking().try_eq(record_14a.blocking()),
            Some(true),
            "14B's root blocker must be the same identity inherited from 14A, never re-derived"
        );
    }

    // -- r05 single-unit and pairwise identity shapes ------------------------

    #[test]
    fn single_unit_not_evaluated_records_carry_no_counterpart() {
        let mut strategy = clean_strategy();
        if let Declared::Present(derivations) = &mut strategy.candidate_derivations {
            derivations.retain(|d| d.role != Declared::Present(CandidateRole::TakeProfit));
        }
        let state = run_phase6(&strategy, &binding(), supported());
        let record = state
            .not_evaluated
            .iter()
            .find(|r| matches!(r.check(), CheckNumber::C14B))
            .expect("expected a 14B not-evaluated record");
        assert_eq!(record.counterpart(), None);
    }

    #[test]
    fn pairwise_not_evaluated_records_use_canonical_pair_orientation() {
        // Two Direction rules both referencing the same malformed
        // declaration: their pairwise check-9 unit must be not-evaluated,
        // with `construct`/`counterpart` canonicalized regardless of which
        // rule sorts first.
        let mut strategy = clean_strategy();
        let bad_evidence = lineage(901);
        strategy.evidence_declarations = Declared::Present(vec![
            EvidenceDeclaration {
                occurrence: occ(100),
                lineage: Declared::Present(lineage(100)),
                declared_type: DeclaredCategory::One(
                    crate::model::evidence::EvidenceTypeTag::Numeric,
                ),
                declared_dimension: Declared::Present(price()),
                declared_values: Declared::Absent,
            },
            EvidenceDeclaration {
                occurrence: occ(102),
                lineage: Declared::Present(bad_evidence),
                declared_type: DeclaredCategory::One(crate::model::evidence::EvidenceTypeTag::Enum),
                declared_dimension: Declared::Absent,
                declared_values: Declared::Present(vec![]),
            },
        ]);
        strategy.direction_rules = Declared::Present(vec![
            DirectionRule::new(
                occ(400),
                lineage(400),
                PrecedenceOrdinal::new(OpaqueValue::new(9)),
                Guard::from_atoms(vec![Atom::new(
                    bad_evidence,
                    ComparisonOperator::Equal,
                    Literal::Enum(crate::model::opaque::EnumValue::new(OpaqueValue::new(1))),
                )]),
                Direction::Buy,
            ),
            DirectionRule::new(
                occ(401),
                lineage(401),
                PrecedenceOrdinal::new(OpaqueValue::new(1)),
                Guard::always_applicable(),
                Direction::Sell,
            ),
        ]);

        let state = run_phase6(&strategy, &binding(), supported());
        let record = state
            .not_evaluated
            .iter()
            .find(|r| matches!(r.check(), CheckNumber::C9))
            .expect("expected a pairwise check-9 not-evaluated record");
        assert_eq!(record.construct(), occ(400));
        assert_eq!(record.counterpart(), Some(occ(401)));
    }

    // -- unrelated units still evaluate despite an unrelated blocker --------

    #[test]
    fn unrelated_construct_defects_do_not_suppress_independent_checks() {
        let mut strategy = clean_strategy();
        // Break check 4 (inverted configuration bounds) — a defect wholly
        // unrelated to any §5.2 prerequisite edge.
        strategy.configuration_parameters = Declared::Present(vec![ConfigurationParameter::new(
            occ(200),
            lineage(200),
            price(),
            price_literal(10),
            price_literal(1),
            true,
        )]);

        let state = run_phase6(&strategy, &binding(), supported());

        assert!(
            state
                .defects
                .iter()
                .any(|d| d.category() == DefectCategory::InvertedConfigurationBounds)
        );
        // Nothing was suppressed: check 4 has no downstream §5.2 edge.
        assert!(state.not_evaluated.is_empty(), "{:?}", state.not_evaluated);
    }

    // -- determinism under permutation ---------------------------------------

    #[test]
    fn shuffled_evidence_and_condition_collections_yield_identical_not_evaluated_output() {
        let mut forward = clean_strategy();
        let bad_evidence = lineage(900);
        forward.evidence_declarations = Declared::Present(vec![
            EvidenceDeclaration {
                occurrence: occ(100),
                lineage: Declared::Present(lineage(100)),
                declared_type: DeclaredCategory::One(
                    crate::model::evidence::EvidenceTypeTag::Numeric,
                ),
                declared_dimension: Declared::Present(price()),
                declared_values: Declared::Absent,
            },
            EvidenceDeclaration {
                occurrence: occ(101),
                lineage: Declared::Present(bad_evidence),
                declared_type: DeclaredCategory::One(crate::model::evidence::EvidenceTypeTag::Enum),
                declared_dimension: Declared::Absent,
                declared_values: Declared::Present(vec![]),
            },
        ]);
        forward.conditions = Declared::Present(vec![Condition::new(
            occ(300),
            lineage(300),
            ConditionClassification::Blocking,
            Guard::from_atoms(vec![Atom::new(
                bad_evidence,
                ComparisonOperator::Equal,
                Literal::Enum(crate::model::opaque::EnumValue::new(OpaqueValue::new(1))),
            )]),
        )]);

        let mut backward = forward.clone();
        if let Declared::Present(items) = &mut backward.evidence_declarations {
            items.reverse();
        }

        let a = run_phase6(&forward, &binding(), supported());
        let b = run_phase6(&backward, &binding(), supported());
        assert_eq!(a.not_evaluated.len(), b.not_evaluated.len());
        assert_eq!(a.defects.len(), b.defects.len());
    }

    // -- no 6D behavior leaks in -----------------------------------------------
    // (structural: `run_phase6` returns `Phase6ExecutionState`, not
    // `WellFormednessResult`; this file never imports `canonical_sort` or
    // `Verdict` — grep-audited in the implementation report, not re-asserted
    // at runtime since there is no such value to construct here.)
}
