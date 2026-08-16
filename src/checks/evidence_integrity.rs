//! Check 6 — evidence-declaration integrity — Slice Contract §5.1 check 6,
//! §3.5, §5.1.1 (m-3); ADR-0003 §6.2.
//!
//! Check 6 is the **exclusive owner of all type-dependent evidence-declaration
//! validity** (§3.5, M-1). Its unconditional properties — lineage identity and
//! declared type (§3.5 positions 1 and 2) — belong to check 1; check 6 assumes
//! they are present and never reports them. The type-dependent properties it owns
//! are the declared dimension (position 3, present iff `NUMERIC`) and the
//! declared value set (position 4, present, non-empty, and duplicate-free iff
//! `ENUM`).
//!
//! Each violation produces exactly one `INVALID_EVIDENCE_DECLARATION` defect,
//! anchored on the offending declaration's occurrence identity and discriminated
//! by the §3.5 property position (PROPERTY kind 2, §6.4.2). The mapping is total
//! and deterministic (§5.1.1); two violations on one declaration are separated by
//! the mapped property's position, and the §5.1.1 rows are mutually exclusive so
//! they never collide.
//!
//! # O-1 set semantics (PBA-0118 / ER-0099, DETERMINATION B)
//!
//! `declared_values` is normatively a **set** (§3.5 "a finite, non-empty set of
//! **distinct** opaque categorical values"; §5.1.1 the "value set is empty" and
//! "value set contains duplicate values" violations). The `Vec<EnumValue>`
//! representation is *deliberately* permissive so check 6 can observe empty and
//! duplicate states — it is not a contract violation and is not changed. Check 6
//! therefore applies **semantic-set interpretation** before evaluating any
//! predicate whose meaning depends on membership:
//!
//! * **order carries no meaning** — `[A, B]` and `[B, A]` are the same value set;
//! * **duplicate-identical members carry no multiplicity** — `[A, A]` and `[A]`
//!   are the same value set;
//! * detecting that duplicates are *present* is a distinct obligation from
//!   allowing duplicate representation to multiply any downstream outcome;
//! * the duplicate-integrity defect is emitted **once per declaration** that has a
//!   duplicate, never once per extra occurrence (no `n−1` counting);
//! * an empty value set is a single distinct violation ("value set is empty"),
//!   separate from "contains duplicate values";
//! * **no collection index, traversal order, serialization position, or
//!   host-language ordering** enters any defect identity (§6.4, §C.1, §4.5).
//!
//! This module computes set facts (`is_empty`, `has_duplicate`, membership) over
//! the declared collection; it never deduplicates emitted `DefectCore` values
//! (§6.3 forbids that) and never lets duplicate representation inflate an
//! unrelated predicate.
//!
//! # Scope and ownership boundaries
//!
//! * **Not check 1** — never reports an absent unconditional property; those are
//!   check 1's exclusively (§3.5).
//! * **Not check 2** — never emits `UNRESOLVED_REFERENCE`, `TYPE_MISMATCH`, or
//!   `ENUM_VALUE_NOT_DECLARED`. Guard-atom value membership (`ENUM_VALUE_NOT_`
//!   `DECLARED`) is check 2's responsibility (§5.1 check 2); check 6 judges only
//!   the declaration's own internal validity.
//! * **Not check 15 / check 16** — never reports absent lineage identity or
//!   semantic-model-version defects.
//!
//! Defect production only; deterministic. No prerequisite gating, not-evaluated
//! record, verdict, or canonical sort (those are Phase 6).

use crate::model::declared::Declared;
use crate::model::declared::DeclaredCategory;
use crate::model::evidence::{EvidenceDeclaration, EvidenceTypeTag};
use crate::model::opaque::{EnumValue, OccurrenceId};
use crate::model::strategy::Strategy;
use crate::result::category::DefectCategory;
use crate::result::discriminator::{Discriminator, PropertyPosition};
use crate::result::identity::DefectCore;

/// One `INVALID_EVIDENCE_DECLARATION` defect anchored on the offending
/// declaration's occurrence identity, discriminated by the §3.5 property
/// position (PROPERTY kind 2, §6.4.2 / §5.1.1 m-3).
fn violation(occurrence: OccurrenceId, property_position: u32) -> DefectCore {
    DefectCore::new(
        occurrence,
        DefectCategory::InvalidEvidenceDeclaration,
        Discriminator::Property(PropertyPosition::new(property_position)),
    )
}

