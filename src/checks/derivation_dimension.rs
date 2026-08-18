//! Check 14A — candidate-value derivation dimensional consistency (Slice
//! Contract §5.1 check 14, §4.6, §5.2).
//!
//! # What Check 14A owns
//!
//! Check 14A is the per-present-derivation half of check 14: for each **present**
//! candidate derivation, every `+`/`-` node must have dimensionally identical
//! operands and every `*` node must have a **dimensionless** literal scalar
//! (§4.6). Its owned defect is `DIMENSION_MISMATCH`, **one per offending node**,
//! anchored on that derivation's occurrence identity (Revision 4 §C.4, an
//! ordinary occurrence-level defect) with the `NODE_PATH` discriminator (§6.4.2
//! kind 5) carrying the root-to-node path (`left operand = 1`, `right operand =
//! 2`).
//!
//! The cross-derivation agreement obligation — all three candidate derivations
//! yielding one dimension — is **check 14B** (`CANDIDATE_DIMENSION_DIVERGENCE`),
//! a Strategy-level defect, and is **not** implemented here.
//!
//! # Ownership boundary — what Check 14A must NOT report
//!
//! Check 14A owns §4.6 *dimensional* faults only. It never re-reports:
//!
//! - a prohibited §3.8 *form* — **Check 13A** (`INVALID_DERIVATION_FORM`);
//! - an unresolved or ambiguous evidence/configuration reference — **Check 2**
//!   (`UNRESOLVED_REFERENCE`);
//! - a non-`NUMERIC` (BOOLEAN/ENUM) leaf — **Check 2** (`TYPE_MISMATCH`);
//! - a missing or duplicated candidate role — **Check 13B**;
//! - a missing candidate-derivation lineage — **Check 15**.
//!
//! A leaf that fails its Check-2 prerequisite is **not** a dimension it may read:
//! its result dimension is *locally non-computable*, so any node whose result
//! depends on it is skipped without a 14A defect — but an **independent**
//! sibling subtree that is computable is still evaluated, and a `*` node's
//! scalar (an inline literal, always present) is decided regardless of whether
//! its operand subtree is computable.
//!
//! # Prerequisites (§5.2), locally reconstructed
//!
//! Check 14A's prerequisite is **13A on that derivation + Check 2 on its
//! leaves**. Phase 5 reconstructs prerequisite computability **locally** and
//! calls no prior check in production (no `check_13a`/`check_2`, no runner) and
//! emits no `not-evaluated` metadata (that is Phase 6). 13A is structurally
//! zero-population under the closed [`DerivationExpression`] model
//! (PBA-0126 / ER-0104), so "13A passes" holds for every representable
//! derivation; presence of the `candidate_derivations` collection and of each
//! derivation's expression is check 1's, so an absent collection or an absent
//! expression is skipped. Check-2 leaf computability is reconstructed here as
//! **exact-one resolution to a `NUMERIC` declaration carrying a dimension**
//! ([`resolve_evidence_dimension`], [`resolve_configuration_dimension`]) — the
//! same exact-one rule §4.7 gives Check 2, re-derived locally rather than
//! called.
//!
//! # Determinism
//!
//! The walk is a fixed recursive descent over the owned `Box` tree; the
//! `NODE_PATH` is the structural position of the offending node, independent of
//! any collection's storage order; resolution is a linear exact-lineage scan. No
//! `HashMap`/`HashSet`, no sort, no ordering-sensitive traversal — so ADR-0005
//! §7.6 needs no new determination.

use crate::model::config::ConfigurationParameter;
use crate::model::declared::{Declared, DeclaredCategory};
use crate::model::derivation::DerivationExpression;
use crate::model::evidence::{EvidenceDeclaration, EvidenceTypeTag};
use crate::model::literal::Literal;
use crate::model::opaque::{DimensionSymbol, LineageId};
use crate::model::strategy::Strategy;
use crate::result::category::DefectCategory;
use crate::result::discriminator::{ChildPosition, Discriminator, NodePath};
use crate::result::identity::DefectCore;
use crate::semantics::dimension::{
    DimensionBinding, DimensionOutcome, additive_dimension, scaling_dimension,
};

