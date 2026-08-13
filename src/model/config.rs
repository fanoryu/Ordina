//! Declared configuration parameters — Slice Contract §3.9.
//!
//! A Slice-1 configuration parameter declares a numeric quantity with permitted
//! bounds. Its **declared structure and bounds** are identity-bearing
//! (ADR-0003 §6, I7); in-bounds *values* are Decision Inputs bound at decision
//! time and are **absent from the Slice-1 input entirely**, which is why no
//! value field appears here.
//!
//! # The structural restriction that enforces §6.2 of ADR-0003
//!
//! §3.9 permits a configuration reference in **exactly one position**: as a
//! leaf of a candidate-value derivation. That is enforced structurally rather
//! than by a rule anyone has to remember — a guard atom's left operand is an
//! evidence reference (§3.6), and no other construct in this model has a field
//! that could hold a configuration reference. A configuration value therefore
//! cannot change a classification, add or remove a rule, alter Direction
//! topology, select a branch, or change evidence structure.

use crate::model::literal::Literal;
use crate::model::opaque::{DimensionSymbol, LineageId};

/// A declared configuration parameter — Slice Contract §3.9.
///
/// Slice 1 admits `NUMERIC` parameters only, so the type is implied by the
/// struct rather than carried as a field: a non-numeric parameter is
/// unrepresentable.
#[derive(Debug, Clone)]
pub struct ConfigurationParameter {
    pub lineage: LineageId,
    pub dimension: DimensionSymbol,
    /// Declared lower bound. §4.11 requires it to share the parameter's
    /// dimension and to be no greater than the upper bound — both are check 4's
    /// business in Phase 5, so a violating pair stays representable here.
    pub lower_bound: Literal,
    pub upper_bound: Literal,
    /// `true` marks a trader-adjustable Decision Input (RFC-0001 §14). Slice 1
    /// attaches no further meaning to the flag.
    pub adjustable: bool,
}

impl ConfigurationParameter {
    #[must_use]
    pub const fn new(
        lineage: LineageId,
        dimension: DimensionSymbol,
        lower_bound: Literal,
        upper_bound: Literal,
        adjustable: bool,
    ) -> Self {
        Self {
            lineage,
            dimension,
            lower_bound,
            upper_bound,
            adjustable,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::opaque::OpaqueValue;
    use crate::numeric::ExactNumber;

    fn bound(value: i64) -> Literal {
        Literal::Numeric {
            value: ExactNumber::from_integer(value),
            dimension: DimensionSymbol::new(OpaqueValue::new(1)),
        }
    }

    #[test]
    fn a_parameter_declares_structure_and_bounds_but_no_value() {
        let parameter = ConfigurationParameter::new(
            LineageId::new(OpaqueValue::new(1)),
            DimensionSymbol::new(OpaqueValue::new(1)),
            bound(1),
            bound(10),
            true,
        );

        assert!(parameter.adjustable);
    }

    #[test]
    fn check_4_faults_remain_representable() {
        // An inverted pair is a §5.1 check 4 defect in Phase 5, so Phase 1 must
        // let it be built.
        let inverted = ConfigurationParameter::new(
            LineageId::new(OpaqueValue::new(2)),
            DimensionSymbol::new(OpaqueValue::new(1)),
            bound(10),
            bound(1),
            false,
        );

        assert!(!inverted.adjustable);
    }
}
