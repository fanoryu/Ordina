//! Check 11 — Unreachable operative rules (Slice Contract §5.1 check 11, §4.10).
//!
//! Check 11 has two accepted populations, both scoped per unit of work (never
//! Strategy-wide):
//!
//! 1. `UNREACHABLE_CONTRADICTORY_GUARD` — a Direction rule whose own guard is
//!    **contradictory** (some referenced lineage's constraint domain is empty,
//!    §4.10). A contradictory guard can never be satisfied, so the rule is
//!    unreachable. One defect per contradictory rule, anchored on that rule's
//!    occurrence, `NONE` discriminator.
//!
//! 2. `UNREACHABLE_SUBSUMED_RULE` — a rule `R` that is **subsumed** by another
//!    single rule `S` of **strictly higher** precedence: `D(R, e) ⊆ D(S, e)` for
//!    every referenced evidence item `e` (§4.10). Wherever `S` applies, `R`
//!    applies too, and `S` wins, so `R` is unreachable. One defect per valid
//!    (subsumed rule, subsuming rule) relation, anchored on `R`'s occurrence,
//!    `COUNTERPART` = `S`'s occurrence. Multiple distinct subsumers remain
//!    multiple distinct defects — they are not collapsed.
//!
//! # Prerequisites (§5.1)
//!
//! The contradiction part requires the rule's guard present and analyzable
//! (check 1, 2, 6 for that rule) — otherwise it is suppressed for that rule.
//! The subsumption part requires both guards present and analyzable and both
//! precedence ordinals present. A non-computable pair is suppressed locally.
//!
//! # Anchor and discriminator (Revision 4 §C.7)
//!
//! The contradiction part anchors on the rule's occurrence with `NONE`. The
//! subsumption part anchors on the **subsumed** rule's occurrence regardless of
//! occurrence order, with `COUNTERPART` = the subsuming rule's occurrence. An
//! absent or duplicated lineage is still representable through occurrence
//! identity — check 11 does **not** take check 15 as a prerequisite.
//!
//! Defect production only; no prerequisite gating framework, verdict, or
//! canonical sorting (those are Phase 6).

use crate::model::declared::Declared;
use crate::model::direction::DirectionRule;
use crate::model::strategy::Strategy;
use crate::result::category::DefectCategory;
use crate::result::discriminator::Discriminator;
use crate::result::identity::DefectCore;

use super::guard_analysis::{analyze_guard, guard_subsumes, is_contradictory};

/// The present Direction-rule collection, or `None` when the collection itself
/// is absent (check 1's, not check 11's).
fn present_rules(strategy: &Strategy) -> Option<&[DirectionRule]> {
    match &strategy.direction_rules {
        Declared::Present(rules) => Some(rules),
        Declared::Absent => None,
    }
}

