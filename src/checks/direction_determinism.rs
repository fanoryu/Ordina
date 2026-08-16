//! Check 10 — Direction determinism (Slice Contract §5.1 check 10, §3.7).
//!
//! Every Direction rule must declare **exactly one** outcome, drawn from the
//! closed set `{BUY, SELL}` (§3.7). A **zero, multiple, or out-of-domain**
//! outcome is a single `INVALID_DIRECTION_OUTCOME` defect, **one per rule**,
//! anchored on that rule's occurrence identity (Revision 4 §C) with the `NONE`
//! discriminator (§6.4.2 — check 10 uses no category-specific discriminator).
//!
//! **Ownership (presence vs. validity).** The Direction-rule *outcome* property
//! is owned **entirely by check 10**, not check 1: the R3 §3.7 property table
//! records the outcome's absence as "owned by check 10 / check 1 emits: **No**".
//! Check 10 therefore reports the zero-outcome (`Absent`) case itself and does
//! **not** duplicate any check-1 absence defect — check 1 emits none for this
//! property. The model carries the outcome as a [`DeclaredCategory`] precisely so
//! that zero/one/multiple/out-of-domain are all representable and check 10 is not
//! vacuous.
//!
//! Check 10's own prerequisite is check 1 on the rule (§5.1 check 10); in Phase 5
//! there is no prerequisite gating (that is Phase 6). Presence of the
//! `direction_rules` collection is check 1's, so check 10 evaluates only over a
//! **present** collection — an absent collection yields no check-10 defect.
//!
//! Defect production only; deterministic; no reference, type, coverage, or
//! precedence analysis — those belong to checks 2, 6, and 8.

use crate::model::declared::{Declared, DeclaredCategory};
use crate::model::strategy::Strategy;
use crate::result::category::DefectCategory;
use crate::result::discriminator::Discriminator;
use crate::result::identity::DefectCore;

