//! The predicate fragment — Slice Contract §3.6.
//!
//! A guard is a finite, possibly empty **conjunction of atoms**, and an atom is
//! exactly `evidence-reference · operator · typed-literal`. The narrowness is
//! deliberate: it is what keeps overlap, subsumption and contradiction decidable
//! by per-reference comparison in Phase 3, and it is why §3.6 excludes the
//! forms below.
//!
//! # Shapes that are unrepresentable here, by construction
//!
//! * **Disjunction and negation** — a guard is a sequence of atoms with no `Or`
//!   or `Not` variant to reach for.
//! * **Nested guards** — an atom cannot contain a guard.
//! * **Evidence-vs-evidence comparison** — an atom's right operand is a
//!   `Literal`, never a second reference.
//! * **Arithmetic inside a guard** — no expression type appears here.
//! * **Configuration references in a guard** — the left operand is an evidence
//!   reference and nothing else, which is how §3.9's single-position rule is
//!   enforced structurally.
//!
//! By contrast, a *type-mismatched* atom — say an ordering operator applied to a
//! `BOOLEAN` referent — **is** representable, because §3.6 assigns that fault to
//! check 2 and a check cannot detect what cannot be built.

use crate::model::literal::Literal;
use crate::model::opaque::LineageId;

/// An admitted comparison operator — Slice Contract §3.6.
///
/// A closed authoritative category (ADR-0005 §7.4). Which operators are legal
/// for which referent type is a §4.1 typing rule enforced by check 2, not a
/// construction rule, so all six are representable against any referent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonOperator {
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    Equal,
    NotEqual,
}

/// A single guard atom — Slice Contract §3.6.
///
/// The atom is a **grammar-level fragment**, not an admitted construct with
/// declared properties of its own: §3 gives it no property table, and no
/// mandatory check owns a missing operand. Its operands are therefore bare
/// values rather than declaration states, and an incomplete atom is
/// unrepresentable rather than defective.
///
/// # Evidence-vs-evidence comparison is a compile error
///
/// §3.6 requires the right operand to be a typed literal and lists
/// "evidence-vs-evidence comparison" among its prohibited forms — a scope
/// boundary that keeps guards inside the per-reference fragment, with no check
/// assigned to it. A second reference has no type to inhabit here:
///
/// ```compile_fail
/// use ordina::model::guard::{Atom, ComparisonOperator};
/// use ordina::model::opaque::{LineageId, OpaqueValue};
///
/// let left = LineageId::new(OpaqueValue::new(1));
/// let right = LineageId::new(OpaqueValue::new(2));
///
/// // The third argument is a `Literal`; a second evidence reference is rejected.
/// let _ = Atom::new(left, ComparisonOperator::Greater, right);
/// ```
///
/// The admitted shape — a reference compared against a typed literal — compiles:
///
/// ```
/// use ordina::model::guard::{Atom, ComparisonOperator};
/// use ordina::model::literal::Literal;
/// use ordina::model::opaque::{LineageId, OpaqueValue};
///
/// let left = LineageId::new(OpaqueValue::new(1));
/// let _ = Atom::new(left, ComparisonOperator::Greater, Literal::Boolean(true));
/// ```
#[derive(Debug, Clone)]
pub struct Atom {
    /// Lineage identity of the declared evidence item this atom constrains.
    pub evidence: LineageId,
    pub operator: ComparisonOperator,
    pub literal: Literal,
}

impl Atom {
    #[must_use]
    pub const fn new(evidence: LineageId, operator: ComparisonOperator, literal: Literal) -> Self {
        Self {
            evidence,
            operator,
            literal,
        }
    }
}

/// A guard — a finite conjunction of atoms (Slice Contract §3.6).
///
/// An **empty** conjunction is the always-applicable guard (§4.5), and it is
/// the only way to write a total guard. §3.7 requires exactly one Direction
/// rule to use one.
#[derive(Debug, Clone, Default)]
pub struct Guard {
    pub atoms: Vec<Atom>,
}

impl Guard {
    /// The always-applicable guard — the empty conjunction (§4.5).
    #[must_use]
    pub const fn always_applicable() -> Self {
        Self { atoms: Vec::new() }
    }

    #[must_use]
    pub const fn from_atoms(atoms: Vec<Atom>) -> Self {
        Self { atoms }
    }

    /// Whether this is the syntactic empty guard.
    ///
    /// Phase 1 reports the *shape* only. What an empty conjunction **means**
    /// (§4.5: it holds always) and what coverage requires of it (§5.1 check 8)
    /// belong to later phases.
    #[must_use]
    pub fn is_syntactically_empty(&self) -> bool {
        self.atoms.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::opaque::{DimensionSymbol, OpaqueValue};
    use crate::numeric::ExactNumber;

    fn evidence(handle: u64) -> LineageId {
        LineageId::new(OpaqueValue::new(handle))
    }

    fn numeric_literal(value: i64) -> Literal {
        Literal::Numeric {
            value: ExactNumber::from_integer(value),
            dimension: DimensionSymbol::new(OpaqueValue::new(1)),
        }
    }

    #[test]
    fn an_atom_constrains_one_reference_against_one_literal() {
        let atom = Atom::new(
            evidence(1),
            ComparisonOperator::Greater,
            numeric_literal(100),
        );

        assert_eq!(atom.evidence, evidence(1));
        assert_eq!(atom.operator, ComparisonOperator::Greater);
    }

    #[test]
    fn the_empty_conjunction_is_representable() {
        let guard = Guard::always_applicable();

        assert!(guard.is_syntactically_empty());
    }

    #[test]
    fn a_conjunction_holds_several_atoms() {
        let guard = Guard::from_atoms(vec![
            Atom::new(
                evidence(1),
                ComparisonOperator::Greater,
                numeric_literal(10),
            ),
            Atom::new(
                evidence(2),
                ComparisonOperator::LessOrEqual,
                numeric_literal(20),
            ),
        ]);

        assert!(!guard.is_syntactically_empty());
        assert_eq!(guard.atoms.len(), 2);
    }

    #[test]
    fn type_mismatched_atoms_remain_representable_for_check_2() {
        // An ordering operator against a BOOLEAN referent is a §3.6 type fault
        // that check 2 must detect in Phase 5. It therefore has to be
        // constructible now.
        let atom = Atom::new(
            evidence(3),
            ComparisonOperator::Less,
            Literal::Boolean(true),
        );

        assert_eq!(atom.operator, ComparisonOperator::Less);
    }
}
