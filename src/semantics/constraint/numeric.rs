//! Numeric constraint domains — Slice Contract §4.10.
//!
//! A numeric constraint domain is an interval — open, closed, or a single point
//! — **minus a finite set of excluded points** (§4.10). The interval is the
//! intersection of the half-lines and points the atoms impose; the excluded set
//! is contributed by `!=` atoms. Every operation is an exact comparison of
//! declared `ExactNumber` values; nothing here is approximate, and no solver is
//! involved.
//!
//! The domain is **per reference**, so every atom shares one declared dimension
//! (§4.10) and the algebra compares values alone — dimension is a typing concern
//! (check 2, Phase 2B), not part of this interval algebra.

use crate::model::guard::ComparisonOperator;
use crate::numeric::ExactNumber;

/// One end of an interval.
///
/// A closed category, matched exhaustively with no catch-all arm.
#[derive(Debug, Clone)]
pub enum Bound {
    /// No bound on this side (±∞).
    Unbounded,
    /// The boundary value is admitted (`<=` / `>=` / `=`).
    Inclusive(ExactNumber),
    /// The boundary value is excluded (`<` / `>`).
    Exclusive(ExactNumber),
}

/// A per-reference `NUMERIC` constraint domain (§4.10).
#[derive(Debug, Clone)]
pub struct NumericDomain {
    lower: Bound,
    upper: Bound,
    excluded: Vec<ExactNumber>,
}

/// Whether the lower half-line defined by `outer` admits every value the lower
/// half-line defined by `inner` admits (`outer ⊇ inner` as lower constraints).
fn lower_contains(outer: &Bound, inner: &Bound) -> bool {
    match outer {
        Bound::Unbounded => true,
        Bound::Inclusive(a) => match inner {
            Bound::Unbounded => false,
            // {x ≥ a} ⊇ {x ≥ b} and {x ≥ a} ⊇ {x > b} both need a ≤ b.
            Bound::Inclusive(b) | Bound::Exclusive(b) => a <= b,
        },
        Bound::Exclusive(a) => match inner {
            Bound::Unbounded => false,
            // {x > a} ⊇ {x ≥ b} needs a < b.
            Bound::Inclusive(b) => a < b,
            // {x > a} ⊇ {x > b} needs a ≤ b.
            Bound::Exclusive(b) => a <= b,
        },
    }
}

/// Whether the upper half-line defined by `outer` admits every value the upper
/// half-line defined by `inner` admits (`outer ⊇ inner` as upper constraints).
fn upper_contains(outer: &Bound, inner: &Bound) -> bool {
    match outer {
        Bound::Unbounded => true,
        Bound::Inclusive(a) => match inner {
            Bound::Unbounded => false,
            Bound::Inclusive(b) | Bound::Exclusive(b) => a >= b,
        },
        Bound::Exclusive(a) => match inner {
            Bound::Unbounded => false,
            Bound::Inclusive(b) => a > b,
            Bound::Exclusive(b) => a >= b,
        },
    }
}

/// The tighter (more restrictive) of two lower bounds.
fn tighter_lower(left: Bound, right: Bound) -> Bound {
    // `lower_contains(left, right)` means left admits at least as much, so right
    // is the tighter constraint.
    if lower_contains(&left, &right) {
        right
    } else {
        left
    }
}

/// The tighter (more restrictive) of two upper bounds.
fn tighter_upper(left: Bound, right: Bound) -> Bound {
    if upper_contains(&left, &right) {
        right
    } else {
        left
    }
}

impl NumericDomain {
    /// The unconstrained domain — every value (`D(G, e)` when no atom references
    /// `e`, §4.10).
    #[must_use]
    pub const fn unconstrained() -> Self {
        Self {
            lower: Bound::Unbounded,
            upper: Bound::Unbounded,
            excluded: Vec::new(),
        }
    }

    /// The constraint domain imposed by a reference's atoms — §4.10.
    ///
    /// Each `(operator, value)` is one atom `e op value`. `<`/`<=` tighten the
    /// upper bound, `>`/`>=` the lower, `=` pins both, and `!=` excludes a point.
    #[must_use]
    pub fn from_constraints(constraints: &[(ComparisonOperator, ExactNumber)]) -> Self {
        let mut domain = Self::unconstrained();
        for (operator, value) in constraints {
            match operator {
                // No catch-all `_` arm (ADR-0005 §7.4).
                ComparisonOperator::Less => {
                    domain.upper = tighter_upper(domain.upper, Bound::Exclusive(value.clone()));
                }
                ComparisonOperator::LessOrEqual => {
                    domain.upper = tighter_upper(domain.upper, Bound::Inclusive(value.clone()));
                }
                ComparisonOperator::Greater => {
                    domain.lower = tighter_lower(domain.lower, Bound::Exclusive(value.clone()));
                }
                ComparisonOperator::GreaterOrEqual => {
                    domain.lower = tighter_lower(domain.lower, Bound::Inclusive(value.clone()));
                }
                ComparisonOperator::Equal => {
                    domain.lower = tighter_lower(domain.lower, Bound::Inclusive(value.clone()));
                    domain.upper = tighter_upper(domain.upper, Bound::Inclusive(value.clone()));
                }
                ComparisonOperator::NotEqual => {
                    if !domain.excluded.iter().any(|p| p == value) {
                        domain.excluded.push(value.clone());
                    }
                }
            }
        }
        domain
    }

