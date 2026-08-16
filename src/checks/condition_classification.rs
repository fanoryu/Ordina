//! Check 3 — condition-classification validity (Slice Contract §5.1 check 3,
//! §3.10, §4.8).
//!
//! Every condition must declare **exactly one** static classification, drawn from
//! the closed set `{BLOCKING, TEMPORARY}` (§3.10). A **zero, multiple, or
//! out-of-domain** classification is a single `INVALID_CONDITION_CLASSIFICATION`
//! defect, **one per condition**, anchored on that condition's occurrence identity
//! (Revision 4 §C) with the `NONE` discriminator (§6.4.2 — check 3 uses no
//! category-specific discriminator; one defect per condition, so key 2 (occurrence)
//! already distinguishes defects and NONE is reachable-safe).
//!
//! **Ownership (presence vs. validity).** Check 3 owns classification validity
//! **exclusively**: the condition model records that "a zero / multiple /
//! out-of-domain classification is check 3's", while an absent **guard** is check
//! 1's and an absent **lineage** is check 15's (a condition is an operative node).
//! Check 3 therefore reports the zero-classification (`Absent`) case itself and
//! duplicates neither check 1 nor check 15 — it reads only the classification.
//!
//! Check 3's own prerequisite is check 1 on the condition (§5.1 check 3); Phase 5
//! applies no prerequisite gating (that is Phase 6). Presence of the `conditions`
//! collection is check 1's, so check 3 evaluates only over a **present** collection.
//!
//! Defect production only; deterministic; the classification is never derived from
//! the guard, evidence, or any default (§4.8).

use crate::model::declared::{Declared, DeclaredCategory};
use crate::model::strategy::Strategy;
use crate::result::category::DefectCategory;
use crate::result::discriminator::Discriminator;
use crate::result::identity::DefectCore;

