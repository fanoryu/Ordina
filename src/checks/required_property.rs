//! Check 1 — required-property completeness — Slice Contract §5.1 check 1.
//!
//! Check 1 emits one `MISSING_REQUIRED_PROPERTY` defect per **(construct, missing
//! property)** for every property the accepted contract marks **unconditional**
//! for that construct kind, and **only** those. Ownership is the accepted
//! Revision-3 §C.3 matrix carried unchanged into Revision 4; the defect anchor is
//! the offending construct's canonical **occurrence identity** (Revision 4 §C.4),
//! so a missing property stays reportable even when the construct's lineage is
//! itself absent.
//!
//! # What check 1 does not own (and must never report)
//!
//! - **Operative-node lineage absence** — the Strategy envelope, a condition, a
//!   Direction rule and a candidate derivation are operative nodes, so their
//!   absent lineage is **check 15's** `MISSING_OPERATIVE_PROVENANCE` exclusively
//!   (Revision 3 §C.1). Check 1 reports nothing for it.
//! - **Envelope semantic-model version absence** — **check 16** exclusively
//!   (Revision 3 §C.2).
//! - **Present-but-invalid** states — an out-of-domain evidence type, a
//!   zero/multiple/out-of-domain classification or outcome, an inverted bound, an
//!   unsupported version. Those belong to checks 6/3/10/4/16. Check 1 tests
//!   **absence only**.
//! - **Type-dependent evidence properties** (dimension, value set) — **check 6**
//!   exclusively (§3.5). In particular, check 1 never inspects
//!   `declared_values` — its set/emptiness/duplicate semantics are O-1/check-6
//!   territory, still OPEN.
//!
//! # Positions are the §3 table positions, not Rust field indices
//!
//! The PROPERTY discriminator (§6.4.2 kind 2) carries the property's declared
//! position in the §3 table for the construct's kind, top to bottom. Those
//! positions are contract-fixed and are **not** renumbered because a property is
//! structurally fixed in the Rust model (a configuration parameter's declared
//! type stays position 2 even though it is unrepresentable as absent).
//!
//! # This is defect production only
//!
//! Each function returns a `Vec<DefectCore>` — empty means the construct passes
//! check 1. No prerequisite gating, not-evaluated record, verdict, revision
//! qualification, or final canonical sort is computed here (those are later
//! orchestration). Local output is deterministic and already in ascending
//! §3-position order for one construct, so it is compatible with the §6.4 order a
//! later phase applies globally.

use crate::model::condition::Condition;
use crate::model::config::ConfigurationParameter;
use crate::model::declared::Declared;
use crate::model::declared::DeclaredCategory;
use crate::model::derivation::CandidateDerivation;
use crate::model::direction::DirectionRule;
use crate::model::evidence::EvidenceDeclaration;
use crate::model::opaque::OccurrenceId;
use crate::model::strategy::Strategy;
use crate::result::category::DefectCategory;
use crate::result::discriminator::{Discriminator, PropertyPosition};
use crate::result::identity::DefectCore;

/// One `MISSING_REQUIRED_PROPERTY` defect anchored on the offending construct's
/// occurrence identity, discriminated by the property's §3 table position.
fn missing(occurrence: OccurrenceId, position: u32) -> DefectCore {
    DefectCore::new(
        occurrence,
        DefectCategory::MissingRequiredProperty,
        Discriminator::Property(PropertyPosition::new(position)),
    )
}

/// Check 1 for one evidence declaration — §3.5 positions 1 (lineage) and 2
/// (declared type) are check-1-owned; the type-dependent dimension (3) and value
/// set (4) are check 6's exclusively and are not inspected here.
#[must_use]
pub fn check_evidence_declaration(declaration: &EvidenceDeclaration) -> Vec<DefectCore> {
    let mut defects = Vec::new();
    if declaration.lineage.is_absent() {
        defects.push(missing(declaration.occurrence, 1));
    }
    // Absence of the declared type is check 1's; a present-but-out-of-domain type
    // is check 6's (§5.1.1 row 1) and is deliberately not tested here.
    if matches!(declaration.declared_type, DeclaredCategory::Absent) {
        defects.push(missing(declaration.occurrence, 2));
    }
    defects
}

