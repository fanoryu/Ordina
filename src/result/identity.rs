//! Deterministic defect identity and canonical ordering — Slice Contract §6.3,
//! §6.4.
//!
//! A defect's authoritative identity (§6.3) is a deterministic function of the
//! offending construct's lineage identity, the defect category, a discriminator,
//! and — **where the caller supplies one** — the applicable Strategy revision
//! identity. The canonical order (§6.4) sorts a defect set the same way for
//! every conforming checker.
//!
//! # The revision-qualification boundary is structural
//!
//! §6.3 (per ARR-0007 D-3) fixes that the revision-**un**qualified identity of a
//! defect and its revision-**qualified** identity are **not interchangeable** and
//! **MUST NOT** be compared, equated, ordered as equivalent, or deduplicated
//! across that boundary — equality there is *undefined*, not `false`. This module
//! makes that structural rather than documentary: [`DefectIdentity`] implements
//! **no** `PartialEq`/`Eq`/`Ord`, so `==` and `sort` cannot silently cross the
//! boundary. Comparison is available only through [`DefectIdentity::try_cmp`] /
//! [`DefectIdentity::try_eq`], which return [`None`] across the boundary (and
//! across differing revision identities), and through [`canonical_sort`], which
//! refuses a mixed set.

use core::cmp::Ordering;

use crate::model::opaque::{LineageId, OpaqueValue};
use crate::result::category::DefectCategory;
use crate::result::discriminator::Discriminator;

/// The §6.4-ordered core of a defect identity: the part present in **both**
/// qualification states.
///
/// Its total order is the §6.4 key sequence: (1) ADR-0004 check number
/// ascending, (2) offending lineage identity under the §3.3.1 supplied order,
/// (3) defect-category ordinal ascending, (4) discriminator under §6.4.2.
/// Because lineage identities are unique within a Strategy and the discriminator
/// distinguishes same-category defects on one construct, the order is **total**.
#[derive(Debug, Clone)]
pub struct DefectCore {
    offending: LineageId,
    category: DefectCategory,
    discriminator: Discriminator,
}

impl DefectCore {
    #[must_use]
    pub const fn new(
        offending: LineageId,
        category: DefectCategory,
        discriminator: Discriminator,
    ) -> Self {
        Self {
            offending,
            category,
            discriminator,
        }
    }

    #[must_use]
    pub const fn offending(&self) -> LineageId {
        self.offending
    }

    #[must_use]
    pub const fn category(&self) -> DefectCategory {
        self.category
    }

    #[must_use]
    pub const fn discriminator(&self) -> &Discriminator {
        &self.discriminator
    }
}

impl Ord for DefectCore {
    fn cmp(&self, other: &Self) -> Ordering {
        // §6.4, in order; the first difference decides.
        self.category
            .check()
            .cmp(&other.category.check())
            .then_with(|| self.offending.cmp(&other.offending))
            .then_with(|| self.category.cmp(&other.category))
            .then_with(|| self.discriminator.cmp(&other.discriminator))
    }
}

impl PartialOrd for DefectCore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for DefectCore {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for DefectCore {}

/// An applicable Strategy revision identity — §6.3 component 4.
///
/// Opaque and **caller-supplied**; Slice 1 computes none itself (ADR-0003 §5
/// fixes `Well-Formed → canonicalize → identity`, so at first-check time no
/// revision identity yet exists). Compared by **exact equality only** — §6.4
/// never orders by it — so, like a version token, it has no `Ord`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RevisionIdentity(OpaqueValue);

impl RevisionIdentity {
    #[must_use]
    pub const fn new(handle: OpaqueValue) -> Self {
        Self(handle)
    }
}

/// A deterministic defect identity — §6.3.
///
/// Exactly two qualification states. The `Unqualified` triple is produced when
/// no revision identity is applicable; the `Qualified` form carries the
/// caller-supplied revision identity. **No `PartialEq`/`Eq`/`Ord` is
/// implemented** — see the module docs: comparison across the qualification
/// boundary (or across differing revision identities) is *undefined* and is
/// exposed only as [`None`] via [`DefectIdentity::try_cmp`] / [`try_eq`].
///
/// [`try_eq`]: DefectIdentity::try_eq
#[derive(Debug, Clone)]
pub enum DefectIdentity {
    /// No applicable revision identity — the unqualified triple (§6.3 (1)(2)(3)).
    Unqualified(DefectCore),
    /// Revision-qualified with the caller-supplied identity (§6.3 (1)(2)(3)(4)).
    Qualified {
        revision: RevisionIdentity,
        core: DefectCore,
    },
}

impl DefectIdentity {
    #[must_use]
    pub const fn unqualified(core: DefectCore) -> Self {
        Self::Unqualified(core)
    }

    #[must_use]
    pub const fn qualified(revision: RevisionIdentity, core: DefectCore) -> Self {
        Self::Qualified { revision, core }
    }

