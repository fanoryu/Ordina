//! Exact numeric representation — ADR-0005 §6.1, §6.3.
//!
//! Slice-1 authoritative numeric values are arbitrary-precision **exact
//! rationals**: no binary floating point, no rounding, no implicit tolerance,
//! no lossy conversion, and no magnitude bound (Slice Contract §4.2).
//!
//! This module is the designated isolation boundary (ADR-0005 §6.3): it is the
//! only module that names the third-party numeric crates, so checker logic
//! never depends on them directly and the representation stays replaceable.
//!
//! **Phase 0: no numeric semantics and no public numeric API.** The dependency
//! decision is exercised only by the availability test below, which proves the
//! approved crates resolve and link without committing to a type surface that
//! Phase 2 should choose deliberately.

#[cfg(test)]
mod tests {
    use num_bigint::BigInt;
    use num_rational::BigRational;

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
}