/// Check 1 for one configuration parameter — §3.9 positions 1, 3, 4, 5, 6 are
/// check-1-owned. Position 2 (declared type) is NUMERIC-only and structurally
/// implied (§3.9); its absence is unrepresentable and is never manufactured.
#[must_use]
pub fn check_configuration_parameter(parameter: &ConfigurationParameter) -> Vec<DefectCore> {
    let mut defects = Vec::new();
    if parameter.lineage.is_absent() {
        defects.push(missing(parameter.occurrence, 1));
    }
    if parameter.dimension.is_absent() {
        defects.push(missing(parameter.occurrence, 3));
    }
    if parameter.lower_bound.is_absent() {
        defects.push(missing(parameter.occurrence, 4));
    }
    if parameter.upper_bound.is_absent() {
        defects.push(missing(parameter.occurrence, 5));
    }
    if parameter.adjustable.is_absent() {
        defects.push(missing(parameter.occurrence, 6));
    }
    defects
}

/// Check 1 for one condition — §3.10 position 3 (guard) is check-1-owned.
/// Position 1 (lineage) is check 15's; position 2 (classification) is check 3's.
#[must_use]
pub fn check_condition(condition: &Condition) -> Vec<DefectCore> {
    let mut defects = Vec::new();
    if condition.guard.is_absent() {
        defects.push(missing(condition.occurrence, 3));
    }
    defects
}

/// Check 1 for one Direction rule — §3.7 positions 2 (precedence) and 3 (guard)
/// are check-1-owned. Position 1 (lineage) is check 15's; position 4 (outcome) is
/// check 10's.
#[must_use]
pub fn check_direction_rule(rule: &DirectionRule) -> Vec<DefectCore> {
    let mut defects = Vec::new();
    if rule.precedence.is_absent() {
        defects.push(missing(rule.occurrence, 2));
    }
    if rule.guard.is_absent() {
        defects.push(missing(rule.occurrence, 3));
    }
    defects
}

/// Check 1 for one candidate derivation — §3.8 positions 2 (role) and 3
/// (expression) are check-1-owned. Position 1 (lineage) is check 15's.
#[must_use]
pub fn check_candidate_derivation(derivation: &CandidateDerivation) -> Vec<DefectCore> {
    let mut defects = Vec::new();
    if derivation.role.is_absent() {
        defects.push(missing(derivation.occurrence, 2));
    }
    if derivation.expression.is_absent() {
        defects.push(missing(derivation.occurrence, 3));
    }
    defects
}