    /// Whether `value` is admitted by this domain.
    #[must_use]
    pub fn admits(&self, value: &ExactNumber) -> bool {
        let lower_ok = match &self.lower {
            Bound::Unbounded => true,
            Bound::Inclusive(a) => value >= a,
            Bound::Exclusive(a) => value > a,
        };
        let upper_ok = match &self.upper {
            Bound::Unbounded => true,
            Bound::Inclusive(b) => value <= b,
            Bound::Exclusive(b) => value < b,
        };
        lower_ok && upper_ok && !self.excluded.iter().any(|p| p == value)
    }

    /// Whether the interval (ignoring exclusions) admits no value.
    fn interval_is_empty(&self) -> bool {
        match (&self.lower, &self.upper) {
            (Bound::Inclusive(a), Bound::Inclusive(b)) => a > b,
            // A half-open or open interval is empty iff its ends meet or cross.
            (Bound::Inclusive(a), Bound::Exclusive(b))
            | (Bound::Exclusive(a), Bound::Inclusive(b))
            | (Bound::Exclusive(a), Bound::Exclusive(b)) => a >= b,
            // An unbounded side leaves an infinite interval — never empty as an
            // interval. Arms written out rather than wildcarded.
            (Bound::Inclusive(_), Bound::Unbounded)
            | (Bound::Exclusive(_), Bound::Unbounded)
            | (Bound::Unbounded, Bound::Inclusive(_))
            | (Bound::Unbounded, Bound::Exclusive(_))
            | (Bound::Unbounded, Bound::Unbounded) => false,
        }
    }

