//! Semantic literal equality and comparison — Slice Contract §4.3, §4.4.
//!
//! Two **named** operations over typed literals, each returning a **closed
//! outcome type** rather than a `bool` or an `Ordering`. The contract gives each
//! relation more than two outcomes — equality can fail as a *type defect* rather
//! than as inequality (§4.3), and comparison is *undefined* on some domains
//! (§4.4) — so a two-valued return could not carry the meaning without silently
//! collapsing a defect into `false`.
//!
//! # Why `Literal` has no `PartialEq` / `Eq` / `PartialOrd` / `Ord` (B-4)
//!
//! §4.3 states that cross-dimension `NUMERIC` equality "is **not** `false` — it
//! is a **type defect**", and §4.4 states that ordering is "**undefined** — and
//! therefore a type defect — on `BOOLEAN` and `ENUM`". Rust's `==` returns
//! `bool` and `<` returns via `Ordering`; neither can express a type defect.
//! Implementing those traits on `Literal` would therefore make ordinary Rust
//! syntax silently answer a question the contract says is ill-typed. `Literal`
//! deliberately implements none of them, so the syntax does not exist and these
//! named operations are the only way to ask:
//!
//! ```compile_fail
//! use ordina::model::literal::Literal;
//! let _ = Literal::Boolean(true) == Literal::Boolean(false);
//! ```
//!
//! ```compile_fail
//! use ordina::model::literal::Literal;
//! let _ = Literal::Boolean(true) < Literal::Boolean(false);
//! ```
//!
//! ```compile_fail
//! use ordina::model::literal::Literal;
//! fn needs_eq<T: Eq>(_: T) {}
//! needs_eq(Literal::Boolean(true));
//! ```
//!
//! ```compile_fail
//! use ordina::model::literal::Literal;
//! fn needs_ord<T: Ord>(_: T) {}
//! needs_ord(Literal::Boolean(true));
//! ```
//!
//! # Separation from other orders
//!
//! [`LiteralComparison`] is §4.4 **semantic** comparison and nothing else. It is
//! not canonical defect ordering (§6.4, a later phase), not representation
//! ordering, and not host-language ordering. Its `Less` / `Equal` / `Greater`
//! variants are named outcomes, not `core::cmp::Ordering` values; an `Ordering`
//! is used only inside [`compare`], privately, after the identical-dimension
//! precondition has already been established.

use core::cmp::Ordering;

use crate::model::literal::Literal;

/// The outcome of §4.3 semantic equality over two literals.
///
/// A closed category (ADR-0005 §7.4). Cross-dimension `NUMERIC` and cross-kind
/// pairs are **defect outcomes**, kept distinct from `NotEqual`: §4.3 forbids
/// collapsing a type defect into ordinary inequality.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiteralEquality {
    /// The operands are well-typed for equality and equal.
    Equal,
    /// The operands are well-typed for equality and unequal.
    NotEqual,
    /// Both are `NUMERIC` but carry different declared dimensions — a §4.6 type
    /// defect, **not** `NotEqual` (§4.3).
    DimensionMismatch,
    /// The operands are of different literal kinds — §4.1 admits no coercion, so
    /// this is a type defect, not `NotEqual`.
    TypeMismatch,
}

/// The outcome of §4.4 semantic comparison over two literals.
///
/// A closed category (ADR-0005 §7.4). Comparison is defined **only** between
/// `NUMERIC` values of an identical declared dimension (§4.4); every other case
/// is a distinct defect outcome, never a manufactured ordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiteralComparison {
    Less,
    Equal,
    Greater,
    /// Both are `NUMERIC` but carry different declared dimensions — a §4.6 type
    /// defect.
    DimensionMismatch,
    /// Both operands are of the same domain, but §4.4 declares **no order** over
    /// it (`BOOLEAN`, `ENUM`) — a type defect.
    OrderingUndefined,
    /// The operands are of different literal kinds — a type defect (§4.1).
    TypeMismatch,
}