/// Check 1 over a whole Strategy — the envelope's own check-1-owned properties
/// plus check 1 for every present construct occurrence.
///
/// The envelope (§3.4) owns the **absence** of its five required collections
/// (positions 3–7); positions 1 (lineage) and 2 (semantic-model version) are
/// **not** check 1's (checks 15 and 16). A **present** collection is not a
/// check-1 fault regardless of its contents — "present but empty" is whatever the
/// owning check (8, 13B, …) says in a later phase — so check 1 only inspects each
/// collection's presence, then evaluates the elements of the present ones.
///
/// This traversal is not prerequisite gating: an absent collection simply has no
/// elements to evaluate. No suppression, ordering-across-constructs, or verdict is
/// computed.
#[must_use]
pub fn check_1(strategy: &Strategy) -> Vec<DefectCore> {
    let mut defects = Vec::new();

    // Envelope positions 3–7: the five required collections' absence.
    if strategy.evidence_declarations.is_absent() {
        defects.push(missing(strategy.occurrence, 3));
    }
    if strategy.configuration_parameters.is_absent() {
        defects.push(missing(strategy.occurrence, 4));
    }
    if strategy.conditions.is_absent() {
        defects.push(missing(strategy.occurrence, 5));
    }
    if strategy.direction_rules.is_absent() {
        defects.push(missing(strategy.occurrence, 6));
    }
    if strategy.candidate_derivations.is_absent() {
        defects.push(missing(strategy.occurrence, 7));
    }

    // Each present collection's elements get their own check-1 evaluation.
    // Exhaustive matches, no catch-all `_` arm (ADR-0005 §7.4).
    match &strategy.evidence_declarations {
        Declared::Present(items) => {
            for item in items {
                defects.extend(check_evidence_declaration(item));
            }
        }
        Declared::Absent => {}
    }
    match &strategy.configuration_parameters {
        Declared::Present(items) => {
            for item in items {
                defects.extend(check_configuration_parameter(item));
            }
        }
        Declared::Absent => {}
    }
    match &strategy.conditions {
        Declared::Present(items) => {
            for item in items {
                defects.extend(check_condition(item));
            }
        }
        Declared::Absent => {}
    }
    match &strategy.direction_rules {
        Declared::Present(items) => {
            for item in items {
                defects.extend(check_direction_rule(item));
            }
        }
        Declared::Absent => {}
    }
    match &strategy.candidate_derivations {
        Declared::Present(items) => {
            for item in items {
                defects.extend(check_candidate_derivation(item));
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
    use crate::model::derivation::{CandidateRole, DerivationExpression};
    use crate::model::direction::Direction;
    use crate::model::evidence::EvidenceTypeTag;
    use crate::model::guard::Guard;
    use crate::model::literal::Literal;
    use crate::model::opaque::{
        DimensionSymbol, EnumValue, LineageId, OpaqueValue, PrecedenceOrdinal, VersionToken,
    };
    use crate::numeric::ExactNumber;
    use crate::result::discriminator::Discriminator;

    fn occ(handle: u64) -> OccurrenceId {
        OccurrenceId::new(OpaqueValue::new(handle))
    }

    fn lineage(handle: u64) -> LineageId {
        LineageId::new(OpaqueValue::new(handle))
    }

    fn dimension() -> DimensionSymbol {
        DimensionSymbol::new(OpaqueValue::new(1))
    }

    fn bound(value: i64) -> Literal {
        Literal::Numeric {
            value: ExactNumber::from_integer(value),
            dimension: dimension(),
        }
    }

    /// The (occurrence, property-position) pairs of a defect set — the two facts a
    /// check-1 defect asserts. Reads the PROPERTY discriminator back through an
    /// exhaustive match so a wrong discriminator kind fails loudly.
    fn signature(defects: &[DefectCore]) -> Vec<(OccurrenceId, u32)> {
        defects
            .iter()
            .map(|d| {
                assert_eq!(d.category(), DefectCategory::MissingRequiredProperty);
                let position = match d.discriminator() {
                    Discriminator::Property(p) => property_position(p),
                    Discriminator::None
                    | Discriminator::Role(_)
                    | Discriminator::Atom(_)
                    | Discriminator::NodePath(_)
                    | Discriminator::Counterpart(_)
                    | Discriminator::Lineage(_) => {
                        panic!("check 1 must use a PROPERTY discriminator")
                    }
                };
                (d.offending(), position)
            })
            .collect()
    }

    /// Recover a property position for assertions by comparing against known
    /// positions — `PropertyPosition` is opaque, so this brackets it via `Ord`.
    fn property_position(p: &PropertyPosition) -> u32 {
        for candidate in 1..=7 {
            if *p == PropertyPosition::new(candidate) {
                return candidate;
            }
        }
        panic!("unexpected property position")
    }

    // -- Strategy envelope --------------------------------------------------

    #[test]
    fn envelope_absent_collections_yield_positions_three_through_seven() {
        let bare = Strategy::wholly_undeclared(occ(1));
        // wholly_undeclared has all five collections absent AND lineage/version
        // absent. Check 1 must report exactly positions 3–7 on the envelope — and
        // NOT position 1 (lineage → check 15) or 2 (version → check 16).
        let defects = check_1(&bare);
        assert_eq!(
            signature(&defects),
            vec![
                (occ(1), 3),
                (occ(1), 4),
                (occ(1), 5),
                (occ(1), 6),
                (occ(1), 7),
            ]
        );
    }

    #[test]
    fn envelope_lineage_and_version_absence_are_never_check_one() {
        // An envelope whose ONLY absences are lineage and version, with all five
        // collections present (and empty). Check 1 must report nothing.
        let strategy = Strategy {
            occurrence: occ(1),
            lineage: Declared::Absent,                // → check 15
            semantic_model_version: Declared::Absent, // → check 16
            evidence_declarations: Declared::Present(vec![]),
            configuration_parameters: Declared::Present(vec![]),
            conditions: Declared::Present(vec![]),
            direction_rules: Declared::Present(vec![]),
            candidate_derivations: Declared::Present(vec![]),
        };
        assert!(check_1(&strategy).is_empty());
    }

    #[test]
    fn a_present_but_empty_collection_is_not_a_check_one_fault() {
        // "present but empty" is a different state from "absent"; only absence is
        // check 1's. (Zero Direction rules is check 8's business, later.)
        let strategy = Strategy::new(
            occ(1),
            lineage(1),
            VersionToken::new(OpaqueValue::new(1)),
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        );
        assert!(check_1(&strategy).is_empty());
    }

    // -- EvidenceDeclaration ------------------------------------------------

    #[test]
    fn evidence_missing_lineage_is_position_one() {
        let declaration = EvidenceDeclaration {
            occurrence: occ(5),
            lineage: Declared::Absent,
            declared_type: DeclaredCategory::One(EvidenceTypeTag::Boolean),
            declared_dimension: Declared::Absent,
            declared_values: Declared::Absent,
        };
        assert_eq!(
            signature(&check_evidence_declaration(&declaration)),
            vec![(occ(5), 1)]
        );
    }

    #[test]
    fn evidence_missing_declared_type_is_position_two() {
        let declaration = EvidenceDeclaration {
            occurrence: occ(5),
            lineage: Declared::Present(lineage(1)),
            declared_type: DeclaredCategory::Absent,
            declared_dimension: Declared::Absent,
            declared_values: Declared::Absent,
        };
        assert_eq!(
            signature(&check_evidence_declaration(&declaration)),
            vec![(occ(5), 2)]
        );
    }

    #[test]
    fn evidence_missing_both_yields_two_defects_on_one_occurrence() {
        // §6.3: both defects anchor on the same occurrence, separated by the
        // PROPERTY discriminator (position 1 before position 2).
        let declaration = EvidenceDeclaration {
            occurrence: occ(5),
            lineage: Declared::Absent,
            declared_type: DeclaredCategory::Absent,
            declared_dimension: Declared::Absent,
            declared_values: Declared::Absent,
        };
        assert_eq!(
            signature(&check_evidence_declaration(&declaration)),
            vec![(occ(5), 1), (occ(5), 2)]
        );
    }

    #[test]
    fn evidence_declared_values_state_does_not_affect_check_one() {
        // O-1 isolation: check 1 never inspects declared_values. Present, empty,
        // duplicate-bearing, or absent — the check-1 output is identical.
        let make = |values: Declared<Vec<EnumValue>>| EvidenceDeclaration {
            occurrence: occ(5),
            lineage: Declared::Present(lineage(1)),
            declared_type: DeclaredCategory::One(EvidenceTypeTag::Enum),
            declared_dimension: Declared::Absent,
            declared_values: values,
        };
        let ev = EnumValue::new(OpaqueValue::new(1));
        assert!(check_evidence_declaration(&make(Declared::Absent)).is_empty());
        assert!(check_evidence_declaration(&make(Declared::Present(vec![]))).is_empty());
        assert!(check_evidence_declaration(&make(Declared::Present(vec![ev, ev]))).is_empty());
    }

    #[test]
    fn evidence_out_of_domain_type_is_not_check_one() {
        // Present-but-out-of-domain declared type is check 6's, not check 1's.
        let declaration = EvidenceDeclaration {
            occurrence: occ(5),
            lineage: Declared::Present(lineage(1)),
            declared_type: DeclaredCategory::OutOfDomain,
            declared_dimension: Declared::Absent,
            declared_values: Declared::Absent,
        };
        assert!(check_evidence_declaration(&declaration).is_empty());
    }

    // -- ConfigurationParameter ---------------------------------------------

    #[test]
    fn config_missing_lineage_is_position_one() {
        let parameter = ConfigurationParameter {
            occurrence: occ(6),
            lineage: Declared::Absent,
            dimension: Declared::Present(dimension()),
            lower_bound: Declared::Present(bound(1)),
            upper_bound: Declared::Present(bound(2)),
            adjustable: Declared::Present(true),
        };
        assert_eq!(
            signature(&check_configuration_parameter(&parameter)),
            vec![(occ(6), 1)]
        );
    }

    #[test]
    fn config_missing_structure_uses_positions_three_four_five_six_never_two() {
        let parameter = ConfigurationParameter {
            occurrence: occ(6),
            lineage: Declared::Present(lineage(1)),
            dimension: Declared::Absent,
            lower_bound: Declared::Absent,
            upper_bound: Declared::Absent,
            adjustable: Declared::Absent,
        };
        let sig = signature(&check_configuration_parameter(&parameter));
        assert_eq!(
            sig,
            vec![(occ(6), 3), (occ(6), 4), (occ(6), 5), (occ(6), 6)]
        );
        // Position 2 (declared type) is never manufactured.
        assert!(!sig.iter().any(|(_, position)| *position == 2));
    }

    // -- DirectionRule ------------------------------------------------------

    #[test]
    fn direction_rule_missing_precedence_and_guard_are_positions_two_and_three() {
        let rule = DirectionRule {
            occurrence: occ(7),
            lineage: Declared::Present(lineage(1)),
            precedence: Declared::Absent,
            guard: Declared::Absent,
            outcome: DeclaredCategory::One(Direction::Buy),
        };
        assert_eq!(
            signature(&check_direction_rule(&rule)),
            vec![(occ(7), 2), (occ(7), 3)]
        );
    }

    #[test]
    fn direction_rule_missing_lineage_alone_is_not_check_one() {
        // Operative-node lineage absence is check 15's exclusively.
        let rule = DirectionRule {
            occurrence: occ(7),
            lineage: Declared::Absent,
            precedence: Declared::Present(PrecedenceOrdinal::new(OpaqueValue::new(1))),
            guard: Declared::Present(Guard::always_applicable()),
            outcome: DeclaredCategory::One(Direction::Buy),
        };
        assert!(check_direction_rule(&rule).is_empty());
    }

    #[test]
    fn direction_rule_outcome_faults_are_not_check_one() {
        // Absent/multiple/out-of-domain outcome is check 10's, not check 1's.
        let rule = DirectionRule {
            occurrence: occ(7),
            lineage: Declared::Present(lineage(1)),
            precedence: Declared::Present(PrecedenceOrdinal::new(OpaqueValue::new(1))),
            guard: Declared::Present(Guard::always_applicable()),
            outcome: DeclaredCategory::Absent,
        };
        assert!(check_direction_rule(&rule).is_empty());
    }

    // -- CandidateDerivation ------------------------------------------------

    #[test]
    fn candidate_missing_role_and_expression_are_positions_two_and_three() {
        let derivation = CandidateDerivation {
            occurrence: occ(8),
            lineage: Declared::Present(lineage(1)),
            role: Declared::Absent,
            expression: Declared::Absent,
        };
        assert_eq!(
            signature(&check_candidate_derivation(&derivation)),
            vec![(occ(8), 2), (occ(8), 3)]
        );
    }

    #[test]
    fn candidate_missing_lineage_alone_is_not_check_one() {
        let derivation = CandidateDerivation {
            occurrence: occ(8),
            lineage: Declared::Absent,
            role: Declared::Present(CandidateRole::Entry),
            expression: Declared::Present(DerivationExpression::EvidenceReference(lineage(1))),
        };
        assert!(check_candidate_derivation(&derivation).is_empty());
    }

    // -- Condition ----------------------------------------------------------

    #[test]
    fn condition_missing_guard_is_position_three() {
        let condition = Condition {
            occurrence: occ(9),
            lineage: Declared::Present(lineage(1)),
            classification: DeclaredCategory::One(ConditionClassification::Blocking),
            guard: Declared::Absent,
        };
        assert_eq!(signature(&check_condition(&condition)), vec![(occ(9), 3)]);
    }

    #[test]
    fn condition_lineage_and_classification_faults_are_not_check_one() {
        // Lineage absence → check 15; classification faults → check 3.
        let condition = Condition {
            occurrence: occ(9),
            lineage: Declared::Absent,
            classification: DeclaredCategory::Absent,
            guard: Declared::Present(Guard::always_applicable()),
        };
        assert!(check_condition(&condition).is_empty());
    }

    // -- Occurrence anchoring (the point of Revision 4) ---------------------

    #[test]
    fn check_one_lineage_defect_is_anchored_by_occurrence_when_lineage_is_absent() {
        // EvidenceDeclaration and ConfigurationParameter: missing lineage is
        // check-1-owned, and the defect anchors on the occurrence even though the
        // lineage (the very missing property) does not exist.
        let evidence = EvidenceDeclaration {
            occurrence: occ(50),
            lineage: Declared::Absent,
            declared_type: DeclaredCategory::One(EvidenceTypeTag::Boolean),
            declared_dimension: Declared::Absent,
            declared_values: Declared::Absent,
        };
        assert_eq!(
            check_evidence_declaration(&evidence)[0].offending(),
            occ(50)
        );

        let config = ConfigurationParameter {
            occurrence: occ(51),
            lineage: Declared::Absent,
            dimension: Declared::Present(dimension()),
            lower_bound: Declared::Present(bound(1)),
            upper_bound: Declared::Present(bound(2)),
            adjustable: Declared::Present(true),
        };
        assert_eq!(
            check_configuration_parameter(&config)[0].offending(),
            occ(51)
        );
    }

    #[test]
    fn duplicate_lineage_occurrences_produce_distinct_check_one_defects() {
        // Two declarations sharing one lineage, each missing its declared type.
        // Their defects are distinct because the occurrence anchors differ — the
        // old lineage anchor would have collapsed them.
        let make = |occurrence| EvidenceDeclaration {
            occurrence,
            lineage: Declared::Present(lineage(900)),
            declared_type: DeclaredCategory::Absent,
            declared_dimension: Declared::Absent,
            declared_values: Declared::Absent,
        };
        let a = check_evidence_declaration(&make(occ(10)));
        let b = check_evidence_declaration(&make(occ(11)));
        assert_eq!(a[0].offending(), occ(10));
        assert_eq!(b[0].offending(), occ(11));
        assert_ne!(a[0].cmp(&b[0]), core::cmp::Ordering::Equal);
    }

    // -- Aggregate over a Strategy ------------------------------------------

    #[test]
    fn aggregate_evaluates_present_elements_and_skips_absent_collections() {
        // An envelope with lineage/version present, one malformed evidence
        // declaration (missing type), and the other four collections absent.
        let strategy = Strategy {
            occurrence: occ(1),
            lineage: Declared::Present(lineage(1)),
            semantic_model_version: Declared::Present(VersionToken::new(OpaqueValue::new(1))),
            evidence_declarations: Declared::Present(vec![EvidenceDeclaration {
                occurrence: occ(100),
                lineage: Declared::Present(lineage(2)),
                declared_type: DeclaredCategory::Absent,
                declared_dimension: Declared::Absent,
                declared_values: Declared::Absent,
            }]),
            configuration_parameters: Declared::Absent,
            conditions: Declared::Absent,
            direction_rules: Declared::Absent,
            candidate_derivations: Declared::Absent,
        };
        // Envelope positions 4,5,6,7 (config/conditions/rules/derivations absent;
        // evidence present so NOT position 3), then the element's position 2.
        assert_eq!(
            signature(&check_1(&strategy)),
            vec![
                (occ(1), 4),
                (occ(1), 5),
                (occ(1), 6),
                (occ(1), 7),
                (occ(100), 2),
            ]
        );
    }
}
