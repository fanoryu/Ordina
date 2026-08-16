//! Check 15 — required provenance presence — Slice Contract §5.1 check 15.
//!
//! Check 15 owns the **absence of an operative node's lineage identity**, and
//! nothing else. Its four operative nodes are the Strategy envelope, a condition,
//! a Direction rule, and a candidate derivation (§3.3). A missing operative
//! lineage produces exactly one `MISSING_OPERATIVE_PROVENANCE` per node
//! (Revision 3 §C.1 makes this **exclusive** — check 1 reports nothing for it).
//!
//! # What check 15 does not own
//!
//! - **EvidenceDeclaration and ConfigurationParameter lineage** — these are
//!   **not** operative nodes (§3.3), so their absent lineage is **check 1's**
//!   `MISSING_REQUIRED_PROPERTY` (Revision 3 §C.3). Check 15 never reports them —
//!   no dual reporting.
//! - **Duplicate present lineage** — that is **check 2's**
//!   `DUPLICATE_LINEAGE_IDENTITY`. Check 15 concerns presence only, never
//!   uniqueness.
//!
//! # Anchor
//!
//! The missing property is the lineage itself, so each defect anchors on the
//! offending node's canonical **occurrence identity** (Revision 4 §C.4) — never a
//! synthesized lineage, the parent, or a collection position. Two lineage-less
//! operative siblings therefore stay distinct through their occurrence anchors.
//!
//! # Discriminator
//!
//! Check 15 uses the **NONE** discriminator (§6.4.2 kind 1): there is one defect
//! per node, so the occurrence anchor (§6.4 key 2) already distinguishes them and
//! no further discriminator is needed.
//!
//! This is defect production only — a `Vec<DefectCore>`, no gating, verdict, or
//! canonical sort (later orchestration).

use crate::model::condition::Condition;
use crate::model::declared::Declared;
use crate::model::derivation::CandidateDerivation;
use crate::model::direction::DirectionRule;
use crate::model::opaque::OccurrenceId;
use crate::model::strategy::Strategy;
use crate::result::category::DefectCategory;
use crate::result::discriminator::Discriminator;
use crate::result::identity::DefectCore;

/// One `MISSING_OPERATIVE_PROVENANCE` defect anchored on the offending operative
/// node's occurrence identity, with the NONE discriminator.
fn missing_provenance(occurrence: OccurrenceId) -> DefectCore {
    DefectCore::new(
        occurrence,
        DefectCategory::MissingOperativeProvenance,
        Discriminator::None,
    )
}

/// Check 15 for the Strategy envelope — an operative node (§3.3).
#[must_use]
pub fn check_envelope_provenance(strategy: &Strategy) -> Vec<DefectCore> {
    if strategy.lineage.is_absent() {
        vec![missing_provenance(strategy.occurrence)]
    } else {
        Vec::new()
    }
}

/// Check 15 for one condition — an operative node (§3.3).
#[must_use]
pub fn check_condition_provenance(condition: &Condition) -> Vec<DefectCore> {
    if condition.lineage.is_absent() {
        vec![missing_provenance(condition.occurrence)]
    } else {
        Vec::new()
    }
}

/// Check 15 for one Direction rule — an operative node (§3.3).
#[must_use]
pub fn check_direction_rule_provenance(rule: &DirectionRule) -> Vec<DefectCore> {
    if rule.lineage.is_absent() {
        vec![missing_provenance(rule.occurrence)]
    } else {
        Vec::new()
    }
}

/// Check 15 for one candidate derivation — an operative node (§3.3).
#[must_use]
pub fn check_candidate_derivation_provenance(derivation: &CandidateDerivation) -> Vec<DefectCore> {
    if derivation.lineage.is_absent() {
        vec![missing_provenance(derivation.occurrence)]
    } else {
        Vec::new()
    }
}

