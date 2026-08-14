//! Enum constraint domains — Slice Contract §4.10.
//!
//! An `ENUM` constraint domain is a subset of the **declared value set** (§4.10).
//! It starts as the full declared set (unconstrained) and each admitted atom
//! (`=`/`!=`, §3.6) narrows it: `= v` intersects with `{v}`, `!= v` removes `v`.
//!
//! The admitted set is order-independent — a categorical value is an **opaque
//! symbol compared by exact equality** (§3.5), so this module compares
//! `EnumValue`s with `==` and never reads their representation or order position.
//! Every operation treats the stored sequence as a set: membership and the
//! relations are invariant under consistent relabelling of the opaque handles.

use super::EqualityOperator;
use crate::model::opaque::EnumValue;

/// A per-reference `ENUM` constraint domain (§4.10) — the admitted subset of the
/// declared value set.
#[derive(Debug, Clone)]
pub struct EnumDomain {
    admitted: Vec<EnumValue>,
}

/// Deduplicates by exact identity, preserving first appearance. The result is
/// treated as a set; order carries no meaning.
fn as_set(values: &[EnumValue]) -> Vec<EnumValue> {
    let mut set: Vec<EnumValue> = Vec::new();
    for value in values {
        if !set.iter().any(|existing| existing == value) {
            set.push(*value);
        }
    }
    set
}

impl EnumDomain {
    /// The unconstrained domain — the full declared value set (§4.10).
    #[must_use]
    pub fn unconstrained(declared: &[EnumValue]) -> Self {
        Self {
            admitted: as_set(declared),
        }
    }

    /// The constraint domain imposed by a reference's atoms — §4.10.
    ///
    /// Starts from the declared set; `= v` restricts to `{v}` (empty if `v` is
    /// not declared), `!= v` removes `v`.
    #[must_use]
    pub fn from_constraints(
        declared: &[EnumValue],
        constraints: &[(EqualityOperator, EnumValue)],
    ) -> Self {
        let mut admitted = as_set(declared);
        for (operator, value) in constraints {
            match operator {
                // No catch-all `_` arm (ADR-0005 §7.4).
                EqualityOperator::Equal => admitted.retain(|candidate| candidate == value),
                EqualityOperator::NotEqual => admitted.retain(|candidate| candidate != value),
            }
        }
        Self { admitted }
    }

    /// Whether `value` is admitted.
    #[must_use]
    pub fn admits(&self, value: &EnumValue) -> bool {
        self.admitted.iter().any(|candidate| candidate == value)
    }

