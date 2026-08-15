//! Candidate-value derivations — Slice Contract §3.8.
//!
//! A Strategy declares exactly three derivations, one per candidate role. Each
//! holds an expression built from the admitted grammar and nothing else.
//!
//! # Shapes that are unrepresentable here, by construction
//!
//! These are grammar-level forms §3.8 does not admit at all. No mandatory check
//! owns them, so nothing downstream would reject them — the type system must.
//!
//! * **Division and exponentiation** — no variant exists. §3.8 excludes
//!   division on *semantic* grounds: it would require deciding divide-by-zero
//!   behaviour and a dimension algebra, neither of which any accepted document
//!   defines.
//! * **Any other operator**, comparison or boolean operator, and conditional or
//!   branching expressions — no variants.
//! * **Multiplication where neither operand is a literal scalar** — `Multiply`
//!   takes an expression and a `Literal`, so `expr * expr` cannot be written.
//! * **References to conditions, Direction rules, or other derivations** — the
//!   only reference leaves are evidence and configuration.
//!
//! Expressions are finite and acyclic by construction: `Box` builds a tree, and
//! nothing here can create a cycle or a self-reference.
//!
//! The derivation's own three properties are a different matter: §3.8 marks all
//! three **unconditional**, and names the reachable case of a derivation missing
//! *both* its lineage identity and its expression. Those are declaration states,
//! because check 1 is required to report them.

use crate::model::declared::Declared;
use crate::model::literal::Literal;
use crate::model::opaque::{LineageId, OccurrenceId};

/// A candidate role — Slice Contract §3.8.
///
/// A closed authoritative category (ADR-0005 §7.4). The declared order of the
/// three roles is the order §6.4.2's `ROLE` discriminator will use in Phase 4.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandidateRole {
    Entry,
    StopLoss,
    TakeProfit,
}

/// A derivation expression — Slice Contract §3.8.
///
/// A closed authoritative category (ADR-0005 §7.4): exactly the six admitted
/// forms.
///
/// # `expr * expr` is a compile error
///
/// §3.8 prohibits "multiplication where neither operand is a dimensionless
/// literal", and no mandatory check owns that grammar-level shape — check 14
/// judges whether a present scalar is *dimensionless*, not whether a scalar is a
/// literal at all. `Multiply::scalar` is therefore a [`Literal`], not a
/// `DerivationExpression`, and the prohibited form cannot be written:
///
/// ```compile_fail
/// use ordina::model::derivation::DerivationExpression;
/// use ordina::model::opaque::{LineageId, OpaqueValue};
///
/// let reference = DerivationExpression::EvidenceReference(LineageId::new(OpaqueValue::new(1)));
/// let other = DerivationExpression::EvidenceReference(LineageId::new(OpaqueValue::new(2)));
///
/// // `scalar` is a `Literal`; a second expression has no type to inhabit here.
/// let _ = DerivationExpression::Multiply {
///     operand: Box::new(reference),
///     scalar: other,
/// };
/// ```
///
/// The admitted form — an expression multiplied by a literal scalar — compiles:
///
/// ```
/// use ordina::model::derivation::DerivationExpression;
/// use ordina::model::literal::Literal;
/// use ordina::model::opaque::{DimensionSymbol, LineageId, OpaqueValue};
/// use ordina::numeric::ExactNumber;
///
/// let reference = DerivationExpression::EvidenceReference(LineageId::new(OpaqueValue::new(1)));
/// let _ = DerivationExpression::Multiply {
///     operand: Box::new(reference),
///     scalar: Literal::Numeric {
///         value: ExactNumber::from_integer(3),
///         dimension: DimensionSymbol::new(OpaqueValue::new(1)),
///     },
/// };
/// ```
#[derive(Debug, Clone)]
pub enum DerivationExpression {
    Literal(Literal),
    /// Lineage identity of a declared evidence item.
    EvidenceReference(LineageId),
    /// Lineage identity of a declared configuration parameter. This is the
    /// **only** position in the whole model where a configuration reference may
    /// appear (§3.9).
    ConfigurationReference(LineageId),
    Add {
        left: Box<DerivationExpression>,
        right: Box<DerivationExpression>,
    },
    Subtract {
        left: Box<DerivationExpression>,
        right: Box<DerivationExpression>,
    },
    /// Multiplication by a literal scalar.
    ///
    /// §4.6 additionally requires the scalar to be *dimensionless*. That is a
    /// dimensional rule, so check 14 owns it in Phase 5 and a dimensioned or
    /// non-numeric scalar stays representable here.
    Multiply {
        operand: Box<DerivationExpression>,
        scalar: Literal,
    },
}