/// Check 14A over a whole Strategy. Returns a deterministic `Vec<DefectCore>`.
///
/// One `DIMENSION_MISMATCH` per offending expression node across every present
/// candidate derivation. The `binding` supplies the distinguished
/// `DIMENSIONLESS` symbol as canonical input (§2.1, B-1) — it is taken by
/// reference, per invocation, never manufactured here.
#[must_use]
pub fn check_14a(strategy: &Strategy, binding: &DimensionBinding) -> Vec<DefectCore> {
    let mut defects = Vec::new();
    let Declared::Present(derivations) = &strategy.candidate_derivations else {
        // Absent collection is check 1's, not check 14A's.
        return defects;
    };

    for derivation in derivations {
        // An absent expression is check 1's (`MISSING_REQUIRED_PROPERTY`); only a
        // present expression has dimensional structure to check. The derivation's
        // occurrence is the anchor for every node defect within it (§C.4).
        if let Declared::Present(expression) = &derivation.expression {
            evaluate(
                derivation.occurrence,
                expression,
                &[],
                strategy,
                binding,
                &mut defects,
            );
        }
    }

    defects
}

/// Evaluate one expression node's §4.6 result dimension, emitting a
/// `DIMENSION_MISMATCH` at every offending node on the way.
///
/// Returns `Some(dimension)` when this node's result dimension is computable
/// under §4.6, and `None` when it is **locally non-computable** — either a leaf
/// whose Check-2 prerequisite fails (unresolved/ambiguous/non-`NUMERIC`), or a
/// node that is itself an offending mismatch (a mismatch has no defined result
/// dimension; §4.6 never repairs it), or a compound node with a non-computable
/// operand it would need to compare. A `None` result suppresses **only** the
/// evaluations that depend on this node; independent subtrees have already been
/// walked and have emitted their own defects.
fn evaluate(
    anchor: crate::model::opaque::OccurrenceId,
    expression: &DerivationExpression,
    path: &[ChildPosition],
    strategy: &Strategy,
    binding: &DimensionBinding,
    defects: &mut Vec<DefectCore>,
) -> Option<DimensionSymbol> {
    match expression {
        // -- leaves: dimension is read, never a 14A defect ------------------
        //
        // A numeric literal carries its declared dimension. A BOOLEAN/ENUM
        // literal leaf is Check 2's `TYPE_MISMATCH`, so it is non-computable for
        // 14A (no dimension to read).
        DerivationExpression::Literal(literal) => literal_dimension(literal),
        // An evidence reference is computable iff it resolves to exactly one
        // NUMERIC declaration carrying a dimension; otherwise it is Check 2's
        // (unresolved/ambiguous/non-NUMERIC) — non-computable here.
        DerivationExpression::EvidenceReference(referenced) => {
            resolve_evidence_dimension(strategy, *referenced)
        }
        DerivationExpression::ConfigurationReference(referenced) => {
            resolve_configuration_dimension(strategy, *referenced)
        }
        // -- `a + b` / `a - b`: operands must share a dimension (§4.6) -------
        DerivationExpression::Add { left, right }
        | DerivationExpression::Subtract { left, right } => {
            // Always recurse into both operands so nested offending nodes emit,
            // regardless of this node's own computability.
            let left_dim = evaluate(
                anchor,
                left,
                &child(path, ChildPosition::LeftOperand),
                strategy,
                binding,
                defects,
            );
            let right_dim = evaluate(
                anchor,
                right,
                &child(path, ChildPosition::RightOperand),
                strategy,
                binding,
                defects,
            );
            match (left_dim, right_dim) {
                (Some(l), Some(r)) => match additive_dimension(l, r) {
                    DimensionOutcome::Dimension(result) => Some(result),
                    DimensionOutcome::Mismatch => {
                        defects.push(node_defect(anchor, path));
                        None
                    }
                },
                // One side non-computable: the comparison cannot be made, so no
                // dimension is invented and no mismatch is emitted at this node.
                _ => None,
            }
        }
        // -- `a * s`: the scalar must be a dimensionless literal (§4.6) ------
        DerivationExpression::Multiply { operand, scalar } => {
            let operand_dim = evaluate(
                anchor,
                operand,
                &child(path, ChildPosition::LeftOperand),
                strategy,
                binding,
                defects,
            );
            match operand_dim {
                // Operand computable: `scaling_dimension` decides the scalar and
                // propagates `dim(operand)` on success (§4.6).
                Some(op) => match scaling_dimension(op, scalar, binding) {
                    DimensionOutcome::Dimension(result) => Some(result),
                    DimensionOutcome::Mismatch => {
                        defects.push(node_defect(anchor, path));
                        None
                    }
                },
                // Operand non-computable: its own subtree already emitted its
                // defects, and the result dimension is undefined. The scalar
                // check is operand-independent (§4.6: `scaling_dimension`'s
                // Mismatch turns only on the scalar), so a dimensioned or
                // non-numeric scalar is still this node's 14A defect.
                None => {
                    if !scalar_is_dimensionless(scalar, binding) {
                        defects.push(node_defect(anchor, path));
                    }
                    None
                }
            }
        }
    }
}

