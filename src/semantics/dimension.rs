//! Dimensional semantics — Slice Contract §4.6.
//!
//! §4.6's dimensional rules over the admitted derivation grammar (§3.8), as
//! semantic **primitives**. This module decides the result dimension of an
//! additive node and a scaling node, and reports a mismatch where the rule is
//! violated. It does **not** traverse a derivation, and it is **not** check 14A
//! — the check composes these primitives in a later phase.
//!
//! # `DIMENSIONLESS` is supplied, never manufactured (§2.1, F-3)
//!
//! §4.6's scaling rule turns on whether the scalar is a **dimensionless
//! literal**, and §2.1 of the accepted contract (Revision 2, PBR-0014) makes the
//! identity of the distinguished `DIMENSIONLESS` dimension symbol **constitutive
//! canonical input**: it is supplied, and a conforming implementation consumes
//! the supplied binding rather than manufacturing, inferring, or defaulting one.
//!
//! [`DimensionBinding`] carries that supplied identity. It is taken **by
//! reference, per invocation** ([B-1]): there is no module-global dimensionless
//! symbol, no static, no registry, no lazy initialization, and no environment
//! lookup anywhere in this module. A caller that does not hold the supplied
//! binding cannot ask a dimensional question, which is the structural expression
//! of B-1.
//!
//! A dimension is an **opaque symbol compared by exact equality only** (§4.6):
//! this module compares `DimensionSymbol` values with `==` and never reads their
//! representation, their handle magnitude, or their position in the
//! contract-supplied order.
//!
//! [B-1]: the distinguished-member binding is passed with the canonical input,
//! per invocation.

use crate::model::literal::Literal;
use crate::model::opaque::DimensionSymbol;

/// The canonical-input binding for the distinguished `DIMENSIONLESS` dimension
/// symbol — Slice Contract §2.1.
///
/// Holds the one admitted `DimensionSymbol` the canonical input designates as
/// dimensionless. The identity is the caller's to supply; this type only carries
/// it and answers the exact-equality question §4.6 needs. It deliberately offers
/// no way to *derive* a dimensionless symbol — no default, no "first symbol", no
/// construction from a name — so the only dimensionless identity that exists is
/// the supplied one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DimensionBinding {
    dimensionless: DimensionSymbol,
}

impl DimensionBinding {
    /// Binds the supplied distinguished dimensionless symbol.
    ///
    /// The argument is the value the canonical input designates; this
    /// constructor selects nothing.
    #[must_use]
    pub const fn new(dimensionless: DimensionSymbol) -> Self {
        Self { dimensionless }
    }

    /// Whether `dimension` is the supplied dimensionless symbol, by exact
    /// `DimensionSymbol` identity (§4.6).
    ///
    /// This is the *only* way to recognise dimensionlessness in this crate.
    #[must_use]
    pub fn is_dimensionless(&self, dimension: DimensionSymbol) -> bool {
        self.dimensionless == dimension
    }

    /// The supplied distinguished dimensionless symbol, for **ADR-0009
    /// persistence transport only** — `pub(crate)`.
    ///
    /// The binding is caller-supplied and round-trips verbatim across a
    /// save/restart; this reader lets `ordina::persistence` transport it without
    /// exposing any public accessor. It selects nothing and derives nothing.
    #[must_use]
    pub(crate) const fn dimensionless(&self) -> DimensionSymbol {
        self.dimensionless
    }
}

/// The result of a §4.6 dimensional rule.
///
/// A closed category (ADR-0005 §7.4): matched exhaustively with no catch-all
/// arm. A dimensional-rule violation is a `Mismatch`, never a repaired or
/// coerced dimension (§4.6: "never repaired by conversion, scaling, or
/// coercion").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DimensionOutcome {
    /// The rule holds; the node's result dimension is this symbol.
    Dimension(DimensionSymbol),
    /// The rule is violated. For `a + b` / `a - b`, the operands disagree; for
    /// `a * s`, the scalar is not a dimensionless literal.
    Mismatch,
}

/// The dimensional rule for `a + b` and `a - b` — §4.6.
///
/// Requires `dim(a) = dim(b)` by exact identity; the result dimension is
/// `dim(a)`. Disagreement is a `Mismatch` (a check-14 defect in a later phase).
#[must_use]
pub fn additive_dimension(left: DimensionSymbol, right: DimensionSymbol) -> DimensionOutcome {
    if left == right {
        DimensionOutcome::Dimension(left)
    } else {
        DimensionOutcome::Mismatch
    }
}

