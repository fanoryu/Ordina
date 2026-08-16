//! Check 5 — configuration structure-preservation (Slice Contract §5.1 check 5,
//! §3.9, §4.7; ADR-0003 §6.2).
//!
//! §3.9 permits a configuration-parameter reference in **exactly one position**:
//! as a leaf of a candidate-value derivation expression. Any configuration
//! reference in a guard, classification, Direction outcome, precedence ordinal,
//! evidence declaration, or envelope position is a
//! `CONFIGURATION_IN_STRUCTURAL_POSITION` defect, **one per offending reference**.
//!
//! # The reachable population under the accepted typed model (O-3)
//!
//! Most of the nominal population is **unrepresentable by construction**: the
//! typed model gives a configuration reference a home only in a derivation leaf
//! (`DerivationExpression::ConfigurationReference`), and no classification,
//! Direction outcome, precedence ordinal, evidence declaration, or envelope field
//! can hold a lineage reference at all. Exactly one reachable structural position
//! remains — a **guard atom**. A guard atom's operand is typed as an *evidence*
//! reference (§3.6), but it is a bare `LineageId`, and the lineage namespace is
//! shared across constructs; so an atom whose lineage **resolves to a declared
//! configuration parameter** is a configuration reference sitting in a guard — a
//! structural position. This is the "real population that remains through lineage
//! resolution" recorded as observation **O-3 (BOARD-NOTED, non-blocking)**. This
//! module implements **only** that reachable population; it neither amends the
//! contract nor resolves O-3 (the other structural positions stay structurally
//! impossible, exactly as before).
//!
//! # Anchor and discriminator
//!
//! The offending sub-element is a guard atom, which carries no occurrence of its
//! own, so the defect anchors on the **enclosing construct's** occurrence (the
//! condition or the Direction rule) and is discriminated by the existing
//! **`ATOM`** kind (§6.4.2) carrying `(referenced lineage, operator, literal)` —
//! the same anchor/discriminator shape check 2 uses for guard-atom faults. `ATOM`
//! is required for §6.4 identity totality when one guard holds more than one
//! offending atom; `NONE` would collide. No new discriminator is introduced.
//!
//! # Resolution and computability
//!
//! "Resolves to a declared configuration parameter" uses the same **exactly-one**
//! resolution as check 2 / §4.7: zero matches or an ambiguous (duplicate-lineage)
//! set is *not* a determinate configuration reference and yields no check-5
//! defect. This is **intrinsic predicate computability** — the check-5 predicate
//! is evaluated only where its own operand (a determinate configuration referent)
//! exists — **not** a §5.2 prerequisite: no gating, no `NotEvaluatedRecord`. Check
//! 5 never emits a check-2 category; a guard atom that resolves to a configuration
//! parameter (rather than an evidence item) is independently a check-2
//! `UNRESOLVED_REFERENCE`, but that is check 2's defect, produced and anchored by
//! check 2, never by check 5.
//!
//! Defect production only; deterministic.

use crate::model::declared::Declared;
use crate::model::guard::{Atom, Guard};
use crate::model::opaque::{LineageId, OccurrenceId};
use crate::model::strategy::Strategy;
use crate::result::category::DefectCategory;
use crate::result::discriminator::{AtomKey, Discriminator};
use crate::result::identity::DefectCore;

/// Whether the referenced lineage resolves to **exactly one** declared
/// configuration parameter (§4.7 exactly-one; zero or ambiguous ⇒ not a
/// determinate configuration reference). Present configuration collection only.
fn resolves_to_one_configuration_parameter(strategy: &Strategy, referenced: LineageId) -> bool {
    let Declared::Present(parameters) = &strategy.configuration_parameters else {
        return false;
    };
    parameters
        .iter()
        .filter(|parameter| parameter.lineage == Declared::Present(referenced))
        .count()
        == 1
}

/// The `ATOM` discriminator for one guard atom (§6.4.2 kind 4) — the same content
/// key check 2 uses; positional-independent (§4.5).
fn atom_discriminator(atom: &Atom) -> Discriminator {
    Discriminator::Atom(AtomKey::new(
        atom.evidence,
        atom.operator,
        atom.literal.clone(),
    ))
}

/// Every offending guard atom in one guard, anchored on the enclosing construct.
fn check_guard(anchor: OccurrenceId, guard: &Guard, strategy: &Strategy) -> Vec<DefectCore> {
    let mut defects = Vec::new();
    for atom in &guard.atoms {
        if resolves_to_one_configuration_parameter(strategy, atom.evidence) {
            defects.push(DefectCore::new(
                anchor,
                DefectCategory::ConfigurationInStructuralPosition,
                atom_discriminator(atom),
            ));
        }
    }
    defects
}