/// Check 15 over a whole Strategy — the envelope plus every present operative
/// child node. Evidence declarations and configuration parameters are **not**
/// operative nodes, so they are deliberately not inspected here.
#[must_use]
pub fn check_15(strategy: &Strategy) -> Vec<DefectCore> {
    let mut defects = check_envelope_provenance(strategy);

    // Exhaustive matches, no catch-all `_` arm (ADR-0005 §7.4).
    match &strategy.conditions {
        Declared::Present(items) => {
            for item in items {
                defects.extend(check_condition_provenance(item));
            }
        }
        Declared::Absent => {}
    }
    match &strategy.direction_rules {
        Declared::Present(items) => {
            for item in items {
                defects.extend(check_direction_rule_provenance(item));
            }
        }
        Declared::Absent => {}
    }
    match &strategy.candidate_derivations {
        Declared::Present(items) => {
            for item in items {
                defects.extend(check_candidate_derivation_provenance(item));
            }
        }
        Declared::Absent => {}
    }

    defects
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::condition::ConditionClassification;
    use crate::model::config::ConfigurationParameter;
    use crate::model::declared::DeclaredCategory;
    use crate::model::derivation::{CandidateRole, DerivationExpression};
    use crate::model::direction::Direction;
    use crate::model::evidence::{EvidenceDeclaration, EvidenceTypeTag};
    use crate::model::opaque::{DimensionSymbol, LineageId, OpaqueValue};
    use crate::result::identity::DefectCore;

    fn occ(handle: u64) -> OccurrenceId {
        OccurrenceId::new(OpaqueValue::new(handle))
    }

    fn lineage(handle: u64) -> LineageId {
        LineageId::new(OpaqueValue::new(handle))
    }

    /// A defect set's (occurrence, category) signature, asserting the NONE
    /// discriminator through an exhaustive match.
    fn anchors(defects: &[DefectCore]) -> Vec<OccurrenceId> {
        defects
            .iter()
            .map(|d| {
                assert_eq!(d.category(), DefectCategory::MissingOperativeProvenance);
                match d.discriminator() {
                    Discriminator::None => {}
                    Discriminator::Property(_)
                    | Discriminator::Role(_)
                    | Discriminator::Atom(_)
                    | Discriminator::NodePath(_)
                    | Discriminator::Counterpart(_)
                    | Discriminator::Lineage(_) => {
                        panic!("check 15 must use the NONE discriminator")
                    }
                }
                d.offending()
            })
            .collect()
    }

    fn condition(occurrence: OccurrenceId, lineage: Declared<LineageId>) -> Condition {
        Condition {
            occurrence,
            lineage,
            classification: DeclaredCategory::One(ConditionClassification::Blocking),
            guard: Declared::Absent,
        }
    }

    fn rule(occurrence: OccurrenceId, lineage: Declared<LineageId>) -> DirectionRule {
        DirectionRule {
            occurrence,
            lineage,
            precedence: Declared::Absent,
            guard: Declared::Absent,
            outcome: DeclaredCategory::One(Direction::Buy),
        }
    }

    fn derivation(occurrence: OccurrenceId, lin: Declared<LineageId>) -> CandidateDerivation {
        CandidateDerivation {
            occurrence,
            lineage: lin,
            role: Declared::Present(CandidateRole::Entry),
            expression: Declared::Present(DerivationExpression::EvidenceReference(lineage(1))),
        }
    }

    #[test]
    fn envelope_absent_lineage_yields_one_provenance_defect() {
        let bare = Strategy::wholly_undeclared(occ(1));
        // wholly_undeclared has absent lineage AND absent child collections, so
        // check 15 reports exactly the envelope's own provenance.
        assert_eq!(anchors(&check_envelope_provenance(&bare)), vec![occ(1)]);
    }

    #[test]
    fn each_operative_child_absent_lineage_yields_one_defect() {
        assert_eq!(
            anchors(&check_condition_provenance(&condition(
                occ(2),
                Declared::Absent
            ))),
            vec![occ(2)]
        );
        assert_eq!(
            anchors(&check_direction_rule_provenance(&rule(
                occ(3),
                Declared::Absent
            ))),
            vec![occ(3)]
        );
        assert_eq!(
            anchors(&check_candidate_derivation_provenance(&derivation(
                occ(4),
                Declared::Absent
            ))),
            vec![occ(4)]
        );
    }

    #[test]
    fn present_operative_lineage_yields_no_defect() {
        assert!(
            check_condition_provenance(&condition(occ(2), Declared::Present(lineage(9))))
                .is_empty()
        );
        assert!(
            check_direction_rule_provenance(&rule(occ(3), Declared::Present(lineage(9))))
                .is_empty()
        );
        assert!(
            check_candidate_derivation_provenance(&derivation(
                occ(4),
                Declared::Present(lineage(9))
            ))
            .is_empty()
        );
    }

    #[test]
    fn evidence_and_config_lineage_absence_is_never_check_fifteen() {
        // Non-operative nodes: their lineage absence is check 1's, not check 15's.
        // check_15 does not even traverse these collections.
        let evidence = EvidenceDeclaration {
            occurrence: occ(100),
            lineage: Declared::Absent,
            declared_type: DeclaredCategory::One(EvidenceTypeTag::Boolean),
            declared_dimension: Declared::Absent,
            declared_values: Declared::Absent,
        };
        let config = ConfigurationParameter {
            occurrence: occ(101),
            lineage: Declared::Absent,
            dimension: Declared::Present(DimensionSymbol::new(OpaqueValue::new(1))),
            lower_bound: Declared::Absent,
            upper_bound: Declared::Absent,
            adjustable: Declared::Absent,
        };
        let strategy = Strategy {
            occurrence: occ(1),
            lineage: Declared::Present(lineage(1)),
            semantic_model_version: Declared::Absent,
            evidence_declarations: Declared::Present(vec![evidence]),
            configuration_parameters: Declared::Present(vec![config]),
            conditions: Declared::Present(vec![]),
            direction_rules: Declared::Present(vec![]),
            candidate_derivations: Declared::Present(vec![]),
        };
        // Envelope lineage present, no operative children with absent lineage ⇒
        // check 15 reports nothing, despite two non-operative absent lineages.
        assert!(check_15(&strategy).is_empty());
    }

    #[test]
    fn two_lineage_less_operative_siblings_stay_distinct_by_occurrence() {
        let a = check_condition_provenance(&condition(occ(10), Declared::Absent));
        let b = check_condition_provenance(&condition(occ(11), Declared::Absent));
        assert_eq!(a[0].offending(), occ(10));
        assert_eq!(b[0].offending(), occ(11));
        assert_ne!(a[0].cmp(&b[0]), core::cmp::Ordering::Equal);
    }

    #[test]
    fn duplicate_present_lineage_is_not_a_check_fifteen_defect() {
        // Two operative nodes sharing one present lineage: check 15 (presence)
        // finds nothing — duplicate lineage is check 2's concern.
        let a = check_condition_provenance(&condition(occ(10), Declared::Present(lineage(5))));
        let b = check_condition_provenance(&condition(occ(11), Declared::Present(lineage(5))));
        assert!(a.is_empty());
        assert!(b.is_empty());
    }

    #[test]
    fn aggregate_reports_envelope_and_present_operative_children_only() {
        let strategy = Strategy {
            occurrence: occ(1),
            lineage: Declared::Absent, // envelope → one defect
            semantic_model_version: Declared::Absent,
            evidence_declarations: Declared::Absent,
            configuration_parameters: Declared::Absent,
            conditions: Declared::Present(vec![condition(occ(20), Declared::Absent)]),
            direction_rules: Declared::Present(vec![rule(occ(21), Declared::Present(lineage(9)))]),
            candidate_derivations: Declared::Present(vec![derivation(occ(22), Declared::Absent)]),
        };
        // Envelope (occ 1), condition (occ 20), derivation (occ 22); the rule has
        // present lineage so no defect.
        assert_eq!(
            anchors(&check_15(&strategy)),
            vec![occ(1), occ(20), occ(22)]
        );
    }
}
