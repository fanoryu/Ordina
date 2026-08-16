//! Check 12 — Contradictory operative rules (Slice Contract §5.1 check 12,
//! §4.10).
//!
//! Check 12 validates **condition** guards: no condition guard may be
//! **contradictory** (some referenced lineage's constraint domain empty, §4.10).
//! A contradictory condition guard can never be satisfied.
//!
//! # Population
//!
//! One `CONTRADICTORY_CONDITION_GUARD` per contradictory condition, anchored on
//! that condition's occurrence, `NONE` discriminator. Note that contradictory
//! **Direction-rule** guards are check 11's `UNREACHABLE_CONTRADICTORY_GUARD`
//! (kept disjoint): check 12 owns conditions only.
//!
//! # Prerequisites (§5.1)
//!
//! The check is scoped to the condition: it requires that condition's guard
//! present and analyzable (check 1 for the guard presence, check 2 and check 6
//! for every declaration that condition's guard references). When a condition's
//! guard is non-computable — an unresolved reference, a `TYPE_MISMATCH`, or a
//! malformed referenced declaration — check 12 is **suppressed for that
//! condition only** and emits nothing; it does not emit check 2's or check 6's
//! categories. Another valid condition is still evaluated independently.
//!
//! # Anchor (Revision 4 §C.1)
//!
//! A condition is an operative node, so an absent or duplicated lineage is
//! check 15's `MISSING_OPERATIVE_PROVENANCE` and does **not** prevent occurrence
//! anchoring — the canonical occurrence identity is always present. Check 12
//! does not take check 15 as a prerequisite.
//!
//! Defect production only; no prerequisite gating framework, verdict, or
//! canonical sorting (those are Phase 6).

use crate::model::condition::Condition;
use crate::model::declared::Declared;
use crate::model::strategy::Strategy;
use crate::result::category::DefectCategory;
use crate::result::discriminator::Discriminator;
use crate::result::identity::DefectCore;

use super::guard_analysis::{analyze_guard, is_contradictory};

/// The present condition collection, or `None` when the collection itself is
/// absent (check 1's, not check 12's).
fn present_conditions(strategy: &Strategy) -> Option<&[Condition]> {
    match &strategy.conditions {
        Declared::Present(conditions) => Some(conditions),
        Declared::Absent => None,
    }
}