    /// The §6.4-ordered core, present in both states.
    #[must_use]
    pub const fn core(&self) -> &DefectCore {
        // No catch-all `_` arm.
        match self {
            Self::Unqualified(core) => core,
            Self::Qualified { core, .. } => core,
        }
    }

    /// Whether the two identities lie in the **same** comparable state: both
    /// unqualified, or both qualified under the **same** revision identity
    /// (§6.3). Only then are `try_cmp` / `try_eq` defined.
    #[must_use]
    pub fn is_comparable_with(&self, other: &Self) -> bool {
        matches!((self, other), (Self::Unqualified(_), Self::Unqualified(_)))
            || matches!(
                (self, other),
                (
                    Self::Qualified { revision: a, .. },
                    Self::Qualified { revision: b, .. },
                ) if a == b
            )
    }

    /// The §6.4 order **within one qualification state**. [`None`] across the
    /// revision-qualification boundary or across differing revision identities —
    /// where §6.3 leaves the comparison undefined.
    #[must_use]
    pub fn try_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_comparable_with(other) {
            Some(self.core().cmp(other.core()))
        } else {
            None
        }
    }

    /// Identity equality **within one qualification state**. [`None`] across the
    /// boundary — where §6.3 makes equality undefined rather than `false`.
    #[must_use]
    pub fn try_eq(&self, other: &Self) -> Option<bool> {
        self.try_cmp(other)
            .map(|ordering| ordering == Ordering::Equal)
    }
}

/// Raised when a canonical sort is attempted over a defect set that mixes
/// qualification states — the §6.3 boundary forbids ordering across it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MixedQualificationBoundary;

