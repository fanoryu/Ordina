//! Candidate-value derivations — Slice Contract §3.8.
//!
//! A Strategy declares exactly three derivations, one per candidate role. Each
//! holds an expression built from the admitted grammar and nothing else.
//!
//! # Shapes that are unrepresentable here, by construction
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

use crate::model::literal::Literal;
use crate::model::opaque::LineageId;

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
#[derive(Debug, Clone)]
pub struct CandidateDerivation {
    pub lineage: LineageId,
    pub role: CandidateRole,
    pub expression: DerivationExpression,
}

impl CandidateDerivation {
    #[must_use]
    pub const fn new(
        lineage: LineageId,
        role: CandidateRole,
        expression: DerivationExpression,
    ) -> Self {
        Self {
            lineage,
            role,
            expression,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::opaque::{DimensionSymbol, OpaqueValue};
    use crate::numeric::ExactNumber;

    fn literal(value: i64) -> Literal {
        Literal::Numeric {
            value: ExactNumber::from_integer(value),
            dimension: DimensionSymbol::new(OpaqueValue::new(1)),
        }
    }

    #[test]
    fn every_admitted_expression_form_is_representable() {
        let nested = DerivationExpression::Add {
            left: Box::new(DerivationExpression::EvidenceReference(LineageId::new(
                OpaqueValue::new(1),
            ))),
            right: Box::new(DerivationExpression::Multiply {
                operand: Box::new(DerivationExpression::ConfigurationReference(
                    LineageId::new(OpaqueValue::new(2)),
                )),
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
                LineageId::new(OpaqueValue::new(1)),
                role,
                DerivationExpression::Literal(literal(1)),
            );
            assert_eq!(derivation.role, role);
        }
    }
}
