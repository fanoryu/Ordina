//! Result contract — Slice Contract §6.
//!
//! Phase 4 implements the **result and defect data model** and the canonical
//! ordering substrate: the §6.1 binary verdict, the §6.2 result contents, the
//! §6.3 deterministic defect identity (with the revision-qualification
//! boundary), and the §6.4 total canonical ordering — including the 26 declared
//! categories ([`category`]), the 6 discriminator kinds and the literal order
//! ([`discriminator`]), and defect identity with its ordering ([`identity`]).
//!
//! These are **model and ordering only**. No mandatory check runs, and **no
//! defect is produced** here: this module defines the shapes a later phase's
//! checks will populate. Ordering never derives from hash iteration or
//! incidental representation (ADR-0005 §7.5, §7.6): every key is either an
//! explicitly declared ordinal or a contract-supplied opaque order, and no
//! `HashMap`/`HashSet` appears on any authoritative path.

pub mod category;
pub mod discriminator;
pub mod identity;
pub mod verdict;

pub use category::{CheckNumber, DefectCategory};
pub use discriminator::{
    AtomKey, ChildPosition, Discriminator, NodePath, PropertyPosition, literal_order,
};
pub use identity::{
    DefectCore, DefectIdentity, MixedQualificationBoundary, RevisionIdentity, canonical_sort,
};
pub use verdict::Verdict;

use crate::model::opaque::OccurrenceId;

/// The Slice-1 checker-semantics version — Slice Contract §6.2 (ADR-0004 §8).
///
/// A single opaque token, **`WF-1`**, declared by the contract and identical for
/// every conforming implementation; no implementation selects it, and every
/// authoritative result binds it. It is compared by exact equality only. A
/// single-variant closed category: widening the semantics it denotes is a `WF-1`
/// change (a new variant), never a silent reuse (§6.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckerSemanticsVersion {
    /// The authoritative Slice-1 checker semantics of this contract.
    Wf1,
}

/// One §6.2 not-evaluated record: a check whose evaluation was suppressed for a
/// unit of work, naming that unit's identity and the blocking defect's
/// identity.
///
/// Unit identity (First Runnable Slice Contract Revision 5, §A, PBR-0017) is
/// occurrence-based, never lineage-based — mirroring the same reason Revision
/// 4 migrated `DefectCore`'s own anchor (§C.4): a construct's lineage may be
/// absent or duplicated, but its occurrence identity is mandatory and unique.
/// `construct` names a single-unit suppressed evaluation's own occurrence, or
/// (for a pairwise-gated unit — checks 9 and 11's subsumption part) the
/// pair member whose occurrence identity sorts first under the supplied
/// occurrence order (Revision 4 §C.7's existing canonical pair orientation,
/// reused verbatim, unmodified). `counterpart` is `None` for a single-unit
/// evaluation and `Some(other member)` for a pairwise one.
///
/// Phase 4 defines the shape; Phase 6 prerequisite gating populates it. No
/// suppression is computed here.
#[derive(Debug, Clone)]
pub struct NotEvaluatedRecord {
    check: CheckNumber,
    construct: OccurrenceId,
    counterpart: Option<OccurrenceId>,
    blocking: DefectIdentity,
}

impl NotEvaluatedRecord {
    /// A single-unit suppressed evaluation — `counterpart` is `None`.
    #[must_use]
    pub const fn new(
        check: CheckNumber,
        construct: OccurrenceId,
        blocking: DefectIdentity,
    ) -> Self {
        Self {
            check,
            construct,
            counterpart: None,
            blocking,
        }
    }

    /// A pairwise suppressed evaluation (checks 9, 11-subsumption) over the
    /// unordered pair `{a, b}`. Canonicalizes by construction under the
    /// existing supplied occurrence order (§C.7, unmodified): the member that
    /// sorts first becomes `construct`, the other becomes `counterpart`, so
    /// `pairwise(check, a, b, blocking)` and `pairwise(check, b, a, blocking)`
    /// always produce an identical record.
    #[must_use]
    pub fn pairwise(
        check: CheckNumber,
        a: OccurrenceId,
        b: OccurrenceId,
        blocking: DefectIdentity,
    ) -> Self {
        let (construct, counterpart) = if a <= b { (a, b) } else { (b, a) };
        Self {
            check,
            construct,
            counterpart: Some(counterpart),
            blocking,
        }
    }