/// Check 11 over a whole Strategy. Returns a deterministic `Vec<DefectCore>`.
#[must_use]
pub fn check_11(strategy: &Strategy) -> Vec<DefectCore> {
    let Some(rules) = present_rules(strategy) else {
        return Vec::new();
    };
    let mut defects = Vec::new();
    for rule in rules {
        // --- Contradiction part: one UNREACHABLE_CONTRADICTORY_GUARD per rule.
        let Declared::Present(guard) = &rule.guard else {
            continue;
        };
        let Some(domains) = analyze_guard(strategy, guard) else {
            continue;
        };
        if is_contradictory(&domains) {
            defects.push(DefectCore::new(
                rule.occurrence,
                DefectCategory::UnreachableContradictoryGuard,
                Discriminator::None,
            ));
        }
        // --- Subsumption part: one UNREACHABLE_SUBSUMED_RULE per (R, S) where S
        // strictly subsumes R and S has strictly higher precedence.
        let Declared::Present(rule_precedence) = &rule.precedence else {
            continue;
        };
        for other in rules {
            if other.occurrence == rule.occurrence {
                continue;
            }
            let Declared::Present(other_precedence) = &other.precedence else {
                continue;
            };
            // Strictly higher precedence only.
            if *other_precedence <= *rule_precedence {
                continue;
            }
            let (Declared::Present(rule_guard), Declared::Present(other_guard)) =
                (&rule.guard, &other.guard)
            else {
                continue;
            };
            let rule_domains = match analyze_guard(strategy, rule_guard) {
                Some(d) => d,
                None => continue,
            };
            let other_domains = match analyze_guard(strategy, other_guard) {
                Some(d) => d,
                None => continue,
            };
            // R subsumed by S ⟺ D(R,e) ⊆ D(S,e) for every e.
            if guard_subsumes(strategy, &other_domains, &rule_domains) {
                defects.push(DefectCore::new(
                    rule.occurrence,
                    DefectCategory::UnreachableSubsumedRule,
                    Discriminator::Counterpart(other.occurrence),
                ));
            }
        }
    }
    defects
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    use crate::model::declared::Declared;
    use crate::model::declared::DeclaredCategory;
    use crate::model::direction::Direction;
    use crate::model::evidence::EvidenceDeclaration;
    use crate::model::guard::{Atom, ComparisonOperator, Guard};
    use crate::model::literal::Literal;
    use crate::model::opaque::{
        DimensionSymbol, LineageId, OccurrenceId, OpaqueValue, PrecedenceOrdinal,
    };
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
    fn precedence(handle: u64) -> PrecedenceOrdinal {
        PrecedenceOrdinal::new(OpaqueValue::new(handle))
    }
    fn dim(handle: u64) -> DimensionSymbol {
        DimensionSymbol::new(OpaqueValue::new(handle))
    }

    fn num_decl(lin: u64) -> EvidenceDeclaration {
        EvidenceDeclaration::numeric(occ(500 + lin), lineage(lin), dim(1))
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

    fn rule(occ_id: u64, prec: u64, guard: Guard) -> DirectionRule {
        DirectionRule::new(
            occ(occ_id),
            lineage(occ_id),
            precedence(prec),
            guard,
            Direction::Buy,
        )
    }

    fn rule_absent_lineage(occ_id: u64, prec: u64, guard: Guard) -> DirectionRule {
        DirectionRule {
            occurrence: occ(occ_id),
            lineage: Declared::Absent,
            precedence: Declared::Present(precedence(prec)),
            guard: Declared::Present(guard),
            outcome: DeclaredCategory::One(Direction::Buy),
        }
    }

    fn strategy(decls: Vec<EvidenceDeclaration>, rules: Vec<DirectionRule>) -> Strategy {
        Strategy {
            occurrence: occ(1),
            lineage: Declared::Present(lineage(1000)),
            semantic_model_version: Declared::Absent,
            evidence_declarations: Declared::Present(decls),
            configuration_parameters: Declared::Present(vec![]),
            conditions: Declared::Present(vec![]),
            direction_rules: Declared::Present(rules),
            candidate_derivations: Declared::Present(vec![]),
        }
    }

    fn categories(defects: &[DefectCore]) -> Vec<DefectCategory> {
        defects.iter().map(DefectCore::category).collect()
    }

    // --- 11a: contradictory Direction guard -> UNREACHABLE_CONTRADICTORY_GUARD.
    #[test]
    fn contradictory_direction_guard_emits_contradictory_guard() {
        let r = rule(
            10,
            5,
            Guard::from_atoms(vec![
                natom(1, ComparisonOperator::Greater, 10),
                natom(1, ComparisonOperator::Less, 5),
            ]),
        );
        let defects = check_11(&strategy(vec![num_decl(1)], vec![r]));
        assert_eq!(defects.len(), 1);
        assert_eq!(
            defects[0].category(),
            DefectCategory::UnreachableContradictoryGuard
        );
        assert_eq!(defects[0].offending(), occ(10));
        assert!(matches!(defects[0].discriminator(), Discriminator::None));
    }

    // --- 11b: non-contradictory guard -> none.
    #[test]
    fn satisfiable_direction_guard_emits_nothing() {
        let r = rule(
            10,
            5,
            Guard::from_atoms(vec![natom(1, ComparisonOperator::Greater, 5)]),
        );
        let defects = check_11(&strategy(vec![num_decl(1)], vec![r]));
        assert!(defects.is_empty(), "got {:?}", categories(&defects));
    }

    // --- 11c: strictly-higher-precedence subsumer -> UNREACHABLE_SUBSUMED_RULE.
    #[test]
    fn higher_precedence_subsumer_emits_subsumed_rule() {
        let r = rule(
            10,
            5,
            Guard::from_atoms(vec![natom(1, ComparisonOperator::Greater, 5)]),
        );
        let s = rule(
            20,
            10,
            Guard::from_atoms(vec![natom(1, ComparisonOperator::Greater, 3)]),
        );
        let defects = check_11(&strategy(vec![num_decl(1)], vec![r, s]));
        assert_eq!(defects.len(), 1);
        assert_eq!(
            defects[0].category(),
            DefectCategory::UnreachableSubsumedRule
        );
        assert_eq!(defects[0].offending(), occ(10));
        match defects[0].discriminator() {
            Discriminator::Counterpart(c) => assert_eq!(*c, occ(20)),
            other => panic!("expected COUNTERPART, got {:?}", other),
        }
    }

    // --- 11d: equal/lower precedence, or higher-but-not-subsuming -> none.
    #[test]
    fn equal_or_lower_precedence_never_subsumes() {
        // Equal precedence: skip.
        let r = rule(
            10,
            5,
            Guard::from_atoms(vec![natom(1, ComparisonOperator::Greater, 5)]),
        );
        let s_eq = rule(
            20,
            5,
            Guard::from_atoms(vec![natom(1, ComparisonOperator::Greater, 3)]),
        );
        assert!(check_11(&strategy(vec![num_decl(1)], vec![r.clone(), s_eq])).is_empty());

        // Lower precedence S: skip.
        let s_low = rule(
            20,
            2,
            Guard::from_atoms(vec![natom(1, ComparisonOperator::Greater, 3)]),
        );
        assert!(check_11(&strategy(vec![num_decl(1)], vec![r.clone(), s_low])).is_empty());

        // Higher precedence but NOT subsuming (R wider than S).
        let r_wide = rule(
            10,
            5,
            Guard::from_atoms(vec![natom(1, ComparisonOperator::Greater, 3)]),
        );
        let s_narrow = rule(
            20,
            10,
            Guard::from_atoms(vec![natom(1, ComparisonOperator::Greater, 10)]),
        );
        let defects = check_11(&strategy(vec![num_decl(1)], vec![r_wide, s_narrow]));
        assert!(defects.is_empty(), "got {:?}", categories(&defects));
    }

    // --- 11e: multiple distinct subsumers remain multiple COUNTERPART defects.
    #[test]
    fn multiple_distinct_subsumers_remain_distinct() {
        let r = rule(
            10,
            5,
            Guard::from_atoms(vec![natom(1, ComparisonOperator::Greater, 5)]),
        );
        let s1 = rule(
            20,
            10,
            Guard::from_atoms(vec![natom(1, ComparisonOperator::Greater, 3)]),
        );
        let s2 = rule(
            30,
            20,
            Guard::from_atoms(vec![natom(1, ComparisonOperator::Greater, 4)]),
        );
        let defects = check_11(&strategy(vec![num_decl(1)], vec![r, s1, s2]));
        assert_eq!(defects.len(), 2);
        let counterparts: Vec<OccurrenceId> = defects
            .iter()
            .map(|d| match d.discriminator() {
                Discriminator::Counterpart(c) => *c,
                _ => panic!("expected COUNTERPART"),
            })
            .collect();
        assert!(counterparts.contains(&occ(20)));
        assert!(counterparts.contains(&occ(30)));
        assert_ne!(defects[0].cmp(&defects[1]), Ordering::Equal);
    }

    // --- 11f: anchor remains the subsumed (offending) occurrence.
    #[test]
    fn subsumed_rule_is_always_the_anchor() {
        // S sorts first by occurrence, but R is the offender.
        let s = rule(
            10,
            10,
            Guard::from_atoms(vec![natom(1, ComparisonOperator::Greater, 3)]),
        );
        let r = rule(
            20,
            5,
            Guard::from_atoms(vec![natom(1, ComparisonOperator::Greater, 5)]),
        );
        let defects = check_11(&strategy(vec![num_decl(1)], vec![s, r]));
        assert_eq!(defects.len(), 1);
        assert_eq!(defects[0].offending(), occ(20));
    }

    // --- 11g: absent lineage remains representable via occurrence.
    #[test]
    fn absent_lineage_rule_still_subsumed() {
        let r = rule_absent_lineage(
            10,
            5,
            Guard::from_atoms(vec![natom(1, ComparisonOperator::Greater, 5)]),
        );
        let s = rule(
            20,
            10,
            Guard::from_atoms(vec![natom(1, ComparisonOperator::Greater, 3)]),
        );
        let defects = check_11(&strategy(vec![num_decl(1)], vec![r, s]));
        assert_eq!(defects.len(), 1);
        assert_eq!(defects[0].offending(), occ(10));
    }

    // --- 11h: non-computable pair suppressed locally.
    #[test]
    fn non_computable_pair_is_suppressed() {
        let r = rule(
            10,
            5,
            Guard::from_atoms(vec![natom(1, ComparisonOperator::Greater, 5)]),
        );
        let unresolved = Atom::new(
            lineage(999),
            ComparisonOperator::Greater,
            Literal::Numeric {
                value: ExactNumber::from_integer(3),
                dimension: dim(1),
            },
        );
        let s = rule(20, 10, Guard::from_atoms(vec![unresolved]));
        let defects = check_11(&strategy(vec![num_decl(1)], vec![r, s]));
        assert!(defects.is_empty());
    }
}
