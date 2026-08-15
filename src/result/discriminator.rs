//! Defect discriminators and literal ordering — Slice Contract §6.4.2, §6.4.3.
//!
//! Where one construct can produce several defects of one category, §6.3 adds a
//! **discriminator**, and §6.4.2 gives the discriminators a total order in six
//! **kinds**. This module models the six kinds and their comparison, plus the
//! §6.4.3 literal order the `ATOM` kind depends on.
//!
//! **All ordering is the contract-supplied order, never incidental Rust
//! representation (ADR-0005 §7.5).** Opaque identities delegate to the
//! model layer's hand-written `Ord` (which reads the caller-supplied order);
//! operators, roles and child positions use ordinals declared explicitly here;
//! and the numeric literal key delegates to the exact-value order of
//! [`ExactNumber`] with the supplied dimension order as tie-break.

use core::cmp::Ordering;

use crate::model::derivation::CandidateRole;
use crate::model::guard::ComparisonOperator;
use crate::model::literal::Literal;
use crate::model::opaque::{LineageId, OccurrenceId};

/// The §6.4.3 type ordinal — `NUMERIC` (1) < `BOOLEAN` (2) < `ENUM` (3).
const fn literal_type_ordinal(literal: &Literal) -> u8 {
    // No catch-all `_` arm: a new literal type must be placed here deliberately.
    match literal {
        Literal::Numeric { .. } => 1,
        Literal::Boolean(_) => 2,
        Literal::Enum(_) => 3,
    }
}

/// The total order over typed literals — §6.4.3.
///
/// This is a **defect-ordering** key, not a §4.3/§4.4 semantic comparison: it is
/// deliberately total across dimensions and types (a cross-dimension numeric
/// pair orders by the supplied dimension symbol, never a type defect here),
/// exactly as §6.4.3 requires and as [`ExactNumber`]'s order documents. It reads
/// no incidental representation.
#[must_use]
pub fn literal_order(left: &Literal, right: &Literal) -> Ordering {
    match (left, right) {
        (
            Literal::Numeric {
                value: lv,
                dimension: ld,
            },
            Literal::Numeric {
                value: rv,
                dimension: rd,
            },
            // Exact value ascending (§4.2); dimension symbol tie-break under the
            // supplied §3.3.1 order.
        ) => lv.cmp(rv).then_with(|| ld.cmp(rd)),
        // `false` (1) < `true` (2): `bool`'s own order already ranks them so.
        (Literal::Boolean(lb), Literal::Boolean(rb)) => lb.cmp(rb),
        // Categorical value under the supplied §3.3.1 order.
        (Literal::Enum(le), Literal::Enum(re)) => le.cmp(re),
        // Cross-type pairs — unreachable in practice (§6.4.3), but the type
        // ordinal makes the order total unconditionally. Arms written out so no
        // catch-all can absorb a same-type case.
        (Literal::Numeric { .. }, Literal::Boolean(_) | Literal::Enum(_))
        | (Literal::Boolean(_), Literal::Numeric { .. } | Literal::Enum(_))
        | (Literal::Enum(_), Literal::Numeric { .. } | Literal::Boolean(_)) => {
            literal_type_ordinal(left).cmp(&literal_type_ordinal(right))
        }
    }
}

/// The declared candidate-role order — `ENTRY` (1) < `STOP_LOSS` (2) <
/// `TAKE_PROFIT` (3), §6.4.2 kind 3.
const fn role_ordinal(role: CandidateRole) -> u8 {
    // No catch-all `_` arm.
    match role {
        CandidateRole::Entry => 1,
        CandidateRole::StopLoss => 2,
        CandidateRole::TakeProfit => 3,
    }
}

/// The declared operator order — `<` (1) `<=` (2) `>` (3) `>=` (4) `=` (5) `!=`
/// (6), §6.4.2 kind 4.
const fn operator_ordinal(operator: ComparisonOperator) -> u8 {
    // No catch-all `_` arm.
    match operator {
        ComparisonOperator::Less => 1,
        ComparisonOperator::LessOrEqual => 2,
        ComparisonOperator::Greater => 3,
        ComparisonOperator::GreaterOrEqual => 4,
        ComparisonOperator::Equal => 5,
        ComparisonOperator::NotEqual => 6,
    }
}

