//! Check 14B — cross-derivation dimensional agreement — Slice Contract §5.1
//! check 14 (part B), §4.6, §5.2.
//!
//! # What Check 14B owns
//!
//! For a Strategy whose three candidate roles (`ENTRY`, `STOP_LOSS`,
//! `TAKE_PROFIT`) are each present exactly once, and whose three derivation
//! expressions each have a valid §4.6 root result dimension, Check 14B compares
//! the three root dimensions. Disagreement is one
//! [`DefectCategory::CandidateDimensionDivergence`] defect, anchored on the
//! Strategy envelope occurrence identity, `Discriminator::None` (§6.4.2 kind 1)
//! — this is a genuinely Strategy-level property (§5.2), never one per pair or
//! one per role. **Maximum one per Strategy.**
//!
//! # Prerequisites (§5.2), locally reconstructed
//!
//! Check 14B's prerequisite is **13B + 14A on all three derivations** (Slice
//! Contract §5.2 dependency graph). This is a genuine semantic prerequisite, not
//! merely independently-callable modules: a 13B violation (a role missing or
//! duplicated) or a 14A violation on any one of the three derivations (§4.6
//! dimensional invalidity, or a Check-2 leaf fault that 14A itself cannot read a
//! dimension through) makes 14B **non-computable** for that Strategy, and 14B
//! emits nothing — never a defect in the same evaluation as an unresolved 14A
//! prerequisite (Check-14B-discovery reconciliation, ARR-0044 cycle).
//!
//! Phase 5 reconstructs both prerequisites **locally** and calls no prior check
//! in production (no `check_13b`, no `check_14a`, no Check 2, no runner) and
//! emits no `not-evaluated` metadata (that is Phase 6):
//!
//! - **13B reconstruction:** count occurrences of each closed [`CandidateRole`];
//!   each of the three must equal exactly one. `Declared::Absent` role
//!   contributes zero, matching Check 13B's own counting rule.
//! - **14A reconstruction:** [`result_dimension`] walks a role's derivation
//!   expression and returns `Some(dim)` iff every leaf's Check-2 prerequisite
//!   holds (exact-one resolution to a `NUMERIC` declaration carrying a
//!   dimension) **and** every `+`/`-`/`*` node satisfies §4.6 — the same
//!   discipline [`crate::checks::derivation_dimension::check_14a`] uses to decide
//!   what to walk and defect, re-derived locally rather than called.
//!
//! # Ownership boundary — what Check 14B must NOT report
//!
//! Check 14B owns the §4.6 *cross-derivation* agreement fault only. It never
//! re-reports:
//!
//! - a missing or duplicated candidate role — **Check 13B**;
//! - a per-node §4.6 dimensional fault within one derivation — **Check 14A**;
//! - an unresolved/ambiguous reference or non-`NUMERIC` leaf — **Check 2**;
//! - a prohibited §3.8 form — **Check 13A**.
//!
//! [`result_dimension`] emits **no** defects itself — it only reconstructs
//! whether 14A would pass on that derivation and what valid root dimension it
//! would yield.
//!
//! # Determinism
//!
//! Role selection is a fixed 3-slot scan (`ENTRY`, `STOP_LOSS`, `TAKE_PROFIT`),
//! independent of the input `Vec`'s storage order; [`result_dimension`] is the
//! same fixed recursive descent over the owned `Box` tree that 14A uses; leaf
//! resolution is a linear exact-lineage scan. No `HashMap`/`HashSet`, no sort,
//! no ordering-sensitive traversal — ADR-0005 §7.6 needs no new determination.

use crate::model::config::ConfigurationParameter;
use crate::model::declared::{Declared, DeclaredCategory};
use crate::model::derivation::{CandidateRole, DerivationExpression};
use crate::model::evidence::{EvidenceDeclaration, EvidenceTypeTag};
use crate::model::literal::Literal;
use crate::model::opaque::{DimensionSymbol, LineageId};
use crate::model::strategy::Strategy;
use crate::result::category::DefectCategory;
use crate::result::discriminator::Discriminator;
use crate::result::identity::DefectCore;
use crate::semantics::dimension::{
    DimensionBinding, DimensionOutcome, additive_dimension, scaling_dimension,
};