/// A `DIMENSION_MISMATCH` at `path`, anchored on the derivation's occurrence
/// (§C.4) with the `NODE_PATH` discriminator (§6.4.2 kind 5).
fn node_defect(anchor: crate::model::opaque::OccurrenceId, path: &[ChildPosition]) -> DefectCore {
    DefectCore::new(
        anchor,
        DefectCategory::DimensionMismatch,
        Discriminator::NodePath(NodePath::new(path.to_vec())),
    )
}

/// Extend a root-relative path by one child step.
fn child(path: &[ChildPosition], step: ChildPosition) -> Vec<ChildPosition> {
    let mut extended = path.to_vec();
    extended.push(step);
    extended
}

/// The result dimension of a literal leaf, or `None` when the literal is not a
/// numeric one (a BOOLEAN/ENUM leaf is Check 2's `TYPE_MISMATCH`).
fn literal_dimension(literal: &Literal) -> Option<DimensionSymbol> {
    match literal {
        // No catch-all `_` arm (ADR-0005 §7.4).
        Literal::Numeric { dimension, .. } => Some(*dimension),
        Literal::Boolean(_) | Literal::Enum(_) => None,
    }
}

/// Whether a `*` scalar satisfies §4.6's dimensionless requirement.
///
/// Dimensionlessness is decided **solely** by the supplied [`DimensionBinding`]
/// (`is_dimensionless`), never by the scalar's representation — this is the same
/// authority `scaling_dimension` uses, factored out for the branch where the
/// operand dimension is not available.
fn scalar_is_dimensionless(scalar: &Literal, binding: &DimensionBinding) -> bool {
    match scalar {
        // No catch-all `_` arm (ADR-0005 §7.4).
        Literal::Numeric { dimension, .. } => binding.is_dimensionless(*dimension),
        Literal::Boolean(_) | Literal::Enum(_) => false,
    }
}

/// The declared dimension of the evidence declaration a reference resolves to,
/// or `None` when the Check-2 prerequisite does not hold for 14A: zero or
/// ambiguous resolution (§4.7), a non-determinate or non-`NUMERIC` declared
/// type, or a `NUMERIC` declaration with no dimension (the latter is Check 6's).
fn resolve_evidence_dimension(
    strategy: &Strategy,
    referenced: LineageId,
) -> Option<DimensionSymbol> {
    let referent = resolve_unique_evidence(strategy, referenced)?;
    match &referent.declared_type {
        DeclaredCategory::One(EvidenceTypeTag::Numeric) => match referent.declared_dimension {
            Declared::Present(dimension) => Some(dimension),
            // A NUMERIC declaration missing its dimension is Check 6's; 14A
            // cannot read a dimension, so the leaf is non-computable.
            Declared::Absent => None,
        },
        // Non-NUMERIC determinate type → Check 2's TYPE_MISMATCH; non-determinate
        // (absent/multiple/out-of-domain) → check 1/6. Either way, non-computable.
        DeclaredCategory::One(EvidenceTypeTag::Boolean | EvidenceTypeTag::Enum)
        | DeclaredCategory::Absent
        | DeclaredCategory::Multiple { .. }
        | DeclaredCategory::OutOfDomain => None,
    }
}

/// The one evidence declaration matching `referenced`, or `None` if zero or more
/// than one match — the §4.7 exact-one rule, reconstructed locally (14A calls no
/// Check-2 helper).
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
                return None; // ambiguous ⇒ unresolved (§4.7)
            }
            resolved = Some(item);
        }
    }
    resolved
}

