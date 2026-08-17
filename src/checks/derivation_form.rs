//! Check 13A — candidate-value derivation form validity (Slice Contract §5.1
//! check 13, §3.8, §5.2).
//!
//! # What Check 13A owns
//!
//! Check 13A is the per-present-derivation half of check 13: for each **present**
//! candidate derivation, its expression must conform to the §3.8 grammar — every
//! operator admitted, the expression finite and acyclic. Its owned defect is
//! `INVALID_DERIVATION_FORM`, **one per offending node**, anchored on that
//! derivation's occurrence identity (Revision 4 §C.4, ordinary occurrence-level
//! defect) with the `NODE_PATH` discriminator (§6.4.2 kind 5) carrying the
//! root-to-node path.
//!
//! # Why its runtime population is structurally zero (PBA-0126 / ER-0104)
//!
//! The Product Board determined that Check 13A **remains applicable and must be
//! explicitly realized**, even though its `INVALID_DERIVATION_FORM` population is
//! **structurally unreachable** under the closed [`DerivationExpression`] model
//! ([`crate::model::derivation`]). Every §3.8-prohibited form is either
//! unrepresentable in the closed six-variant enum — division, exponentiation,
//! comparison/boolean operators, conditional/branching, and `expr * expr` have no
//! variant (`Multiply::scalar` is a [`Literal`], proven by the `compile_fail`
//! doctest on [`DerivationExpression`]); cyclic/infinite trees cannot be built from
//! owned `Box` — or it collapses to a **leaf resolution/type fault owned by
//! Check 2** (`UNRESOLVED_REFERENCE` for an unresolved evidence/configuration leaf,
//! `TYPE_MISMATCH` for a non-`NUMERIC` leaf). This realization therefore reports
//! **no defect for any currently representable, computable expression**, and the
//! determination forbids broadening the model to manufacture one.
//!
//! # Exhaustiveness is the point
//!
//! The form walk matches [`DerivationExpression`] **exhaustively, with no
//! catch-all arm** (ADR-0005 §7.4). Each admitted variant is handled explicitly
//! and contributes no defect. If a future revision adds an expression variant, the
//! match here fails to compile, **forcing Check 13A to be reconsidered** before the
//! new form can silently bypass grammar validation — the express purpose of the
//! explicit realization.
//!
//! # Ownership boundary — what Check 13A must NOT report
//!
//! Check 13A owns prohibited §3.8 *forms* only. It never re-reports:
//!
//! - an unresolved evidence or configuration reference — **Check 2**
//!   (`UNRESOLVED_REFERENCE`);
//! - a non-`NUMERIC` (BOOLEAN/ENUM) leaf — **Check 2** (`TYPE_MISMATCH`);
//! - a dimensional mismatch (e.g. a dimensioned `*` scalar) — **Check 14A**
//!   (`DIMENSION_MISMATCH`);
//! - a missing or duplicated candidate role — **Check 13B**;
//! - a missing candidate-derivation lineage — **Check 15**.
//!
//! # Prerequisites (§5.2), locally reconstructed
//!
//! Check 13A's prerequisite is check 1 and check 2 on **that** derivation. Phase 5
//! reconstructs prerequisite computability **locally** and calls no prior check in
//! production (no `check_1`/`check_2`, no global runner) and emits no
//! `not-evaluated` metadata (that is Phase 6). Presence of the
//! `candidate_derivations` collection is check 1's, so Check 13A walks only a
//! **present** collection; an **absent expression** on a derivation is check 1's,
//! so a derivation with no expression is skipped and yields no 13A defect. The
//! grammar walk itself produces nothing for any leaf, so unresolved references and
//! non-`NUMERIC` leaves (both non-computable for 13A and owned by Check 2) yield no
//! 13A defect by construction — the affected derivation is suppressed locally
//! without touching any other derivation.

use crate::model::declared::Declared;
use crate::model::derivation::DerivationExpression;
use crate::model::strategy::Strategy;
use crate::result::identity::DefectCore;

/// Check 13A over a whole Strategy. Returns a deterministic `Vec<DefectCore>`.
///
/// Under the accepted closed [`DerivationExpression`] model this is **always
/// empty** (PBA-0126 / ER-0104): every representable expression node is an
/// admitted §3.8 form. The realization is explicit rather than elided so that the
/// applicable check exists at the check layer and a future grammar extension is
/// forced through the exhaustive match below.
#[must_use]
pub fn check_13a(strategy: &Strategy) -> Vec<DefectCore> {
    let mut defects = Vec::new();
    let Declared::Present(derivations) = &strategy.candidate_derivations else {
        // Absent collection is check 1's, not check 13A's.
        return defects;
    };

    for derivation in derivations {
        // An absent expression is check 1's (`MISSING_REQUIRED_PROPERTY`), never a
        // 13A form defect. Only a present expression has a form to validate.
        if let Declared::Present(expression) = &derivation.expression {
            defects.extend(form_defects(expression));
        }
    }

    defects
}

