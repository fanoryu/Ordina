//! Boolean constraint domains — Slice Contract §4.10.
//!
//! A `BOOLEAN` constraint domain is a subset of `{true, false}` (§4.10). Each
//! admitted atom (`=`/`!=`, §3.6) pins the reference to a single value, so the
//! domain is the intersection of those singletons with the full set.

use super::EqualityOperator;

/// A per-reference `BOOLEAN` constraint domain (§4.10).
#[derive(Debug, Clone)]
pub struct BooleanDomain {
    allows_true: bool,
    allows_false: bool,
}

impl BooleanDomain {
    /// The unconstrained domain — `{true, false}` (§4.10, no atom references the
    /// reference).
    #[must_use]
    pub const fn unconstrained() -> Self {
        Self {
            allows_true: true,
            allows_false: true,
        }
    }

    /// The constraint domain imposed by a reference's atoms — §4.10.
    ///
    /// `= v` admits only `v`; `!= v` admits only `!v`. Each atom therefore pins
    /// to one value, and the domain is their intersection.
    #[must_use]
    pub fn from_constraints(constraints: &[(EqualityOperator, bool)]) -> Self {
        let mut domain = Self::unconstrained();
        for (operator, value) in constraints {
            let admitted = match operator {
                // No catch-all `_` arm (ADR-0005 §7.4).
                EqualityOperator::Equal => *value,
                EqualityOperator::NotEqual => !value,
            };
            domain.allows_true = domain.allows_true && admitted;
            domain.allows_false = domain.allows_false && !admitted;
        }
        domain
    }

    /// Whether `value` is admitted.
    #[must_use]
    pub const fn admits(&self, value: bool) -> bool {
        if value {
            self.allows_true
        } else {
            self.allows_false
        }
    }

    /// Contradiction, per reference — §4.10: the domain is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        !self.allows_true && !self.allows_false
    }

    /// Intersection — §4.10.
    #[must_use]
    pub const fn intersect(&self, other: &Self) -> Self {
        Self {
            allows_true: self.allows_true && other.allows_true,
            allows_false: self.allows_false && other.allows_false,
        }
    }

    /// Overlap, per reference — §4.10: the intersection is non-empty.
    #[must_use]
    pub const fn overlaps(&self, other: &Self) -> bool {
        !self.intersect(other).is_empty()
    }

    /// Containment — §4.10: whether `inner ⊆ self`.
    #[must_use]
    pub const fn contains(&self, inner: &Self) -> bool {
        (!inner.allows_true || self.allows_true) && (!inner.allows_false || self.allows_false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use EqualityOperator::{Equal, NotEqual};

    #[test]
    fn unconstrained_admits_both() {
        let d = BooleanDomain::unconstrained();
        assert!(d.admits(true));
        assert!(d.admits(false));
        assert!(!d.is_empty());
    }

    #[test]
    fn equality_pins_to_one_value() {
        let t = BooleanDomain::from_constraints(&[(Equal, true)]);
        assert!(t.admits(true));
        assert!(!t.admits(false));

        let f = BooleanDomain::from_constraints(&[(Equal, false)]);
        assert!(!f.admits(true));
        assert!(f.admits(false));
    }

    #[test]
    fn inequality_pins_to_the_other_value() {
        let not_true = BooleanDomain::from_constraints(&[(NotEqual, true)]);
        assert!(!not_true.admits(true));
        assert!(not_true.admits(false));
    }

    #[test]
    fn conflicting_atoms_are_a_contradiction() {
        // = true and = false
        assert!(BooleanDomain::from_constraints(&[(Equal, true), (Equal, false)]).is_empty());
        // = true and != true
        assert!(BooleanDomain::from_constraints(&[(Equal, true), (NotEqual, true)]).is_empty());
    }

    #[test]
    fn intersection_and_overlap() {
        let t = BooleanDomain::from_constraints(&[(Equal, true)]);
        let f = BooleanDomain::from_constraints(&[(Equal, false)]);
        let full = BooleanDomain::unconstrained();

        assert!(t.intersect(&f).is_empty());
        assert!(!t.overlaps(&f));
        assert!(t.overlaps(&full));
        assert!(t.overlaps(&t));
    }

    #[test]
    fn intersection_is_commutative_and_idempotent() {
        let a = BooleanDomain::from_constraints(&[(Equal, true)]);
        let b = BooleanDomain::unconstrained();
        let ab = a.intersect(&b);
        let ba = b.intersect(&a);
        assert!(ab.contains(&ba) && ba.contains(&ab));
        let aa = a.intersect(&a);
        assert!(aa.contains(&a) && a.contains(&aa));
    }

    #[test]
    fn contradiction_absorbs_intersection() {
        let empty = BooleanDomain::from_constraints(&[(Equal, true), (Equal, false)]);
        let full = BooleanDomain::unconstrained();
        assert!(empty.intersect(&full).is_empty());
        assert!(full.intersect(&empty).is_empty());
    }

    #[test]
    fn containment_is_reflexive_and_directional() {
        let full = BooleanDomain::unconstrained();
        let t = BooleanDomain::from_constraints(&[(Equal, true)]);
        assert!(full.contains(&full)); // reflexive
        assert!(full.contains(&t));
        assert!(!t.contains(&full));
    }

    #[test]
    fn containment_transitivity() {
        let full = BooleanDomain::unconstrained();
        let t = BooleanDomain::from_constraints(&[(Equal, true)]);
        let empty = BooleanDomain::from_constraints(&[(Equal, true), (Equal, false)]);
        // empty ⊆ t ⊆ full
        assert!(t.contains(&empty) && full.contains(&t));
        assert!(full.contains(&empty));
    }

    #[test]
    fn empty_is_contained_in_anything() {
        let empty = BooleanDomain::from_constraints(&[(Equal, true), (Equal, false)]);
        let t = BooleanDomain::from_constraints(&[(Equal, true)]);
        assert!(t.contains(&empty));
    }
}