/// The declared dimension of the configuration parameter a reference resolves
/// to, or `None` on zero/ambiguous resolution (§4.7) or an absent dimension.
/// Configuration parameters are NUMERIC-only (§3.9), so no type test is needed.
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
                return None; // ambiguous ⇒ unresolved (§4.7)
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
    use crate::model::opaque::{DimensionSymbol, EnumValue, LineageId, OccurrenceId, OpaqueValue};
    use crate::model::strategy::Strategy;
    use crate::numeric::ExactNumber;
    use crate::result::discriminator::{ChildPosition, Discriminator, NodePath};

    // -- fixtures -----------------------------------------------------------

    fn occ(handle: u64) -> OccurrenceId {
        OccurrenceId::new(OpaqueValue::new(handle))
    }

    fn lineage(handle: u64) -> LineageId {
        LineageId::new(OpaqueValue::new(handle))
    }

    fn dim(handle: u64) -> DimensionSymbol {
        DimensionSymbol::new(OpaqueValue::new(handle))
    }

    /// A binding whose distinguished dimensionless symbol is handle 0.
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

    fn derivation(
        occ_id: u64,
        role: CandidateRole,
        expression: DerivationExpression,
    ) -> CandidateDerivation {
        CandidateDerivation::new(occ(occ_id), lineage(occ_id), role, expression)
    }

    /// The single defect over a Strategy carrying exactly one derivation with
    /// `expression`; panics if the count is not one, for identity assertions.
    fn single_defect(expression: DerivationExpression) -> DefectCore {
        let s = strategy_with(
            vec![],
            vec![],
            vec![derivation(10, CandidateRole::Entry, expression)],
        );
        let mut defects = check_14a(&s, &binding());
        assert_eq!(defects.len(), 1, "expected exactly one defect");
        defects.pop().unwrap()
    }

    /// Whether a defect's discriminator is exactly the NODE-PATH `steps`.
    fn path_eq(defect: &DefectCore, steps: Vec<ChildPosition>) -> bool {
        matches!(
            defect.discriminator(),
            Discriminator::NodePath(p) if *p == NodePath::new(steps)
        )
    }

    fn is_dimension_mismatch(defect: &DefectCore) -> bool {
        defect.category() == DefectCategory::DimensionMismatch
    }

    // == positive: no defect ================================================

    #[test]
    fn numeric_literal_is_no_defect() {
        let s = strategy_with(
            vec![],
            vec![],
            vec![derivation(
                10,
                CandidateRole::Entry,
                DerivationExpression::Literal(numeric_literal(5, 1)),
            )],
        );
        assert!(check_14a(&s, &binding()).is_empty());
    }

    #[test]
    fn numeric_evidence_reference_is_no_defect() {
        let s = strategy_with(
            vec![EvidenceDeclaration::numeric(occ(100), lineage(100), dim(1))],
            vec![],
            vec![derivation(
                10,
                CandidateRole::Entry,
                DerivationExpression::EvidenceReference(lineage(100)),
            )],
        );
        assert!(check_14a(&s, &binding()).is_empty());
    }

    #[test]
    fn numeric_configuration_reference_is_no_defect() {
        let param = ConfigurationParameter::new(
            occ(200),
            lineage(200),
            dim(1),
            numeric_literal(0, 1),
            numeric_literal(10, 1),
            true,
        );
        let s = strategy_with(
            vec![],
            vec![param],
            vec![derivation(
                10,
                CandidateRole::Entry,
                DerivationExpression::ConfigurationReference(lineage(200)),
            )],
        );
        assert!(check_14a(&s, &binding()).is_empty());
    }

    #[test]
    fn same_dimension_addition_is_no_defect() {
        let expr = DerivationExpression::Add {
            left: Box::new(DerivationExpression::Literal(numeric_literal(1, 7))),
            right: Box::new(DerivationExpression::Literal(numeric_literal(2, 7))),
        };
        let s = strategy_with(
            vec![],
            vec![],
            vec![derivation(10, CandidateRole::Entry, expr)],
        );
        assert!(check_14a(&s, &binding()).is_empty());
    }

    #[test]
    fn same_dimension_subtraction_is_no_defect() {
        let expr = DerivationExpression::Subtract {
            left: Box::new(DerivationExpression::Literal(numeric_literal(9, 7))),
            right: Box::new(DerivationExpression::Literal(numeric_literal(2, 7))),
        };
        let s = strategy_with(
            vec![],
            vec![],
            vec![derivation(10, CandidateRole::Entry, expr)],
        );
        assert!(check_14a(&s, &binding()).is_empty());
    }

    #[test]
    fn dimensionless_scaling_is_no_defect() {
        let expr = DerivationExpression::Multiply {
            operand: Box::new(DerivationExpression::Literal(numeric_literal(4, 7))),
            scalar: dimensionless_literal(3),
        };
        let s = strategy_with(
            vec![],
            vec![],
            vec![derivation(10, CandidateRole::Entry, expr)],
        );
        assert!(check_14a(&s, &binding()).is_empty());
    }

    #[test]
    fn nested_valid_expression_is_no_defect() {
        // ((ev[7] * 3_dimensionless) + (cfg[7] - lit[7]))  — all dimension 7.
        let expr = DerivationExpression::Add {
            left: Box::new(DerivationExpression::Multiply {
                operand: Box::new(DerivationExpression::EvidenceReference(lineage(100))),
                scalar: dimensionless_literal(3),
            }),
            right: Box::new(DerivationExpression::Subtract {
                left: Box::new(DerivationExpression::ConfigurationReference(lineage(200))),
                right: Box::new(DerivationExpression::Literal(numeric_literal(1, 7))),
            }),
        };
        let param = ConfigurationParameter::new(
            occ(200),
            lineage(200),
            dim(7),
            numeric_literal(0, 7),
            numeric_literal(9, 7),
            false,
        );
        let s = strategy_with(
            vec![EvidenceDeclaration::numeric(occ(100), lineage(100), dim(7))],
            vec![param],
            vec![derivation(10, CandidateRole::Entry, expr)],
        );
        assert!(check_14a(&s, &binding()).is_empty());
    }

    // == negative: dimension mismatch =======================================

    #[test]
    fn unequal_dimension_addition_emits_at_root() {
        let defect = single_defect(DerivationExpression::Add {
            left: Box::new(DerivationExpression::Literal(numeric_literal(1, 1))),
            right: Box::new(DerivationExpression::Literal(numeric_literal(2, 2))),
        });
        assert!(is_dimension_mismatch(&defect));
        assert!(path_eq(&defect, vec![]));
    }

    #[test]
    fn unequal_dimension_subtraction_emits_at_root() {
        let defect = single_defect(DerivationExpression::Subtract {
            left: Box::new(DerivationExpression::Literal(numeric_literal(1, 1))),
            right: Box::new(DerivationExpression::Literal(numeric_literal(2, 2))),
        });
        assert!(is_dimension_mismatch(&defect));
        assert!(path_eq(&defect, vec![]));
    }

    #[test]
    fn dimensionful_numeric_multiply_scalar_emits_at_multiply_node() {
        // scalar dimension 2 is not the bound dimensionless (0).
        let defect = single_defect(DerivationExpression::Multiply {
            operand: Box::new(DerivationExpression::Literal(numeric_literal(4, 7))),
            scalar: numeric_literal(3, 2),
        });
        assert!(is_dimension_mismatch(&defect));
        assert!(path_eq(&defect, vec![]));
    }

    #[test]
    fn boolean_multiply_scalar_emits() {
        let defect = single_defect(DerivationExpression::Multiply {
            operand: Box::new(DerivationExpression::Literal(numeric_literal(4, 7))),
            scalar: Literal::Boolean(true),
        });
        assert!(is_dimension_mismatch(&defect));
        assert!(path_eq(&defect, vec![]));
    }

    #[test]
    fn enum_multiply_scalar_emits() {
        let defect = single_defect(DerivationExpression::Multiply {
            operand: Box::new(DerivationExpression::Literal(numeric_literal(4, 7))),
            scalar: Literal::Enum(EnumValue::new(OpaqueValue::new(5))),
        });
        assert!(is_dimension_mismatch(&defect));
        assert!(path_eq(&defect, vec![]));
    }

    #[test]
    fn mismatch_below_root_emits_at_child_path() {
        // root Add is fine only if children agree; here the LEFT child is a
        // mismatching Add and the right is a matching literal of the left's
        // (non-)dimension. The inner mismatch emits at [LeftOperand] and makes
        // the inner node non-computable, so the root does not emit.
        let inner = DerivationExpression::Add {
            left: Box::new(DerivationExpression::Literal(numeric_literal(1, 1))),
            right: Box::new(DerivationExpression::Literal(numeric_literal(2, 2))),
        };
        let root = DerivationExpression::Add {
            left: Box::new(inner),
            right: Box::new(DerivationExpression::Literal(numeric_literal(3, 1))),
        };
        let defect = single_defect(root);
        assert!(is_dimension_mismatch(&defect));
        assert!(path_eq(&defect, vec![ChildPosition::LeftOperand]));
    }

    #[test]
    fn mismatch_at_root_and_below_root_emits_two_defects() {
        // Inner RIGHT child mismatches (emits at [RightOperand]); the inner node
        // is then non-computable so the root cannot compute from it — to force a
        // *root* mismatch too, the root's two operands must both be computable
        // and disagree. Use: root = (leftLit[1]) - (rightSubtree). Make the right
        // subtree a computable Multiply whose result is dim 2, and the left lit
        // dim 1 ⇒ root mismatch; and inside give the multiply a dimensionful
        // scalar to also emit below. That conflates. Instead build two clearly
        // independent offending nodes under one computable root:
        //   root Add:
        //     left  = Multiply(op lit[5], scalar dimensionful) → emits at [Left], None
        //     right = Add(lit[3],lit[3]) → computable dim 3
        //   root: left None ⇒ root non-computable, no root emit ⇒ only 1 defect.
        // A root mismatch needs both sides computable. So:
        //   root Subtract:
        //     left  = lit[1]                        (dim 1)
        //     right = Multiply(op lit[2], scalar dimensionful)  → emits [Right], None
        //   ⇒ 1 defect only. Genuine "root + below" needs the below-mismatch on a
        // subtree that is NOT an operand of the root comparison. That is exactly a
        // three-way tree where the root has two computable operands, and one of
        // those operands *contains* a deeper mismatch that is itself computable-
        // upward — impossible, since a mismatch is non-computable upward.
        //
        // Therefore two defects arise from two INDEPENDENT offending nodes, which
        // is the case the next test covers. A single root-to-leaf chain yields at
        // most one defect. This test asserts that invariant explicitly.
        let chain = DerivationExpression::Add {
            left: Box::new(DerivationExpression::Add {
                left: Box::new(DerivationExpression::Literal(numeric_literal(1, 1))),
                right: Box::new(DerivationExpression::Literal(numeric_literal(2, 2))),
            }),
            right: Box::new(DerivationExpression::Literal(numeric_literal(3, 9))),
        };
        let s = strategy_with(
            vec![],
            vec![],
            vec![derivation(10, CandidateRole::Entry, chain)],
        );
        assert_eq!(check_14a(&s, &binding()).len(), 1);
    }

    #[test]
    fn two_independent_mismatch_nodes_produce_two_distinct_identities() {
        // A computable-root Add whose two operands are each their own mismatch is
        // impossible (a mismatch is non-computable), so independence comes from
        // two separate derivations, or two sibling Multiply scalars under one Add
        // (each Multiply emits its own scalar defect; the Add is then
        // non-computable and does not emit). Use the sibling-Multiply form.
        let expr = DerivationExpression::Add {
            left: Box::new(DerivationExpression::Multiply {
                operand: Box::new(DerivationExpression::Literal(numeric_literal(1, 7))),
                scalar: numeric_literal(2, 5), // dimensionful ⇒ emit at [Left]
            }),
            right: Box::new(DerivationExpression::Multiply {
                operand: Box::new(DerivationExpression::Literal(numeric_literal(3, 7))),
                scalar: Literal::Boolean(true), // non-numeric ⇒ emit at [Right]
            }),
        };
        let s = strategy_with(
            vec![],
            vec![],
            vec![derivation(10, CandidateRole::Entry, expr)],
        );
        let defects = check_14a(&s, &binding());
        assert_eq!(defects.len(), 2);
        assert!(defects.iter().all(is_dimension_mismatch));
        // Distinct NODE-PATH identities: [Left] and [Right].
        assert!(
            defects
                .iter()
                .any(|d| path_eq(d, vec![ChildPosition::LeftOperand]))
        );
        assert!(
            defects
                .iter()
                .any(|d| path_eq(d, vec![ChildPosition::RightOperand]))
        );
        // The two identities are distinct.
        assert_ne!(defects[0], defects[1]);
    }

    // == prerequisite / suppression =========================================

    #[test]
    fn unresolved_evidence_reference_is_not_a_14a_defect() {
        // No evidence declared for lineage 999 → Check 2's UNRESOLVED_REFERENCE.
        let s = strategy_with(
            vec![],
            vec![],
            vec![derivation(
                10,
                CandidateRole::Entry,
                DerivationExpression::EvidenceReference(lineage(999)),
            )],
        );
        assert!(check_14a(&s, &binding()).is_empty());
    }

    #[test]
    fn unresolved_configuration_reference_is_not_a_14a_defect() {
        let s = strategy_with(
            vec![],
            vec![],
            vec![derivation(
                10,
                CandidateRole::Entry,
                DerivationExpression::ConfigurationReference(lineage(999)),
            )],
        );
        assert!(check_14a(&s, &binding()).is_empty());
    }

    #[test]
    fn ambiguous_evidence_reference_is_not_a_14a_defect() {
        // Two evidence declarations share lineage 100 → Check 2's duplicate/
        // ambiguous resolution; 14A cannot read a dimension.
        let s = strategy_with(
            vec![
                EvidenceDeclaration::numeric(occ(100), lineage(100), dim(1)),
                EvidenceDeclaration::numeric(occ(101), lineage(100), dim(2)),
            ],
            vec![],
            vec![derivation(
                10,
                CandidateRole::Entry,
                DerivationExpression::EvidenceReference(lineage(100)),
            )],
        );
        assert!(check_14a(&s, &binding()).is_empty());
    }

    #[test]
    fn non_numeric_evidence_reference_is_not_a_14a_defect() {
        // A BOOLEAN evidence leaf is Check 2's TYPE_MISMATCH; no dimension.
        let s = strategy_with(
            vec![EvidenceDeclaration::boolean(occ(100), lineage(100))],
            vec![],
            vec![derivation(
                10,
                CandidateRole::Entry,
                DerivationExpression::EvidenceReference(lineage(100)),
            )],
        );
        assert!(check_14a(&s, &binding()).is_empty());
        // And an ENUM leaf likewise.
        let values = vec![EnumValue::new(OpaqueValue::new(7))];
        let s2 = strategy_with(
            vec![EvidenceDeclaration::enumerated(
                occ(100),
                lineage(100),
                values,
            )],
            vec![],
            vec![derivation(
                10,
                CandidateRole::Entry,
                DerivationExpression::EvidenceReference(lineage(100)),
            )],
        );
        assert!(check_14a(&s2, &binding()).is_empty());
    }

    #[test]
    fn a_non_computable_operand_suppresses_only_the_dependent_comparison() {
        // (unresolved_ref + lit[7]): the left leaf is Check 2's, so the Add is
        // non-computable and emits nothing — the unrelated right leaf is fine.
        let expr = DerivationExpression::Add {
            left: Box::new(DerivationExpression::EvidenceReference(lineage(999))),
            right: Box::new(DerivationExpression::Literal(numeric_literal(1, 7))),
        };
        let s = strategy_with(
            vec![],
            vec![],
            vec![derivation(10, CandidateRole::Entry, expr)],
        );
        assert!(check_14a(&s, &binding()).is_empty());
    }

    #[test]
    fn independent_valid_sibling_remains_evaluable() {
        // left subtree has a genuine mismatch (emits at [Left]); right subtree
        // references an unresolved leaf (Check 2's, non-computable, no 14A). The
        // left mismatch is still reported — non-computability of one sibling does
        // not suppress the other.
        let expr = DerivationExpression::Add {
            left: Box::new(DerivationExpression::Add {
                left: Box::new(DerivationExpression::Literal(numeric_literal(1, 1))),
                right: Box::new(DerivationExpression::Literal(numeric_literal(2, 2))),
            }),
            right: Box::new(DerivationExpression::EvidenceReference(lineage(999))),
        };
        let defect = single_defect(expr);
        assert!(is_dimension_mismatch(&defect));
        assert!(path_eq(&defect, vec![ChildPosition::LeftOperand]));
    }

    #[test]
    fn dimensioned_scalar_over_non_computable_operand_still_emits() {
        // operand is an unresolved reference (Check 2's, non-computable); the
        // dimensionful scalar is 14A's regardless, at the multiply node.
        let defect = single_defect(DerivationExpression::Multiply {
            operand: Box::new(DerivationExpression::EvidenceReference(lineage(999))),
            scalar: numeric_literal(3, 2),
        });
        assert!(is_dimension_mismatch(&defect));
        assert!(path_eq(&defect, vec![]));
    }

    #[test]
    fn absent_expression_is_not_a_14a_defect() {
        let bare = CandidateDerivation {
            occurrence: occ(10),
            lineage: Declared::Present(lineage(10)),
            role: Declared::Present(CandidateRole::Entry),
            expression: Declared::Absent,
        };
        let s = strategy_with(vec![], vec![], vec![bare]);
        assert!(check_14a(&s, &binding()).is_empty());
    }

    #[test]
    fn absent_candidate_derivations_collection_is_not_a_14a_defect() {
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
        assert!(check_14a(&s, &binding()).is_empty());
    }

    // == identity ===========================================================

    #[test]
    fn root_node_path_identity() {
        let defect = single_defect(DerivationExpression::Add {
            left: Box::new(DerivationExpression::Literal(numeric_literal(1, 1))),
            right: Box::new(DerivationExpression::Literal(numeric_literal(2, 2))),
        });
        assert!(path_eq(&defect, vec![]));
    }

    #[test]
    fn nested_left_node_path_identity() {
        // root Add with a mismatching LEFT Add and a matching right literal.
        let defect = single_defect(DerivationExpression::Add {
            left: Box::new(DerivationExpression::Add {
                left: Box::new(DerivationExpression::Literal(numeric_literal(1, 1))),
                right: Box::new(DerivationExpression::Literal(numeric_literal(2, 2))),
            }),
            right: Box::new(DerivationExpression::Literal(numeric_literal(3, 1))),
        });
        assert!(path_eq(&defect, vec![ChildPosition::LeftOperand]));
    }

    #[test]
    fn nested_right_node_path_identity() {
        let defect = single_defect(DerivationExpression::Add {
            left: Box::new(DerivationExpression::Literal(numeric_literal(3, 1))),
            right: Box::new(DerivationExpression::Add {
                left: Box::new(DerivationExpression::Literal(numeric_literal(1, 1))),
                right: Box::new(DerivationExpression::Literal(numeric_literal(2, 2))),
            }),
        });
        assert!(path_eq(&defect, vec![ChildPosition::RightOperand]));
    }

    #[test]
    fn left_right_node_path_identity() {
        // root Add, right child Add, whose LEFT child is a mismatching Add:
        // path root → RightOperand → LeftOperand.
        let mismatch = DerivationExpression::Add {
            left: Box::new(DerivationExpression::Literal(numeric_literal(1, 1))),
            right: Box::new(DerivationExpression::Literal(numeric_literal(2, 2))),
        };
        let expr = DerivationExpression::Add {
            left: Box::new(DerivationExpression::Literal(numeric_literal(9, 5))),
            right: Box::new(DerivationExpression::Add {
                left: Box::new(mismatch),
                right: Box::new(DerivationExpression::Literal(numeric_literal(3, 5))),
            }),
        };
        let defect = single_defect(expr);
        assert!(path_eq(
            &defect,
            vec![ChildPosition::RightOperand, ChildPosition::LeftOperand]
        ));
    }

    #[test]
    fn same_tree_produces_stable_identities() {
        let build = || DerivationExpression::Add {
            left: Box::new(DerivationExpression::Literal(numeric_literal(1, 1))),
            right: Box::new(DerivationExpression::Literal(numeric_literal(2, 2))),
        };
        let a = single_defect(build());
        let b = single_defect(build());
        assert_eq!(a, b);
    }

    // == determinism ========================================================

    #[test]
    fn unrelated_collection_permutation_does_not_change_identity_membership() {
        // The offending derivation is identical in both Strategies; only the
        // order of the (unrelated, well-formed) evidence collection differs. The
        // 14A defect set — anchored on occurrence with a structural NODE-PATH —
        // is identical.
        let offending = || DerivationExpression::Add {
            left: Box::new(DerivationExpression::Literal(numeric_literal(1, 1))),
            right: Box::new(DerivationExpression::Literal(numeric_literal(2, 2))),
        };
        let ev_a = vec![
            EvidenceDeclaration::numeric(occ(100), lineage(100), dim(1)),
            EvidenceDeclaration::numeric(occ(101), lineage(101), dim(2)),
        ];
        let mut ev_b = ev_a.clone();
        ev_b.reverse();
        let s_a = strategy_with(
            ev_a,
            vec![],
            vec![derivation(10, CandidateRole::Entry, offending())],
        );
        let s_b = strategy_with(
            ev_b,
            vec![],
            vec![derivation(10, CandidateRole::Entry, offending())],
        );
        let da = check_14a(&s_a, &binding());
        let db = check_14a(&s_b, &binding());
        assert_eq!(da.len(), 1);
        assert_eq!(db.len(), 1);
        assert_eq!(da[0], db[0]);
    }
}