    /// Contradiction, per reference — §4.10: the domain is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.admitted.is_empty()
    }

    /// Intersection — §4.10: the values admitted by both.
    #[must_use]
    pub fn intersect(&self, other: &Self) -> Self {
        let admitted = self
            .admitted
            .iter()
            .filter(|value| other.admits(value))
            .copied()
            .collect();
        Self { admitted }
    }

    /// Overlap, per reference — §4.10: the intersection is non-empty.
    #[must_use]
    pub fn overlaps(&self, other: &Self) -> bool {
        self.admitted.iter().any(|value| other.admits(value))
    }

    /// Containment — §4.10: whether `inner ⊆ self`.
    #[must_use]
    pub fn contains(&self, inner: &Self) -> bool {
        inner.admitted.iter().all(|value| self.admits(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::opaque::OpaqueValue;
    use EqualityOperator::{Equal, NotEqual};

    fn ev(handle: u64) -> EnumValue {
        EnumValue::new(OpaqueValue::new(handle))
    }

    #[test]
    fn unconstrained_admits_the_full_declared_set() {
        let declared = [ev(1), ev(2), ev(3)];
        let d = EnumDomain::unconstrained(&declared);
        assert!(d.admits(&ev(1)) && d.admits(&ev(2)) && d.admits(&ev(3)));
        assert!(!d.admits(&ev(4)));
        assert!(!d.is_empty());
    }

    #[test]
    fn equality_restricts_to_one_value() {
        let declared = [ev(1), ev(2), ev(3)];
        let d = EnumDomain::from_constraints(&declared, &[(Equal, ev(2))]);
        assert!(d.admits(&ev(2)));
        assert!(!d.admits(&ev(1)));
        assert!(!d.admits(&ev(3)));
    }

    #[test]
    fn inequality_removes_a_value() {
        let declared = [ev(1), ev(2), ev(3)];
        let d = EnumDomain::from_constraints(&declared, &[(NotEqual, ev(2))]);
        assert!(d.admits(&ev(1)) && d.admits(&ev(3)));
        assert!(!d.admits(&ev(2)));
    }

    #[test]
    fn equality_to_an_undeclared_value_is_empty() {
        let declared = [ev(1), ev(2)];
        let d = EnumDomain::from_constraints(&declared, &[(Equal, ev(9))]);
        assert!(d.is_empty());
    }

    #[test]
    fn removing_every_value_is_empty() {
        let declared = [ev(1), ev(2)];
        let d = EnumDomain::from_constraints(&declared, &[(NotEqual, ev(1)), (NotEqual, ev(2))]);
        assert!(d.is_empty());
    }

    #[test]
    fn disjoint_subsets_do_not_overlap() {
        let declared = [ev(1), ev(2), ev(3), ev(4)];
        let a = EnumDomain::from_constraints(&declared, &[(NotEqual, ev(3)), (NotEqual, ev(4))]); // {1,2}
        let b = EnumDomain::from_constraints(&declared, &[(NotEqual, ev(1)), (NotEqual, ev(2))]); // {3,4}
        assert!(!a.overlaps(&b));
        assert!(a.intersect(&b).is_empty());
    }

    #[test]
    fn intersection_and_overlap() {
        let declared = [ev(1), ev(2), ev(3)];
        let a = EnumDomain::from_constraints(&declared, &[(NotEqual, ev(3))]); // {1,2}
        let b = EnumDomain::from_constraints(&declared, &[(NotEqual, ev(1))]); // {2,3}
        assert!(a.overlaps(&b));
        let i = a.intersect(&b); // {2}
        assert!(i.admits(&ev(2)) && !i.admits(&ev(1)) && !i.admits(&ev(3)));
    }

    #[test]
    fn intersection_is_commutative_and_idempotent() {
        let declared = [ev(1), ev(2), ev(3)];
        let a = EnumDomain::from_constraints(&declared, &[(NotEqual, ev(3))]);
        let b = EnumDomain::from_constraints(&declared, &[(NotEqual, ev(1))]);
        let ab = a.intersect(&b);
        let ba = b.intersect(&a);
        assert!(ab.contains(&ba) && ba.contains(&ab));
        let aa = a.intersect(&a);
        assert!(aa.contains(&a) && a.contains(&aa));
    }

    #[test]
    fn contradiction_absorbs_intersection() {
        let declared = [ev(1), ev(2)];
        let empty = EnumDomain::from_constraints(&declared, &[(Equal, ev(9))]);
        let full = EnumDomain::unconstrained(&declared);
        assert!(empty.intersect(&full).is_empty());
        assert!(full.intersect(&empty).is_empty());
    }

    #[test]
    fn containment_reflexive_transitive_and_directional() {
        let declared = [ev(1), ev(2), ev(3)];
        let full = EnumDomain::unconstrained(&declared); // {1,2,3}
        let two = EnumDomain::from_constraints(&declared, &[(NotEqual, ev(3))]); // {1,2}
        let one = EnumDomain::from_constraints(&declared, &[(Equal, ev(1))]); // {1}
        assert!(full.contains(&full)); // reflexive
        assert!(full.contains(&two) && two.contains(&one));
        assert!(full.contains(&one)); // transitive
        assert!(!one.contains(&two)); // directional
    }

    #[test]
    fn empty_is_contained_in_anything() {
        let declared = [ev(1), ev(2)];
        let empty = EnumDomain::from_constraints(&declared, &[(Equal, ev(9))]);
        let some = EnumDomain::from_constraints(&declared, &[(Equal, ev(1))]);
        assert!(some.contains(&empty));
    }

    #[test]
    fn containment_implies_overlap_for_non_empty_domains() {
        let declared = [ev(1), ev(2), ev(3)];
        let outer = EnumDomain::unconstrained(&declared);
        let inner = EnumDomain::from_constraints(&declared, &[(Equal, ev(2))]);
        assert!(outer.contains(&inner) && !inner.is_empty());
        assert!(outer.overlaps(&inner));
    }

    #[test]
    fn relations_are_invariant_under_consistent_relabelling() {
        // Scenario A over handles {1,2,3}; scenario B over disjoint {41,42,43}
        // with the SAME semantic structure ({1,2} vs {2,3} ~ {41,42} vs {42,43}).
        let da = [ev(1), ev(2), ev(3)];
        let a1 = EnumDomain::from_constraints(&da, &[(NotEqual, ev(3))]); // {1,2}
        let a2 = EnumDomain::from_constraints(&da, &[(NotEqual, ev(1))]); // {2,3}

        let db = [ev(41), ev(42), ev(43)];
        let b1 = EnumDomain::from_constraints(&db, &[(NotEqual, ev(43))]); // {41,42}
        let b2 = EnumDomain::from_constraints(&db, &[(NotEqual, ev(41))]); // {42,43}

        // Every relation agrees across the relabelling — behaviour depends on
        // structure, not on the incidental handle values or their order.
        assert_eq!(a1.overlaps(&a2), b1.overlaps(&b2));
        assert_eq!(a1.contains(&a2), b1.contains(&b2));
        assert_eq!(a1.intersect(&a2).is_empty(), b1.intersect(&b2).is_empty());
    }

    #[test]
    fn admitted_set_is_order_independent() {
        // Declared in two different orders yields the same relations.
        let forward = EnumDomain::unconstrained(&[ev(1), ev(2), ev(3)]);
        let reversed = EnumDomain::unconstrained(&[ev(3), ev(2), ev(1)]);
        assert!(forward.contains(&reversed) && reversed.contains(&forward));
    }
}