/// One candidate-value derivation — Slice Contract §3.8.
///
/// §3.8 marks lineage identity, role and derivation expression all
/// **unconditional**, and works through the case of a derivation missing both
/// its lineage identity and its expression. Under Slice Contract Revision 3
/// (§C.1, §C.4) that case yields **two** defects owned by different checks: the
/// missing lineage identity is check 15's `MISSING_OPERATIVE_PROVENANCE` (a
/// candidate derivation is an operative node), and the missing expression is
/// check 1's `MISSING_REQUIRED_PROPERTY`. They are separated by §6.4 key 1 (the
/// ADR-0004 check number, check 1 before check 15), not by the PROPERTY
/// discriminator; the missing role would likewise be check 1's. The §3.8
/// property positions (lineage 1, role 2, expression 3) are unchanged, and both
/// absences must be constructible for the ordering to ever be exercised.
///
/// The role is a plain [`Declared`] rather than a `DeclaredCategory`: §3.8
/// prohibits "a missing role" and "a duplicated role", but duplication is a
/// **Strategy-level** fault across the three derivations (check 13B), not a
/// multiplicity within one derivation.
#[derive(Debug, Clone)]
pub struct CandidateDerivation {
    /// Canonical occurrence identity (Revision 4 §C.1) — mandatory, present even
    /// when `lineage` is absent. A candidate derivation is an operative node, so
    /// lineage absence is check 15's; the occurrence identity anchors that defect.
    pub occurrence: OccurrenceId,
    pub lineage: Declared<LineageId>,
    pub role: Declared<CandidateRole>,
    pub expression: Declared<DerivationExpression>,
}

impl CandidateDerivation {
    /// A derivation with every property declared — the well-formed shape.
    /// Malformed shapes are built from the fields directly.
    #[must_use]
    pub const fn new(
        occurrence: OccurrenceId,
        lineage: LineageId,
        role: CandidateRole,
        expression: DerivationExpression,
    ) -> Self {
        Self {
            occurrence,
            lineage: Declared::Present(lineage),
            role: Declared::Present(role),
            expression: Declared::Present(expression),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::opaque::{DimensionSymbol, OpaqueValue};
    use crate::numeric::ExactNumber;

    fn lineage(handle: u64) -> LineageId {
        LineageId::new(OpaqueValue::new(handle))
    }

    fn occ(handle: u64) -> OccurrenceId {
        OccurrenceId::new(OpaqueValue::new(handle))
    }

    fn literal(value: i64) -> Literal {
        Literal::Numeric {
            value: ExactNumber::from_integer(value),
            dimension: DimensionSymbol::new(OpaqueValue::new(1)),
        }
    }

    #[test]
    fn every_admitted_expression_form_is_representable() {
        let nested = DerivationExpression::Add {
            left: Box::new(DerivationExpression::EvidenceReference(lineage(1))),
            right: Box::new(DerivationExpression::Multiply {
                operand: Box::new(DerivationExpression::ConfigurationReference(lineage(2))),
                scalar: literal(3),
            }),
        };
        let subtracted = DerivationExpression::Subtract {
            left: Box::new(DerivationExpression::Literal(literal(10))),
            right: Box::new(nested),
        };

        // Exhaustive match, no catch-all arm (ADR-0005 §7.4).
        let form = match subtracted {
            DerivationExpression::Literal(_) => "literal",
            DerivationExpression::EvidenceReference(_) => "evidence",
            DerivationExpression::ConfigurationReference(_) => "configuration",
            DerivationExpression::Add { .. } => "add",
            DerivationExpression::Subtract { .. } => "subtract",
            DerivationExpression::Multiply { .. } => "multiply",
        };

        assert_eq!(form, "subtract");
    }

    #[test]
    fn all_three_roles_are_representable() {
        let roles = [
            CandidateRole::Entry,
            CandidateRole::StopLoss,
            CandidateRole::TakeProfit,
        ];

        for role in roles {
            let derivation = CandidateDerivation::new(
                occ(1),
                lineage(1),
                role,
                DerivationExpression::Literal(literal(1)),
            );
            assert_eq!(derivation.role, Declared::Present(role));
        }
    }

    #[test]
    fn check_1_and_check_15_absences_remain_representable() {
        // §3.8 works through exactly this case: a derivation missing both its
        // lineage identity and its expression. Under Revision 3 (§C.1, §C.4) the
        // missing lineage is check 15's `MISSING_OPERATIVE_PROVENANCE` (a
        // candidate derivation is an operative node) and the missing expression
        // is check 1's `MISSING_REQUIRED_PROPERTY`; the two are separated by §6.4
        // key 1 (check number), not the PROPERTY discriminator. Both absences
        // must be buildable regardless of which check owns each.
        let bare = CandidateDerivation {
            occurrence: occ(2),
            lineage: Declared::Absent,
            role: Declared::Absent,
            expression: Declared::Absent,
        };

        assert!(bare.lineage.is_absent());
        assert!(bare.role.is_absent());
        assert!(bare.expression.is_absent());
    }
}
