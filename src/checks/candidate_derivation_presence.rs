//! Check 13B — Strategy-level candidate-role completeness/uniqueness — Slice
//! Contract §5.1 check 13 (part B).
//!
//! For a present `candidate_derivations` collection, each closed [`CandidateRole`]
//! (`ENTRY`, `STOP_LOSS`, `TAKE_PROFIT`) must occur exactly once.
//!
//! - count == 0 → one [`DefectCategory::MissingCandidateDerivation`] / ROLE(role)
//! - count == 1 → no Check-13B defect
//! - count >= 2 → one [`DefectCategory::DuplicateCandidateDerivation`] / ROLE(role)
//!
//! This is a **Strategy-level** check. Both defect types anchor on the Strategy
//! envelope occurrence identity, discriminated by the ROLE discriminator (§6.4.2
//! kind 3). Multiplicity is one per missing/duplicated role, **not** one per extra
//! occurrence — `ENTRY × 3` still yields exactly one duplicate `ENTRY` defect.
//!
//! **Prerequisite:** Check 1 on the envelope's `candidate_derivations` collection.
//! If the collection is absent, this check returns no defects locally (Phase-5 local
//! non-computability; Check 1 owns the collection absence via
//! `MISSING_REQUIRED_PROPERTY` position 7).
//!
//! **Ownership:** Check 13B inspects the `role` field of each present derivation only.
//! It does **not** inspect expression presence (Check 1, position 3), lineage presence
//! (Check 15), reference resolution or typing (Check 2), or dimensional consistency
//! (Check 14A). A derivation with a present role but absent expression, absent lineage,
//! unresolved references, or dimensional faults still counts toward role completeness.
//!
//! **Phase 5 only** — pure defect production; no prerequisite gating, no
//! `not-evaluated` metadata, no verdict, no canonical sort (those are Phase 6).

use crate::model::declared::Declared;
use crate::model::derivation::CandidateRole;
use crate::model::strategy::Strategy;
use crate::result::category::DefectCategory;
use crate::result::discriminator::Discriminator;
use crate::result::identity::DefectCore;

/// Role ordinal for fixed-array indexing — mirrors the §6.4.2 kind-3 declared order:
/// `ENTRY` (0) < `STOP_LOSS` (1) < `TAKE_PROFIT` (2).
fn role_index(role: CandidateRole) -> usize {
    match role {
        CandidateRole::Entry => 0,
        CandidateRole::StopLoss => 1,
        CandidateRole::TakeProfit => 2,
    }
}

/// Check 13B over a whole Strategy.
///
/// If the `candidate_derivations` collection is absent, returns `Vec::new()`
/// locally — the collection's absence is Check 1's domain (position 7), and 13B's
/// prerequisite is not satisfied. Otherwise, counts occurrences of each role across
/// the present derivations and emits one defect per missing or duplicated role.
///
/// Both defect types anchor on the Strategy envelope occurrence identity. Emission
/// order is deterministic: missing-role defects in ROLE ordinal order (ENTRY →
/// STOP_LOSS → TAKE_PROFIT), then duplicate-role defects in ROLE ordinal order.
/// This order is independent of the input `Vec` storage order.
pub fn check_13b(strategy: &Strategy) -> Vec<DefectCore> {
    // Prerequisite: collection must be present (Check 1 owns absence).
    let derivations = match &strategy.candidate_derivations {
        Declared::Present(items) => items,
        Declared::Absent => return Vec::new(),
    };

    // Count occurrences of each role. `Declared::Absent` contributes 0 to every
    // role count — the role is simply not present on that derivation.
    let mut counts = [0usize; 3];
    for derivation in derivations {
        if let Declared::Present(role) = &derivation.role {
            counts[role_index(*role)] += 1;
        }
    }

    let mut defects = Vec::new();
    let envelope = strategy.occurrence;

    // Missing-role defects in ROLE ordinal order.
    for role in [
        CandidateRole::Entry,
        CandidateRole::StopLoss,
        CandidateRole::TakeProfit,
    ] {
        if counts[role_index(role)] == 0 {
            defects.push(make_defect(
                envelope,
                DefectCategory::MissingCandidateDerivation,
                role,
            ));
        }
    }

    // Duplicate-role defects in ROLE ordinal order. One defect per duplicated role,
    // regardless of how many extra occurrences exist (count >= 2 → exactly one).
    for role in [
        CandidateRole::Entry,
        CandidateRole::StopLoss,
        CandidateRole::TakeProfit,
    ] {
        if counts[role_index(role)] >= 2 {
            defects.push(make_defect(
                envelope,
                DefectCategory::DuplicateCandidateDerivation,
                role,
            ));
        }
    }

    defects
}