    #[must_use]
    pub const fn check(&self) -> CheckNumber {
        self.check
    }

    #[must_use]
    pub const fn construct(&self) -> OccurrenceId {
        self.construct
    }

    #[must_use]
    pub const fn counterpart(&self) -> Option<OccurrenceId> {
        self.counterpart
    }

    #[must_use]
    pub const fn blocking(&self) -> &DefectIdentity {
        &self.blocking
    }
}

/// A Well-Formedness result — Slice Contract §6.2.
///
/// Carries the verdict, the defect set (empty **iff** `WellFormed`, §6.2), the
/// not-evaluated set, and the bound checker-semantics version. The two
/// constructors enforce the §6.2 empty-iff-well-formed invariant by
/// construction. Phase 4 builds results; it does not run checks to fill them.
#[derive(Debug, Clone)]
pub struct WellFormednessResult {
    verdict: Verdict,
    defects: Vec<DefectIdentity>,
    not_evaluated: Vec<NotEvaluatedRecord>,
    semantics_version: CheckerSemanticsVersion,
}

impl WellFormednessResult {
    /// A `WellFormed` result — an empty defect set (§6.2), with whatever
    /// not-evaluated records the run produced.
    #[must_use]
    pub fn well_formed(not_evaluated: Vec<NotEvaluatedRecord>) -> Self {
        Self {
            verdict: Verdict::WellFormed,
            defects: Vec::new(),
            not_evaluated,
            semantics_version: CheckerSemanticsVersion::Wf1,
        }
    }

    /// An `Unusable` result carrying its **non-empty** defect set (§6.2).
    ///
    /// §6.2 binds "the defect set — empty **iff** `WELL-FORMED`", so an
    /// `Unusable` result with an empty defect set is a state the contract makes
    /// impossible. That impossibility is **structural** here: the first defect is
    /// a required positional argument, so no safe call can produce the invalid
    /// state — there is nothing to reject at runtime because there is nothing to
    /// construct. The remaining defects follow; the caller supplies them already
    /// in §6.4 canonical order (see [`canonical_sort`]).
    ///
    /// An empty `Unusable` therefore does not compile — there is no defect to
    /// pass as the first argument:
    ///
    /// ```compile_fail
    /// use ordina::result::WellFormednessResult;
    ///
    /// // No first defect exists to supply, so this cannot be written.
    /// let _ = WellFormednessResult::unusable(Vec::new(), Vec::new());
    /// ```
    #[must_use]
    pub fn unusable(
        first_defect: DefectIdentity,
        remaining_defects: Vec<DefectIdentity>,
        not_evaluated: Vec<NotEvaluatedRecord>,
    ) -> Self {
        let mut defects = Vec::with_capacity(remaining_defects.len() + 1);
        defects.push(first_defect);
        defects.extend(remaining_defects);
        Self {
            verdict: Verdict::Unusable,
            defects,
            not_evaluated,
            semantics_version: CheckerSemanticsVersion::Wf1,
        }
    }

    #[must_use]
    pub const fn verdict(&self) -> Verdict {
        self.verdict
    }

    #[must_use]
    pub fn defects(&self) -> &[DefectIdentity] {
        &self.defects
    }

    #[must_use]
    pub fn not_evaluated(&self) -> &[NotEvaluatedRecord] {
        &self.not_evaluated
    }