/// §4.3 semantic equality.
///
/// * `NUMERIC` vs `NUMERIC`: equal declared dimensions are required (§4.6); only
///   then are the exact values compared via Phase-2A [`ExactNumber`] equality.
///   Different dimensions yield [`LiteralEquality::DimensionMismatch`], never
///   `NotEqual`.
/// * `BOOLEAN` vs `BOOLEAN`: ordinary two-valued equality.
/// * `ENUM` vs `ENUM`: equality of the canonical categorical value.
/// * Different kinds: [`LiteralEquality::TypeMismatch`] — no coercion (§4.1).
///
/// `!=` is the exact complement of `=` on a well-typed pair (§4.3): it is
/// `Equal` negated, i.e. `NotEqual`, and is not a separate operation.
///
/// [`ExactNumber`]: crate::numeric::ExactNumber
#[must_use]
pub fn equal(left: &Literal, right: &Literal) -> LiteralEquality {
    // The pair is matched with no catch-all `_` arm: each inner match names the
    // other closed variants explicitly (ADR-0005 §7.4).
    match left {
        Literal::Numeric {
            value: left_value,
            dimension: left_dimension,
        } => match right {
            Literal::Numeric {
                value: right_value,
                dimension: right_dimension,
            } => {
                if left_dimension == right_dimension {
                    if left_value == right_value {
                        LiteralEquality::Equal
                    } else {
                        LiteralEquality::NotEqual
                    }
                } else {
                    LiteralEquality::DimensionMismatch
                }
            }
            Literal::Boolean(_) | Literal::Enum(_) => LiteralEquality::TypeMismatch,
        },
        Literal::Boolean(left_boolean) => match right {
            Literal::Boolean(right_boolean) => {
                if left_boolean == right_boolean {
                    LiteralEquality::Equal
                } else {
                    LiteralEquality::NotEqual
                }
            }
            Literal::Numeric { .. } | Literal::Enum(_) => LiteralEquality::TypeMismatch,
        },
        Literal::Enum(left_value) => match right {
            Literal::Enum(right_value) => {
                if left_value == right_value {
                    LiteralEquality::Equal
                } else {
                    LiteralEquality::NotEqual
                }
            }
            Literal::Numeric { .. } | Literal::Boolean(_) => LiteralEquality::TypeMismatch,
        },
    }
}