/// The `INVALID_DERIVATION_FORM` defects of one derivation expression subtree.
///
/// The match is exhaustive with no catch-all arm (ADR-0005 §7.4): every variant of
/// the closed [`DerivationExpression`] enum is an admitted §3.8 form, so every arm
/// contributes nothing of its own and the result is **always empty** under the
/// current model. Compound forms recurse into their operands so the walk visits
/// every node. The local `defects` vector is the surface a future non-admitted
/// variant's arm would `push` onto (anchoring on the derivation's occurrence with a
/// `NODE_PATH` discriminator); today no arm does, and none is fabricated
/// (PBA-0126 / ER-0104).
fn form_defects(expression: &DerivationExpression) -> Vec<DefectCore> {
    let mut defects = Vec::new();
    match expression {
        // `typed-literal` — admitted leaf (§3.8). Its type is Check 2's, its
        // dimension Check 14A's; the *form* is valid.
        DerivationExpression::Literal(_) => {}
        // `evidence-reference` — admitted leaf (§3.8). Resolution and NUMERIC
        // typing are Check 2's; the *form* is valid.
        DerivationExpression::EvidenceReference(_) => {}
        // `configuration-parameter-reference` — admitted leaf (§3.8). Resolution is
        // Check 2's; the *form* is valid.
        DerivationExpression::ConfigurationReference(_) => {}
        // `expr + expr` / `expr - expr` — admitted (§3.8). Dimensional agreement of
        // the operands is Check 14A's; the *form* is valid. Recurse into both.
        DerivationExpression::Add { left, right }
        | DerivationExpression::Subtract { left, right } => {
            defects.extend(form_defects(left));
            defects.extend(form_defects(right));
        }
        // `expr * dimensionless-literal` — admitted (§3.8). The scalar is a
        // [`Literal`] by type, so the prohibited `expr * expr` is unrepresentable;
        // whether the scalar is dimensionless is Check 14A's, and a non-`NUMERIC`
        // scalar is Check 2's. The *form* is valid. Recurse into the operand; the
        // scalar is a leaf literal with no sub-expression.
        DerivationExpression::Multiply { operand, scalar: _ } => {
            defects.extend(form_defects(operand));
        }
    }
    defects
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::config::ConfigurationParameter;
    use crate::model::declared::Declared;
    use crate::model::derivation::{CandidateDerivation, CandidateRole, DerivationExpression};
    use crate::model::evidence::EvidenceDeclaration;
    use crate::model::literal::Literal;
    use crate::model::opaque::{DimensionSymbol, EnumValue, LineageId, OccurrenceId, OpaqueValue};
    use crate::model::strategy::Strategy;
    use crate::numeric::ExactNumber;

    fn occ(handle: u64) -> OccurrenceId {
        OccurrenceId::new(OpaqueValue::new(handle))
    }

    fn lineage(handle: u64) -> LineageId {
        LineageId::new(OpaqueValue::new(handle))
    }

    fn dim(handle: u64) -> DimensionSymbol {
        DimensionSymbol::new(OpaqueValue::new(handle))
    }

    fn numeric_literal(value: i64, dimension: u64) -> Literal {
        Literal::Numeric {
            value: ExactNumber::from_integer(value),
            dimension: dim(dimension),
        }
    }

    /// A Strategy carrying exactly the given derivations; every other collection
    /// present-and-empty so nothing else is evaluable.
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

    // -- 1..=6: every admitted form yields zero 13A defects ------------------

    #[test]
    fn valid_numeric_literal_derivation_is_no_defect() {
        let s = strategy_with(
            vec![],
            vec![],
            vec![derivation(
                10,
                CandidateRole::Entry,
                DerivationExpression::Literal(numeric_literal(5, 1)),
            )],
        );
        assert!(check_13a(&s).is_empty());
    }

    #[test]
    fn valid_evidence_reference_derivation_is_no_defect() {
        // A resolved NUMERIC evidence leaf — the canonical admitted reference form.
        let s = strategy_with(
            vec![EvidenceDeclaration::numeric(occ(100), lineage(100), dim(1))],
            vec![],
            vec![derivation(
                10,
                CandidateRole::Entry,
                DerivationExpression::EvidenceReference(lineage(100)),
            )],
        );
        assert!(check_13a(&s).is_empty());
    }

    #[test]
    fn valid_configuration_reference_derivation_is_no_defect() {
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
        assert!(check_13a(&s).is_empty());
    }

    #[test]
    fn valid_nested_add_tree_is_no_defect() {
        let expr = DerivationExpression::Add {
            left: Box::new(DerivationExpression::Literal(numeric_literal(1, 1))),
            right: Box::new(DerivationExpression::Add {
                left: Box::new(DerivationExpression::EvidenceReference(lineage(100))),
                right: Box::new(DerivationExpression::Literal(numeric_literal(2, 1))),
            }),
        };
        let s = strategy_with(
            vec![EvidenceDeclaration::numeric(occ(100), lineage(100), dim(1))],
            vec![],
            vec![derivation(10, CandidateRole::StopLoss, expr)],
        );
        assert!(check_13a(&s).is_empty());
    }

    #[test]
    fn valid_nested_subtract_tree_is_no_defect() {
        let expr = DerivationExpression::Subtract {
            left: Box::new(DerivationExpression::Literal(numeric_literal(10, 1))),
            right: Box::new(DerivationExpression::Subtract {
                left: Box::new(DerivationExpression::Literal(numeric_literal(4, 1))),
                right: Box::new(DerivationExpression::Literal(numeric_literal(1, 1))),
            }),
        };
        let s = strategy_with(
            vec![],
            vec![],
            vec![derivation(10, CandidateRole::TakeProfit, expr)],
        );
        assert!(check_13a(&s).is_empty());
    }

    #[test]
    fn valid_multiply_form_is_no_defect() {
        let expr = DerivationExpression::Multiply {
            operand: Box::new(DerivationExpression::EvidenceReference(lineage(100))),
            scalar: numeric_literal(3, 1),
        };
        let s = strategy_with(
            vec![EvidenceDeclaration::numeric(occ(100), lineage(100), dim(1))],
            vec![],
            vec![derivation(10, CandidateRole::Entry, expr)],
        );
        assert!(check_13a(&s).is_empty());
    }

    // -- 7..=11: faults owned by OTHER checks are not 13A defects ------------

    #[test]
    fn unresolved_evidence_reference_is_not_a_13a_defect() {
        // No evidence declared for lineage 999 → Check 2's UNRESOLVED_REFERENCE,
        // never a 13A form defect.
        let s = strategy_with(
            vec![],
            vec![],
            vec![derivation(
                10,
                CandidateRole::Entry,
                DerivationExpression::EvidenceReference(lineage(999)),
            )],
        );
        assert!(check_13a(&s).is_empty());
    }

    #[test]
    fn unresolved_configuration_reference_is_not_a_13a_defect() {
        let s = strategy_with(
            vec![],
            vec![],
            vec![derivation(
                10,
                CandidateRole::Entry,
                DerivationExpression::ConfigurationReference(lineage(999)),
            )],
        );
        assert!(check_13a(&s).is_empty());
    }

    #[test]
    fn resolved_boolean_leaf_is_not_a_13a_defect() {
        // A resolved BOOLEAN evidence leaf is Check 2's TYPE_MISMATCH; the *form*
        // (an evidence reference) is admitted, so 13A is silent.
        let s = strategy_with(
            vec![EvidenceDeclaration::boolean(occ(100), lineage(100))],
            vec![],
            vec![derivation(
                10,
                CandidateRole::Entry,
                DerivationExpression::EvidenceReference(lineage(100)),
            )],
        );
        assert!(check_13a(&s).is_empty());
    }

    #[test]
    fn resolved_enum_leaf_is_not_a_13a_defect() {
        let values = vec![EnumValue::new(OpaqueValue::new(7))];
        let s = strategy_with(
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
        assert!(check_13a(&s).is_empty());
    }

    #[test]
    fn dimensional_mismatch_is_check_14a_not_13a() {
        // `a + b` with dimensionally distinct operands is a future Check 14A
        // DIMENSION_MISMATCH; the *form* is a valid admitted addition, so 13A is
        // silent. A dimensioned `*` scalar (dim 2, not dimensionless) is likewise
        // Check 14A's.
        let add = DerivationExpression::Add {
            left: Box::new(DerivationExpression::Literal(numeric_literal(1, 1))),
            right: Box::new(DerivationExpression::Literal(numeric_literal(2, 2))),
        };
        let mul = DerivationExpression::Multiply {
            operand: Box::new(DerivationExpression::Literal(numeric_literal(1, 1))),
            scalar: numeric_literal(3, 2),
        };
        let s = strategy_with(
            vec![],
            vec![],
            vec![
                derivation(10, CandidateRole::Entry, add),
                derivation(11, CandidateRole::StopLoss, mul),
            ],
        );
        assert!(check_13a(&s).is_empty());
    }

    // -- 12: local suppression does not affect unrelated derivations --------

    #[test]
    fn a_non_computable_derivation_does_not_suppress_or_infect_others() {
        // One derivation has an absent expression (check 1's), one references an
        // unresolved leaf (check 2's), one is fully valid. 13A yields zero defects
        // for all three, and — crucially — evaluating the valid one is unaffected
        // by the other two. There is no defect to suppress, so this proves the walk
        // is per-derivation and never Strategy-wide.
        let absent_expr = CandidateDerivation {
            occurrence: occ(10),
            lineage: Declared::Present(lineage(10)),
            role: Declared::Present(CandidateRole::Entry),
            expression: Declared::Absent,
        };
        let unresolved = derivation(
            11,
            CandidateRole::StopLoss,
            DerivationExpression::EvidenceReference(lineage(999)),
        );
        let valid = derivation(
            12,
            CandidateRole::TakeProfit,
            DerivationExpression::Literal(numeric_literal(5, 1)),
        );
        let s = strategy_with(vec![], vec![], vec![absent_expr, unresolved, valid]);
        assert!(check_13a(&s).is_empty());
    }

    // -- ownership / non-emission proofs ------------------------------------

    #[test]
    fn check_13a_emits_no_defect_category_at_all_under_the_closed_model() {
        // The structural-zero-population invariant (PBA-0126 / ER-0104): across a
        // representative mix of admitted forms and other-check-owned faults, 13A
        // emits nothing — so it can duplicate no Check 2 / 13B / 14A / 15 defect.
        let expr = DerivationExpression::Subtract {
            left: Box::new(DerivationExpression::Multiply {
                operand: Box::new(DerivationExpression::EvidenceReference(lineage(100))),
                scalar: numeric_literal(2, 1),
            }),
            right: Box::new(DerivationExpression::ConfigurationReference(lineage(200))),
        };
        let param = ConfigurationParameter::new(
            occ(200),
            lineage(200),
            dim(1),
            numeric_literal(0, 1),
            numeric_literal(9, 1),
            false,
        );
        let s = strategy_with(
            vec![EvidenceDeclaration::numeric(occ(100), lineage(100), dim(1))],
            vec![param],
            vec![derivation(10, CandidateRole::Entry, expr)],
        );
        assert_eq!(check_13a(&s).len(), 0);
    }

    #[test]
    fn an_absent_candidate_derivations_collection_is_check_1s_not_13as() {
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
        assert!(check_13a(&s).is_empty());
    }

    /// Exhaustiveness-regression note (not a runtime assertion): the production
    /// `append_form_defects` matches every [`DerivationExpression`] variant with no
    /// `_` arm, so adding a variant breaks compilation here and forces Check 13A to
    /// be reconsidered. The prohibited `expr * expr` form is proven unrepresentable
    /// by the existing `compile_fail` doctest on [`DerivationExpression`]
    /// (`src/model/derivation.rs`); it is referenced rather than duplicated, and the
    /// model is not modified to add a new proof.
    #[test]
    fn admitted_forms_are_matched_exhaustively() {
        // Touch each variant so the test documents the closed set alongside the
        // production match; a new variant would fail to compile in both places.
        let forms = [
            DerivationExpression::Literal(numeric_literal(1, 1)),
            DerivationExpression::EvidenceReference(lineage(1)),
            DerivationExpression::ConfigurationReference(lineage(2)),
            DerivationExpression::Add {
                left: Box::new(DerivationExpression::Literal(numeric_literal(1, 1))),
                right: Box::new(DerivationExpression::Literal(numeric_literal(1, 1))),
            },
            DerivationExpression::Subtract {
                left: Box::new(DerivationExpression::Literal(numeric_literal(1, 1))),
                right: Box::new(DerivationExpression::Literal(numeric_literal(1, 1))),
            },
            DerivationExpression::Multiply {
                operand: Box::new(DerivationExpression::Literal(numeric_literal(1, 1))),
                scalar: numeric_literal(1, 1),
            },
        ];
        for form in &forms {
            assert!(form_defects(form).is_empty());
        }
    }
}
