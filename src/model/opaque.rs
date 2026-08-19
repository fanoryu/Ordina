//! The opaque value domains — Slice Contract §3.3, §3.3.1 (and §C.1 of
//! Revision 4).
//!
//! Lineage identities, dimension symbols, `ENUM` categorical values, version
//! tokens and precedence ordinals are the five **semantic** opaque domains: the
//! contract selects no encoding, format or representation for them, and the
//! checker must never take authoritative meaning from whatever representation it
//! happens to use.
//!
//! Revision 4 §C.1 adds a sixth opaque domain — **construct-occurrence
//! identities** ([`OccurrenceId`]). It is opaque and supplied-ordered exactly
//! like the semantic domains, but it is an **addressing / conformance** identity,
//! not a semantic one: it identifies one concrete construct occurrence in the
//! canonical input and carries no product semantics (§C.1, §C.3). It is distinct
//! from lineage identity and never substitutes for it.
//!
//! Each domain is its own newtype, so a dimension symbol cannot be compared
//! against a lineage identity by accident — that is a compile error, not a bug
//! to be found later.

use core::cmp::Ordering;

/// A caller-supplied opaque handle for a value in one of the opaque domains.
///
/// Slice Contract §3.3.1 requires that **both** the values of an opaque domain
/// **and** a deterministic total order over them arrive as part of the
/// already-canonical input (§2). The checker never generates, infers or chooses
/// an order.
///
/// The caller therefore assigns handles consistent with the domain's supplied
/// total order. Comparing handles compares *the order the caller supplied*, not
/// an encoding this implementation picked. Nothing else about the integer
/// carries meaning.
///
/// # The integer is the representation of a supplied order, not an order this
/// implementation chose
///
/// Stated explicitly, because the two are easy to confuse and only one of them
/// is permitted: §3.3.1 requires the total order to **arrive with the input**,
/// and ADR-0005 §7.5 prohibits deriving authoritative order from incidental
/// representation. This type does neither of the prohibited things. The integer
/// handle is the **caller's encoding of the contract-supplied order**, chosen by
/// whoever produced the already-canonical input (§2), and the comparisons below
/// read that encoding back. No ordering is invented, inferred, defaulted, or
/// selected here.
///
/// Two consequences follow, and both are the contract's, not this module's.
/// Callers supplying handles inconsistent with the supplied total order get
/// comparisons that disagree with the contract — the input was not canonical, and
/// no implementation could detect that, because the order is not otherwise
/// available to compare against. And two conforming checkers agree because they
/// receive the same order, never because they agree on a representation.
///
/// A different encoding of the same supplied order would be an equally
/// conforming implementation choice. Nothing in the checker may read the integer
/// for any purpose other than the comparisons below.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OpaqueValue(u64);

impl OpaqueValue {
    /// Wraps a caller-supplied handle.
    #[must_use]
    pub const fn new(handle: u64) -> Self {
        Self(handle)
    }

    /// The supplied order position. Used only by the hand-written `Ord`
    /// implementations below, never to infer anything else.
    #[must_use]
    const fn supplied_order(self) -> u64 {
        self.0
    }

    /// The caller-supplied handle, for canonical revision-identity encoding
    /// only — ADR-0008 §2 tag `0x07` encodes an opaque-domain reference as the
    /// 8-byte big-endian `u64` of the referenced value's underlying handle.
    ///
    /// Deliberately `pub(crate)`: this is not a public accessor, and nothing
    /// outside this crate may read the handle. Nothing inside the crate may
    /// read it for any purpose other than that encoding — the handle still
    /// carries no meaning beyond the supplied order (see the type docs).
    #[must_use]
    pub(crate) const fn identity_handle(self) -> u64 {
        self.0
    }
}