/// §4.4 semantic comparison.
///
/// Defined **only** for `NUMERIC` operands of an identical declared dimension,
/// where it is the ordinary total order on exact rationals. `BOOLEAN` and `ENUM`
/// have no declared order, so a same-kind pair there yields
/// [`LiteralComparison::OrderingUndefined`]; a cross-kind pair yields
/// [`LiteralComparison::TypeMismatch`]; and a cross-dimension `NUMERIC` pair
/// yields [`LiteralComparison::DimensionMismatch`].
#[must_use]
pub fn compare(left: &Literal, right: &Literal) -> LiteralComparison {
    // No catch-all `_` arm (ADR-0005 §7.4).
    match left {
        Literal::Numeric {
            value: left_value,
            dimension: left_dimension,
        } => match right {
            Literal::Numeric {
                value: right_value,
                dimension: right_dimension,
            } => {
                if left_dimension == right_dimension {
                    // `Ordering` is used only here, privately, after the
                    // identical-dimension precondition holds; the public result
                    // is the explicit semantic outcome.
                    match left_value.cmp(right_value) {
                        Ordering::Less => LiteralComparison::Less,
                        Ordering::Equal => LiteralComparison::Equal,
                        Ordering::Greater => LiteralComparison::Greater,
                    }
                } else {
                    LiteralComparison::DimensionMismatch
                }
            }
            Literal::Boolean(_) | Literal::Enum(_) => LiteralComparison::TypeMismatch,
        },
        Literal::Boolean(_) => match right {
            Literal::Boolean(_) => LiteralComparison::OrderingUndefined,
            Literal::Numeric { .. } | Literal::Enum(_) => LiteralComparison::TypeMismatch,
        },
        Literal::Enum(_) => match right {
            Literal::Enum(_) => LiteralComparison::OrderingUndefined,
            Literal::Numeric { .. } | Literal::Boolean(_) => LiteralComparison::TypeMismatch,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::opaque::{DimensionSymbol, EnumValue, OpaqueValue};
    use crate::numeric::ExactNumber;

    fn dimension(handle: u64) -> DimensionSymbol {
        DimensionSymbol::new(OpaqueValue::new(handle))
    }

    fn numeric(value: i64, dim: DimensionSymbol) -> Literal {
        Literal::Numeric {
            value: ExactNumber::from_integer(value),
            dimension: dim,
        }
    }

    fn categorical(handle: u64) -> Literal {
        Literal::Enum(EnumValue::new(OpaqueValue::new(handle)))
    }

    // -- §4.3 equality ------------------------------------------------------

    #[test]
    fn equal_numeric_same_dimension() {
        assert_eq!(
            equal(&numeric(5, dimension(1)), &numeric(5, dimension(1))),
            LiteralEquality::Equal
        );
    }

    #[test]
    fn unequal_numeric_same_dimension() {
        assert_eq!(
            equal(&numeric(5, dimension(1)), &numeric(6, dimension(1))),
            LiteralEquality::NotEqual
        );
    }

    #[test]
    fn numerically_equal_values_with_different_dimensions_are_a_type_defect_not_false() {
        // §4.3: cross-dimension equality is not `false` — it is a type defect.
        assert_eq!(
            equal(&numeric(5, dimension(1)), &numeric(5, dimension(2))),
            LiteralEquality::DimensionMismatch
        );
    }

    #[test]
    fn boolean_equality() {
        assert_eq!(
            equal(&Literal::Boolean(true), &Literal::Boolean(true)),
            LiteralEquality::Equal
        );
        assert_eq!(
            equal(&Literal::Boolean(true), &Literal::Boolean(false)),
            LiteralEquality::NotEqual
        );
    }

    #[test]
    fn enum_equality_is_of_the_canonical_value() {
        assert_eq!(
            equal(&categorical(10), &categorical(10)),
            LiteralEquality::Equal
        );
        assert_eq!(
            equal(&categorical(10), &categorical(11)),
            LiteralEquality::NotEqual
        );
    }

    #[test]
    fn cross_kind_equality_is_a_type_mismatch() {
        assert_eq!(
            equal(&numeric(1, dimension(1)), &Literal::Boolean(true)),
            LiteralEquality::TypeMismatch
        );
        assert_eq!(
            equal(&Literal::Boolean(true), &categorical(10)),
            LiteralEquality::TypeMismatch
        );
        assert_eq!(
            equal(&categorical(10), &numeric(1, dimension(1))),
            LiteralEquality::TypeMismatch
        );
    }

    // -- §4.4 comparison ----------------------------------------------------

    #[test]
    fn numeric_comparison_within_one_dimension() {
        assert_eq!(
            compare(&numeric(1, dimension(1)), &numeric(2, dimension(1))),
            LiteralComparison::Less
        );
        assert_eq!(
            compare(&numeric(2, dimension(1)), &numeric(2, dimension(1))),
            LiteralComparison::Equal
        );
        assert_eq!(
            compare(&numeric(3, dimension(1)), &numeric(2, dimension(1))),
            LiteralComparison::Greater
        );
    }

    #[test]
    fn cross_dimension_numeric_comparison_is_a_dimension_defect() {
        assert_eq!(
            compare(&numeric(1, dimension(1)), &numeric(2, dimension(2))),
            LiteralComparison::DimensionMismatch
        );
    }

    #[test]
    fn boolean_and_enum_have_no_order() {
        assert_eq!(
            compare(&Literal::Boolean(true), &Literal::Boolean(false)),
            LiteralComparison::OrderingUndefined
        );
        assert_eq!(
            compare(&categorical(10), &categorical(11)),
            LiteralComparison::OrderingUndefined
        );
    }

    #[test]
    fn cross_kind_comparison_is_a_type_mismatch() {
        assert_eq!(
            compare(&numeric(1, dimension(1)), &Literal::Boolean(true)),
            LiteralComparison::TypeMismatch
        );
        assert_eq!(
            compare(&Literal::Boolean(true), &categorical(10)),
            LiteralComparison::TypeMismatch
        );
    }

    // -- representation neutrality ------------------------------------------

    #[test]
    fn equality_result_survives_consistent_dimension_relabelling() {
        // Same values, disjoint dimension handles used consistently on both
        // sides. Equality depends on dimension *identity*, not on which handle
        // is used, so both relabellings agree.
        let a = equal(&numeric(5, dimension(1)), &numeric(5, dimension(1)));
        let b = equal(&numeric(5, dimension(99)), &numeric(5, dimension(99)));
        assert_eq!(a, LiteralEquality::Equal);
        assert_eq!(b, LiteralEquality::Equal);

        // And a cross-dimension pair remains a mismatch under any handles.
        let a_bad = equal(&numeric(5, dimension(1)), &numeric(5, dimension(2)));
        let b_bad = equal(&numeric(5, dimension(40)), &numeric(5, dimension(41)));
        assert_eq!(a_bad, LiteralEquality::DimensionMismatch);
        assert_eq!(b_bad, LiteralEquality::DimensionMismatch);
    }

    #[test]
    fn outcome_types_are_exhaustively_matchable() {
        // No catch-all `_` arm (ADR-0005 §7.4).
        let equality = equal(&Literal::Boolean(true), &Literal::Boolean(true));
        let _ = match equality {
            LiteralEquality::Equal => 0,
            LiteralEquality::NotEqual => 1,
            LiteralEquality::DimensionMismatch => 2,
            LiteralEquality::TypeMismatch => 3,
        };

        let comparison = compare(&numeric(1, dimension(1)), &numeric(1, dimension(1)));
        let _ = match comparison {
            LiteralComparison::Less => 0,
            LiteralComparison::Equal => 1,
            LiteralComparison::Greater => 2,
            LiteralComparison::DimensionMismatch => 3,
            LiteralComparison::OrderingUndefined => 4,
            LiteralComparison::TypeMismatch => 5,
        };
    }
}
