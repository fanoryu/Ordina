//! Deterministic defect identity and canonical ordering — Slice Contract §6.3,
//! §6.4.
//!
//! A defect's authoritative identity (§6.3) is a deterministic function of the
//! offending construct's **occurrence identity** (Revision 4 §C.4), the defect
//! category, a discriminator, and — **where the caller supplies one** — the
//! applicable Strategy revision identity. The canonical order (§6.4) sorts a
//! defect set the same way for every conforming checker.
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

use crate::model::opaque::{OccurrenceId, OpaqueValue};
use crate::result::category::DefectCategory;
use crate::result::discriminator::Discriminator;

/// The §6.4-ordered core of a defect identity: the part present in **both**
/// qualification states.
///
/// Its total order is the §6.4 key sequence: (1) ADR-0004 check number
/// ascending, (2) offending **occurrence identity** under the supplied
/// occurrence order (Revision 4 §C.9), (3) defect-category ordinal ascending,
/// (4) discriminator under §6.4.2. Because occurrence identities are unique
/// within one canonical Strategy input and the discriminator distinguishes
/// same-category defects sharing one anchor, the order is **total** — and it
/// stays total even when a construct's lineage is absent or duplicated, which is
/// exactly why Revision 4 replaced the lineage anchor with the occurrence anchor.
#[derive(Debug, Clone)]
pub struct DefectCore {
    offending: OccurrenceId,
    category: DefectCategory,
    discriminator: Discriminator,
}

impl DefectCore {
    #[must_use]
    pub const fn new(
        offending: OccurrenceId,
        category: DefectCategory,
        discriminator: Discriminator,
    ) -> Self {
        Self {
            offending,
            category,
            discriminator,
        }
    }