/// Check 10 over a whole Strategy. Returns a deterministic `Vec<DefectCore>`.
#[must_use]
pub fn check_10(strategy: &Strategy) -> Vec<DefectCore> {
    let mut defects = Vec::new();
    let Declared::Present(rules) = &strategy.direction_rules else {
        return defects;
    };

    for rule in rules {
        // Exactly one in-domain outcome is well-formed; the `Direction` enum is
        // closed to `{BUY, SELL}`, so `One(_)` is necessarily in-domain. Zero,
        // multiple, and out-of-domain each yield one defect on the rule. No
        // catch-all arm (ADR-0005 §7.4).
        match &rule.outcome {
            DeclaredCategory::One(_) => {}
            DeclaredCategory::Absent
            | DeclaredCategory::Multiple { .. }
            | DeclaredCategory::OutOfDomain => {
                defects.push(DefectCore::new(
                    rule.occurrence,
                    DefectCategory::InvalidDirectionOutcome,
                    Discriminator::None,
                ));
            }
        }
    }

    defects
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::direction::{Direction, DirectionRule};
    use crate::model::guard::Guard;
    use crate::model::opaque::{LineageId, OccurrenceId, OpaqueValue, PrecedenceOrdinal};

    fn occ(handle: u64) -> OccurrenceId {
        OccurrenceId::new(OpaqueValue::new(handle))
    }

    fn lineage(handle: u64) -> LineageId {
        LineageId::new(OpaqueValue::new(handle))
    }

    fn precedence(handle: u64) -> PrecedenceOrdinal {
        PrecedenceOrdinal::new(OpaqueValue::new(handle))
    }

    /// A rule with the given occurrence and outcome; guard/precedence/lineage
    /// present and irrelevant to check 10.
    fn rule_with_outcome(occ_id: u64, outcome: DeclaredCategory<Direction>) -> DirectionRule {
        DirectionRule {
            occurrence: occ(occ_id),
            lineage: Declared::Present(lineage(occ_id)),
            precedence: Declared::Present(precedence(occ_id)),
            guard: Declared::Present(Guard::always_applicable()),
            outcome,
        }
    }

    fn strategy_with(direction_rules: Declared<Vec<DirectionRule>>) -> Strategy {
        Strategy {
            occurrence: occ(1),
            lineage: Declared::Present(lineage(1000)),
            semantic_model_version: Declared::Absent,
            evidence_declarations: Declared::Present(vec![]),
            configuration_parameters: Declared::Present(vec![]),
            conditions: Declared::Present(vec![]),
            direction_rules,
            candidate_derivations: Declared::Present(vec![]),
        }
    }

    fn categories(defects: &[DefectCore]) -> Vec<DefectCategory> {
        defects.iter().map(DefectCore::category).collect()
    }

    #[test]
    fn exactly_one_outcome_is_no_defect() {
        let defects = check_10(&strategy_with(Declared::Present(vec![
            rule_with_outcome(10, DeclaredCategory::One(Direction::Buy)),
            rule_with_outcome(11, DeclaredCategory::One(Direction::Sell)),
        ])));
        assert!(defects.is_empty());
    }

    #[test]
    fn a_zero_outcome_rule_is_invalid_direction_outcome() {
        let defects = check_10(&strategy_with(Declared::Present(vec![rule_with_outcome(
            10,
            DeclaredCategory::Absent,
        )])));
        assert_eq!(
            categories(&defects),
            vec![DefectCategory::InvalidDirectionOutcome]
        );
        // Anchored on the rule occurrence, NONE discriminator.
        assert_eq!(defects[0].offending(), occ(10));
        assert!(matches!(defects[0].discriminator(), Discriminator::None));
    }

    #[test]
    fn a_multiple_outcome_rule_is_invalid_direction_outcome() {
        let defects = check_10(&strategy_with(Declared::Present(vec![rule_with_outcome(
            10,
            DeclaredCategory::multiple(Direction::Buy, Direction::Sell, vec![]),
        )])));
        assert_eq!(
            categories(&defects),
            vec![DefectCategory::InvalidDirectionOutcome]
        );
        assert_eq!(defects[0].offending(), occ(10));
    }

    #[test]
    fn an_out_of_domain_outcome_rule_is_invalid_direction_outcome() {
        let defects = check_10(&strategy_with(Declared::Present(vec![rule_with_outcome(
            10,
            DeclaredCategory::OutOfDomain,
        )])));
        assert_eq!(
            categories(&defects),
            vec![DefectCategory::InvalidDirectionOutcome]
        );
        assert_eq!(defects[0].offending(), occ(10));
    }

    #[test]
    fn one_defect_per_offending_rule_each_on_its_own_occurrence() {
        // Two invalid rules and one valid one between them ⇒ exactly two defects,
        // each anchored on its own rule occurrence.
        let defects = check_10(&strategy_with(Declared::Present(vec![
            rule_with_outcome(10, DeclaredCategory::Absent),
            rule_with_outcome(11, DeclaredCategory::One(Direction::Buy)),
            rule_with_outcome(12, DeclaredCategory::OutOfDomain),
        ])));
        assert_eq!(
            categories(&defects),
            vec![
                DefectCategory::InvalidDirectionOutcome,
                DefectCategory::InvalidDirectionOutcome,
            ]
        );
        assert_eq!(defects[0].offending(), occ(10));
        assert_eq!(defects[1].offending(), occ(12));
    }

    #[test]
    fn an_absent_direction_rules_collection_is_check_1s_not_check_10s() {
        // Presence of the collection is check 1's; check 10 stays silent, and in
        // particular does not duplicate any check-1 absence population.
        assert!(check_10(&strategy_with(Declared::Absent)).is_empty());
    }

    #[test]
    fn an_empty_collection_yields_no_outcome_defect() {
        // Zero rules ⇒ nothing to evaluate for check 10 (the empty-collection
        // coverage failure is check 8's, not check 10's).
        assert!(check_10(&strategy_with(Declared::Present(vec![]))).is_empty());
    }
}