/// The dimensional rule for `a * s` — §4.6.
///
/// §4.6 admits scaling only where `s` is a **dimensionless literal**; the result
/// dimension is then `dim(a)`. Whether the scalar is dimensionless is decided by
/// the supplied [`DimensionBinding`], never by the scalar's representation. A
/// scalar that is not a numeric literal carrying the dimensionless symbol yields
/// `Mismatch`.
#[must_use]
pub fn scaling_dimension(
    operand: DimensionSymbol,
    scalar: &Literal,
    binding: &DimensionBinding,
) -> DimensionOutcome {
    match scalar {
        // No catch-all `_` arm (ADR-0005 §7.4).
        Literal::Numeric { dimension, .. } => {
            if binding.is_dimensionless(*dimension) {
                DimensionOutcome::Dimension(operand)
            } else {
                DimensionOutcome::Mismatch
            }
        }
        Literal::Boolean(_) | Literal::Enum(_) => DimensionOutcome::Mismatch,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::opaque::{DimensionSymbol, OpaqueValue};
    use crate::numeric::ExactNumber;

    fn dimension(handle: u64) -> DimensionSymbol {
        DimensionSymbol::new(OpaqueValue::new(handle))
    }

    fn numeric(dim: DimensionSymbol) -> Literal {
        Literal::Numeric {
            value: ExactNumber::from_integer(1),
            dimension: dim,
        }
    }

    // -- §4.6 additive rule -------------------------------------------------

    #[test]
    fn additive_agrees_on_identical_dimensions() {
        assert_eq!(
            additive_dimension(dimension(7), dimension(7)),
            DimensionOutcome::Dimension(dimension(7))
        );
    }

    #[test]
    fn additive_rejects_differing_dimensions() {
        assert_eq!(
            additive_dimension(dimension(7), dimension(8)),
            DimensionOutcome::Mismatch
        );
    }

    // -- §4.6 scaling rule --------------------------------------------------

    #[test]
    fn scaling_accepts_a_dimensionless_scalar_and_keeps_the_operand_dimension() {
        let binding = DimensionBinding::new(dimension(0));
        let scalar = numeric(dimension(0));

        assert_eq!(
            scaling_dimension(dimension(7), &scalar, &binding),
            DimensionOutcome::Dimension(dimension(7))
        );
    }

    #[test]
    fn scaling_rejects_a_dimensioned_scalar() {
        let binding = DimensionBinding::new(dimension(0));
        // Same numeric value, but a non-dimensionless dimension.
        let scalar = numeric(dimension(7));

        assert_eq!(
            scaling_dimension(dimension(7), &scalar, &binding),
            DimensionOutcome::Mismatch
        );
    }

    #[test]
    fn scaling_rejects_a_non_numeric_scalar() {
        let binding = DimensionBinding::new(dimension(0));

        assert_eq!(
            scaling_dimension(dimension(7), &Literal::Boolean(true), &binding),
            DimensionOutcome::Mismatch
        );
        assert_eq!(
            scaling_dimension(
                dimension(7),
                &Literal::Enum(crate::model::opaque::EnumValue::new(OpaqueValue::new(3))),
                &binding
            ),
            DimensionOutcome::Mismatch
        );
    }

    // -- F-3 / §2.1: dimensionlessness follows the supplied binding ---------

    #[test]
    fn dimensionlessness_follows_the_binding_not_the_handle() {
        // Handle 0 is dimensionless under one binding and dimensioned under
        // another. The scalar is byte-for-byte the same; only the supplied
        // binding differs, and the result follows the binding.
        let scalar = numeric(dimension(0));

        let binding_zero = DimensionBinding::new(dimension(0));
        let binding_five = DimensionBinding::new(dimension(5));

        assert_eq!(
            scaling_dimension(dimension(7), &scalar, &binding_zero),
            DimensionOutcome::Dimension(dimension(7))
        );
        assert_eq!(
            scaling_dimension(dimension(7), &scalar, &binding_five),
            DimensionOutcome::Mismatch
        );
    }

    #[test]
    fn order_position_does_not_determine_dimensionlessness() {
        // The dimensionless symbol has the *highest* handle here, so no
        // "first / lowest is dimensionless" convention could produce this. The
        // binding — not order position — decides.
        let binding = DimensionBinding::new(dimension(9));
        let low = numeric(dimension(1));
        let high = numeric(dimension(9));

        assert_eq!(
            scaling_dimension(dimension(2), &low, &binding),
            DimensionOutcome::Mismatch
        );
        assert_eq!(
            scaling_dimension(dimension(2), &high, &binding),
            DimensionOutcome::Dimension(dimension(2))
        );
    }

    #[test]
    fn relabelling_handles_and_binding_consistently_preserves_the_result() {
        // Two semantically equivalent scenarios with disjoint opaque handles.
        // Scenario A: operand dim 7, dimensionless dim 0.
        // Scenario B: operand dim 70, dimensionless dim 100.
        // With the binding relabelled consistently, the scaling result is the
        // correspondingly-relabelled `Dimension(operand)` in both — behaviour
        // depends on identity/binding, not on the incidental handle values.
        let a = scaling_dimension(
            dimension(7),
            &numeric(dimension(0)),
            &DimensionBinding::new(dimension(0)),
        );
        let b = scaling_dimension(
            dimension(70),
            &numeric(dimension(100)),
            &DimensionBinding::new(dimension(100)),
        );

        assert_eq!(a, DimensionOutcome::Dimension(dimension(7)));
        assert_eq!(b, DimensionOutcome::Dimension(dimension(70)));

        // And the mismatch case relabels identically: a scalar whose dimension
        // is not the bound one fails in both scenarios.
        let a_bad = scaling_dimension(
            dimension(7),
            &numeric(dimension(1)),
            &DimensionBinding::new(dimension(0)),
        );
        let b_bad = scaling_dimension(
            dimension(70),
            &numeric(dimension(101)),
            &DimensionBinding::new(dimension(100)),
        );
        assert_eq!(a_bad, DimensionOutcome::Mismatch);
        assert_eq!(b_bad, DimensionOutcome::Mismatch);
    }

    #[test]
    fn the_dimension_outcome_is_exhaustively_matchable() {
        // No catch-all `_` arm (ADR-0005 §7.4).
        let outcome = additive_dimension(dimension(1), dimension(1));
        let described = match outcome {
            DimensionOutcome::Dimension(_) => "dimension",
            DimensionOutcome::Mismatch => "mismatch",
        };
        assert_eq!(described, "dimension");
    }
}