/// Check 14B over a whole Strategy. Returns a deterministic `Vec<DefectCore>`
/// with at most one element.
///
/// Emits `CandidateDimensionDivergence` iff:
/// - each of `ENTRY`, `STOP_LOSS`, `TAKE_PROFIT` occurs exactly once (the local
///   13B reconstruction), and
/// - each of the three derivations' expressions has a valid §4.6 root result
///   dimension (the local 14A reconstruction via [`result_dimension`]), and
/// - the three root dimensions are not all equal.
///
/// Otherwise returns an empty `Vec` — including when any prerequisite above
/// fails, which suppresses 14B entirely rather than reporting anything.
#[must_use]
pub fn check_14b(strategy: &Strategy, binding: &DimensionBinding) -> Vec<DefectCore> {
    let Declared::Present(derivations) = &strategy.candidate_derivations else {
        // Absent collection: 13B prerequisite fails locally (Check 1's, and 13B
        // itself is non-computable), so 14B is suppressed.
        return Vec::new();
    };

    // Local 13B reconstruction: exactly one derivation per role, using the
    // *last* one encountered as the sole candidate for role `r` only when
    // count == 1 — matches Check 13B's counting semantics exactly (missing =
    // 0, duplicate = >= 2, both suppress 14B here).
    let entry = unique_role_derivation(derivations, CandidateRole::Entry);
    let stop_loss = unique_role_derivation(derivations, CandidateRole::StopLoss);
    let take_profit = unique_role_derivation(derivations, CandidateRole::TakeProfit);

    let (Some(entry), Some(stop_loss), Some(take_profit)) = (entry, stop_loss, take_profit) else {
        return Vec::new();
    };

    // Local 14A reconstruction: each role's expression must have a valid root
    // result dimension. An absent expression is Check 1's and non-computable
    // here, exactly as it is for 14A.
    let (Some(entry_dim), Some(stop_loss_dim), Some(take_profit_dim)) = (
        expression_dimension(entry, strategy, binding),
        expression_dimension(stop_loss, strategy, binding),
        expression_dimension(take_profit, strategy, binding),
    ) else {
        return Vec::new();
    };

    if entry_dim == stop_loss_dim && entry_dim == take_profit_dim {
        Vec::new()
    } else {
        vec![DefectCore::new(
            strategy.occurrence,
            DefectCategory::CandidateDimensionDivergence,
            Discriminator::None,
        )]
    }
}

/// The one derivation carrying `role`, or `None` if zero or more than one
/// derivation declares that role — the local 13B exact-one reconstruction.
/// Independent of `derivations`' storage order: the count, not positional
/// selection, decides missing/duplicate.
fn unique_role_derivation(
    derivations: &[crate::model::derivation::CandidateDerivation],
    role: CandidateRole,
) -> Option<&crate::model::derivation::CandidateDerivation> {
    let mut resolved = None;
    for derivation in derivations {
        if derivation.role == Declared::Present(role) {
            if resolved.is_some() {
                return None; // duplicate role ⇒ 13B prerequisite fails
            }
            resolved = Some(derivation);
        }
    }
    resolved
}

/// The §4.6 root result dimension of a candidate derivation's expression, or
/// `None` if the expression is absent or [`result_dimension`] finds it
/// non-computable.
fn expression_dimension(
    derivation: &crate::model::derivation::CandidateDerivation,
    strategy: &Strategy,
    binding: &DimensionBinding,
) -> Option<DimensionSymbol> {
    let Declared::Present(expression) = &derivation.expression else {
        return None;
    };
    result_dimension(expression, strategy, binding)
}

/// Reconstructs, without emitting any defect, whether Check 14A would pass on
/// `expression` and — if so — the one valid §4.6 result dimension its root
/// would yield.
///
/// `Some(dim)` iff the expression is locally resolvable/type-valid, every
/// required Check-2 leaf prerequisite holds, and every `+`/`-`/`*` node
/// satisfies its §4.6 dimensional rule. `None` if any required reference is
/// unresolved/ambiguous, any required leaf is non-`NUMERIC`, any required
/// declaration dimension is absent, any `+`/`-` node has unequal operand
/// dimensions, any `*` node has a non-dimensionless scalar, or any descendant
/// is non-computable. This mirrors
/// [`crate::checks::derivation_dimension::check_14a`]'s own per-node
/// `evaluate` exactly, but never pushes a defect — the caller only needs the
/// `Option`.
fn result_dimension(
    expression: &DerivationExpression,
    strategy: &Strategy,
    binding: &DimensionBinding,
) -> Option<DimensionSymbol> {
    match expression {
        DerivationExpression::Literal(literal) => literal_dimension(literal),
        DerivationExpression::EvidenceReference(referenced) => {
            resolve_evidence_dimension(strategy, *referenced)
        }
        DerivationExpression::ConfigurationReference(referenced) => {
            resolve_configuration_dimension(strategy, *referenced)
        }
        DerivationExpression::Add { left, right }
        | DerivationExpression::Subtract { left, right } => {
            let left_dim = result_dimension(left, strategy, binding);
            let right_dim = result_dimension(right, strategy, binding);
            match (left_dim, right_dim) {
                (Some(l), Some(r)) => match additive_dimension(l, r) {
                    DimensionOutcome::Dimension(result) => Some(result),
                    DimensionOutcome::Mismatch => None,
                },
                _ => None,
            }
        }
        DerivationExpression::Multiply { operand, scalar } => {
            let operand_dim = result_dimension(operand, strategy, binding)?;
            match scaling_dimension(operand_dim, scalar, binding) {
                DimensionOutcome::Dimension(result) => Some(result),
                DimensionOutcome::Mismatch => None,
            }
        }
    }
}

