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
//!
//! Structural prevention is the right tool *there* because §3.9's restriction is
//! about which positions exist at all. It is the wrong tool for the parameter's
//! own properties: §3.9 lists "an absent bound" among its prohibited shapes and
//! check 1 owns it, so each declared property below is a declaration state.

use crate::model::declared::Declared;
use crate::model::literal::Literal;
use crate::model::opaque::{DimensionSymbol, LineageId};

/// A declared configuration parameter — Slice Contract §3.9.
///
/// Slice 1 admits `NUMERIC` parameters only, so the declared type is implied by
/// the struct rather than carried as a field: a non-numeric parameter is
/// unrepresentable, and no check owns that shape.
#[derive(Debug, Clone)]
pub struct ConfigurationParameter {
    pub lineage: Declared<LineageId>,
    pub dimension: Declared<DimensionSymbol>,
    /// Declared lower bound. §4.11 requires it to share the parameter's
    /// dimension and to be no greater than the upper bound — both are check 4's
    /// business in Phase 5, so a violating pair stays representable here.
    pub lower_bound: Declared<Literal>,
    pub upper_bound: Declared<Literal>,
    /// `true` marks a trader-adjustable Decision Input (RFC-0001 §14). Slice 1
    /// attaches no further meaning to the flag.
    pub adjustable: Declared<bool>,
}

impl ConfigurationParameter {
    /// A parameter with every property declared — the well-formed shape.
    /// Malformed shapes are built from the fields directly.
    #[must_use]
    pub const fn new(
        lineage: LineageId,
        dimension: DimensionSymbol,
        lower_bound: Literal,
        upper_bound: Literal,
        adjustable: bool,
    ) -> Self {
        Self {
            lineage: Declared::Present(lineage),
            dimension: Declared::Present(dimension),
            lower_bound: Declared::Present(lower_bound),
            upper_bound: Declared::Present(upper_bound),
            adjustable: Declared::Present(adjustable),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::opaque::OpaqueValue;
    use crate::numeric::ExactNumber;

    fn lineage(handle: u64) -> LineageId {
        LineageId::new(OpaqueValue::new(handle))
    }

    fn dimension() -> DimensionSymbol {
        DimensionSymbol::new(OpaqueValue::new(1))
    }

    fn bound(value: i64) -> Literal {
        Literal::Numeric {
            value: ExactNumber::from_integer(value),
            dimension: dimension(),
        }
    }

    #[test]
    fn a_parameter_declares_structure_and_bounds_but_no_value() {
        let parameter =
            ConfigurationParameter::new(lineage(1), dimension(), bound(1), bound(10), true);

        assert_eq!(parameter.adjustable, Declared::Present(true));
        assert!(parameter.lower_bound.is_present());
        assert!(parameter.upper_bound.is_present());
    }

    #[test]
    fn check_4_faults_remain_representable() {
        // An inverted pair is a §5.1 check 4 defect in Phase 5, so Phase 1 must
        // let it be built.
        let inverted =
            ConfigurationParameter::new(lineage(2), dimension(), bound(10), bound(1), false);

        assert_eq!(inverted.adjustable, Declared::Present(false));
    }

    #[test]
    fn check_1_absences_remain_representable() {
        // §3.9 lists "an absent bound" among its prohibited shapes, and every
        // property in its table is required, so check 1 must be able to find
        // each one missing.
        let bare = ConfigurationParameter {
            lineage: Declared::Absent,
            dimension: Declared::Absent,
            lower_bound: Declared::Absent,
            upper_bound: Declared::Absent,
            adjustable: Declared::Absent,
        };

        assert!(bare.lineage.is_absent());
        assert!(bare.dimension.is_absent());
        assert!(bare.lower_bound.is_absent());
        assert!(bare.upper_bound.is_absent());
        assert!(bare.adjustable.is_absent());
    }
}