/// Check 12 over a whole Strategy. Returns a deterministic `Vec<DefectCore>`.
#[must_use]
pub fn check_12(strategy: &Strategy) -> Vec<DefectCore> {
    let Some(conditions) = present_conditions(strategy) else {
        return Vec::new();
    };
    let mut defects = Vec::new();
    for condition in conditions {
        let Declared::Present(guard) = &condition.guard else {
            continue;
        };
        let Some(domains) = analyze_guard(strategy, guard) else {
            continue;
        };
        if is_contradictory(&domains) {
            defects.push(DefectCore::new(
                condition.occurrence,
                DefectCategory::ContradictoryConditionGuard,
                Discriminator::None,
            ));
        }
    }
    defects
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    use crate::model::condition::{Condition, ConditionClassification};
    use crate::model::declared::Declared;
    use crate::model::declared::DeclaredCategory;
    use crate::model::evidence::{EvidenceDeclaration, EvidenceTypeTag};
    use crate::model::guard::{Atom, ComparisonOperator, Guard};
    use crate::model::literal::Literal;
    use crate::model::opaque::{DimensionSymbol, EnumValue, LineageId, OccurrenceId, OpaqueValue};
    use crate::model::strategy::Strategy;
    use crate::numeric::ExactNumber;
    use crate::result::category::DefectCategory;
    use crate::result::discriminator::Discriminator;
    use crate::result::identity::DefectCore;

    fn occ(handle: u64) -> OccurrenceId {
        OccurrenceId::new(OpaqueValue::new(handle))
    }
    fn lineage(handle: u64) -> LineageId {
        LineageId::new(OpaqueValue::new(handle))
    }
    fn dim(handle: u64) -> DimensionSymbol {
        DimensionSymbol::new(OpaqueValue::new(handle))
    }
    fn enum_value(handle: u64) -> EnumValue {
        EnumValue::new(OpaqueValue::new(handle))
    }

    fn num_decl(lin: u64) -> EvidenceDeclaration {
        EvidenceDeclaration::numeric(occ(500 + lin), lineage(lin), dim(1))
    }
    fn bool_decl(lin: u64) -> EvidenceDeclaration {
        EvidenceDeclaration::boolean(occ(500 + lin), lineage(lin))
    }
    fn enum_decl(lin: u64, vals: &[u64]) -> EvidenceDeclaration {
        EvidenceDeclaration::enumerated(
            occ(500 + lin),
            lineage(lin),
            vals.iter().map(|v| enum_value(*v)).collect(),
        )
    }

    fn natom(lin: u64, op: ComparisonOperator, value: i64) -> Atom {
        Atom::new(
            lineage(lin),
            op,
            Literal::Numeric {
                value: ExactNumber::from_integer(value),
                dimension: dim(1),
            },
        )
    }
    fn batom(lin: u64, op: ComparisonOperator, value: bool) -> Atom {
        Atom::new(lineage(lin), op, Literal::Boolean(value))
    }
    fn eatom(lin: u64, op: ComparisonOperator, value: u64) -> Atom {
        Atom::new(lineage(lin), op, Literal::Enum(enum_value(value)))
    }

    fn condition(occ_id: u64, guard: Guard) -> Condition {
        Condition::new(
            occ(occ_id),
            lineage(occ_id),
            ConditionClassification::Blocking,
            guard,
        )
    }

    fn condition_absent_lineage(occ_id: u64, guard: Guard) -> Condition {
        Condition {
            occurrence: occ(occ_id),
            lineage: Declared::Absent,
            classification: DeclaredCategory::One(ConditionClassification::Blocking),
            guard: Declared::Present(guard),
        }
    }

    fn strategy_with_conditions(
        decls: Vec<EvidenceDeclaration>,
        conds: Vec<Condition>,
    ) -> Strategy {
        Strategy {
            occurrence: occ(1),
            lineage: Declared::Present(lineage(1000)),
            semantic_model_version: Declared::Absent,
            evidence_declarations: Declared::Present(decls),
            configuration_parameters: Declared::Present(vec![]),
            conditions: Declared::Present(conds),
            direction_rules: Declared::Present(vec![]),
            candidate_derivations: Declared::Present(vec![]),
        }
    }

    fn categories(defects: &[DefectCore]) -> Vec<DefectCategory> {
        defects.iter().map(DefectCore::category).collect()
    }

    // --- 12a: contradictory NUMERIC condition guard.
    #[test]
    fn contradictory_numeric_condition_guard() {
        // x > 10 AND x < 5.
        let c = condition(
            10,
            Guard::from_atoms(vec![
                natom(1, ComparisonOperator::Greater, 10),
                natom(1, ComparisonOperator::Less, 5),
            ]),
        );
        let defects = check_12(&strategy_with_conditions(vec![num_decl(1)], vec![c]));
        assert_eq!(defects.len(), 1);
        assert_eq!(
            defects[0].category(),
            DefectCategory::ContradictoryConditionGuard
        );
        assert_eq!(defects[0].offending(), occ(10));
        assert!(matches!(defects[0].discriminator(), Discriminator::None));
    }

    // --- 12b: contradictory BOOLEAN condition guard (= true AND = false).
    #[test]
    fn contradictory_boolean_condition_guard() {
        let c = condition(
            10,
            Guard::from_atoms(vec![
                batom(2, ComparisonOperator::Equal, true),
                batom(2, ComparisonOperator::Equal, false),
            ]),
        );
        let defects = check_12(&strategy_with_conditions(vec![bool_decl(2)], vec![c]));
        assert_eq!(defects.len(), 1);
        assert_eq!(
            defects[0].category(),
            DefectCategory::ContradictoryConditionGuard
        );
    }

    // --- 12c: contradictory ENUM condition guard (= A AND = B).
    #[test]
    fn contradictory_enum_condition_guard() {
        let c = condition(
            10,
            Guard::from_atoms(vec![
                eatom(3, ComparisonOperator::Equal, 1),
                eatom(3, ComparisonOperator::Equal, 2),
            ]),
        );
        let defects = check_12(&strategy_with_conditions(
            vec![enum_decl(3, &[1, 2, 3])],
            vec![c],
        ));
        assert_eq!(defects.len(), 1);
        assert_eq!(
            defects[0].category(),
            DefectCategory::ContradictoryConditionGuard
        );
    }

    // --- 12d: satisfiable guard -> none.
    #[test]
    fn satisfiable_condition_guard_emits_nothing() {
        let c = condition(
            10,
            Guard::from_atoms(vec![natom(1, ComparisonOperator::Greater, 5)]),
        );
        let defects = check_12(&strategy_with_conditions(vec![num_decl(1)], vec![c]));
        assert!(defects.is_empty(), "got {:?}", categories(&defects));
    }

    // --- 12e: duplicate-identical atoms do not change the result.
    #[test]
    fn duplicate_identical_atoms_do_not_alter_result() {
        // Contradictory, with duplicated atoms -> still exactly one defect.
        let contradictory = condition(
            10,
            Guard::from_atoms(vec![
                natom(1, ComparisonOperator::Greater, 10),
                natom(1, ComparisonOperator::Greater, 10),
                natom(1, ComparisonOperator::Less, 5),
                natom(1, ComparisonOperator::Less, 5),
            ]),
        );
        let defects = check_12(&strategy_with_conditions(
            vec![num_decl(1)],
            vec![contradictory],
        ));
        assert_eq!(defects.len(), 1);

        // Satisfiable, with duplicated atoms -> still zero defects.
        let satisfiable = condition(
            20,
            Guard::from_atoms(vec![
                natom(1, ComparisonOperator::Greater, 5),
                natom(1, ComparisonOperator::Greater, 5),
            ]),
        );
        let defects2 = check_12(&strategy_with_conditions(
            vec![num_decl(1)],
            vec![satisfiable],
        ));
        assert!(defects2.is_empty());
    }

    // --- 12f: TYPE_MISMATCH on one condition suppresses only that condition;
    // another valid contradictory condition is still evaluated.
    #[test]
    fn type_mismatch_suppresses_only_that_condition() {
        let malformed = condition(
            10,
            Guard::from_atoms(vec![batom(1, ComparisonOperator::Equal, true)]),
        );
        let contradictory = condition(
            20,
            Guard::from_atoms(vec![
                natom(1, ComparisonOperator::Greater, 10),
                natom(1, ComparisonOperator::Less, 5),
            ]),
        );
        // L=1 is numeric; the boolean atom is a TYPE_MISMATCH -> non-computable.
        let defects = check_12(&strategy_with_conditions(
            vec![num_decl(1)],
            vec![malformed, contradictory],
        ));
        assert_eq!(defects.len(), 1);
        assert_eq!(defects[0].offending(), occ(20));
    }

    // --- 12g: unresolved reference suppresses only that condition.
    #[test]
    fn unresolved_reference_suppresses_only_that_condition() {
        let unresolved = condition(
            10,
            Guard::from_atoms(vec![natom(999, ComparisonOperator::Greater, 5)]),
        );
        let contradictory = condition(
            20,
            Guard::from_atoms(vec![
                natom(1, ComparisonOperator::Greater, 10),
                natom(1, ComparisonOperator::Less, 5),
            ]),
        );
        let defects = check_12(&strategy_with_conditions(
            vec![num_decl(1)],
            vec![unresolved, contradictory],
        ));
        assert_eq!(defects.len(), 1);
        assert_eq!(defects[0].offending(), occ(20));
    }

    // --- 12h: invalid referenced declaration suppresses only that condition.
    #[test]
    fn invalid_declaration_suppresses_only_that_condition() {
        // Lineage 99 carries an invalid (Multiple) declared type; L=1 is valid.
        let invalid_decl = EvidenceDeclaration {
            occurrence: occ(599),
            lineage: Declared::Present(lineage(99)),
            declared_type: DeclaredCategory::Multiple {
                first: EvidenceTypeTag::Numeric,
                second: EvidenceTypeTag::Numeric,
                rest: vec![],
            },
            declared_dimension: Declared::Absent,
            declared_values: Declared::Absent,
        };
        // References the invalid declaration -> non-computable.
        let malformed = condition(
            10,
            Guard::from_atoms(vec![natom(99, ComparisonOperator::Greater, 5)]),
        );
        // References a valid numeric declaration -> contradictory, emitted.
        let contradictory = condition(
            20,
            Guard::from_atoms(vec![
                natom(1, ComparisonOperator::Greater, 10),
                natom(1, ComparisonOperator::Less, 5),
            ]),
        );
        let defects = check_12(&strategy_with_conditions(
            vec![num_decl(1), invalid_decl],
            vec![malformed, contradictory],
        ));
        assert_eq!(defects.len(), 1);
        assert_eq!(defects[0].offending(), occ(20));
    }

    // --- 12i/12j: absent lineage does not prevent occurrence anchoring.
    #[test]
    fn absent_lineage_condition_still_anchored() {
        let c = condition_absent_lineage(
            10,
            Guard::from_atoms(vec![
                natom(1, ComparisonOperator::Greater, 10),
                natom(1, ComparisonOperator::Less, 5),
            ]),
        );
        let defects = check_12(&strategy_with_conditions(vec![num_decl(1)], vec![c]));
        assert_eq!(defects.len(), 1);
        assert_eq!(defects[0].offending(), occ(10));
    }

    // --- Identity: two contradictory conditions remain distinct defects.
    #[test]
    fn two_contradictory_conditions_are_distinct() {
        let a = condition(
            10,
            Guard::from_atoms(vec![
                natom(1, ComparisonOperator::Greater, 10),
                natom(1, ComparisonOperator::Less, 5),
            ]),
        );
        let b = condition(
            20,
            Guard::from_atoms(vec![
                batom(2, ComparisonOperator::Equal, true),
                batom(2, ComparisonOperator::Equal, false),
            ]),
        );
        let defects = check_12(&strategy_with_conditions(
            vec![num_decl(1), bool_decl(2)],
            vec![a, b],
        ));
        assert_eq!(defects.len(), 2);
        assert_ne!(defects[0].cmp(&defects[1]), Ordering::Equal);
    }
}