    /// The offending construct's occurrence identity — §6.3 component 1 as
    /// amended by Revision 4 §C.4 (an occurrence anchor, not a lineage anchor).
    #[must_use]
    pub const fn offending(&self) -> OccurrenceId {
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

    fn occurrence(handle: u64) -> OccurrenceId {
        OccurrenceId::new(OpaqueValue::new(handle))
    }

    fn revision(handle: u64) -> RevisionIdentity {
        RevisionIdentity::new(OpaqueValue::new(handle))
    }

    fn core(offending: u64, category: DefectCategory) -> DefectCore {
        DefectCore::new(occurrence(offending), category, Discriminator::None)
    }

    // -- §6.4 ordering ------------------------------------------------------

    #[test]
    fn order_is_check_then_occurrence_then_category_then_discriminator() {
        // check number dominates: category 26 (check 16) sorts after category 1
        // (check 1) regardless of the occurrence anchor.
        let early_check = core(9, DefectCategory::MissingRequiredProperty); // check 1
        let late_check = core(1, DefectCategory::UnknownConstruct); // check 16
        assert_eq!(early_check.cmp(&late_check), Ordering::Less);

        // same check, occurrence anchor (key 2) dominates category ordinal
        let a = core(1, DefectCategory::UnresolvedReference); // check 2, ord 3
        let b = core(2, DefectCategory::DuplicateLineageIdentity); // check 2, ord 2
        assert_eq!(a.cmp(&b), Ordering::Less); // occurrence 1 < 2 wins

        // same check and occurrence, category ordinal decides
        let c = core(1, DefectCategory::DuplicateLineageIdentity); // ord 2
        let d = core(1, DefectCategory::TypeMismatch); // ord 4
        assert_eq!(c.cmp(&d), Ordering::Less);
    }

    #[test]
    fn discriminator_breaks_same_construct_same_category_ties() {
        let low = DefectCore::new(
            occurrence(1),
            DefectCategory::MissingRequiredProperty,
            Discriminator::Property(PropertyPosition::new(1)),
        );
        let high = DefectCore::new(
            occurrence(1),
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
        // Two sets with disjoint occurrence handles but the SAME relative supplied
        // order sort into the same sequence — the order depends on the supplied
        // opaque occurrence order, not the incidental handle values.
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
        // Both sort ascending by supplied occurrence order; positions correspond.
        assert_eq!(low, vec![occurrence(1), occurrence(2), occurrence(3)]);
        assert_eq!(high, vec![occurrence(10), occurrence(20), occurrence(30)]);
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

    // -- Revision 4 occurrence-anchor semantics (§C.4–§C.9) -----------------
    //
    // These exercise the result model's identity/order behaviour under the
    // occurrence anchor. No check runs and no defect is produced; the model is
    // shown to *represent* the accepted anchor rules.

    use crate::model::guard::ComparisonOperator;
    use crate::model::literal::Literal;
    use crate::model::opaque::{DimensionSymbol, LineageId};
    use crate::numeric::ExactNumber;
    use crate::result::discriminator::AtomKey;

    fn lineage(handle: u64) -> LineageId {
        LineageId::new(OpaqueValue::new(handle))
    }

    #[test]
    fn two_identical_defects_on_distinct_occurrences_stay_distinct() {
        // §C.9: same category and discriminator, different occurrence anchor ⇒
        // distinct identities, separated at key 2. This is the collapse the old
        // lineage anchor could not prevent for absent/duplicate lineage.
        let a = core(1, DefectCategory::MissingRequiredProperty);
        let b = core(2, DefectCategory::MissingRequiredProperty);
        assert_ne!(a.cmp(&b), Ordering::Equal);
        assert_eq!(a.offending(), occurrence(1));
        assert_eq!(b.offending(), occurrence(2));
    }

    #[test]
    fn absent_lineage_does_not_prevent_defect_anchoring() {
        // §C.4/§C.9: a DefectCore is built purely from an occurrence identity —
        // no lineage is consulted — so a defect on a construct whose lineage is
        // absent is fully anchored and orderable.
        let defect = core(5, DefectCategory::MissingOperativeProvenance); // check 15
        assert_eq!(defect.offending(), occurrence(5));
    }

    #[test]
    fn multiple_duplicate_lineage_classes_share_the_envelope_anchor_yet_stay_distinct() {
        // §C.5: DUPLICATE_LINEAGE_IDENTITY is envelope-anchored (same occurrence
        // for both classes) and separated by the LINEAGE discriminator.
        let envelope = occurrence(1);
        let l4 = DefectCore::new(
            envelope,
            DefectCategory::DuplicateLineageIdentity,
            Discriminator::Lineage(lineage(4)),
        );
        let l7 = DefectCore::new(
            envelope,
            DefectCategory::DuplicateLineageIdentity,
            Discriminator::Lineage(lineage(7)),
        );
        assert_eq!(l4.offending(), l7.offending()); // same envelope anchor
        assert_ne!(l4.cmp(&l7), Ordering::Equal); // distinct via LINEAGE
        assert_eq!(l4.cmp(&l7), Ordering::Less); // supplied lineage order 4 < 7
    }

    #[test]
    fn canonical_order_uses_the_occurrence_anchor_at_key_two() {
        // §C.9: within one check and after category, key 2 is the occurrence
        // anchor under the supplied occurrence order.
        let mut set = vec![
            DefectIdentity::unqualified(core(3, DefectCategory::TypeMismatch)),
            DefectIdentity::unqualified(core(1, DefectCategory::TypeMismatch)),
            DefectIdentity::unqualified(core(2, DefectCategory::TypeMismatch)),
        ];
        canonical_sort(&mut set).expect("homogeneous");
        let anchors: Vec<_> = set.iter().map(|d| d.core().offending()).collect();
        assert_eq!(anchors, vec![occurrence(1), occurrence(2), occurrence(3)]);
    }

    fn atom_discriminator(reference: u64) -> Discriminator {
        Discriminator::Atom(AtomKey::new(
            lineage(reference),
            ComparisonOperator::Less,
            Literal::Numeric {
                value: ExactNumber::from_integer(0),
                dimension: DimensionSymbol::new(OpaqueValue::new(1)),
            },
        ))
    }

    #[test]
    fn atom_discriminator_reference_stays_lineage_based() {
        // §C.8: the ATOM discriminator's referenced payload is a semantic lineage
        // reference and is NOT migrated to occurrence identity. Two atoms on
        // different referenced lineages stay distinct through that lineage.
        let a = DefectCore::new(
            occurrence(1),
            DefectCategory::TypeMismatch,
            atom_discriminator(1),
        );
        let b = DefectCore::new(
            occurrence(1),
            DefectCategory::TypeMismatch,
            atom_discriminator(2),
        );
        assert_ne!(a.cmp(&b), Ordering::Equal);
    }

    #[test]
    fn pairwise_symmetric_representation_is_deterministic_under_occurrence_order() {
        // §C.7 check 9: the canonical anchor is the lower occurrence and the other
        // member is COUNTERPART. Building the pair "either discovery order" yields
        // the same identity because the anchor is chosen by supplied occurrence
        // order, not by how the pair was reached.
        let build = |anchor: u64, counterpart: u64| {
            DefectCore::new(
                occurrence(anchor.min(counterpart)),
                DefectCategory::DirectionPrecedenceTie,
                Discriminator::Counterpart(occurrence(anchor.max(counterpart))),
            )
        };
        let one_way = build(10, 20);
        let other_way = build(20, 10);
        assert_eq!(one_way.cmp(&other_way), Ordering::Equal);
        assert_eq!(one_way.offending(), occurrence(10));
    }

    #[test]
    fn pairwise_asymmetric_keeps_subsumed_occurrence_as_anchor() {
        // §C.7 check 11: the subsumed (offending) rule is ALWAYS the anchor and the
        // subsuming rule is COUNTERPART — even when the subsuming occurrence sorts
        // first. Occurrence order does not select the offender here.
        let subsumed = occurrence(20);
        let subsuming = occurrence(10); // sorts first, but is NOT the offender
        let defect = DefectCore::new(
            subsumed,
            DefectCategory::UnreachableSubsumedRule,
            Discriminator::Counterpart(subsuming),
        );
        assert_eq!(defect.offending(), occurrence(20)); // subsumed, regardless of order
        match defect.discriminator() {
            Discriminator::Counterpart(c) => assert_eq!(*c, occurrence(10)),
            _ => panic!("expected a COUNTERPART discriminator"),
        }
    }
}