fn make_defect(
    anchor: crate::model::opaque::OccurrenceId,
    category: DefectCategory,
    role: CandidateRole,
) -> DefectCore {
    DefectCore::new(anchor, category, Discriminator::Role(role))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::derivation::{CandidateDerivation, DerivationExpression};
    use crate::model::literal::Literal;
    use crate::model::opaque::{DimensionSymbol, LineageId, OpaqueValue, VersionToken};
    use crate::numeric::ExactNumber;

    fn occ(handle: u64) -> crate::model::opaque::OccurrenceId {
        crate::model::opaque::OccurrenceId::new(OpaqueValue::new(handle))
    }

    fn lineage(handle: u64) -> LineageId {
        LineageId::new(OpaqueValue::new(handle))
    }

    fn strategy_with(derivations: Vec<CandidateDerivation>) -> Strategy {
        Strategy::new(
            occ(1),
            lineage(1),
            VersionToken::new(OpaqueValue::new(1)),
            vec![],
            vec![],
            vec![],
            vec![],
            derivations,
        )
    }

    fn literal(value: i64) -> Literal {
        Literal::Numeric {
            value: ExactNumber::from_integer(value),
            dimension: DimensionSymbol::new(OpaqueValue::new(1)),
        }
    }

    fn derivation(
        occ: crate::model::opaque::OccurrenceId,
        role: CandidateRole,
    ) -> CandidateDerivation {
        CandidateDerivation {
            occurrence: occ,
            lineage: Declared::Present(lineage(1)),
            role: Declared::Present(role),
            expression: Declared::Present(DerivationExpression::Literal(literal(1))),
        }
    }

    fn derivation_absent_role(occ: crate::model::opaque::OccurrenceId) -> CandidateDerivation {
        CandidateDerivation {
            occurrence: occ,
            lineage: Declared::Present(lineage(1)),
            role: Declared::Absent,
            expression: Declared::Present(DerivationExpression::Literal(literal(1))),
        }
    }

    fn summarize(defects: &[DefectCore]) -> Vec<(DefectCategory, CandidateRole)> {
        defects
            .iter()
            .map(|d| {
                let role = match d.discriminator() {
                    Discriminator::Role(r) => *r,
                    _ => panic!("check 13B must use ROLE discriminator"),
                };
                (d.category(), role)
            })
            .collect()
    }

    fn role_ord(role: CandidateRole) -> u8 {
        match role {
            CandidateRole::Entry => 0,
            CandidateRole::StopLoss => 1,
            CandidateRole::TakeProfit => 2,
        }
    }

    /// Sort a defect summary by (category ordinal, role ordinal) for stable assertions.
    fn sort_summary(summary: &mut [(DefectCategory, CandidateRole)]) {
        summary.sort_by(|(cat_a, role_a), (cat_b, role_b)| {
            cat_a
                .ordinal()
                .cmp(&cat_b.ordinal())
                .then_with(|| role_ord(*role_a).cmp(&role_ord(*role_b)))
        });
    }

    // -- clean cases ---------------------------------------------------------

    #[test]
    fn three_roles_once_each_yields_no_defects() {
        let strategy = strategy_with(vec![
            derivation(occ(10), CandidateRole::Entry),
            derivation(occ(11), CandidateRole::StopLoss),
            derivation(occ(12), CandidateRole::TakeProfit),
        ]);
        assert!(check_13b(&strategy).is_empty());
    }

    #[test]
    fn permutations_of_three_roles_yield_identical_zero_results() {
        let order_a = vec![
            derivation(occ(10), CandidateRole::Entry),
            derivation(occ(11), CandidateRole::StopLoss),
            derivation(occ(12), CandidateRole::TakeProfit),
        ];
        let order_b = vec![
            derivation(occ(10), CandidateRole::TakeProfit),
            derivation(occ(11), CandidateRole::Entry),
            derivation(occ(12), CandidateRole::StopLoss),
        ];
        let order_c = vec![
            derivation(occ(10), CandidateRole::StopLoss),
            derivation(occ(11), CandidateRole::TakeProfit),
            derivation(occ(12), CandidateRole::Entry),
        ];
        assert!(check_13b(&strategy_with(order_a)).is_empty());
        assert!(check_13b(&strategy_with(order_b)).is_empty());
        assert!(check_13b(&strategy_with(order_c)).is_empty());
    }

    // -- missing roles --------------------------------------------------------

    #[test]
    fn one_missing_role_yields_one_missing_defect() {
        let strategy = strategy_with(vec![
            derivation(occ(10), CandidateRole::Entry),
            derivation(occ(11), CandidateRole::StopLoss),
        ]);
        let defects = check_13b(&strategy);
        assert_eq!(defects.len(), 1);
        let mut actual = summarize(&defects);
        sort_summary(&mut actual);
        assert_eq!(
            actual,
            vec![(
                DefectCategory::MissingCandidateDerivation,
                CandidateRole::TakeProfit,
            )]
        );
    }

    #[test]
    fn two_missing_roles_yield_two_missing_defects() {
        let strategy = strategy_with(vec![derivation(occ(10), CandidateRole::Entry)]);
        let defects = check_13b(&strategy);
        assert_eq!(defects.len(), 2);
        let mut actual = summarize(&defects);
        sort_summary(&mut actual);
        assert_eq!(
            actual,
            vec![
                (
                    DefectCategory::MissingCandidateDerivation,
                    CandidateRole::StopLoss,
                ),
                (
                    DefectCategory::MissingCandidateDerivation,
                    CandidateRole::TakeProfit,
                ),
            ]
        );
    }

    #[test]
    fn empty_collection_yields_three_missing_defects() {
        let strategy = strategy_with(vec![]);
        let defects = check_13b(&strategy);
        assert_eq!(defects.len(), 3);
        let mut actual = summarize(&defects);
        sort_summary(&mut actual);
        assert_eq!(
            actual,
            vec![
                (
                    DefectCategory::MissingCandidateDerivation,
                    CandidateRole::Entry,
                ),
                (
                    DefectCategory::MissingCandidateDerivation,
                    CandidateRole::StopLoss,
                ),
                (
                    DefectCategory::MissingCandidateDerivation,
                    CandidateRole::TakeProfit,
                ),
            ]
        );
    }

    #[test]
    fn absent_collection_yields_no_defects_locally() {
        let strategy = Strategy {
            occurrence: occ(1),
            lineage: Declared::Present(lineage(1)),
            semantic_model_version: Declared::Present(VersionToken::new(OpaqueValue::new(1))),
            evidence_declarations: Declared::Present(vec![]),
            configuration_parameters: Declared::Present(vec![]),
            conditions: Declared::Present(vec![]),
            direction_rules: Declared::Present(vec![]),
            candidate_derivations: Declared::Absent,
        };
        assert!(check_13b(&strategy).is_empty());
    }

    // -- duplicate roles -----------------------------------------------------

    #[test]
    fn one_role_twice_yields_one_duplicate_defect() {
        let strategy = strategy_with(vec![
            derivation(occ(10), CandidateRole::Entry),
            derivation(occ(11), CandidateRole::Entry),
            derivation(occ(12), CandidateRole::StopLoss),
            derivation(occ(13), CandidateRole::TakeProfit),
        ]);
        let defects = check_13b(&strategy);
        assert_eq!(defects.len(), 1);
        let mut actual = summarize(&defects);
        sort_summary(&mut actual);
        assert_eq!(
            actual,
            vec![(
                DefectCategory::DuplicateCandidateDerivation,
                CandidateRole::Entry,
            )]
        );
    }

    #[test]
    fn three_occurrences_of_one_role_yield_exactly_one_duplicate_defect() {
        // ENTRY × 3, STOP_LOSS × 1 → ENTRY duplicated once, TAKE_PROFIT missing.
        let strategy = strategy_with(vec![
            derivation(occ(10), CandidateRole::Entry),
            derivation(occ(11), CandidateRole::Entry),
            derivation(occ(12), CandidateRole::Entry),
            derivation(occ(13), CandidateRole::StopLoss),
        ]);
        let defects = check_13b(&strategy);
        assert_eq!(defects.len(), 2);

        let mut actual = summarize(&defects);
        sort_summary(&mut actual);
        assert_eq!(
            actual,
            vec![
                (
                    DefectCategory::MissingCandidateDerivation,
                    CandidateRole::TakeProfit,
                ),
                (
                    DefectCategory::DuplicateCandidateDerivation,
                    CandidateRole::Entry,
                ),
            ]
        );

        // Explicitly prove: exactly one duplicate-ENTRY defect, not two.
        let dup_entry = defects
            .iter()
            .filter(|d| {
                d.category() == DefectCategory::DuplicateCandidateDerivation
                    && matches!(d.discriminator(), Discriminator::Role(CandidateRole::Entry))
            })
            .count();
        assert_eq!(dup_entry, 1);
    }

    #[test]
    fn two_different_duplicated_roles_yield_two_duplicate_defects() {
        let strategy = strategy_with(vec![
            derivation(occ(10), CandidateRole::Entry),
            derivation(occ(11), CandidateRole::Entry),
            derivation(occ(12), CandidateRole::StopLoss),
            derivation(occ(13), CandidateRole::StopLoss),
            derivation(occ(14), CandidateRole::TakeProfit),
        ]);
        let defects = check_13b(&strategy);
        assert_eq!(defects.len(), 2);
        let mut actual = summarize(&defects);
        sort_summary(&mut actual);
        assert_eq!(
            actual,
            vec![
                (
                    DefectCategory::DuplicateCandidateDerivation,
                    CandidateRole::Entry,
                ),
                (
                    DefectCategory::DuplicateCandidateDerivation,
                    CandidateRole::StopLoss,
                ),
            ]
        );
    }

    #[test]
    fn duplicate_plus_missing_coexist() {
        let strategy = strategy_with(vec![
            derivation(occ(10), CandidateRole::Entry),
            derivation(occ(11), CandidateRole::Entry),
            derivation(occ(12), CandidateRole::StopLoss),
        ]);
        let defects = check_13b(&strategy);
        assert_eq!(defects.len(), 2);
        let mut actual = summarize(&defects);
        sort_summary(&mut actual);
        assert_eq!(
            actual,
            vec![
                (
                    DefectCategory::MissingCandidateDerivation,
                    CandidateRole::TakeProfit,
                ),
                (
                    DefectCategory::DuplicateCandidateDerivation,
                    CandidateRole::Entry,
                ),
            ]
        );
    }

    #[test]
    fn all_three_roles_duplicated_yield_three_duplicate_defects() {
        let strategy = strategy_with(vec![
            derivation(occ(10), CandidateRole::Entry),
            derivation(occ(11), CandidateRole::Entry),
            derivation(occ(12), CandidateRole::StopLoss),
            derivation(occ(13), CandidateRole::StopLoss),
            derivation(occ(14), CandidateRole::TakeProfit),
            derivation(occ(15), CandidateRole::TakeProfit),
        ]);
        let defects = check_13b(&strategy);
        assert_eq!(defects.len(), 3);
        let mut actual = summarize(&defects);
        sort_summary(&mut actual);
        assert_eq!(
            actual,
            vec![
                (
                    DefectCategory::DuplicateCandidateDerivation,
                    CandidateRole::Entry,
                ),
                (
                    DefectCategory::DuplicateCandidateDerivation,
                    CandidateRole::StopLoss,
                ),
                (
                    DefectCategory::DuplicateCandidateDerivation,
                    CandidateRole::TakeProfit,
                ),
            ]
        );
    }

    // -- Declared::Absent role boundary --------------------------------------

    #[test]
    fn absent_role_on_one_derivation_contributes_zero_to_that_role_count() {
        // Two derivations: one ENTRY present, one ENTRY absent. ENTRY count = 1.
        // STOP_LOSS and TAKE_PROFIT missing. 13B emits 2 missing defects, not 3.
        let strategy = strategy_with(vec![
            derivation(occ(10), CandidateRole::Entry),
            derivation_absent_role(occ(11)),
        ]);
        let defects = check_13b(&strategy);
        assert_eq!(defects.len(), 2);
        let mut actual = summarize(&defects);
        sort_summary(&mut actual);
        assert_eq!(
            actual,
            vec![
                (
                    DefectCategory::MissingCandidateDerivation,
                    CandidateRole::StopLoss,
                ),
                (
                    DefectCategory::MissingCandidateDerivation,
                    CandidateRole::TakeProfit,
                ),
            ]
        );
    }

    #[test]
    fn three_present_derivations_all_with_absent_roles_yield_three_missing_defects() {
        // All three derivations present but all have `role: Declared::Absent`.
        // Check 13B: ENTRY=0, STOP_LOSS=0, TAKE_PROFIT=0 → 3 missing defects.
        // This is intentional dual reporting: Check 1 reports absent role per derivation
        // (3 × MISSING_REQUIRED_PROPERTY/PROPERTY(2)); Check 13B reports the Strategy-level
        // consequence (3 × MISSING_CANDIDATE_DERIVATION).
        let strategy = strategy_with(vec![
            derivation_absent_role(occ(10)),
            derivation_absent_role(occ(11)),
            derivation_absent_role(occ(12)),
        ]);
        let defects = check_13b(&strategy);
        assert_eq!(defects.len(), 3);
        let mut actual = summarize(&defects);
        sort_summary(&mut actual);
        assert_eq!(
            actual,
            vec![
                (
                    DefectCategory::MissingCandidateDerivation,
                    CandidateRole::Entry,
                ),
                (
                    DefectCategory::MissingCandidateDerivation,
                    CandidateRole::StopLoss,
                ),
                (
                    DefectCategory::MissingCandidateDerivation,
                    CandidateRole::TakeProfit,
                ),
            ]
        );
    }

    // -- other-check ownership -----------------------------------------------

    #[test]
    fn correct_roles_with_absent_expressions_yield_no_13b_defects() {
        let strategy = strategy_with(vec![
            CandidateDerivation {
                occurrence: occ(10),
                lineage: Declared::Present(lineage(10)),
                role: Declared::Present(CandidateRole::Entry),
                expression: Declared::Absent,
            },
            CandidateDerivation {
                occurrence: occ(11),
                lineage: Declared::Present(lineage(11)),
                role: Declared::Present(CandidateRole::StopLoss),
                expression: Declared::Absent,
            },
            CandidateDerivation {
                occurrence: occ(12),
                lineage: Declared::Present(lineage(12)),
                role: Declared::Present(CandidateRole::TakeProfit),
                expression: Declared::Absent,
            },
        ]);
        assert!(check_13b(&strategy).is_empty());
    }

    #[test]
    fn correct_roles_with_absent_lineage_yield_no_13b_defects() {
        let strategy = strategy_with(vec![
            CandidateDerivation {
                occurrence: occ(10),
                lineage: Declared::Absent,
                role: Declared::Present(CandidateRole::Entry),
                expression: Declared::Present(DerivationExpression::Literal(literal(1))),
            },
            CandidateDerivation {
                occurrence: occ(11),
                lineage: Declared::Absent,
                role: Declared::Present(CandidateRole::StopLoss),
                expression: Declared::Present(DerivationExpression::Literal(literal(1))),
            },
            CandidateDerivation {
                occurrence: occ(12),
                lineage: Declared::Absent,
                role: Declared::Present(CandidateRole::TakeProfit),
                expression: Declared::Present(DerivationExpression::Literal(literal(1))),
            },
        ]);
        assert!(check_13b(&strategy).is_empty());
    }

    #[test]
    fn correct_roles_with_unresolved_reference_yield_no_13b_defects() {
        let strategy = strategy_with(vec![
            derivation(occ(10), CandidateRole::Entry),
            derivation(occ(11), CandidateRole::StopLoss),
            CandidateDerivation {
                occurrence: occ(12),
                lineage: Declared::Present(lineage(12)),
                role: Declared::Present(CandidateRole::TakeProfit),
                expression: Declared::Present(DerivationExpression::EvidenceReference(
                    LineageId::new(OpaqueValue::new(999)),
                )),
            },
        ]);
        assert!(check_13b(&strategy).is_empty());
    }

    #[test]
    fn correct_roles_with_dimension_mismatch_yield_no_13b_defects() {
        let strategy = strategy_with(vec![
            derivation(occ(10), CandidateRole::Entry),
            derivation(occ(11), CandidateRole::StopLoss),
            CandidateDerivation {
                occurrence: occ(12),
                lineage: Declared::Present(lineage(12)),
                role: Declared::Present(CandidateRole::TakeProfit),
                expression: Declared::Present(DerivationExpression::Multiply {
                    operand: Box::new(DerivationExpression::Literal(literal(1))),
                    scalar: Literal::Numeric {
                        value: ExactNumber::from_integer(2),
                        dimension: DimensionSymbol::new(OpaqueValue::new(2)),
                    },
                }),
            },
        ]);
        assert!(check_13b(&strategy).is_empty());
    }

    // -- identity ------------------------------------------------------------

    #[test]
    fn missing_role_defect_anchors_on_envelope_occurrence() {
        let strategy = strategy_with(vec![
            derivation(occ(10), CandidateRole::Entry),
            derivation(occ(11), CandidateRole::StopLoss),
        ]);
        let defects = check_13b(&strategy);
        assert_eq!(defects.len(), 1);
        assert_eq!(defects[0].offending(), occ(1));
    }

    #[test]
    fn duplicate_role_defect_anchors_on_envelope_occurrence() {
        let strategy = strategy_with(vec![
            derivation(occ(10), CandidateRole::Entry),
            derivation(occ(11), CandidateRole::Entry),
            derivation(occ(12), CandidateRole::StopLoss),
            derivation(occ(13), CandidateRole::TakeProfit),
        ]);
        let defects = check_13b(&strategy);
        assert_eq!(defects.len(), 1);
        assert_eq!(defects[0].offending(), occ(1));
    }

    #[test]
    fn different_missing_roles_use_distinct_role_discriminators() {
        let strategy = strategy_with(vec![derivation(occ(10), CandidateRole::Entry)]);
        let defects = check_13b(&strategy);
        let roles: Vec<_> = defects
            .iter()
            .map(|d| match d.discriminator() {
                Discriminator::Role(r) => *r,
                _ => panic!("expected ROLE"),
            })
            .collect();
        assert_eq!(
            roles,
            vec![CandidateRole::StopLoss, CandidateRole::TakeProfit]
        );
        assert_ne!(roles[0], roles[1]);
    }

    #[test]
    fn permutation_with_defects_preserves_exact_identities() {
        let order_a = vec![
            derivation(occ(10), CandidateRole::Entry),
            derivation(occ(11), CandidateRole::Entry),
            derivation(occ(12), CandidateRole::TakeProfit),
        ];
        let order_b = vec![
            derivation(occ(10), CandidateRole::TakeProfit),
            derivation(occ(11), CandidateRole::Entry),
            derivation(occ(12), CandidateRole::Entry),
        ];
        let defects_a = check_13b(&strategy_with(order_a));
        let defects_b = check_13b(&strategy_with(order_b));

        assert_eq!(defects_a.len(), 2);
        assert_eq!(defects_b.len(), 2);

        let cats_a: Vec<_> = defects_a.iter().map(|d| d.category()).collect();
        let cats_b: Vec<_> = defects_b.iter().map(|d| d.category()).collect();
        assert_eq!(cats_a, cats_b);

        let anchors_a: Vec<_> = defects_a.iter().map(|d| d.offending()).collect();
        let anchors_b: Vec<_> = defects_b.iter().map(|d| d.offending()).collect();
        assert_eq!(anchors_a, anchors_b);

        let discs_a: Vec<_> = defects_a
            .iter()
            .map(|d| d.discriminator().clone())
            .collect();
        let discs_b: Vec<_> = defects_b
            .iter()
            .map(|d| d.discriminator().clone())
            .collect();
        assert_eq!(discs_a, discs_b);
    }

    // -- multiplicity --------------------------------------------------------

    #[test]
    fn multiplicity_one_per_duplicated_role_not_per_extra_occurrence() {
        // ENTRY appears 5 times. Still exactly one duplicate-ENTRY defect.
        let strategy = strategy_with(vec![
            derivation(occ(10), CandidateRole::Entry),
            derivation(occ(11), CandidateRole::Entry),
            derivation(occ(12), CandidateRole::Entry),
            derivation(occ(13), CandidateRole::Entry),
            derivation(occ(14), CandidateRole::Entry),
            derivation(occ(15), CandidateRole::StopLoss),
        ]);
        let defects = check_13b(&strategy);
        assert_eq!(defects.len(), 2);

        let dup_count = defects
            .iter()
            .filter(|d| d.category() == DefectCategory::DuplicateCandidateDerivation)
            .count();
        assert_eq!(dup_count, 1);

        let missing_count = defects
            .iter()
            .filter(|d| d.category() == DefectCategory::MissingCandidateDerivation)
            .count();
        assert_eq!(missing_count, 1);
    }
}