    /// Contradiction, per reference — §4.10: the domain is empty.
    ///
    /// Empty in exactly two ways: the interval is degenerate (lower above upper,
    /// or a strict bound meeting its opposite), or the interval is a single
    /// closed point that its own `!=` excludes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        if self.interval_is_empty() {
            return true;
        }
        // The only remaining empty case is a single closed point `[a, a]`
        // excluded by its own `!=`. Every other shape is a non-degenerate,
        // dense interval that finitely many exclusions cannot empty. The arms
        // are written out rather than wildcarded so a change to `Bound` is
        // surfaced here.
        match (&self.lower, &self.upper) {
            (Bound::Inclusive(a), Bound::Inclusive(b)) if a == b => {
                self.excluded.iter().any(|p| p == a)
            }
            (Bound::Inclusive(_), Bound::Inclusive(_))
            | (Bound::Inclusive(_), Bound::Exclusive(_))
            | (Bound::Inclusive(_), Bound::Unbounded)
            | (Bound::Exclusive(_), Bound::Inclusive(_))
            | (Bound::Exclusive(_), Bound::Exclusive(_))
            | (Bound::Exclusive(_), Bound::Unbounded)
            | (Bound::Unbounded, Bound::Inclusive(_))
            | (Bound::Unbounded, Bound::Exclusive(_))
            | (Bound::Unbounded, Bound::Unbounded) => false,
        }
    }

    /// Intersection — §4.10. The result is the tighter interval on each side and
    /// the union of the excluded sets.
    #[must_use]
    pub fn intersect(&self, other: &Self) -> Self {
        let lower = tighter_lower(self.lower.clone(), other.lower.clone());
        let upper = tighter_upper(self.upper.clone(), other.upper.clone());
        let mut excluded = self.excluded.clone();
        for p in &other.excluded {
            if !excluded.iter().any(|q| q == p) {
                excluded.push(p.clone());
            }
        }
        Self {
            lower,
            upper,
            excluded,
        }
    }

    /// Overlap, per reference — §4.10: the intersection is non-empty.
    #[must_use]
    pub fn overlaps(&self, other: &Self) -> bool {
        !self.intersect(other).is_empty()
    }

    /// Containment — §4.10: whether `inner ⊆ self` (every value `inner` admits,
    /// `self` admits).
    ///
    /// Subsumption uses this: `D2` is subsumed by `D1` iff `D1.contains(&D2)`.
    #[must_use]
    pub fn contains(&self, inner: &Self) -> bool {
        if inner.is_empty() {
            return true;
        }
        lower_contains(&self.lower, &inner.lower)
            && upper_contains(&self.upper, &inner.upper)
            // Any point `self` excludes must not be admitted by `inner`.
            && self.excluded.iter().all(|p| !inner.admits(p))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ComparisonOperator::{Equal, Greater, GreaterOrEqual, Less, LessOrEqual, NotEqual};

    fn n(value: i64) -> ExactNumber {
        ExactNumber::from_integer(value)
    }

    fn domain(constraints: &[(ComparisonOperator, i64)]) -> NumericDomain {
        let owned: Vec<(ComparisonOperator, ExactNumber)> =
            constraints.iter().map(|(op, v)| (*op, n(*v))).collect();
        NumericDomain::from_constraints(&owned)
    }

    // -- interval / admission -----------------------------------------------

    #[test]
    fn unconstrained_admits_everything() {
        let d = NumericDomain::unconstrained();
        assert!(d.admits(&n(-1000)));
        assert!(d.admits(&n(0)));
        assert!(d.admits(&n(1000)));
        assert!(!d.is_empty());
    }

    #[test]
    fn bounded_interval_admits_within_and_rejects_outside() {
        // [1, 5]
        let d = domain(&[(GreaterOrEqual, 1), (LessOrEqual, 5)]);
        assert!(d.admits(&n(1)));
        assert!(d.admits(&n(3)));
        assert!(d.admits(&n(5)));
        assert!(!d.admits(&n(0)));
        assert!(!d.admits(&n(6)));
    }

    #[test]
    fn one_sided_bounds() {
        let lower = domain(&[(Greater, 2)]); // (2, +inf)
        assert!(!lower.admits(&n(2)));
        assert!(lower.admits(&n(3)));

        let upper = domain(&[(LessOrEqual, 2)]); // (-inf, 2]
        assert!(upper.admits(&n(2)));
        assert!(!upper.admits(&n(3)));
    }

    #[test]
    fn exact_boundary_inclusion_and_exclusion() {
        let closed = domain(&[(GreaterOrEqual, 4)]);
        assert!(closed.admits(&n(4)));
        let open = domain(&[(Greater, 4)]);
        assert!(!open.admits(&n(4)));
    }

    #[test]
    fn excluded_points() {
        // [0, 10] minus {5}
        let d = domain(&[(GreaterOrEqual, 0), (LessOrEqual, 10), (NotEqual, 5)]);
        assert!(d.admits(&n(4)));
        assert!(!d.admits(&n(5)));
        assert!(d.admits(&n(6)));
        assert!(!d.is_empty());

        // multiple exclusions
        let d2 = domain(&[(NotEqual, 1), (NotEqual, 2)]);
        assert!(!d2.admits(&n(1)));
        assert!(!d2.admits(&n(2)));
        assert!(d2.admits(&n(3)));
    }

    // -- contradiction / emptiness ------------------------------------------

    #[test]
    fn interval_with_lower_above_upper_is_empty() {
        assert!(domain(&[(GreaterOrEqual, 5), (LessOrEqual, 4)]).is_empty());
    }

    #[test]
    fn a_strict_bound_meeting_its_opposite_is_empty() {
        // (3, 3] and [3, 3) and (3,3) are all empty; [3,3] is a point.
        assert!(domain(&[(Greater, 3), (LessOrEqual, 3)]).is_empty());
        assert!(domain(&[(GreaterOrEqual, 3), (Less, 3)]).is_empty());
        assert!(domain(&[(Greater, 3), (Less, 3)]).is_empty());
        assert!(!domain(&[(GreaterOrEqual, 3), (LessOrEqual, 3)]).is_empty());
    }

    #[test]
    fn two_equalities_to_different_points_are_empty() {
        assert!(domain(&[(Equal, 1), (Equal, 2)]).is_empty());
        assert!(!domain(&[(Equal, 7), (Equal, 7)]).is_empty());
    }

    #[test]
    fn a_point_excluded_by_its_own_inequality_is_empty() {
        // [5, 5] minus {5}
        assert!(domain(&[(Equal, 5), (NotEqual, 5)]).is_empty());
    }

    #[test]
    fn exclusion_within_a_wide_interval_never_empties_it() {
        // [0, 10] minus {5} is dense — non-empty.
        assert!(!domain(&[(GreaterOrEqual, 0), (LessOrEqual, 10), (NotEqual, 5)]).is_empty());
    }

    // -- intersection / overlap ---------------------------------------------

    #[test]
    fn intersection_is_the_tighter_interval() {
        let a = domain(&[(GreaterOrEqual, 0), (LessOrEqual, 6)]); // [0,6]
        let b = domain(&[(GreaterOrEqual, 4), (LessOrEqual, 10)]); // [4,10]
        let i = a.intersect(&b); // [4,6]
        assert!(i.admits(&n(4)));
        assert!(i.admits(&n(6)));
        assert!(!i.admits(&n(3)));
        assert!(!i.admits(&n(7)));
    }

    #[test]
    fn overlap_matches_non_empty_intersection() {
        let a = domain(&[(GreaterOrEqual, 0), (LessOrEqual, 5)]);
        let b = domain(&[(GreaterOrEqual, 5), (LessOrEqual, 10)]); // touch at 5
        let c = domain(&[(Greater, 5), (LessOrEqual, 10)]); // (5,10], disjoint from a
        assert!(a.overlaps(&b));
        assert!(!a.overlaps(&c));
    }

    #[test]
    fn intersection_is_commutative_and_idempotent() {
        let a = domain(&[(GreaterOrEqual, 1), (LessOrEqual, 8), (NotEqual, 4)]);
        let b = domain(&[(GreaterOrEqual, 3), (LessOrEqual, 6)]);
        // Commutativity: same admitted set both ways (mutual containment).
        let ab = a.intersect(&b);
        let ba = b.intersect(&a);
        assert!(ab.contains(&ba) && ba.contains(&ab));
        // Idempotence.
        let aa = a.intersect(&a);
        assert!(aa.contains(&a) && a.contains(&aa));
    }

    #[test]
    fn contradiction_absorbs_intersection() {
        let empty = domain(&[(GreaterOrEqual, 5), (LessOrEqual, 4)]);
        let any = domain(&[(GreaterOrEqual, 0)]);
        assert!(empty.intersect(&any).is_empty());
        assert!(any.intersect(&empty).is_empty());
    }

    // -- containment / subsumption ------------------------------------------

    #[test]
    fn containment_of_intervals() {
        let wide = domain(&[(GreaterOrEqual, 0), (LessOrEqual, 10)]);
        let narrow = domain(&[(GreaterOrEqual, 2), (LessOrEqual, 8)]);
        assert!(wide.contains(&narrow));
        assert!(!narrow.contains(&wide));
    }

    #[test]
    fn containment_is_reflexive_and_transitive() {
        let a = domain(&[(GreaterOrEqual, 0), (LessOrEqual, 12)]);
        let b = domain(&[(GreaterOrEqual, 2), (LessOrEqual, 10)]);
        let c = domain(&[(GreaterOrEqual, 4), (LessOrEqual, 8)]);
        assert!(a.contains(&a)); // reflexive
        assert!(a.contains(&b) && b.contains(&c));
        assert!(a.contains(&c)); // transitive
    }

    #[test]
    fn containment_respects_the_outer_exclusions() {
        // outer = [0,10] minus {5}; inner = [4,6] (admits 5) — NOT contained,
        // because 5 ∈ inner but 5 ∉ outer.
        let outer = domain(&[(GreaterOrEqual, 0), (LessOrEqual, 10), (NotEqual, 5)]);
        let inner = domain(&[(GreaterOrEqual, 4), (LessOrEqual, 6)]);
        assert!(!outer.contains(&inner));
        // But inner minus {5} IS contained.
        let inner_ok = domain(&[(GreaterOrEqual, 4), (LessOrEqual, 6), (NotEqual, 5)]);
        assert!(outer.contains(&inner_ok));
    }

    #[test]
    fn an_empty_domain_is_contained_in_anything() {
        let empty = domain(&[(GreaterOrEqual, 5), (LessOrEqual, 4)]);
        let any = domain(&[(GreaterOrEqual, 0), (LessOrEqual, 1)]);
        assert!(any.contains(&empty));
    }

    #[test]
    fn containment_implies_overlap_for_non_contradictory_domains() {
        let outer = domain(&[(GreaterOrEqual, 0), (LessOrEqual, 10)]);
        let inner = domain(&[(GreaterOrEqual, 3), (LessOrEqual, 6)]);
        assert!(outer.contains(&inner));
        assert!(!inner.is_empty());
        assert!(outer.overlaps(&inner));
    }

    #[test]
    fn exclusions_are_order_independent() {
        // [0,10] minus {3,7} built in either order behaves identically.
        let a = domain(&[
            (GreaterOrEqual, 0),
            (LessOrEqual, 10),
            (NotEqual, 3),
            (NotEqual, 7),
        ]);
        let b = domain(&[
            (NotEqual, 7),
            (GreaterOrEqual, 0),
            (NotEqual, 3),
            (LessOrEqual, 10),
        ]);
        assert!(a.contains(&b) && b.contains(&a));
        assert!(!a.admits(&n(3)) && !a.admits(&n(7)));
        assert!(!b.admits(&n(3)) && !b.admits(&n(7)));
    }
}