fn literal_dimension(literal: &Literal) -> Option<DimensionSymbol> {
    match literal {
        Literal::Numeric { dimension, .. } => Some(*dimension),
        Literal::Boolean(_) | Literal::Enum(_) => None,
    }
}

fn resolve_evidence_dimension(
    strategy: &Strategy,
    referenced: LineageId,
) -> Option<DimensionSymbol> {
    let referent = resolve_unique_evidence(strategy, referenced)?;
    match &referent.declared_type {
        DeclaredCategory::One(EvidenceTypeTag::Numeric) => match referent.declared_dimension {
            Declared::Present(dimension) => Some(dimension),
            Declared::Absent => None,
        },
        DeclaredCategory::One(EvidenceTypeTag::Boolean | EvidenceTypeTag::Enum)
        | DeclaredCategory::Absent
        | DeclaredCategory::Multiple { .. }
        | DeclaredCategory::OutOfDomain => None,
    }
}

fn resolve_unique_evidence(
    strategy: &Strategy,
    referenced: LineageId,
) -> Option<&EvidenceDeclaration> {
    let Declared::Present(items) = &strategy.evidence_declarations else {
        return None;
    };
    let mut resolved = None;
    for item in items {
        if item.lineage == Declared::Present(referenced) {
            if resolved.is_some() {
                return None;
            }
            resolved = Some(item);
        }
    }
    resolved
}