/// Check 3 over a whole Strategy. Returns a deterministic `Vec<DefectCore>`.
#[must_use]
pub fn check_3(strategy: &Strategy) -> Vec<DefectCore> {
    let mut defects = Vec::new();
    let Declared::Present(conditions) = &strategy.conditions else {
        return defects;
    };

    for condition in conditions {
        // Exactly one classification is well-formed. Zero, multiple, and
        // out-of-domain each yield one defect on the condition. No catch-all arm
        // (ADR-0005 §7.4).
        match &condition.classification {
            DeclaredCategory::One(_) => {}
            DeclaredCategory::Absent
            | DeclaredCategory::Multiple { .. }
            | DeclaredCategory::OutOfDomain => {
                defects.push(DefectCore::new(
                    condition.occurrence,
                    DefectCategory::InvalidConditionClassification,
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
    use crate::model::condition::{Condition, ConditionClassification};
    use crate::model::guard::Guard;
    use crate::model::opaque::{LineageId, OccurrenceId, OpaqueValue};

    fn occ(handle: u64) -> OccurrenceId {
        OccurrenceId::new(OpaqueValue::new(handle))
    }

    fn lineage(handle: u64) -> LineageId {
        LineageId::new(OpaqueValue::new(handle))
    }

    /// A condition with the given occurrence and classification; guard/lineage
    /// present and irrelevant to check 3.
    fn condition_with(
        occ_id: u64,
        classification: DeclaredCategory<ConditionClassification>,
    ) -> Condition {
        Condition {
            occurrence: occ(occ_id),
            lineage: Declared::Present(lineage(occ_id)),
            classification,
            guard: Declared::Present(Guard::always_applicable()),
        }
    }

    fn strategy_with(conditions: Declared<Vec<Condition>>) -> Strategy {
        Strategy {
            occurrence: occ(1),
            lineage: Declared::Present(lineage(1000)),
            semantic_model_version: Declared::Absent,
            evidence_declarations: Declared::Present(vec![]),
            configuration_parameters: Declared::Present(vec![]),
            conditions,
            direction_rules: Declared::Present(vec![]),
            candidate_derivations: Declared::Present(vec![]),
        }
    }

    fn categories(defects: &[DefectCore]) -> Vec<DefectCategory> {
        defects.iter().map(DefectCore::category).collect()
    }

    #[test]
    fn a_valid_classification_is_no_defect() {
        let defects = check_3(&strategy_with(Declared::Present(vec![
            condition_with(10, DeclaredCategory::One(ConditionClassification::Blocking)),
            condition_with(
                11,
                DeclaredCategory::One(ConditionClassification::Temporary),
            ),
        ])));
        assert!(defects.is_empty());
    }

    #[test]
    fn a_zero_classification_is_invalid_condition_classification() {
        let defects = check_3(&strategy_with(Declared::Present(vec![condition_with(
            10,
            DeclaredCategory::Absent,
        )])));
        assert_eq!(
            categories(&defects),
            vec![DefectCategory::InvalidConditionClassification]
        );
        // Anchored on the condition occurrence, NONE discriminator.
        assert_eq!(defects[0].offending(), occ(10));
        assert!(matches!(defects[0].discriminator(), Discriminator::None));
    }

    #[test]
    fn a_multiple_classification_is_invalid_condition_classification() {
        let defects = check_3(&strategy_with(Declared::Present(vec![condition_with(
            10,
            DeclaredCategory::multiple(
                ConditionClassification::Blocking,
                ConditionClassification::Temporary,
                vec![],
            ),
        )])));
        assert_eq!(
            categories(&defects),
            vec![DefectCategory::InvalidConditionClassification]
        );
        assert_eq!(defects[0].offending(), occ(10));
    }

    #[test]
    fn an_out_of_domain_classification_is_invalid_condition_classification() {
        let defects = check_3(&strategy_with(Declared::Present(vec![condition_with(
            10,
            DeclaredCategory::OutOfDomain,
        )])));
        assert_eq!(
            categories(&defects),
            vec![DefectCategory::InvalidConditionClassification]
        );
        assert_eq!(defects[0].offending(), occ(10));
    }

    #[test]
    fn a_missing_guard_does_not_become_a_check_3_defect() {
        // Absent guard is check 1's; the classification is valid, so check 3 stays
        // silent even though the guard is missing.
        let condition = Condition {
            occurrence: occ(10),
            lineage: Declared::Present(lineage(10)),
            classification: DeclaredCategory::One(ConditionClassification::Blocking),
            guard: Declared::Absent,
        };
        assert!(check_3(&strategy_with(Declared::Present(vec![condition]))).is_empty());
    }

    #[test]
    fn an_absent_lineage_condition_is_still_classification_anchored() {
        // Absent lineage is check 15's; the occurrence still anchors the check-3
        // defect for an invalid classification.
        let condition = Condition {
            occurrence: occ(10),
            lineage: Declared::Absent,
            classification: DeclaredCategory::Absent,
            guard: Declared::Present(Guard::always_applicable()),
        };
        let defects = check_3(&strategy_with(Declared::Present(vec![condition])));
        assert_eq!(
            categories(&defects),
            vec![DefectCategory::InvalidConditionClassification]
        );
        assert_eq!(defects[0].offending(), occ(10));
    }

    #[test]
    fn one_defect_per_offending_condition() {
        let defects = check_3(&strategy_with(Declared::Present(vec![
            condition_with(10, DeclaredCategory::Absent),
            condition_with(11, DeclaredCategory::One(ConditionClassification::Blocking)),
            condition_with(12, DeclaredCategory::OutOfDomain),
        ])));
        assert_eq!(
            categories(&defects),
            vec![
                DefectCategory::InvalidConditionClassification,
                DefectCategory::InvalidConditionClassification,
            ]
        );
        assert_eq!(defects[0].offending(), occ(10));
        assert_eq!(defects[1].offending(), occ(12));
    }

    #[test]
    fn an_absent_conditions_collection_is_check_1s_not_check_3s() {
        assert!(check_3(&strategy_with(Declared::Absent)).is_empty());
    }
}