/// Stable rule/node lineage identity — ADR-0003 §9, Slice Contract §3.3.
///
/// # Cross-domain comparison is a compile error
///
/// ADR-0005 §7.5 requires the five opaque domains to be distinct newtypes "so
/// values from unrelated domains cannot be compared or substituted
/// accidentally". Each domain implements `PartialEq` and `Ord` against **itself
/// only**, so mixing two domains has no implementation to select. Equality:
///
/// ```compile_fail
/// use ordina::model::opaque::{DimensionSymbol, LineageId, OpaqueValue};
///
/// let lineage = LineageId::new(OpaqueValue::new(1));
/// let dimension = DimensionSymbol::new(OpaqueValue::new(1));
///
/// let _ = lineage == dimension;
/// ```
///
/// Ordering, which matters because §6.4 sorts by these domains:
///
/// ```compile_fail
/// use ordina::model::opaque::{LineageId, OpaqueValue, PrecedenceOrdinal};
///
/// let lineage = LineageId::new(OpaqueValue::new(1));
/// let precedence = PrecedenceOrdinal::new(OpaqueValue::new(2));
///
/// let _ = lineage < precedence;
/// ```
///
/// And substitution, which is the same guarantee seen from the other side — a
/// dimension symbol cannot be passed where a lineage identity is required:
///
/// ```compile_fail
/// use ordina::model::opaque::{DimensionSymbol, LineageId, OpaqueValue};
///
/// fn takes_lineage(_: LineageId) {}
///
/// takes_lineage(DimensionSymbol::new(OpaqueValue::new(1)));
/// ```
///
/// Comparison **within** one domain is exactly what is permitted, and it reads
/// the contract-supplied order:
///
/// ```
/// use ordina::model::opaque::{LineageId, OpaqueValue};
///
/// assert!(LineageId::new(OpaqueValue::new(1)) < LineageId::new(OpaqueValue::new(2)));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LineageId(OpaqueValue);

/// A dimension symbol, compared by exact equality only — Slice Contract §4.6.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DimensionSymbol(OpaqueValue);

/// A categorical value of an `ENUM` evidence domain — Slice Contract §3.5.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EnumValue(OpaqueValue);

/// A semantic-model / checker-semantics version token — Slice Contract §3.4.1,
/// §6.2. Equality only: the contract supplies **no** order over version tokens,
/// so this type deliberately has no `Ord` implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VersionToken(OpaqueValue);

/// A Direction-rule precedence ordinal — Slice Contract §3.7.
///
/// Unlike the domains above, this order is **semantically operative**: it
/// decides which Direction rule resolves (§4.9). It is still supplied with the
/// input and never invented here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrecedenceOrdinal(OpaqueValue);

/// A canonical construct-occurrence identity — Slice Contract Revision 4 §C.1.
///
/// Identifies **one concrete construct occurrence** in one canonical Strategy
/// input. It is mandatory and non-optional (every occurrence has exactly one,
/// even a malformed one), opaque, caller-supplied, and unique within one Strategy
/// input. It is a **separate domain from [`LineageId`]** — the two are never
/// compared or substituted (that is a compile error) — and it exists even when a
/// construct's lineage is absent, is duplicated, or its content is identical to
/// another occurrence's.
///
/// Its order is the §C.1 caller-supplied total order — "ordering only, carries no
/// semantics" — read back from the supplied handle exactly as the semantic
/// domains are, never derived from representation, collection position, or
/// traversal order (ADR-0005 §7.5).
///
/// Occurrence identity and lineage identity are distinct domains and cannot be
/// compared:
///
/// ```compile_fail
/// use ordina::model::opaque::{LineageId, OccurrenceId, OpaqueValue};
///
/// let occurrence = OccurrenceId::new(OpaqueValue::new(1));
/// let lineage = LineageId::new(OpaqueValue::new(1));
///
/// let _ = occurrence == lineage;
/// ```
///
/// Comparison **within** the occurrence domain reads the contract-supplied order:
///
/// ```
/// use ordina::model::opaque::{OccurrenceId, OpaqueValue};
///
/// assert!(OccurrenceId::new(OpaqueValue::new(1)) < OccurrenceId::new(OpaqueValue::new(2)));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OccurrenceId(OpaqueValue);

impl LineageId {
    #[must_use]
    pub const fn new(value: OpaqueValue) -> Self {
        Self(value)
    }
}

// The five **semantic** opaque domains expose their underlying handle to the
// crate — and only to the crate — so ADR-0008 §2's tag `0x07` frame can be
// built. `OccurrenceId` deliberately exposes none: ADR-0007 places it outside
// ADR-0003 §6's taxonomy entirely, so it is never walked and never encoded.
// The absence of an accessor there is the structural enforcement of that ruling.

impl LineageId {
    #[must_use]
    pub(crate) const fn identity_handle(self) -> u64 {
        self.0.identity_handle()
    }
}

impl DimensionSymbol {
    #[must_use]
    pub(crate) const fn identity_handle(self) -> u64 {
        self.0.identity_handle()
    }
}

impl EnumValue {
    #[must_use]
    pub(crate) const fn identity_handle(self) -> u64 {
        self.0.identity_handle()
    }
}

impl VersionToken {
    #[must_use]
    pub(crate) const fn identity_handle(self) -> u64 {
        self.0.identity_handle()
    }
}

impl PrecedenceOrdinal {
    #[must_use]
    pub(crate) const fn identity_handle(self) -> u64 {
        self.0.identity_handle()
    }
}

impl OccurrenceId {
    #[must_use]
    pub const fn new(value: OpaqueValue) -> Self {
        Self(value)
    }
}