/// Sort a defect set into the §6.4 canonical order.
///
/// The whole set must share one qualification state (and, if qualified, one
/// revision identity); a mixed set is a §6.3 boundary violation and is rejected
/// with [`MixedQualificationBoundary`] rather than silently ordered. On success
/// the order is total — no two distinct defects compare equal (§6.4).
///
/// # Errors
///
/// Returns [`MixedQualificationBoundary`] if the set mixes qualification states.
pub fn canonical_sort(defects: &mut [DefectIdentity]) -> Result<(), MixedQualificationBoundary> {
    if let Some((head, tail)) = defects.split_first() {
        // Every element comparable with the head ⇒ all share one state, so the
        // pairwise comparisons in the sort below are all defined.
        if tail.iter().any(|other| !head.is_comparable_with(other)) {
            return Err(MixedQualificationBoundary);
        }
    }
    defects.sort_by(|a, b| {
        a.try_cmp(b)
            .expect("homogeneity verified above, so every pair is comparable")
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::opaque::OpaqueValue;
    use crate::result::discriminator::{Discriminator, PropertyPosition};

    fn lineage(handle: u64) -> LineageId {
        LineageId::new(OpaqueValue::new(handle))
    }

    fn revision(handle: u64) -> RevisionIdentity {
        RevisionIdentity::new(OpaqueValue::new(handle))
    }

    fn core(offending: u64, category: DefectCategory) -> DefectCore {
        DefectCore::new(lineage(offending), category, Discriminator::None)
    }

    // -- §6.4 ordering ------------------------------------------------------

    #[test]
    fn order_is_check_then_lineage_then_category_then_discriminator() {
        // check number dominates: category 26 (check 16) sorts after category 1
        // (check 1) regardless of lineage.
        let early_check = core(9, DefectCategory::MissingRequiredProperty); // check 1
        let late_check = core(1, DefectCategory::UnknownConstruct); // check 16
        assert_eq!(early_check.cmp(&late_check), Ordering::Less);

        // same check, lineage dominates category ordinal
        let a = core(1, DefectCategory::UnresolvedReference); // check 2, ord 3
        let b = core(2, DefectCategory::DuplicateLineageIdentity); // check 2, ord 2
        assert_eq!(a.cmp(&b), Ordering::Less); // lineage 1 < 2 wins

        // same check and lineage, category ordinal decides
        let c = core(1, DefectCategory::DuplicateLineageIdentity); // ord 2
        let d = core(1, DefectCategory::TypeMismatch); // ord 4
        assert_eq!(c.cmp(&d), Ordering::Less);
    }

    #[test]
    fn discriminator_breaks_same_construct_same_category_ties() {
        let low = DefectCore::new(
            lineage(1),
            DefectCategory::MissingRequiredProperty,
            Discriminator::Property(PropertyPosition::new(1)),
        );
        let high = DefectCore::new(
            lineage(1),
            DefectCategory::MissingRequiredProperty,
            Discriminator::Property(PropertyPosition::new(2)),
        );
        assert_eq!(low.cmp(&high), Ordering::Less);
    }

    #[test]
    fn distinct_defects_never_compare_equal() {
        let a = core(1, DefectCategory::UnresolvedReference);
        let b = core(1, DefectCategory::TypeMismatch);
        assert_ne!(a.cmp(&b), Ordering::Equal);
        // identical cores are equal
        let c = core(1, DefectCategory::UnresolvedReference);
        assert_eq!(a.cmp(&c), Ordering::Equal);
    }

    // -- §6.3 qualification boundary ---------------------------------------

    #[test]
    fn same_state_comparisons_are_defined() {
        let a = DefectIdentity::unqualified(core(1, DefectCategory::TypeMismatch));
        let b = DefectIdentity::unqualified(core(2, DefectCategory::TypeMismatch));
        assert_eq!(a.try_cmp(&b), Some(Ordering::Less));
        assert_eq!(a.try_eq(&b), Some(false));

        let r = revision(7);
        let qa = DefectIdentity::qualified(r, core(1, DefectCategory::TypeMismatch));
        let qb = DefectIdentity::qualified(r, core(1, DefectCategory::TypeMismatch));
        assert_eq!(qa.try_cmp(&qb), Some(Ordering::Equal));
        assert_eq!(qa.try_eq(&qb), Some(true));
    }

    #[test]
    fn cross_boundary_comparison_is_undefined_not_false() {
        let unq = DefectIdentity::unqualified(core(1, DefectCategory::TypeMismatch));
        let qual = DefectIdentity::qualified(revision(1), core(1, DefectCategory::TypeMismatch));
        assert_eq!(unq.try_cmp(&qual), None);
        assert_eq!(unq.try_eq(&qual), None);
    }

    #[test]
    fn differing_revision_identities_are_incomparable() {
        let qa = DefectIdentity::qualified(revision(1), core(1, DefectCategory::TypeMismatch));
        let qb = DefectIdentity::qualified(revision(2), core(1, DefectCategory::TypeMismatch));
        assert_eq!(qa.try_cmp(&qb), None);
        assert_eq!(qa.try_eq(&qb), None);
    }

    // -- canonical_sort -----------------------------------------------------

    #[test]
    fn canonical_sort_orders_a_homogeneous_set() {
        let mut defects = vec![
            DefectIdentity::unqualified(core(2, DefectCategory::UnknownConstruct)),
            DefectIdentity::unqualified(core(1, DefectCategory::MissingRequiredProperty)),
            DefectIdentity::unqualified(core(1, DefectCategory::TypeMismatch)),
        ];
        canonical_sort(&mut defects).expect("homogeneous");
        // check 1 (MissingRequiredProperty) first, then check 2 (TypeMismatch),
        // then check 16 (UnknownConstruct).
        let categories: Vec<_> = defects.iter().map(|d| d.core().category()).collect();
        assert_eq!(
            categories,
            vec![
                DefectCategory::MissingRequiredProperty,
                DefectCategory::TypeMismatch,
                DefectCategory::UnknownConstruct,
            ]
        );
    }

    #[test]
    fn canonical_sort_rejects_a_mixed_set() {
        let mut defects = vec![
            DefectIdentity::unqualified(core(1, DefectCategory::TypeMismatch)),
            DefectIdentity::qualified(revision(1), core(1, DefectCategory::TypeMismatch)),
        ];
        assert_eq!(
            canonical_sort(&mut defects),
            Err(MixedQualificationBoundary)
        );
    }

    #[test]
    fn canonical_sort_rejects_mixed_revision_identities() {
        let mut defects = vec![
            DefectIdentity::qualified(revision(1), core(1, DefectCategory::TypeMismatch)),
            DefectIdentity::qualified(revision(2), core(1, DefectCategory::TypeMismatch)),
        ];
        assert_eq!(
            canonical_sort(&mut defects),
            Err(MixedQualificationBoundary)
        );
    }

    #[test]
    fn canonical_order_is_invariant_under_order_preserving_relabelling() {
        // Two sets with disjoint lineage handles but the SAME relative supplied
        // order sort into the same category sequence — the order depends on the
        // supplied opaque order, not the incidental handle values.
        let build = |handles: [u64; 3]| {
            let mut set = vec![
                DefectIdentity::unqualified(core(handles[2], DefectCategory::TypeMismatch)),
                DefectIdentity::unqualified(core(handles[0], DefectCategory::TypeMismatch)),
                DefectIdentity::unqualified(core(handles[1], DefectCategory::TypeMismatch)),
            ];
            canonical_sort(&mut set).expect("homogeneous");
            set.iter().map(|d| d.core().offending()).collect::<Vec<_>>()
        };
        // Relative order 1<2<3 mirrored by 10<20<30.
        let low = build([1, 2, 3]);
        let high = build([10, 20, 30]);
        // Both sort ascending by supplied order; positions correspond.
        assert_eq!(low, vec![lineage(1), lineage(2), lineage(3)]);
        assert_eq!(high, vec![lineage(10), lineage(20), lineage(30)]);
    }

    #[test]
    fn empty_and_singleton_sets_sort_trivially() {
        let mut empty: Vec<DefectIdentity> = Vec::new();
        assert_eq!(canonical_sort(&mut empty), Ok(()));
        let mut one = vec![DefectIdentity::unqualified(core(
            1,
            DefectCategory::TypeMismatch,
        ))];
        assert_eq!(canonical_sort(&mut one), Ok(()));
    }
}