    #[must_use]
    pub const fn semantics_version(&self) -> CheckerSemanticsVersion {
        self.semantics_version
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::opaque::OpaqueValue;

    fn occurrence(handle: u64) -> crate::model::opaque::OccurrenceId {
        crate::model::opaque::OccurrenceId::new(OpaqueValue::new(handle))
    }

    fn a_defect() -> DefectIdentity {
        // Defect identity anchors on an occurrence (Revision 4 §C.4); the
        // not-evaluated record's own unit identity is occurrence-based too
        // (Revision 5 §A, PBR-0017).
        DefectIdentity::unqualified(DefectCore::new(
            occurrence(1),
            DefectCategory::TypeMismatch,
            Discriminator::None,
        ))
    }

    #[test]
    fn well_formed_has_no_defects_and_binds_wf1() {
        let result = WellFormednessResult::well_formed(Vec::new());
        assert_eq!(result.verdict(), Verdict::WellFormed);
        assert!(result.defects().is_empty());
        assert_eq!(result.semantics_version(), CheckerSemanticsVersion::Wf1);
    }

    #[test]
    fn unusable_carries_its_defect_set_and_binds_wf1() {
        let result = WellFormednessResult::unusable(a_defect(), Vec::new(), Vec::new());
        assert_eq!(result.verdict(), Verdict::Unusable);
        assert_eq!(result.defects().len(), 1);
        assert_eq!(result.semantics_version(), CheckerSemanticsVersion::Wf1);
    }

    #[test]
    fn unusable_prepends_the_required_first_defect_to_the_rest() {
        let result = WellFormednessResult::unusable(a_defect(), vec![a_defect()], Vec::new());
        assert_eq!(result.verdict(), Verdict::Unusable);
        assert_eq!(result.defects().len(), 2);
    }

    #[test]
    fn the_defect_set_is_empty_iff_well_formed() {
        // WellFormed ⟹ empty (structural: no way to add a defect here).
        let wf = WellFormednessResult::well_formed(Vec::new());
        assert_eq!(wf.verdict(), Verdict::WellFormed);
        assert!(wf.defects().is_empty());

        // Unusable ⟹ non-empty (structural: the first defect is required, so
        // an empty Unusable cannot be constructed — see the compile_fail doctest
        // on `unusable`).
        let unusable = WellFormednessResult::unusable(a_defect(), Vec::new(), Vec::new());
        assert_eq!(unusable.verdict(), Verdict::Unusable);
        assert!(!unusable.defects().is_empty());
    }

    #[test]
    fn not_evaluated_record_names_check_construct_and_blocker() {
        let record = NotEvaluatedRecord::new(CheckNumber::C11, occurrence(3), a_defect());
        assert_eq!(record.check(), CheckNumber::C11);
        assert_eq!(record.construct(), occurrence(3));
        assert_eq!(record.counterpart(), None);
        assert!(matches!(record.blocking(), DefectIdentity::Unqualified(_)));
    }

    #[test]
    fn not_evaluated_record_pairwise_canonicalizes_regardless_of_argument_order() {
        let forward =
            NotEvaluatedRecord::pairwise(CheckNumber::C9, occurrence(5), occurrence(9), a_defect());
        let backward =
            NotEvaluatedRecord::pairwise(CheckNumber::C9, occurrence(9), occurrence(5), a_defect());
        assert_eq!(forward.construct(), occurrence(5));
        assert_eq!(forward.counterpart(), Some(occurrence(9)));
        assert_eq!(backward.construct(), forward.construct());
        assert_eq!(backward.counterpart(), forward.counterpart());
    }

    #[test]
    fn not_evaluated_record_pairwise_distinguishes_distinct_pairs_sharing_a_member() {
        let ab =
            NotEvaluatedRecord::pairwise(CheckNumber::C9, occurrence(1), occurrence(2), a_defect());
        let ac =
            NotEvaluatedRecord::pairwise(CheckNumber::C9, occurrence(1), occurrence(3), a_defect());
        assert_eq!(ab.construct(), ac.construct());
        assert_ne!(ab.counterpart(), ac.counterpart());
    }

    #[test]
    fn there_is_exactly_one_checker_semantics_version() {
        // WF-1 equals itself; the type admits no other value to confuse it with.
        assert_eq!(CheckerSemanticsVersion::Wf1, CheckerSemanticsVersion::Wf1);
    }
}