/// The §3.5 property positions owned by check 6.
const DECLARED_DIMENSION: u32 = 3;
const DECLARED_VALUE_SET: u32 = 4;

/// Whether the declared value set contains a duplicate member, under set
/// semantics: two members compare equal (their `EnumValue` `Eq` is exact
/// opaque-value equality, §4.6). Order is irrelevant and duplicate occurrences
/// do not count extra — a set either is duplicate-bearing or it is not.
fn value_set_has_duplicate(values: &[EnumValue]) -> bool {
    for i in 0..values.len() {
        for j in (i + 1)..values.len() {
            if values[i] == values[j] {
                return true;
            }
        }
    }
    false
}

/// Check 6 over one evidence declaration — its type-dependent validity only.
fn check_declaration(declaration: &EvidenceDeclaration) -> Vec<DefectCore> {
    let mut defects = Vec::new();
    let occurrence = declaration.occurrence;

    match &declaration.declared_type {
        DeclaredCategory::One(EvidenceTypeTag::Numeric) => {
            // Dimension required; a value set is prohibited.
            if declaration.declared_dimension.is_absent() {
                defects.push(violation(occurrence, DECLARED_DIMENSION));
            }
            if declaration.declared_values.is_present() {
                defects.push(violation(occurrence, DECLARED_VALUE_SET));
            }
        }
        DeclaredCategory::One(EvidenceTypeTag::Boolean) => {
            // Neither dimension nor value set permitted.
            if declaration.declared_dimension.is_present() {
                defects.push(violation(occurrence, DECLARED_DIMENSION));
            }
            if declaration.declared_values.is_present() {
                defects.push(violation(occurrence, DECLARED_VALUE_SET));
            }
        }
        DeclaredCategory::One(EvidenceTypeTag::Enum) => {
            // Value set required, and must be present, non-empty, and
            // duplicate-free. A dimension is prohibited.
            if declaration.declared_dimension.is_present() {
                defects.push(violation(occurrence, DECLARED_DIMENSION));
            }
            match &declaration.declared_values {
                Declared::Absent => {
                    defects.push(violation(occurrence, DECLARED_VALUE_SET));
                }
                Declared::Present(values) => {
                    if values.is_empty() {
                        defects.push(violation(occurrence, DECLARED_VALUE_SET));
                    } else if value_set_has_duplicate(values) {
                        // One duplicate-integrity defect per declaration, regardless
                        // of how many members repeat or how often (O-1 set
                        // semantics, ER-0099 DETERMINATION B).
                        defects.push(violation(occurrence, DECLARED_VALUE_SET));
                    }
                }
            }
        }
        // An out-of-domain type is check 6's "declared type is not one of the
        // three admitted" violation, mapped to the declared-type property
        // (position 2) — also owned exclusively by check 6 under §3.5.
        DeclaredCategory::Absent
        | DeclaredCategory::Multiple { .. }
        | DeclaredCategory::OutOfDomain => {
            defects.push(violation(occurrence, 2));
        }
    }

    defects
}