fn resolve_configuration_dimension(
    strategy: &Strategy,
    referenced: LineageId,
) -> Option<DimensionSymbol> {
    let Declared::Present(items) = &strategy.configuration_parameters else {
        return None;
    };
    let mut resolved: Option<&ConfigurationParameter> = None;
    for item in items {
        if item.lineage == Declared::Present(referenced) {
            if resolved.is_some() {
                return None;
            }
            resolved = Some(item);
        }
    }
    match resolved?.dimension {
        Declared::Present(dimension) => Some(dimension),
        Declared::Absent => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::config::ConfigurationParameter;
    use crate::model::declared::Declared;
    use crate::model::derivation::{CandidateDerivation, CandidateRole, DerivationExpression};
    use crate::model::evidence::EvidenceDeclaration;
    use crate::model::opaque::{DimensionSymbol, LineageId, OccurrenceId, OpaqueValue};
    use crate::model::strategy::Strategy;
    use crate::numeric::ExactNumber;
    use crate::result::category::DefectCategory;
    use crate::result::discriminator::Discriminator;

    // -- fixtures -------------------------------------------------------

    fn occ(handle: u64) -> OccurrenceId {
        OccurrenceId::new(OpaqueValue::new(handle))
    }

    fn lineage(handle: u64) -> LineageId {
        LineageId::new(OpaqueValue::new(handle))
    }

    fn dim(handle: u64) -> DimensionSymbol {
        DimensionSymbol::new(OpaqueValue::new(handle))
    }

    fn binding() -> DimensionBinding {
        DimensionBinding::new(dim(0))
    }

    fn numeric_literal(value: i64, dimension: u64) -> Literal {
        Literal::Numeric {
            value: ExactNumber::from_integer(value),
            dimension: dim(dimension),
        }
    }

    fn dimensionless_literal(value: i64) -> Literal {
        numeric_literal(value, 0)
    }

    fn derivation(
        occ_id: u64,
        role: CandidateRole,
        expression: DerivationExpression,
    ) -> CandidateDerivation {
        CandidateDerivation::new(occ(occ_id), lineage(occ_id), role, expression)
    }

    fn strategy_with(
        evidence: Vec<EvidenceDeclaration>,
        configuration: Vec<ConfigurationParameter>,
        derivations: Vec<CandidateDerivation>,
    ) -> Strategy {
        Strategy {
            occurrence: occ(1),
            lineage: Declared::Present(lineage(1000)),
            semantic_model_version: Declared::Absent,
            evidence_declarations: Declared::Present(evidence),
            configuration_parameters: Declared::Present(configuration),
            conditions: Declared::Present(vec![]),
            direction_rules: Declared::Present(vec![]),
            candidate_derivations: Declared::Present(derivations),
        }
    }

    /// Three roles, each a plain literal of the given dimension.
    fn three_literal_strategy(entry: u64, stop_loss: u64, take_profit: u64) -> Strategy {
        strategy_with(
            vec![],
            vec![],
            vec![
                derivation(
                    10,
                    CandidateRole::Entry,
                    DerivationExpression::Literal(numeric_literal(1, entry)),
                ),
                derivation(
                    11,
                    CandidateRole::StopLoss,
                    DerivationExpression::Literal(numeric_literal(2, stop_loss)),
                ),
                derivation(
                    12,
                    CandidateRole::TakeProfit,
                    DerivationExpression::Literal(numeric_literal(3, take_profit)),
                ),
            ],
        )
    }

    fn is_divergence(defect: &DefectCore) -> bool {
        defect.category() == DefectCategory::CandidateDimensionDivergence
            && *defect.discriminator() == Discriminator::None
    }

    // == PASS: no divergence =============================================

    #[test]
    fn three_literals_same_dimension_no_defect() {
        let s = three_literal_strategy(7, 7, 7);
        assert!(check_14b(&s, &binding()).is_empty());
    }

    #[test]
    fn three_evidence_refs_same_dimension_no_defect() {
        let s = strategy_with(
            vec![
                EvidenceDeclaration::numeric(occ(100), lineage(100), dim(7)),
                EvidenceDeclaration::numeric(occ(101), lineage(101), dim(7)),
                EvidenceDeclaration::numeric(occ(102), lineage(102), dim(7)),
            ],
            vec![],
            vec![
                derivation(
                    10,
                    CandidateRole::Entry,
                    DerivationExpression::EvidenceReference(lineage(100)),
                ),
                derivation(
                    11,
                    CandidateRole::StopLoss,
                    DerivationExpression::EvidenceReference(lineage(101)),
                ),
                derivation(
                    12,
                    CandidateRole::TakeProfit,
                    DerivationExpression::EvidenceReference(lineage(102)),
                ),
            ],
        );
        assert!(check_14b(&s, &binding()).is_empty());
    }

    #[test]
    fn three_config_refs_same_dimension_no_defect() {
        let param = |handle: u64| {
            ConfigurationParameter::new(
                occ(handle),
                lineage(handle),
                dim(7),
                numeric_literal(0, 7),
                numeric_literal(9, 7),
                true,
            )
        };
        let s = strategy_with(
            vec![],
            vec![param(200), param(201), param(202)],
            vec![
                derivation(
                    10,
                    CandidateRole::Entry,
                    DerivationExpression::ConfigurationReference(lineage(200)),
                ),
                derivation(
                    11,
                    CandidateRole::StopLoss,
                    DerivationExpression::ConfigurationReference(lineage(201)),
                ),
                derivation(
                    12,
                    CandidateRole::TakeProfit,
                    DerivationExpression::ConfigurationReference(lineage(202)),
                ),
            ],
        );
        assert!(check_14b(&s, &binding()).is_empty());
    }

    #[test]
    fn three_compound_expressions_same_result_dimension_no_defect() {
        let build = || DerivationExpression::Add {
            left: Box::new(DerivationExpression::Literal(numeric_literal(1, 7))),
            right: Box::new(DerivationExpression::Multiply {
                operand: Box::new(DerivationExpression::Literal(numeric_literal(2, 7))),
                scalar: dimensionless_literal(3),
            }),
        };
        let s = strategy_with(
            vec![],
            vec![],
            vec![
                derivation(10, CandidateRole::Entry, build()),
                derivation(11, CandidateRole::StopLoss, build()),
                derivation(12, CandidateRole::TakeProfit, build()),
            ],
        );
        assert!(check_14b(&s, &binding()).is_empty());
    }

    #[test]
    fn three_dimensionless_results_no_defect() {
        let s = three_literal_strategy(0, 0, 0);
        assert!(check_14b(&s, &binding()).is_empty());
    }

    // == Divergence =======================================================

    #[test]
    fn d1_d1_d2_emits_exactly_one() {
        let s = three_literal_strategy(1, 1, 2);
        let defects = check_14b(&s, &binding());
        assert_eq!(defects.len(), 1);
        assert!(is_divergence(&defects[0]));
    }

    #[test]
    fn d1_d2_d1_emits_exactly_one() {
        let s = three_literal_strategy(1, 2, 1);
        assert_eq!(check_14b(&s, &binding()).len(), 1);
    }

    #[test]
    fn d2_d1_d1_emits_exactly_one() {
        let s = three_literal_strategy(2, 1, 1);
        assert_eq!(check_14b(&s, &binding()).len(), 1);
    }

    #[test]
    fn d1_d2_d3_emits_exactly_one() {
        let s = three_literal_strategy(1, 2, 3);
        assert_eq!(check_14b(&s, &binding()).len(), 1);
    }

    #[test]
    fn dimensionless_d1_d1_emits_exactly_one() {
        let s = three_literal_strategy(0, 1, 1);
        assert_eq!(check_14b(&s, &binding()).len(), 1);
    }

    // == 13B suppression ==================================================

    #[test]
    fn absent_candidate_collection_suppresses_14b() {
        let s = Strategy {
            occurrence: occ(1),
            lineage: Declared::Present(lineage(1000)),
            semantic_model_version: Declared::Absent,
            evidence_declarations: Declared::Present(vec![]),
            configuration_parameters: Declared::Present(vec![]),
            conditions: Declared::Present(vec![]),
            direction_rules: Declared::Present(vec![]),
            candidate_derivations: Declared::Absent,
        };
        assert!(check_14b(&s, &binding()).is_empty());
    }

    #[test]
    fn missing_entry_suppresses_14b() {
        let s = strategy_with(
            vec![],
            vec![],
            vec![
                derivation(
                    11,
                    CandidateRole::StopLoss,
                    DerivationExpression::Literal(numeric_literal(1, 7)),
                ),
                derivation(
                    12,
                    CandidateRole::TakeProfit,
                    DerivationExpression::Literal(numeric_literal(2, 8)),
                ),
            ],
        );
        assert!(check_14b(&s, &binding()).is_empty());
    }

    #[test]
    fn missing_stop_loss_suppresses_14b() {
        let s = strategy_with(
            vec![],
            vec![],
            vec![
                derivation(
                    10,
                    CandidateRole::Entry,
                    DerivationExpression::Literal(numeric_literal(1, 7)),
                ),
                derivation(
                    12,
                    CandidateRole::TakeProfit,
                    DerivationExpression::Literal(numeric_literal(2, 8)),
                ),
            ],
        );
        assert!(check_14b(&s, &binding()).is_empty());
    }

    #[test]
    fn missing_take_profit_suppresses_14b() {
        let s = strategy_with(
            vec![],
            vec![],
            vec![
                derivation(
                    10,
                    CandidateRole::Entry,
                    DerivationExpression::Literal(numeric_literal(1, 7)),
                ),
                derivation(
                    11,
                    CandidateRole::StopLoss,
                    DerivationExpression::Literal(numeric_literal(2, 8)),
                ),
            ],
        );
        assert!(check_14b(&s, &binding()).is_empty());
    }

    #[test]
    fn duplicate_entry_suppresses_14b() {
        let mut s = three_literal_strategy(7, 7, 7);
        if let Declared::Present(derivations) = &mut s.candidate_derivations {
            derivations.push(derivation(
                13,
                CandidateRole::Entry,
                DerivationExpression::Literal(numeric_literal(1, 7)),
            ));
        }
        assert!(check_14b(&s, &binding()).is_empty());
    }

    #[test]
    fn duplicate_stop_loss_suppresses_14b() {
        let mut s = three_literal_strategy(7, 7, 7);
        if let Declared::Present(derivations) = &mut s.candidate_derivations {
            derivations.push(derivation(
                13,
                CandidateRole::StopLoss,
                DerivationExpression::Literal(numeric_literal(1, 7)),
            ));
        }
        assert!(check_14b(&s, &binding()).is_empty());
    }

    #[test]
    fn duplicate_take_profit_suppresses_14b() {
        let mut s = three_literal_strategy(7, 7, 7);
        if let Declared::Present(derivations) = &mut s.candidate_derivations {
            derivations.push(derivation(
                13,
                CandidateRole::TakeProfit,
                DerivationExpression::Literal(numeric_literal(1, 7)),
            ));
        }
        assert!(check_14b(&s, &binding()).is_empty());
    }

    #[test]
    fn absent_role_derivation_contributes_zero() {
        // A derivation with Declared::Absent role contributes to no role count,
        // so it neither fills a role nor causes a duplicate; ENTRY is still
        // missing, suppressing 14B — matches Check 13B's own counting rule.
        let bare = CandidateDerivation {
            occurrence: occ(20),
            lineage: Declared::Present(lineage(20)),
            role: Declared::Absent,
            expression: Declared::Present(DerivationExpression::Literal(numeric_literal(1, 7))),
        };
        let s = strategy_with(
            vec![],
            vec![],
            vec![
                bare,
                derivation(
                    11,
                    CandidateRole::StopLoss,
                    DerivationExpression::Literal(numeric_literal(2, 7)),
                ),
                derivation(
                    12,
                    CandidateRole::TakeProfit,
                    DerivationExpression::Literal(numeric_literal(3, 7)),
                ),
            ],
        );
        assert!(check_14b(&s, &binding()).is_empty());
    }

    // == 14A / Check-2 suppression ========================================

    #[test]
    fn entry_root_add_mismatch_suppresses_14b() {
        let s = strategy_with(
            vec![],
            vec![],
            vec![
                derivation(
                    10,
                    CandidateRole::Entry,
                    DerivationExpression::Add {
                        left: Box::new(DerivationExpression::Literal(numeric_literal(1, 1))),
                        right: Box::new(DerivationExpression::Literal(numeric_literal(2, 2))),
                    },
                ),
                derivation(
                    11,
                    CandidateRole::StopLoss,
                    DerivationExpression::Literal(numeric_literal(1, 7)),
                ),
                derivation(
                    12,
                    CandidateRole::TakeProfit,
                    DerivationExpression::Literal(numeric_literal(1, 7)),
                ),
            ],
        );
        assert!(check_14b(&s, &binding()).is_empty());
    }

    #[test]
    fn entry_nested_mismatch_suppresses_14b() {
        let inner = DerivationExpression::Add {
            left: Box::new(DerivationExpression::Literal(numeric_literal(1, 1))),
            right: Box::new(DerivationExpression::Literal(numeric_literal(2, 2))),
        };
        let root = DerivationExpression::Add {
            left: Box::new(inner),
            right: Box::new(DerivationExpression::Literal(numeric_literal(3, 1))),
        };
        let s = strategy_with(
            vec![],
            vec![],
            vec![
                derivation(10, CandidateRole::Entry, root),
                derivation(
                    11,
                    CandidateRole::StopLoss,
                    DerivationExpression::Literal(numeric_literal(1, 7)),
                ),
                derivation(
                    12,
                    CandidateRole::TakeProfit,
                    DerivationExpression::Literal(numeric_literal(1, 7)),
                ),
            ],
        );
        assert!(check_14b(&s, &binding()).is_empty());
    }

    #[test]
    fn entry_dimensionful_multiply_scalar_suppresses_14b() {
        let s = strategy_with(
            vec![],
            vec![],
            vec![
                derivation(
                    10,
                    CandidateRole::Entry,
                    DerivationExpression::Multiply {
                        operand: Box::new(DerivationExpression::Literal(numeric_literal(4, 7))),
                        scalar: numeric_literal(3, 2),
                    },
                ),
                derivation(
                    11,
                    CandidateRole::StopLoss,
                    DerivationExpression::Literal(numeric_literal(1, 7)),
                ),
                derivation(
                    12,
                    CandidateRole::TakeProfit,
                    DerivationExpression::Literal(numeric_literal(1, 7)),
                ),
            ],
        );
        assert!(check_14b(&s, &binding()).is_empty());
    }

    #[test]
    fn entry_unresolved_evidence_ref_suppresses_14b() {
        let s = strategy_with(
            vec![],
            vec![],
            vec![
                derivation(
                    10,
                    CandidateRole::Entry,
                    DerivationExpression::EvidenceReference(lineage(999)),
                ),
                derivation(
                    11,
                    CandidateRole::StopLoss,
                    DerivationExpression::Literal(numeric_literal(1, 7)),
                ),
                derivation(
                    12,
                    CandidateRole::TakeProfit,
                    DerivationExpression::Literal(numeric_literal(1, 7)),
                ),
            ],
        );
        assert!(check_14b(&s, &binding()).is_empty());
    }

    #[test]
    fn stop_loss_unresolved_ref_suppresses_14b() {
        let s = strategy_with(
            vec![],
            vec![],
            vec![
                derivation(
                    10,
                    CandidateRole::Entry,
                    DerivationExpression::Literal(numeric_literal(1, 7)),
                ),
                derivation(
                    11,
                    CandidateRole::StopLoss,
                    DerivationExpression::EvidenceReference(lineage(999)),
                ),
                derivation(
                    12,
                    CandidateRole::TakeProfit,
                    DerivationExpression::Literal(numeric_literal(1, 7)),
                ),
            ],
        );
        assert!(check_14b(&s, &binding()).is_empty());
    }

    #[test]
    fn take_profit_unresolved_ref_suppresses_14b() {
        let s = strategy_with(
            vec![],
            vec![],
            vec![
                derivation(
                    10,
                    CandidateRole::Entry,
                    DerivationExpression::Literal(numeric_literal(1, 7)),
                ),
                derivation(
                    11,
                    CandidateRole::StopLoss,
                    DerivationExpression::Literal(numeric_literal(1, 7)),
                ),
                derivation(
                    12,
                    CandidateRole::TakeProfit,
                    DerivationExpression::EvidenceReference(lineage(999)),
                ),
            ],
        );
        assert!(check_14b(&s, &binding()).is_empty());
    }

    #[test]
    fn ambiguous_evidence_ref_suppresses_14b() {
        let s = strategy_with(
            vec![
                EvidenceDeclaration::numeric(occ(100), lineage(100), dim(1)),
                EvidenceDeclaration::numeric(occ(101), lineage(100), dim(2)),
            ],
            vec![],
            vec![
                derivation(
                    10,
                    CandidateRole::Entry,
                    DerivationExpression::EvidenceReference(lineage(100)),
                ),
                derivation(
                    11,
                    CandidateRole::StopLoss,
                    DerivationExpression::Literal(numeric_literal(1, 7)),
                ),
                derivation(
                    12,
                    CandidateRole::TakeProfit,
                    DerivationExpression::Literal(numeric_literal(1, 7)),
                ),
            ],
        );
        assert!(check_14b(&s, &binding()).is_empty());
    }

    #[test]
    fn ambiguous_config_ref_suppresses_14b() {
        let param = |handle: u64, d: u64| {
            ConfigurationParameter::new(
                occ(handle),
                lineage(200),
                dim(d),
                numeric_literal(0, d),
                numeric_literal(9, d),
                true,
            )
        };
        let s = strategy_with(
            vec![],
            vec![param(200, 1), param(201, 2)],
            vec![
                derivation(
                    10,
                    CandidateRole::Entry,
                    DerivationExpression::ConfigurationReference(lineage(200)),
                ),
                derivation(
                    11,
                    CandidateRole::StopLoss,
                    DerivationExpression::Literal(numeric_literal(1, 7)),
                ),
                derivation(
                    12,
                    CandidateRole::TakeProfit,
                    DerivationExpression::Literal(numeric_literal(1, 7)),
                ),
            ],
        );
        assert!(check_14b(&s, &binding()).is_empty());
    }

    #[test]
    fn non_numeric_evidence_ref_suppresses_14b() {
        let s = strategy_with(
            vec![EvidenceDeclaration::boolean(occ(100), lineage(100))],
            vec![],
            vec![
                derivation(
                    10,
                    CandidateRole::Entry,
                    DerivationExpression::EvidenceReference(lineage(100)),
                ),
                derivation(
                    11,
                    CandidateRole::StopLoss,
                    DerivationExpression::Literal(numeric_literal(1, 7)),
                ),
                derivation(
                    12,
                    CandidateRole::TakeProfit,
                    DerivationExpression::Literal(numeric_literal(1, 7)),
                ),
            ],
        );
        assert!(check_14b(&s, &binding()).is_empty());
    }

    #[test]
    fn absent_expression_on_entry_suppresses_14b() {
        let bare = CandidateDerivation {
            occurrence: occ(10),
            lineage: Declared::Present(lineage(10)),
            role: Declared::Present(CandidateRole::Entry),
            expression: Declared::Absent,
        };
        let s = strategy_with(
            vec![],
            vec![],
            vec![
                bare,
                derivation(
                    11,
                    CandidateRole::StopLoss,
                    DerivationExpression::Literal(numeric_literal(1, 7)),
                ),
                derivation(
                    12,
                    CandidateRole::TakeProfit,
                    DerivationExpression::Literal(numeric_literal(1, 7)),
                ),
            ],
        );
        assert!(check_14b(&s, &binding()).is_empty());
    }

    #[test]
    fn absent_expression_on_stop_loss_suppresses_14b() {
        let bare = CandidateDerivation {
            occurrence: occ(11),
            lineage: Declared::Present(lineage(11)),
            role: Declared::Present(CandidateRole::StopLoss),
            expression: Declared::Absent,
        };
        let s = strategy_with(
            vec![],
            vec![],
            vec![
                derivation(
                    10,
                    CandidateRole::Entry,
                    DerivationExpression::Literal(numeric_literal(1, 7)),
                ),
                bare,
                derivation(
                    12,
                    CandidateRole::TakeProfit,
                    DerivationExpression::Literal(numeric_literal(1, 7)),
                ),
            ],
        );
        assert!(check_14b(&s, &binding()).is_empty());
    }

    #[test]
    fn absent_expression_on_take_profit_suppresses_14b() {
        let bare = CandidateDerivation {
            occurrence: occ(12),
            lineage: Declared::Present(lineage(12)),
            role: Declared::Present(CandidateRole::TakeProfit),
            expression: Declared::Absent,
        };
        let s = strategy_with(
            vec![],
            vec![],
            vec![
                derivation(
                    10,
                    CandidateRole::Entry,
                    DerivationExpression::Literal(numeric_literal(1, 7)),
                ),
                derivation(
                    11,
                    CandidateRole::StopLoss,
                    DerivationExpression::Literal(numeric_literal(1, 7)),
                ),
                bare,
            ],
        );
        assert!(check_14b(&s, &binding()).is_empty());
    }

    // == corrected invariant ==============================================

    #[test]
    fn strategy_with_14a_dimension_mismatch_never_also_receives_a_14b_defect() {
        // ENTRY has a genuine 14A DIMENSION_MISMATCH at its root; verify
        // independently that check_14a (the real check) emits a defect here
        // while check_14b (this check) emits none in the same evaluation —
        // the corrected Check-14B-discovery invariant.
        use crate::checks::derivation_dimension::check_14a;

        let entry_expr = DerivationExpression::Add {
            left: Box::new(DerivationExpression::Literal(numeric_literal(1, 1))),
            right: Box::new(DerivationExpression::Literal(numeric_literal(2, 2))),
        };
        let s = strategy_with(
            vec![],
            vec![],
            vec![
                derivation(10, CandidateRole::Entry, entry_expr),
                derivation(
                    11,
                    CandidateRole::StopLoss,
                    DerivationExpression::Literal(numeric_literal(1, 7)),
                ),
                derivation(
                    12,
                    CandidateRole::TakeProfit,
                    DerivationExpression::Literal(numeric_literal(1, 8)),
                ),
            ],
        );

        let a_defects = check_14a(&s, &binding());
        assert_eq!(a_defects.len(), 1, "14A must report its own mismatch");

        let b_defects = check_14b(&s, &binding());
        assert!(
            b_defects.is_empty(),
            "14B must never emit while a prerequisite 14A defect exists"
        );
    }

    // == identity ==========================================================

    #[test]
    fn divergence_defect_identity() {
        let s = three_literal_strategy(1, 1, 2);
        let defects = check_14b(&s, &binding());
        assert_eq!(defects.len(), 1);
        let defect = &defects[0];
        assert_eq!(defect.offending(), s.occurrence);
        assert_eq!(
            defect.category(),
            DefectCategory::CandidateDimensionDivergence
        );
        assert_eq!(*defect.discriminator(), Discriminator::None);
    }

    // == determinism ========================================================

    #[test]
    fn candidate_derivation_permutation_does_not_change_result() {
        let s_a = three_literal_strategy(1, 1, 2);
        let mut s_b = s_a.clone();
        if let Declared::Present(derivations) = &mut s_b.candidate_derivations {
            derivations.reverse();
        }
        let da = check_14b(&s_a, &binding());
        let db = check_14b(&s_b, &binding());
        assert_eq!(da.len(), 1);
        assert_eq!(db.len(), 1);
        assert_eq!(da[0], db[0]);
    }

    #[test]
    fn evidence_declaration_permutation_does_not_change_result() {
        let ev_a = vec![
            EvidenceDeclaration::numeric(occ(100), lineage(100), dim(7)),
            EvidenceDeclaration::numeric(occ(101), lineage(101), dim(9)),
        ];
        let mut ev_b = ev_a.clone();
        ev_b.reverse();
        let derivations = || {
            vec![
                derivation(
                    10,
                    CandidateRole::Entry,
                    DerivationExpression::EvidenceReference(lineage(100)),
                ),
                derivation(
                    11,
                    CandidateRole::StopLoss,
                    DerivationExpression::EvidenceReference(lineage(101)),
                ),
                derivation(
                    12,
                    CandidateRole::TakeProfit,
                    DerivationExpression::Literal(numeric_literal(1, 7)),
                ),
            ]
        };
        let s_a = strategy_with(ev_a, vec![], derivations());
        let s_b = strategy_with(ev_b, vec![], derivations());
        let da = check_14b(&s_a, &binding());
        let db = check_14b(&s_b, &binding());
        assert_eq!(da.len(), 1);
        assert_eq!(db.len(), 1);
        assert_eq!(da[0], db[0]);
    }

    #[test]
    fn configuration_declaration_permutation_does_not_change_result() {
        let cfg_a = vec![
            ConfigurationParameter::new(
                occ(200),
                lineage(200),
                dim(7),
                numeric_literal(0, 7),
                numeric_literal(9, 7),
                true,
            ),
            ConfigurationParameter::new(
                occ(201),
                lineage(201),
                dim(9),
                numeric_literal(0, 9),
                numeric_literal(9, 9),
                true,
            ),
        ];
        let mut cfg_b = cfg_a.clone();
        cfg_b.reverse();
        let derivations = || {
            vec![
                derivation(
                    10,
                    CandidateRole::Entry,
                    DerivationExpression::ConfigurationReference(lineage(200)),
                ),
                derivation(
                    11,
                    CandidateRole::StopLoss,
                    DerivationExpression::ConfigurationReference(lineage(201)),
                ),
                derivation(
                    12,
                    CandidateRole::TakeProfit,
                    DerivationExpression::Literal(numeric_literal(1, 7)),
                ),
            ]
        };
        let s_a = strategy_with(vec![], cfg_a, derivations());
        let s_b = strategy_with(vec![], cfg_b, derivations());
        let da = check_14b(&s_a, &binding());
        let db = check_14b(&s_b, &binding());
        assert_eq!(da.len(), 1);
        assert_eq!(db.len(), 1);
        assert_eq!(da[0], db[0]);
    }

    #[test]
    fn repeated_identical_evaluation_produces_identical_defect() {
        let s = three_literal_strategy(1, 1, 2);
        let d1 = check_14b(&s, &binding());
        let d2 = check_14b(&s, &binding());
        assert_eq!(d1, d2);
    }
}
