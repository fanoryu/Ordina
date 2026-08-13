//! Exact numeric representation — ADR-0005 §6.1, §6.3.
//!
//! Slice-1 authoritative numeric values are arbitrary-precision **exact
//! rationals**: no binary floating point, no rounding, no implicit tolerance,
//! no lossy conversion, no magnitude bound (Slice Contract §4.2).
//!
//! This module is the ADR-0005 §6.3 isolation boundary — the only module that
//! names the approved numeric crates, so checker logic never depends on them
//! directly and the representation stays replaceable.
//!
//! # Phase 1 scope
//!
//! Phase 1 introduces **only** what makes a `NUMERIC` literal structurally
//! representable, as the Product Board approved when disposing of the Phase-1
//! sequencing dependency. That permission is *strictly representational*.
//!
//! Deliberately **not** implemented here yet, and reserved for Phase 2:
//! §4.2 numeric semantics, §4.3 equality, §4.4 comparison, §4.6 dimensional
//! rules, and arithmetic. `ExactNumber` therefore has **no** `PartialEq`, `Ord`
//! or arithmetic implementation — defining equality now would be defining
//! §4.3 semantics early, and the contract's equality additionally requires an
//! identical declared dimension, which is not this type's business.

use core::num::NonZeroI64;

use num_bigint::BigInt;
use num_rational::BigRational;

/// An exact rational value — the authoritative Slice-1 numeric representation.
///
/// The inner value is unread in Phase 1 by design: nothing may interpret a
/// number until Phase 2 defines what interpretation means.
#[derive(Debug, Clone)]
pub struct ExactNumber(#[allow(dead_code)] BigRational);

impl ExactNumber {
    /// An exact integer value.
    #[must_use]
    pub fn from_integer(value: i64) -> Self {
        Self(BigRational::from(BigInt::from(value)))
    }

    /// An exact ratio.
    ///
    /// The denominator is a `NonZeroI64`, so a zero denominator is
    /// unrepresentable rather than a runtime panic. Note this is a
    /// *construction* guarantee, not division: §3.8 excludes division from the
    /// admitted expression grammar entirely.
    #[must_use]
    pub fn from_ratio(numerator: i64, denominator: NonZeroI64) -> Self {
        Self(BigRational::new(
            BigInt::from(numerator),
            BigInt::from(denominator.get()),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Proves the ADR-0005 §6.2 dependencies resolve, link, and deliver *exact*
    /// arithmetic. One third summed three times is exactly one — an identity
    /// binary floating point cannot represent, which is the point of the
    /// exact-rational decision.
    #[test]
    fn approved_numeric_dependencies_resolve_and_are_exact() {
        let third = BigRational::new(BigInt::from(1), BigInt::from(3));
        let sum = third.clone() + third.clone() + third.clone();

        assert_eq!(sum, BigRational::from(BigInt::from(1)));
    }

    #[test]
    fn exact_numbers_are_constructible() {
        let _integer = ExactNumber::from_integer(42);
        let _ratio = ExactNumber::from_ratio(3, NonZeroI64::new(2).expect("2 is non-zero"));
    }
}
