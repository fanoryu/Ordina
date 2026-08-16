//! Check 9 — Direction precedence validity (Slice Contract §5.1 check 9, §3.7,
//! §4.10).
//!
//! Every unordered pair of Direction rules must either have **mutually
//! exclusive** guards (no overlap, §4.10) **or** declared precedence ordinals
//! that **differ**. A pair whose guards **overlap** and whose precedence
//! ordinals are **equal** is a `DIRECTION_PRECEDENCE_TIE`: which rule wins when
//! both apply is indeterminate.
//!
//! # Population
//!
//! The population is the unordered pairs of Direction rules. The defect is
//! emitted exactly **once per offending unordered pair**.
//!
//! # Prerequisites (§5.1)
//!
//! The pair is evaluated only when it is computable: both rules' guards are
//! present and analyzable under check 1, check 2, and check 6 (resolved
//! references, valid declared types, type-consistent atoms — see
//! [`guard_analysis`]), and both rules' precedence ordinals are present. When a
//! pair is not computable it is suppressed **for that pair only**.
//!
//! # Anchor and discriminator (Revision 4 §C.7)
//!
//! The canonical anchor is the rule whose [`OccurrenceId`] sorts first under the
//! supplied occurrence order; the other member is the `COUNTERPART`
//! discriminator (kind 6). The anchor is chosen by occurrence order, never by
//! how the pair was discovered, so the identity is deterministic and symmetric.
//! A rule whose lineage is absent or duplicated is still representable through
//! its occurrence identity — check 9 does **not** take check 15 as a
//! prerequisite.
//!
//! Defect production only; no prerequisite gating framework, verdict, or
//! canonical sorting (those are Phase 6).

use crate::model::declared::Declared;
use crate::model::direction::DirectionRule;
use crate::model::strategy::Strategy;
use crate::result::category::DefectCategory;
use crate::result::discriminator::Discriminator;
use crate::result::identity::DefectCore;

use super::guard_analysis::{analyze_guard, guards_overlap};

/// The present Direction-rule collection, or `None` when the collection itself
/// is absent (check 1's `MISSING_REQUIRED_PROPERTY`, not check 9's).
fn present_rules(strategy: &Strategy) -> Option<&[DirectionRule]> {
    match &strategy.direction_rules {
        Declared::Present(rules) => Some(rules),
        Declared::Absent => None,
    }
}