impl DimensionSymbol {
    #[must_use]
    pub const fn new(value: OpaqueValue) -> Self {
        Self(value)
    }
}

impl EnumValue {
    #[must_use]
    pub const fn new(value: OpaqueValue) -> Self {
        Self(value)
    }
}

impl VersionToken {
    #[must_use]
    pub const fn new(value: OpaqueValue) -> Self {
        Self(value)
    }
}

impl PrecedenceOrdinal {
    #[must_use]
    pub const fn new(value: OpaqueValue) -> Self {
        Self(value)
    }
}

// ---------------------------------------------------------------------------
// Ordering.
//
// ADR-0005 §7.5 prohibits `#[derive(Ord)]` on an opaque-domain newtype: a
// derived implementation takes its order from whatever the newtype happens to
// wrap, which is incidental representation, not the order the contract
// supplies. Each implementation below is written by hand and delegates
// explicitly to the caller-supplied order, which is what §7.5 permits and
// expects.
//
// `VersionToken` is absent by design — the contract supplies no order for it.
// ---------------------------------------------------------------------------

impl Ord for LineageId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.supplied_order().cmp(&other.0.supplied_order())
    }
}

impl PartialOrd for LineageId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OccurrenceId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.supplied_order().cmp(&other.0.supplied_order())
    }
}

impl PartialOrd for OccurrenceId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DimensionSymbol {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.supplied_order().cmp(&other.0.supplied_order())
    }
}

impl PartialOrd for DimensionSymbol {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EnumValue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.supplied_order().cmp(&other.0.supplied_order())
    }
}

impl PartialOrd for EnumValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PrecedenceOrdinal {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.supplied_order().cmp(&other.0.supplied_order())
    }
}

impl PartialOrd for PrecedenceOrdinal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lineage(handle: u64) -> LineageId {
        LineageId::new(OpaqueValue::new(handle))
    }

    #[test]
    fn ordering_follows_the_supplied_order() {
        assert!(lineage(1) < lineage(2));
        assert!(lineage(9) > lineage(3));
        assert_eq!(lineage(4), lineage(4));
    }

    #[test]
    fn ordering_is_a_total_order() {
        let mut sorted = vec![lineage(3), lineage(1), lineage(2)];
        sorted.sort();

        assert_eq!(sorted, vec![lineage(1), lineage(2), lineage(3)]);
    }

    #[test]
    fn domains_are_distinct_types() {
        // Cross-domain comparison is a *compile* error, so it cannot be
        // asserted at runtime — the `compile_fail` doctests on `LineageId`
        // cover it instead. Equality within each domain is what remains
        // testable here, and each domain keeps its own identity.
        let dimension = DimensionSymbol::new(OpaqueValue::new(1));
        let enum_value = EnumValue::new(OpaqueValue::new(1));

        assert_eq!(dimension, DimensionSymbol::new(OpaqueValue::new(1)));
        assert_eq!(enum_value, EnumValue::new(OpaqueValue::new(1)));
    }

    #[test]
    fn version_tokens_compare_by_equality_only() {
        let a = VersionToken::new(OpaqueValue::new(1));

        assert_eq!(a, VersionToken::new(OpaqueValue::new(1)));
        assert_ne!(a, VersionToken::new(OpaqueValue::new(2)));
    }

    fn occurrence(handle: u64) -> OccurrenceId {
        OccurrenceId::new(OpaqueValue::new(handle))
    }

    #[test]
    fn occurrence_identity_follows_the_supplied_order() {
        // Revision 4 §C.1: the occurrence order is the caller-supplied total
        // order, read back from the handle exactly as the semantic domains are.
        assert!(occurrence(1) < occurrence(2));
        assert!(occurrence(9) > occurrence(3));
        assert_eq!(occurrence(4), occurrence(4));

        let mut sorted = vec![occurrence(3), occurrence(1), occurrence(2)];
        sorted.sort();
        assert_eq!(sorted, vec![occurrence(1), occurrence(2), occurrence(3)]);
    }

    #[test]
    fn occurrence_and_lineage_are_distinct_domains() {
        // Cross-domain comparison is a *compile* error (see the `compile_fail`
        // doctest on `OccurrenceId`). What is testable is that the same handle in
        // the two domains is two independent, non-substitutable values: an
        // occurrence identity is never a lineage identity.
        let occ = occurrence(1);
        let lin = LineageId::new(OpaqueValue::new(1));

        assert_eq!(occ, OccurrenceId::new(OpaqueValue::new(1)));
        assert_eq!(lin, LineageId::new(OpaqueValue::new(1)));
        // Distinct occurrence identities never coincide.
        assert_ne!(occurrence(1), occurrence(2));
    }
}
