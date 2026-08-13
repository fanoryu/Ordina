//! Typed literals — Slice Contract §3.6, §4.1.
//!
//! A literal is the right operand of every admitted guard atom and a leaf of
//! every candidate derivation. Its type must match the referent's declared type
//! (§4.1) — but that is a *check*, not a construction rule, so Phase 1 makes
//! mismatches representable and leaves detecting them to check 2 in Phase 5.

use crate::model::opaque::{DimensionSymbol, EnumValue};
use crate::numeric::ExactNumber;

/// A typed literal — Slice Contract §3.6.
///
/// A closed authoritative category (ADR-0005 §7.4): the three admitted types
/// and nothing else. There is no null, absent, `NaN` or infinite variant, and
/// no floating-point variant — §4.1 and §4.2 forbid all of them, and ADR-0005
/// §7.2 makes floats unreachable by construction because no variant admits one.
#[derive(Debug, Clone)]
pub enum Literal {
    /// Carries its declared dimension, which §4.1 requires the referent to
    /// match exactly. §4.6's `DIMENSIONLESS` is simply one such symbol.
    Numeric {
        value: ExactNumber,
        dimension: DimensionSymbol,
    },
    Boolean(bool),
    Enum(EnumValue),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::opaque::OpaqueValue;

    #[test]
    fn all_three_literal_types_are_representable() {
        let numeric = Literal::Numeric {
            value: ExactNumber::from_integer(100),
            dimension: DimensionSymbol::new(OpaqueValue::new(1)),
        };
        let boolean = Literal::Boolean(true);
        let categorical = Literal::Enum(EnumValue::new(OpaqueValue::new(5)));

        for literal in [numeric, boolean, categorical] {
            match literal {
                Literal::Numeric { .. } | Literal::Boolean(_) | Literal::Enum(_) => {}
            }
        }
    }
}