/// Check 9 over a whole Strategy. Returns a deterministic `Vec<DefectCore>`.
#[must_use]
pub fn check_9(strategy: &Strategy) -> Vec<DefectCore> {
    let Some(rules) = present_rules(strategy) else {
        return Vec::new();
    };
    let mut defects = Vec::new();
    for i in 0..rules.len() {
        let a = &rules[i];
        for b in &rules[i + 1..] {
            // Both guards must be present and analyzable.
            let (Declared::Present(guard_a), Declared::Present(guard_b)) = (&a.guard, &b.guard)
            else {
                continue;
            };
            let domains_a = match analyze_guard(strategy, guard_a) {
                Some(d) => d,
                None => continue,
            };
            let domains_b = match analyze_guard(strategy, guard_b) {
                Some(d) => d,
                None => continue,
            };
            // Both precedence ordinals must be present.
            let (Declared::Present(precedence_a), Declared::Present(precedence_b)) =
                (&a.precedence, &b.precedence)
            else {
                continue;
            };
            // Overlap AND equal precedence → tie.
            if guards_overlap(strategy, &domains_a, &domains_b) && precedence_a == precedence_b {
                let anchor = a.occurrence.min(b.occurrence);
                let counterpart = a.occurrence.max(b.occurrence);
                defects.push(DefectCore::new(
                    anchor,
                    DefectCategory::DirectionPrecedenceTie,
                    Discriminator::Counterpart(counterpart),
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

    use crate::model::declared::DeclaredCategory;
    use crate::model::direction::Direction;
    use crate::model::guard::{Atom, ComparisonOperator, Guard};
    use crate::model::literal::Literal;
    use crate::model::opaque::{LineageId, OccurrenceId, OpaqueValue, PrecedenceOrdinal};
    use crate::model::strategy::Strategy;
    use crate::numeric::ExactNumber;

    fn occ(handle: u64) -> OccurrenceId {
        OccurrenceId::new(OpaqueValue::new(handle))
    }
    fn lineage(handle: u64) -> LineageId {
        LineageId::new(OpaqueValue::new(handle))
    }
    fn precedence(handle: u64) -> PrecedenceOrdinal {
        PrecedenceOrdinal::new(OpaqueValue::new(handle))
    }
    fn dim(handle: u64) -> crate::model::opaque::DimensionSymbol {
        crate::model::opaque::DimensionSymbol::new(OpaqueValue::new(handle))
    }

    /// Numeric declaration L=1, dimension D=1.
    fn num_decl() -> crate::model::evidence::EvidenceDeclaration {
        crate::model::evidence::EvidenceDeclaration::numeric(occ(500), lineage(1), dim(1))
    }

    /// A numeric `lit op value` atom on L=1 with dimension D=1.
    fn natom(op: ComparisonOperator, value: i64) -> Atom {
        Atom::new(
            lineage(1),
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

    /// A rule whose lineage is absent (check 15's concern) but whose occurrence
    /// is present — used to prove occurrence-based representation.
    fn rule_absent_lineage(occ_id: u64, prec: u64, guard: Guard) -> DirectionRule {
        DirectionRule {
            occurrence: occ(occ_id),
            lineage: crate::model::declared::Declared::Absent,
            precedence: crate::model::declared::Declared::Present(precedence(prec)),
            guard: crate::model::declared::Declared::Present(guard),
            outcome: DeclaredCategory::One(Direction::Buy),
        }
    }

    fn strategy(
        decls: Vec<crate::model::evidence::EvidenceDeclaration>,
        rules: Vec<DirectionRule>,
    ) -> Strategy {
        Strategy {
            occurrence: occ(1),
            lineage: crate::model::declared::Declared::Present(lineage(1000)),
            semantic_model_version: crate::model::declared::Declared::Absent,
            evidence_declarations: crate::model::declared::Declared::Present(decls),
            configuration_parameters: crate::model::declared::Declared::Present(vec![]),
            conditions: crate::model::declared::Declared::Present(vec![]),
            direction_rules: crate::model::declared::Declared::Present(rules),
            candidate_derivations: crate::model::declared::Declared::Present(vec![]),
        }
    }

    fn categories(defects: &[DefectCore]) -> Vec<DefectCategory> {
        defects.iter().map(DefectCore::category).collect()
    }

    // --- 9a: non-overlapping equal precedence -> no defect.
    #[test]
    fn non_overlapping_equal_precedence_is_not_a_tie() {
        let a = rule(
            10,
            5,
            Guard::from_atoms(vec![natom(ComparisonOperator::Greater, 10)]),
        );
        let b = rule(
            20,
            5,
            Guard::from_atoms(vec![natom(ComparisonOperator::Less, 5)]),
        );
        let defects = check_9(&strategy(vec![num_decl()], vec![a, b]));
        assert!(defects.is_empty(), "got {:?}", categories(&defects));
    }

    // --- 9b: overlapping different precedence -> no defect.
    #[test]
    fn overlapping_different_precedence_is_not_a_tie() {
        let a = rule(
            10,
            2,
            Guard::from_atoms(vec![natom(ComparisonOperator::Greater, 5)]),
        );
        let b = rule(
            20,
            7,
            Guard::from_atoms(vec![natom(ComparisonOperator::Greater, 3)]),
        );
        let defects = check_9(&strategy(vec![num_decl()], vec![a, b]));
        assert!(defects.is_empty(), "got {:?}", categories(&defects));
    }

    // --- 9c: overlapping equal precedence -> exactly one defect.
    #[test]
    fn overlapping_equal_precedence_is_one_tie() {
        let a = rule(
            10,
            5,
            Guard::from_atoms(vec![natom(ComparisonOperator::Greater, 5)]),
        );
        let b = rule(
            20,
            5,
            Guard::from_atoms(vec![natom(ComparisonOperator::Greater, 3)]),
        );
        let defects = check_9(&strategy(vec![num_decl()], vec![a, b]));
        assert_eq!(defects.len(), 1);
        assert_eq!(
            defects[0].category(),
            DefectCategory::DirectionPrecedenceTie
        );
        // Anchor is the lower occurrence; COUNTERPART the higher.
        assert_eq!(defects[0].offending(), occ(10));
        match defects[0].discriminator() {
            Discriminator::Counterpart(c) => assert_eq!(*c, occ(20)),
            other => panic!("expected COUNTERPART, got {:?}", other),
        }
    }

    // --- 9d: reversed storage order yields identical identity.
    #[test]
    fn reversed_storage_order_is_identical_identity() {
        let a = rule(
            10,
            5,
            Guard::from_atoms(vec![natom(ComparisonOperator::Greater, 5)]),
        );
        let b = rule(
            20,
            5,
            Guard::from_atoms(vec![natom(ComparisonOperator::Greater, 3)]),
        );
        let forward = check_9(&strategy(vec![num_decl()], vec![a.clone(), b.clone()]));
        let reversed = check_9(&strategy(vec![num_decl()], vec![b, a]));
        assert_eq!(forward.len(), 1);
        assert_eq!(reversed.len(), 1);
        assert_eq!(forward[0].cmp(&reversed[0]), Ordering::Equal);
        assert_eq!(forward[0].offending(), occ(10));
    }

    // --- 9e: multiple offending unordered pairs -> one per pair.
    #[test]
    fn three_equal_precedence_overlapping_rules_yield_three_ties() {
        let a = rule(
            10,
            5,
            Guard::from_atoms(vec![natom(ComparisonOperator::Greater, 5)]),
        );
        let b = rule(
            20,
            5,
            Guard::from_atoms(vec![natom(ComparisonOperator::Greater, 4)]),
        );
        let c = rule(
            30,
            5,
            Guard::from_atoms(vec![natom(ComparisonOperator::Greater, 3)]),
        );
        let defects = check_9(&strategy(vec![num_decl()], vec![a, b, c]));
        assert_eq!(defects.len(), 3);
        assert!(
            defects
                .iter()
                .all(|d| d.category() == DefectCategory::DirectionPrecedenceTie)
        );
        // All three pairwise defects are distinct identities.
        for i in 0..defects.len() {
            for j in (i + 1)..defects.len() {
                assert_ne!(defects[i].cmp(&defects[j]), Ordering::Equal);
            }
        }
    }

    // --- 9f: absent-lineage rules remain representable via occurrence identity.
    #[test]
    fn absent_lineage_rules_still_representable() {
        let a = rule_absent_lineage(
            10,
            5,
            Guard::from_atoms(vec![natom(ComparisonOperator::Greater, 5)]),
        );
        let b = rule(
            20,
            5,
            Guard::from_atoms(vec![natom(ComparisonOperator::Greater, 3)]),
        );
        let defects = check_9(&strategy(vec![num_decl()], vec![a, b]));
        assert_eq!(defects.len(), 1);
        assert_eq!(defects[0].offending(), occ(10));
    }

    // --- 9g: unresolved / type-invalid pair suppressed locally (no defect, no panic).
    #[test]
    fn non_computable_pair_is_suppressed_locally() {
        // Rule B references an undeclared lineage -> non-computable for that pair.
        let unresolved = Atom::new(
            lineage(999),
            ComparisonOperator::Greater,
            Literal::Numeric {
                value: ExactNumber::from_integer(3),
                dimension: dim(1),
            },
        );
        let a = rule(
            10,
            5,
            Guard::from_atoms(vec![natom(ComparisonOperator::Greater, 5)]),
        );
        let b = rule(20, 5, Guard::from_atoms(vec![unresolved]));
        let defects = check_9(&strategy(vec![num_decl()], vec![a, b]));
        assert!(defects.is_empty());

        // Type mismatch: numeric referent given a boolean literal.
        let type_mismatch = Atom::new(
            lineage(1),
            ComparisonOperator::Equal,
            Literal::Boolean(true),
        );
        let c = rule(30, 5, Guard::from_atoms(vec![type_mismatch]));
        let d = rule(
            40,
            5,
            Guard::from_atoms(vec![natom(ComparisonOperator::Greater, 3)]),
        );
        let defects2 = check_9(&strategy(vec![num_decl()], vec![c, d]));
        assert!(defects2.is_empty());
    }
}