/// Check 6 over a whole Strategy — every present evidence declaration.
///
/// Check 6 assumes check 1 has passed for each declaration (its unconditional
/// properties — lineage identity and declared type presence — are present). It
/// therefore inspects only type-dependent validity and never reports an absent
/// unconditional property. Returns a deterministic `Vec<DefectCore>`.
#[must_use]
pub fn check_6(strategy: &Strategy) -> Vec<DefectCore> {
    let mut defects = Vec::new();

    if let Declared::Present(declarations) = &strategy.evidence_declarations {
        for declaration in declarations {
            defects.extend(check_declaration(declaration));
        }
    }

    defects
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::declared::DeclaredCategory;
    use crate::model::evidence::EvidenceDeclaration;
    use crate::model::opaque::{DimensionSymbol, EnumValue, LineageId, OccurrenceId, OpaqueValue};
    use crate::model::strategy::Strategy;
    use crate::result::category::DefectCategory;
    use crate::result::discriminator::{Discriminator, PropertyPosition};

    fn occ(handle: u64) -> OccurrenceId {
        OccurrenceId::new(OpaqueValue::new(handle))
    }

    fn lin(handle: u64) -> LineageId {
        LineageId::new(OpaqueValue::new(handle))
    }

    fn dim(handle: u64) -> DimensionSymbol {
        DimensionSymbol::new(OpaqueValue::new(handle))
    }

    fn enum_value(handle: u64) -> EnumValue {
        EnumValue::new(OpaqueValue::new(handle))
    }

    /// A well-formed `ENUM` declaration with the given value set.
    fn enum_decl(occ_id: u64, values: Vec<EnumValue>) -> EvidenceDeclaration {
        EvidenceDeclaration {
            occurrence: occ(occ_id),
            lineage: Declared::Present(lin(occ_id)),
            declared_type: DeclaredCategory::One(EvidenceTypeTag::Enum),
            declared_dimension: Declared::Absent,
            declared_values: Declared::Present(values),
        }
    }

    fn numeric_decl(occ_id: u64, dimension: DimensionSymbol) -> EvidenceDeclaration {
        EvidenceDeclaration {
            occurrence: occ(occ_id),
            lineage: Declared::Present(lin(occ_id)),
            declared_type: DeclaredCategory::One(EvidenceTypeTag::Numeric),
            declared_dimension: Declared::Present(dimension),
            declared_values: Declared::Absent,
        }
    }

    fn boolean_decl(occ_id: u64) -> EvidenceDeclaration {
        EvidenceDeclaration {
            occurrence: occ(occ_id),
            lineage: Declared::Present(lin(occ_id)),
            declared_type: DeclaredCategory::One(EvidenceTypeTag::Boolean),
            declared_dimension: Declared::Absent,
            declared_values: Declared::Absent,
        }
    }

    fn strategy_with(declarations: Vec<EvidenceDeclaration>) -> Strategy {
        Strategy {
            occurrence: occ(1),
            lineage: Declared::Present(lin(1000)),
            semantic_model_version: Declared::Absent,
            evidence_declarations: Declared::Present(declarations),
            configuration_parameters: Declared::Present(vec![]),
            conditions: Declared::Present(vec![]),
            direction_rules: Declared::Present(vec![]),
            candidate_derivations: Declared::Present(vec![]),
        }
    }

    fn categories_and_properties(
        defects: &[DefectCore],
    ) -> Vec<(DefectCategory, PropertyPosition)> {
        defects
            .iter()
            .map(|d| {
                (
                    d.category(),
                    match d.discriminator() {
                        Discriminator::Property(p) => *p,
                        _ => panic!("check 6 uses only PROPERTY discriminators"),
                    },
                )
            })
            .collect()
    }

    #[test]
    fn a_valid_enum_declaration_has_no_defect() {
        let s = strategy_with(vec![enum_decl(10, vec![enum_value(1), enum_value(2)])]);
        assert!(check_6(&s).is_empty());
    }

    #[test]
    fn a_valid_numeric_declaration_has_no_defect() {
        let s = strategy_with(vec![numeric_decl(10, dim(1))]);
        assert!(check_6(&s).is_empty());
    }

    #[test]
    fn a_valid_boolean_declaration_has_no_defect() {
        let s = strategy_with(vec![boolean_decl(10)]);
        assert!(check_6(&s).is_empty());
    }

    #[test]
    fn an_enum_declaration_without_a_value_set_is_a_value_set_defect() {
        let declaration = EvidenceDeclaration {
            occurrence: occ(10),
            lineage: Declared::Present(lin(10)),
            declared_type: DeclaredCategory::One(EvidenceTypeTag::Enum),
            declared_dimension: Declared::Absent,
            declared_values: Declared::Absent,
        };
        let s = strategy_with(vec![declaration]);
        let defects = check_6(&s);
        assert_eq!(
            categories_and_properties(&defects),
            vec![(
                DefectCategory::InvalidEvidenceDeclaration,
                PropertyPosition::new(4)
            )]
        );
        assert_eq!(defects[0].offending(), occ(10));
    }

    #[test]
    fn an_empty_enum_value_set_is_a_value_set_defect() {
        let s = strategy_with(vec![enum_decl(10, vec![])]);
        let defects = check_6(&s);
        assert_eq!(
            categories_and_properties(&defects),
            vec![(
                DefectCategory::InvalidEvidenceDeclaration,
                PropertyPosition::new(4)
            )]
        );
    }

    #[test]
    fn a_numeric_declaration_without_a_dimension_is_a_dimension_defect() {
        // Build the actual no-dimension numeric declaration.
        let declaration = EvidenceDeclaration {
            occurrence: occ(10),
            lineage: Declared::Present(lin(10)),
            declared_type: DeclaredCategory::One(EvidenceTypeTag::Numeric),
            declared_dimension: Declared::Absent,
            declared_values: Declared::Absent,
        };
        let s = strategy_with(vec![declaration]);
        let defects = check_6(&s);
        assert_eq!(
            categories_and_properties(&defects),
            vec![(
                DefectCategory::InvalidEvidenceDeclaration,
                PropertyPosition::new(3)
            )]
        );
    }

    #[test]
    fn a_dimension_on_a_boolean_declaration_is_a_dimension_defect() {
        let declaration = EvidenceDeclaration {
            occurrence: occ(10),
            lineage: Declared::Present(lin(10)),
            declared_type: DeclaredCategory::One(EvidenceTypeTag::Boolean),
            declared_dimension: Declared::Present(dim(1)),
            declared_values: Declared::Absent,
        };
        let s = strategy_with(vec![declaration]);
        let defects = check_6(&s);
        assert_eq!(
            categories_and_properties(&defects),
            vec![(
                DefectCategory::InvalidEvidenceDeclaration,
                PropertyPosition::new(3)
            )]
        );
    }

    #[test]
    fn a_dimension_on_an_enum_declaration_is_a_dimension_defect() {
        let declaration = EvidenceDeclaration {
            occurrence: occ(10),
            lineage: Declared::Present(lin(10)),
            declared_type: DeclaredCategory::One(EvidenceTypeTag::Enum),
            declared_dimension: Declared::Present(dim(1)),
            declared_values: Declared::Present(vec![enum_value(1)]),
        };
        let s = strategy_with(vec![declaration]);
        let defects = check_6(&s);
        // Dimension present on ENUM is a dimension violation; the value set itself
        // is valid.
        assert_eq!(
            categories_and_properties(&defects),
            vec![(
                DefectCategory::InvalidEvidenceDeclaration,
                PropertyPosition::new(3)
            )]
        );
    }

    #[test]
    fn a_value_set_on_a_numeric_declaration_is_a_value_set_defect() {
        let declaration = EvidenceDeclaration {
            occurrence: occ(10),
            lineage: Declared::Present(lin(10)),
            declared_type: DeclaredCategory::One(EvidenceTypeTag::Numeric),
            declared_dimension: Declared::Present(dim(1)),
            declared_values: Declared::Present(vec![enum_value(1)]),
        };
        let s = strategy_with(vec![declaration]);
        let defects = check_6(&s);
        assert_eq!(
            categories_and_properties(&defects),
            vec![(
                DefectCategory::InvalidEvidenceDeclaration,
                PropertyPosition::new(4)
            )]
        );
    }

    // ---- O-1 duplicate-integrity behaviour (ER-0099 DETERMINATION B) ----

    #[test]
    fn a_duplicate_value_member_is_one_value_set_defect_regardless_of_count() {
        // [A, A]: exactly one duplicate-integrity defect.
        let s = strategy_with(vec![enum_decl(10, vec![enum_value(1), enum_value(1)])]);
        let defects = check_6(&s);
        assert_eq!(
            categories_and_properties(&defects),
            vec![(
                DefectCategory::InvalidEvidenceDeclaration,
                PropertyPosition::new(4)
            )]
        );
        assert_eq!(defects.len(), 1);
    }

    #[test]
    fn three_identical_members_are_still_one_duplicate_defect() {
        // [A, A, A]: one defect, not two or three.
        let s = strategy_with(vec![enum_decl(
            10,
            vec![enum_value(1), enum_value(1), enum_value(1)],
        )]);
        let defects = check_6(&s);
        assert_eq!(defects.len(), 1);
        assert_eq!(
            categories_and_properties(&defects),
            vec![(
                DefectCategory::InvalidEvidenceDeclaration,
                PropertyPosition::new(4)
            )]
        );
    }

    #[test]
    fn two_distinct_members_then_a_duplicate_are_one_duplicate_defect() {
        // [A, B, A]: one duplicate-integrity defect (the set {A, B} is
        // duplicate-bearing), not two.
        let s = strategy_with(vec![enum_decl(
            10,
            vec![enum_value(1), enum_value(2), enum_value(1)],
        )]);
        let defects = check_6(&s);
        assert_eq!(defects.len(), 1);
        assert_eq!(
            categories_and_properties(&defects),
            vec![(
                DefectCategory::InvalidEvidenceDeclaration,
                PropertyPosition::new(4)
            )]
        );
    }

    #[test]
    fn two_duplicated_members_are_one_duplicate_defect_not_two() {
        // [A, A, B, B]: the set is duplicate-bearing once; one defect.
        let s = strategy_with(vec![enum_decl(
            10,
            vec![enum_value(1), enum_value(1), enum_value(2), enum_value(2)],
        )]);
        let defects = check_6(&s);
        assert_eq!(defects.len(), 1);
        assert_eq!(
            categories_and_properties(&defects),
            vec![(
                DefectCategory::InvalidEvidenceDeclaration,
                PropertyPosition::new(4)
            )]
        );
    }

    #[test]
    fn order_permutation_is_equivalent_for_duplicate_detection() {
        // [A, B] and [B, A] are the same set: neither is duplicate-bearing.
        let s1 = strategy_with(vec![enum_decl(10, vec![enum_value(1), enum_value(2)])]);
        let s2 = strategy_with(vec![enum_decl(10, vec![enum_value(2), enum_value(1)])]);
        assert!(check_6(&s1).is_empty());
        assert!(check_6(&s2).is_empty());

        // [A, A, B] and [B, A, A] are the same duplicate-bearing set: one defect.
        let t1 = strategy_with(vec![enum_decl(
            10,
            vec![enum_value(1), enum_value(1), enum_value(2)],
        )]);
        let t2 = strategy_with(vec![enum_decl(
            10,
            vec![enum_value(2), enum_value(1), enum_value(1)],
        )]);
        assert_eq!(check_6(&t1).len(), 1);
        assert_eq!(check_6(&t2).len(), 1);
    }

    #[test]
    fn an_empty_value_set_is_distinct_from_a_duplicate_value_set() {
        // Empty and duplicate are mutually exclusive value-set violations; a
        // declaration is never both, so they are never reported together.
        let empty = strategy_with(vec![enum_decl(10, vec![])]);
        let dup = strategy_with(vec![enum_decl(10, vec![enum_value(1), enum_value(1)])]);
        assert_eq!(check_6(&empty).len(), 1);
        assert_eq!(check_6(&dup).len(), 1);
        // And a declaration that is both empty-capable and duplicate-capable is
        // impossible (empty has no members to duplicate), so no single declaration
        // yields two value-set defects.
    }

    #[test]
    fn duplicate_representation_does_not_multiply_unrelated_defects() {
        // An ENUM declaration that is duplicate-bearing AND carries a prohibited
        // dimension yields exactly the duplicate-integrity defect and the dimension
        // defect — two defects, one each, not three or more. The duplicate member
        // count does not inflate the dimension violation.
        let declaration = EvidenceDeclaration {
            occurrence: occ(10),
            lineage: Declared::Present(lin(10)),
            declared_type: DeclaredCategory::One(EvidenceTypeTag::Enum),
            declared_dimension: Declared::Present(dim(1)),
            declared_values: Declared::Present(vec![enum_value(1), enum_value(1)]),
        };
        let s = strategy_with(vec![declaration]);
        let defects = check_6(&s);
        assert_eq!(defects.len(), 2);
        let mut props: Vec<u32> = defects
            .iter()
            .map(|d| match d.discriminator() {
                Discriminator::Property(p) => p.position(),
                _ => panic!(),
            })
            .collect();
        props.sort_unstable();
        assert_eq!(props, vec![3, 4]);
    }

    #[test]
    fn multiple_declarations_on_distinct_occurrences_stay_distinct() {
        // Two declarations, each with its own value-set defect, on distinct
        // occurrences: two defects, each anchored on its own declaration.
        let s = strategy_with(vec![
            enum_decl(10, vec![enum_value(1), enum_value(1)]),
            enum_decl(11, vec![enum_value(2), enum_value(2)]),
        ]);
        let defects = check_6(&s);
        assert_eq!(defects.len(), 2);
        let mut anchors: Vec<OccurrenceId> = defects.iter().map(DefectCore::offending).collect();
        anchors.sort_unstable();
        assert_eq!(anchors, vec![occ(10), occ(11)]);
    }

    #[test]
    fn check_6_uses_only_property_discriminators() {
        let s = strategy_with(vec![enum_decl(10, vec![enum_value(1), enum_value(1)])]);
        for d in check_6(&s) {
            assert!(matches!(d.discriminator(), Discriminator::Property(_)));
        }
    }
}