/// Check 5 over a whole Strategy — every present guard (conditions and Direction
/// rules). Returns a deterministic `Vec<DefectCore>`.
#[must_use]
pub fn check_5(strategy: &Strategy) -> Vec<DefectCore> {
    let mut defects = Vec::new();

    if let Declared::Present(conditions) = &strategy.conditions {
        for condition in conditions {
            if let Declared::Present(guard) = &condition.guard {
                defects.extend(check_guard(condition.occurrence, guard, strategy));
            }
        }
    }
    if let Declared::Present(rules) = &strategy.direction_rules {
        for rule in rules {
            if let Declared::Present(guard) = &rule.guard {
                defects.extend(check_guard(rule.occurrence, guard, strategy));
            }
        }
    }

    defects
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::condition::{Condition, ConditionClassification};
    use crate::model::config::ConfigurationParameter;
    use crate::model::declared::DeclaredCategory;
    use crate::model::direction::{Direction, DirectionRule};
    use crate::model::guard::ComparisonOperator;
    use crate::model::literal::Literal;
    use crate::model::opaque::{DimensionSymbol, OpaqueValue, PrecedenceOrdinal};

    fn occ(handle: u64) -> OccurrenceId {
        OccurrenceId::new(OpaqueValue::new(handle))
    }

    fn lineage(handle: u64) -> LineageId {
        LineageId::new(OpaqueValue::new(handle))
    }

    fn atom(evidence: u64) -> Atom {
        Atom::new(
            lineage(evidence),
            ComparisonOperator::Equal,
            Literal::Boolean(true),
        )
    }

    fn config_parameter(occ_id: u64, lin: u64) -> ConfigurationParameter {
        ConfigurationParameter {
            occurrence: occ(occ_id),
            lineage: Declared::Present(lineage(lin)),
            dimension: Declared::Present(DimensionSymbol::new(OpaqueValue::new(1))),
            lower_bound: Declared::Present(Literal::Numeric {
                value: crate::numeric::ExactNumber::from_integer(1),
                dimension: DimensionSymbol::new(OpaqueValue::new(1)),
            }),
            upper_bound: Declared::Present(Literal::Numeric {
                value: crate::numeric::ExactNumber::from_integer(10),
                dimension: DimensionSymbol::new(OpaqueValue::new(1)),
            }),
            adjustable: Declared::Present(true),
        }
    }

    fn condition_with_guard(occ_id: u64, guard: Guard) -> Condition {
        Condition {
            occurrence: occ(occ_id),
            lineage: Declared::Present(lineage(occ_id)),
            classification: DeclaredCategory::One(ConditionClassification::Blocking),
            guard: Declared::Present(guard),
        }
    }

    fn rule_with_guard(occ_id: u64, guard: Guard) -> DirectionRule {
        DirectionRule::new(
            occ(occ_id),
            lineage(occ_id),
            PrecedenceOrdinal::new(OpaqueValue::new(occ_id)),
            guard,
            Direction::Buy,
        )
    }

    /// An envelope carrying the given configuration parameters, conditions, and
    /// Direction rules; other collections present-and-empty.
    fn strategy(
        parameters: Vec<ConfigurationParameter>,
        conditions: Vec<Condition>,
        rules: Vec<DirectionRule>,
    ) -> Strategy {
        Strategy {
            occurrence: occ(1),
            lineage: Declared::Present(lineage(1000)),
            semantic_model_version: Declared::Absent,
            evidence_declarations: Declared::Present(vec![]),
            configuration_parameters: Declared::Present(parameters),
            conditions: Declared::Present(conditions),
            direction_rules: Declared::Present(rules),
            candidate_derivations: Declared::Present(vec![]),
        }
    }

    fn categories(defects: &[DefectCore]) -> Vec<DefectCategory> {
        defects.iter().map(DefectCore::category).collect()
    }

    #[test]
    fn a_guard_atom_referencing_a_config_parameter_is_a_structural_position_defect() {
        // Atom references lineage 4, which is a declared configuration parameter.
        let s = strategy(
            vec![config_parameter(50, 4)],
            vec![condition_with_guard(20, Guard::from_atoms(vec![atom(4)]))],
            vec![],
        );
        let defects = check_5(&s);
        assert_eq!(
            categories(&defects),
            vec![DefectCategory::ConfigurationInStructuralPosition]
        );
        // Anchored on the enclosing condition, ATOM discriminator.
        assert_eq!(defects[0].offending(), occ(20));
        assert!(matches!(defects[0].discriminator(), Discriminator::Atom(_)));
    }

    #[test]
    fn a_guard_atom_referencing_evidence_only_is_no_defect() {
        // Lineage 7 is not a configuration parameter, so this is an ordinary
        // evidence reference — check 5 says nothing (its resolution is check 2's).
        let s = strategy(
            vec![config_parameter(50, 4)],
            vec![condition_with_guard(20, Guard::from_atoms(vec![atom(7)]))],
            vec![],
        );
        assert!(check_5(&s).is_empty());
    }

    #[test]
    fn a_direction_rule_guard_atom_referencing_a_config_parameter_is_caught() {
        let s = strategy(
            vec![config_parameter(50, 4)],
            vec![],
            vec![rule_with_guard(30, Guard::from_atoms(vec![atom(4)]))],
        );
        let defects = check_5(&s);
        assert_eq!(
            categories(&defects),
            vec![DefectCategory::ConfigurationInStructuralPosition]
        );
        // Anchored on the Direction rule occurrence.
        assert_eq!(defects[0].offending(), occ(30));
    }

    #[test]
    fn an_ambiguous_config_reference_is_not_arbitrarily_resolved() {
        // Lineage 4 belongs to two configuration parameters (duplicate lineage):
        // the reference is ambiguous, so it is not a determinate configuration
        // reference and check 5 emits nothing (it never picks one target).
        let s = strategy(
            vec![config_parameter(50, 4), config_parameter(51, 4)],
            vec![condition_with_guard(20, Guard::from_atoms(vec![atom(4)]))],
            vec![],
        );
        assert!(check_5(&s).is_empty());
    }

    #[test]
    fn a_zero_target_reference_is_not_a_check_5_defect() {
        // No configuration parameters at all ⇒ nothing resolves ⇒ no check-5
        // defect (and check 5 never emits check-2's UNRESOLVED_REFERENCE).
        let s = strategy(
            vec![],
            vec![condition_with_guard(20, Guard::from_atoms(vec![atom(4)]))],
            vec![],
        );
        assert!(check_5(&s).is_empty());
    }

    #[test]
    fn two_offending_atoms_in_one_guard_stay_distinct_by_atom_discriminator() {
        // Two atoms in one condition guard reference two configuration
        // parameters; both are structural-position defects on the same condition
        // occurrence, kept distinct only by the ATOM discriminator.
        let s = strategy(
            vec![config_parameter(50, 4), config_parameter(51, 5)],
            vec![condition_with_guard(
                20,
                Guard::from_atoms(vec![atom(4), atom(5)]),
            )],
            vec![],
        );
        let defects = check_5(&s);
        assert_eq!(
            categories(&defects),
            vec![
                DefectCategory::ConfigurationInStructuralPosition,
                DefectCategory::ConfigurationInStructuralPosition,
            ]
        );
        assert_eq!(defects[0].offending(), occ(20));
        assert_eq!(defects[1].offending(), occ(20));
        assert_ne!(defects[0].cmp(&defects[1]), core::cmp::Ordering::Equal);
    }

    #[test]
    fn an_absent_lineage_config_parameter_does_not_break_anchoring() {
        // The referenced parameter (occ 50) resolves by its present lineage; an
        // additional config parameter with an absent lineage neither matches nor
        // disturbs resolution, and the defect still anchors on the condition.
        let absent_lineage_param = ConfigurationParameter {
            occurrence: occ(51),
            lineage: Declared::Absent,
            dimension: Declared::Present(DimensionSymbol::new(OpaqueValue::new(1))),
            lower_bound: Declared::Absent,
            upper_bound: Declared::Absent,
            adjustable: Declared::Absent,
        };
        let s = strategy(
            vec![config_parameter(50, 4), absent_lineage_param],
            vec![condition_with_guard(20, Guard::from_atoms(vec![atom(4)]))],
            vec![],
        );
        let defects = check_5(&s);
        assert_eq!(
            categories(&defects),
            vec![DefectCategory::ConfigurationInStructuralPosition]
        );
        assert_eq!(defects[0].offending(), occ(20));
    }

    #[test]
    fn an_empty_guard_has_no_structural_position_defect() {
        let s = strategy(
            vec![config_parameter(50, 4)],
            vec![condition_with_guard(20, Guard::always_applicable())],
            vec![],
        );
        assert!(check_5(&s).is_empty());
    }
}