/// The declared position of a property in the §3 property table for a
/// construct's kind — §6.4.2 kind 2, top to bottom ascending.
///
/// The position is a declared, contract-fixed ordinal supplied by the caller
/// (the check that raises the defect); this type only carries and orders it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PropertyPosition(u32);

impl PropertyPosition {
    #[must_use]
    pub const fn new(position: u32) -> Self {
        Self(position)
    }
}

/// One step of a derivation-node path — §6.4.2 kind 5. `LeftOperand` (1) <
/// `RightOperand` (2). A closed category; the order is declared, not derived.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChildPosition {
    LeftOperand,
    RightOperand,
}

impl ChildPosition {
    const fn ordinal(self) -> u8 {
        // No catch-all `_` arm.
        match self {
            Self::LeftOperand => 1,
            Self::RightOperand => 2,
        }
    }
}

impl Ord for ChildPosition {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ordinal().cmp(&other.ordinal())
    }
}

impl PartialOrd for ChildPosition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// The path from a derivation's root to the offending node — §6.4.2 kind 5.
///
/// The finite sequence of child positions taken to reach the node. Compared
/// element-wise ascending; where one is a proper prefix of the other, the
/// **shorter sorts first** — exactly the lexicographic order of the position
/// sequence, whose element order is [`ChildPosition`]'s declared order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodePath(Vec<ChildPosition>);

impl NodePath {
    #[must_use]
    pub fn new(steps: Vec<ChildPosition>) -> Self {
        Self(steps)
    }

    /// The root node itself — the empty path, which sorts before any longer path.
    #[must_use]
    pub fn root() -> Self {
        Self(Vec::new())
    }
}

impl Ord for NodePath {
    fn cmp(&self, other: &Self) -> Ordering {
        // `Vec`'s lexicographic order over `ChildPosition`'s declared order is
        // precisely "element-wise ascending, shorter proper prefix first".
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for NodePath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// The offending guard atom, by its content — §6.4.2 kind 4.
///
/// The triple *(referenced lineage identity, operator, literal)*, compared left
/// to right. This is **content-based, not positional**: it does not depend on
/// atom order, consistent with §4.5's rule that atom order carries no meaning.
/// `Literal` has no `Ord` of its own (B-4); the literal key is [`literal_order`].
#[derive(Debug, Clone)]
pub struct AtomKey {
    referenced: LineageId,
    operator: ComparisonOperator,
    literal: Literal,
}

impl AtomKey {
    #[must_use]
    pub const fn new(
        referenced: LineageId,
        operator: ComparisonOperator,
        literal: Literal,
    ) -> Self {
        Self {
            referenced,
            operator,
            literal,
        }
    }

    fn compare(&self, other: &Self) -> Ordering {
        self.referenced
            .cmp(&other.referenced)
            .then_with(|| operator_ordinal(self.operator).cmp(&operator_ordinal(other.operator)))
            .then_with(|| literal_order(&self.literal, &other.literal))
    }
}

/// A defect discriminator — one of the seven §6.4.2 kinds (Revision 4 §C.8).
///
/// Compared by **kind ordinal** first, then by that kind's rule. A closed
/// authoritative category; `Literal` inside the `Atom` kind is why this type
/// carries a hand-written order and equality rather than a derive.
///
/// Revision 4 §C.8 makes exactly two changes to the Revision-3 six kinds: the
/// `Counterpart` payload migrates from lineage identity to **occurrence
/// identity** (kind 6, unchanged ordinal), and a new **`Lineage`** kind 7 is
/// added for `DUPLICATE_LINEAGE_IDENTITY`. Kinds 1–5 are unchanged.
#[derive(Debug, Clone)]
pub enum Discriminator {
    /// Kind 1 — no discriminator; the preceding keys already distinguish.
    None,
    /// Kind 2 — a property's declared position (checks 1, 4, 6).
    Property(PropertyPosition),
    /// Kind 3 — a candidate role (check 13B, incl. `DUPLICATE_CANDIDATE_DERIVATION`).
    Role(CandidateRole),
    /// Kind 4 — a guard atom, by content (checks 2, 12). Its referenced-lineage
    /// payload stays lineage identity (a semantic reference, not an anchor).
    Atom(AtomKey),
    /// Kind 5 — a derivation-node path (checks 2, 13A, 14A).
    NodePath(NodePath),
    /// Kind 6 — the counterpart construct's **occurrence identity** (checks 9, 11).
    ///
    /// Revision 4 §C.7/§C.8 migrated this payload from lineage to occurrence
    /// identity so a pairwise defect stays representable when a rule's lineage is
    /// absent or duplicated. Ordered by the supplied occurrence order.
    Counterpart(OccurrenceId),
    /// Kind 7 — the **duplicated lineage identity** of a
    /// `DUPLICATE_LINEAGE_IDENTITY` defect (check 2), Revision 4 §C.5/§C.8.
    ///
    /// That defect is envelope-anchored (§6.3), so this discriminator separates
    /// multiple duplicated-lineage classes within one Strategy. Its payload is a
    /// semantic lineage identity, ordered by the supplied lineage order.
    Lineage(LineageId),
}

impl Discriminator {
    /// The §6.4.2 kind ordinal, 1..=7 (Revision 4 §C.8). Declared, not derived
    /// from layout.
    #[must_use]
    pub const fn kind_ordinal(&self) -> u8 {
        // No catch-all `_` arm: a new kind must be placed here deliberately.
        match self {
            Self::None => 1,
            Self::Property(_) => 2,
            Self::Role(_) => 3,
            Self::Atom(_) => 4,
            Self::NodePath(_) => 5,
            Self::Counterpart(_) => 6,
            Self::Lineage(_) => 7,
        }
    }
}

impl Ord for Discriminator {
    fn cmp(&self, other: &Self) -> Ordering {
        let by_kind = self.kind_ordinal().cmp(&other.kind_ordinal());
        if by_kind != Ordering::Equal {
            return by_kind;
        }
        // Equal kind ordinals mean the same variant; compare by that kind's rule.
        match (self, other) {
            (Self::None, Self::None) => Ordering::Equal,
            (Self::Property(a), Self::Property(b)) => a.cmp(b),
            (Self::Role(a), Self::Role(b)) => role_ordinal(*a).cmp(&role_ordinal(*b)),
            (Self::Atom(a), Self::Atom(b)) => a.compare(b),
            (Self::NodePath(a), Self::NodePath(b)) => a.cmp(b),
            // Counterpart orders by the supplied occurrence order (§C.8).
            (Self::Counterpart(a), Self::Counterpart(b)) => a.cmp(b),
            // Lineage orders by the supplied lineage order (§C.8).
            (Self::Lineage(a), Self::Lineage(b)) => a.cmp(b),
            _ => unreachable!("discriminator kinds already separated by kind ordinal"),
        }
    }
}

impl PartialOrd for Discriminator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Discriminator {
    fn eq(&self, other: &Self) -> bool {
        // Equality is agreement under the total order — `Literal` has no `Eq`,
        // so the order is the single source of truth for both.
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Discriminator {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::opaque::OpaqueValue;
    use crate::numeric::ExactNumber;

    fn lineage(handle: u64) -> LineageId {
        LineageId::new(OpaqueValue::new(handle))
    }

    fn occ(handle: u64) -> OccurrenceId {
        OccurrenceId::new(OpaqueValue::new(handle))
    }

    fn num(value: i64, dimension: u64) -> Literal {
        Literal::Numeric {
            value: ExactNumber::from_integer(value),
            dimension: crate::model::opaque::DimensionSymbol::new(OpaqueValue::new(dimension)),
        }
    }

    fn enum_val(handle: u64) -> crate::model::opaque::EnumValue {
        crate::model::opaque::EnumValue::new(OpaqueValue::new(handle))
    }

    // -- literal order (§6.4.3) --------------------------------------------

    #[test]
    fn numeric_orders_by_value_then_dimension() {
        assert_eq!(literal_order(&num(1, 9), &num(2, 9)), Ordering::Less);
        // equal value, dimension tie-break by supplied order
        assert_eq!(literal_order(&num(5, 1), &num(5, 2)), Ordering::Less);
        assert_eq!(literal_order(&num(5, 2), &num(5, 1)), Ordering::Greater);
        assert_eq!(literal_order(&num(5, 1), &num(5, 1)), Ordering::Equal);
    }

    #[test]
    fn boolean_orders_false_before_true() {
        assert_eq!(
            literal_order(&Literal::Boolean(false), &Literal::Boolean(true)),
            Ordering::Less
        );
    }

    #[test]
    fn enum_orders_by_supplied_value_order() {
        let a = Literal::Enum(enum_val(1));
        let b = Literal::Enum(enum_val(2));
        assert_eq!(literal_order(&a, &b), Ordering::Less);
        assert_eq!(literal_order(&b, &a), Ordering::Greater);
    }

    #[test]
    fn cross_type_orders_numeric_boolean_enum() {
        let n = num(0, 1);
        let b = Literal::Boolean(true);
        let e = Literal::Enum(enum_val(1));
        assert_eq!(literal_order(&n, &b), Ordering::Less);
        assert_eq!(literal_order(&b, &e), Ordering::Less);
        assert_eq!(literal_order(&n, &e), Ordering::Less);
        assert_eq!(literal_order(&e, &n), Ordering::Greater);
    }

    // -- discriminator kinds (§6.4.2) --------------------------------------

    #[test]
    fn kinds_sort_by_kind_ordinal_first() {
        let discriminators = [
            Discriminator::None,
            Discriminator::Property(PropertyPosition::new(0)),
            Discriminator::Role(CandidateRole::Entry),
            Discriminator::Atom(AtomKey::new(
                lineage(1),
                ComparisonOperator::Less,
                num(0, 1),
            )),
            Discriminator::NodePath(NodePath::root()),
            Discriminator::Counterpart(occ(1)),
            Discriminator::Lineage(lineage(1)),
        ];
        for (i, a) in discriminators.iter().enumerate() {
            for (j, b) in discriminators.iter().enumerate() {
                assert_eq!(a.cmp(b), i.cmp(&j), "kind {i} vs {j}");
            }
        }
    }

    #[test]
    fn discriminator_kind_ordinals_are_one_through_seven() {
        // Revision 4 §C.8: kinds 1–6 keep their ordinals; LINEAGE is the new 7.
        assert_eq!(Discriminator::None.kind_ordinal(), 1);
        assert_eq!(
            Discriminator::Property(PropertyPosition::new(0)).kind_ordinal(),
            2
        );
        assert_eq!(Discriminator::Role(CandidateRole::Entry).kind_ordinal(), 3);
        assert_eq!(
            Discriminator::Atom(AtomKey::new(
                lineage(1),
                ComparisonOperator::Less,
                num(0, 1)
            ))
            .kind_ordinal(),
            4
        );
        assert_eq!(Discriminator::NodePath(NodePath::root()).kind_ordinal(), 5);
        assert_eq!(Discriminator::Counterpart(occ(1)).kind_ordinal(), 6);
        assert_eq!(Discriminator::Lineage(lineage(1)).kind_ordinal(), 7);
    }

    #[test]
    fn role_orders_entry_stop_take() {
        let entry = Discriminator::Role(CandidateRole::Entry);
        let stop = Discriminator::Role(CandidateRole::StopLoss);
        let take = Discriminator::Role(CandidateRole::TakeProfit);
        assert!(entry < stop && stop < take);
    }

    #[test]
    fn atom_orders_by_reference_then_operator_then_literal() {
        let base = Discriminator::Atom(AtomKey::new(
            lineage(1),
            ComparisonOperator::Less,
            num(0, 1),
        ));
        let higher_ref = Discriminator::Atom(AtomKey::new(
            lineage(2),
            ComparisonOperator::Less,
            num(0, 1),
        ));
        let higher_op = Discriminator::Atom(AtomKey::new(
            lineage(1),
            ComparisonOperator::Greater,
            num(0, 1),
        ));
        let higher_lit = Discriminator::Atom(AtomKey::new(
            lineage(1),
            ComparisonOperator::Less,
            num(5, 1),
        ));
        // Referenced identity is the most significant key, then operator, then
        // literal.
        assert!(base < higher_ref);
        assert!(base < higher_op);
        assert!(base < higher_lit);
        // A larger reference outranks any operator/literal difference below it.
        assert!(higher_op < higher_ref);
    }

    #[test]
    fn operator_declared_order_is_lt_le_gt_ge_eq_ne() {
        let ops = [
            ComparisonOperator::Less,
            ComparisonOperator::LessOrEqual,
            ComparisonOperator::Greater,
            ComparisonOperator::GreaterOrEqual,
            ComparisonOperator::Equal,
            ComparisonOperator::NotEqual,
        ];
        for (i, a) in ops.iter().enumerate() {
            for (j, b) in ops.iter().enumerate() {
                let da = Discriminator::Atom(AtomKey::new(lineage(1), *a, num(0, 1)));
                let db = Discriminator::Atom(AtomKey::new(lineage(1), *b, num(0, 1)));
                assert_eq!(da.cmp(&db), i.cmp(&j));
            }
        }
    }

    #[test]
    fn node_path_shorter_proper_prefix_sorts_first() {
        let root = NodePath::root();
        let left = NodePath::new(vec![ChildPosition::LeftOperand]);
        let left_right = NodePath::new(vec![
            ChildPosition::LeftOperand,
            ChildPosition::RightOperand,
        ]);
        let right = NodePath::new(vec![ChildPosition::RightOperand]);
        assert!(root < left); // empty prefix first
        assert!(left < left_right); // proper prefix first
        assert!(left < right); // Left (1) < Right (2)
    }

    #[test]
    fn counterpart_orders_by_supplied_occurrence_order() {
        // Revision 4 §C.8: COUNTERPART payload is now an OccurrenceId, ordered by
        // the supplied occurrence order.
        let a = Discriminator::Counterpart(occ(1));
        let b = Discriminator::Counterpart(occ(2));
        assert!(a < b);
    }

    #[test]
    fn counterpart_represents_rules_regardless_of_lineage() {
        // §C.7: because COUNTERPART carries an occurrence identity (always
        // present), a pairwise defect's counterpart is representable even when the
        // counterpart rule's lineage is absent or shared — no lineage is consulted
        // here at all. Two distinct counterpart occurrences stay distinct.
        let a = Discriminator::Counterpart(occ(10));
        let b = Discriminator::Counterpart(occ(11));
        assert_ne!(a, b);
        assert!(a < b);
    }

    #[test]
    fn lineage_discriminator_orders_by_supplied_lineage_order() {
        // §C.5/§C.8: the LINEAGE kind carries the duplicated lineage identity,
        // ordered by the supplied lineage order — this is what separates multiple
        // duplicate-lineage classes sharing the envelope anchor.
        let l4 = Discriminator::Lineage(lineage(4));
        let l7 = Discriminator::Lineage(lineage(7));
        assert!(l4 < l7);
        assert_ne!(l4, l7);
        assert_eq!(l4, Discriminator::Lineage(lineage(4)));
    }

    #[test]
    fn counterpart_occurrence_and_lineage_class_are_different_kinds() {
        // A COUNTERPART (kind 6, occurrence payload) and a LINEAGE (kind 7,
        // lineage payload) are separated by kind ordinal first, so they never
        // compare equal and the occurrence/lineage domains never mix here.
        let counterpart = Discriminator::Counterpart(occ(1));
        let lineage_class = Discriminator::Lineage(lineage(1));
        assert!(counterpart < lineage_class);
        assert_ne!(counterpart, lineage_class);
    }

    #[test]
    fn equality_agrees_with_the_total_order() {
        let a = Discriminator::Property(PropertyPosition::new(3));
        let b = Discriminator::Property(PropertyPosition::new(3));
        let c = Discriminator::Property(PropertyPosition::new(4));
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
