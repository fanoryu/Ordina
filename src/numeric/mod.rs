//! Exact numeric representation — Slice Contract §4.2, ADR-0005 §6.1, §6.3.
//!
//! Slice-1 authoritative numeric values are arbitrary-precision **exact
//! rationals**: no binary floating point, no rounding, no implicit tolerance,
//! no lossy conversion, no magnitude bound (Slice Contract §4.2).
//!
//! This module is the ADR-0005 §6.3 isolation boundary — the only module that
//! names the approved numeric crates, so checker logic never depends on them
//! directly and the representation stays replaceable.
//!
//! # Phase 2A scope — the value layer, and nothing dimensional
//!
//! Phase 2A implements §4.2 in full: exactness, unbounded magnitude,
//! representation-independent equality, the exact value order, and the three
//! admitted operations.
//!
//! It is deliberately **dimension-free**. §4.3's rule that `NUMERIC` equality
//! holds "**only within an identical declared dimension**", §4.4's
//! dimension-scoped comparison, and §4.6's dimensional rules are **Phase 2B**
//! and appear nowhere here. `ExactNumber` carries no dimension and cannot
//! answer a dimensional question, which is why nothing in this module can
//! silently decide one.
//!
//! Deliberately **not** implemented, and reserved for later phases: §4.3, §4.4,
//! §4.6, and every mandatory Well-Formedness check.
//!
//! # The admissible domain is narrower than the representation
//!
//! ADR-0005 §7.3 is explicit: `BigRational` can represent **every** rational,
//! while the contract admits exact **decimal** rationals. Representation
//! capability must not expand semantic admissibility.
//!
//! The control is structural. [`ExactNumber::from_decimal`] builds a value from
//! a sign, a digit sequence and a decimal point — so the denominator is always
//! a power of ten and a non-decimal rational such as `1/3` has no constructor
//! to come through. No runtime "invalid rational" path exists because the
//! invalid state is unconstructible, and no check owns it: §5.1 assigns no
//! defect condition to a non-decimal literal.
//!
//! Arithmetic cannot escape the domain either, and ADR-0005 §7.3 says why:
//! Slice 1 admits only `+`, `−` and `×`, and decimal rationals are closed under
//! all three. Division — the one operation that would leave the domain — is
//! excluded by §3.8 and is absent from this module.
//!
//! # No parser, no syntax
//!
//! Slice-1 input arrives **already canonical and in memory** (§2). The decimal
//! constructor therefore takes a decomposed value — sign, integer digits,
//! fraction digits — and not text. Nothing here parses, decodes a string,
//! deserializes, or defines an authoring syntax, and no serialization format is
//! selected.

use core::cmp::Ordering;

use num_bigint::BigInt;
use num_rational::BigRational;

/// The sign of a decimal value — Slice Contract §4.2.
///
/// A closed category: matching over it is exhaustive and no catch-all `_` arm
/// is permitted. Zero is signless in value terms — negating zero yields zero —
/// so `Positive` and `Negative` produce the same value for an all-zero digit
/// sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    Positive,
    Negative,
}

/// One decimal digit.
///
/// A closed ten-variant category rather than an integer, so a value outside
/// `0..=9` is **unconstructible**. The alternative — accepting a `u8` and
/// rejecting `10` at runtime — would invent a failure path for a state the
/// contract never contemplates. §15 of the governing authorization prefers the
/// structural proof, and so does every other closed category in this crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Digit {
    /// The digit's numeric value.
    const fn value(self) -> u8 {
        // No catch-all `_` arm: a change to this category must break here.
        match self {
            Self::Zero => 0,
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
        }
    }
}

/// An exact rational value — the authoritative Slice-1 numeric representation.
///
/// # Equality is of the value, never of the representation
///
/// §4.2 states it directly: `1.0`, `1.00` and `1e0` denote the **same value**,
/// and "representation is not meaning". Equality here is therefore exact
/// mathematical equality, independent of the digit sequence, the decimal scale,
/// the construction path, and the internal numerator/denominator. Comparison is
/// exact — §4.2 prohibits approximate or tolerance-based comparison, and no
/// epsilon exists anywhere in this module.
///
/// # This type carries no dimension
///
/// §4.3 makes `NUMERIC` equality valid **only within an identical declared
/// dimension**, and cross-dimension equality is "not `false` — it is a **type
/// defect**". That rule cannot be expressed by a type that has no dimension to
/// compare, which is exactly why it is not expressed here. A caller needing
/// §4.3 or §4.4 semantics must wait for Phase 2B; this type answers only the
/// dimension-free question.
///
/// # Three structural boundaries, preserved as compile-fail evidence
///
/// The guarantees this type rests on are **impossibilities**, and an
/// impossibility no test can observe is one a later change can remove while the
/// whole suite stays green. Each of the three below is therefore held open by a
/// `compile_fail` doctest — the same dependency-free idiom the model layer uses
/// for its grammar-level restrictions.
///
/// Each fails because the API shape makes the code unwritable, not because of
/// any particular compiler wording; nothing here matches diagnostic text.
///
/// **Arbitrary-ratio construction is unavailable.** This is what keeps the
/// admitted domain decimal (§4.2, ADR-0005 §7.3): no public entry point accepts
/// a denominator, so a non-decimal rational such as `1/3` cannot be built.
///
/// ```compile_fail
/// use ordina::numeric::ExactNumber;
///
/// let _ = ExactNumber::from_ratio(1, 3);
/// ```
///
/// **The internal representation is inaccessible.** This is what keeps the
/// ADR-0005 §6.3 isolation boundary intact: a caller cannot reach the
/// `BigRational` and so cannot bypass the decimal-domain constructors or bind
/// checker logic to a third-party numeric API.
///
/// ```compile_fail
/// use ordina::numeric::ExactNumber;
///
/// let n = ExactNumber::from_integer(1);
/// let _ = n.0;
/// ```
///
/// **Division is unavailable.** §3.8 excludes division from the admitted
/// grammar, and its absence is also what keeps the decimal domain **closed**
/// under the arithmetic offered here — `+`, `−` and `×` cannot leave the domain,
/// whereas division would.
///
/// ```compile_fail
/// use ordina::numeric::ExactNumber;
///
/// let a = ExactNumber::from_integer(1);
/// let b = ExactNumber::from_integer(2);
///
/// let _ = a.divide(&b);
/// ```
///
/// The admitted operations, by contrast, compile:
///
/// ```
/// use ordina::numeric::ExactNumber;
///
/// let a = ExactNumber::from_integer(1);
/// let b = ExactNumber::from_integer(2);
///
/// assert_eq!(a.add(&b), ExactNumber::from_integer(3));
/// assert_eq!(a.subtract(&b), ExactNumber::from_integer(-1));
/// assert_eq!(a.multiply(&b), ExactNumber::from_integer(2));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactNumber(BigRational);

impl ExactNumber {
    /// An exact integer value.
    ///
    /// A bounded convenience over the admitted domain — every `i64` is an exact
    /// decimal rational. It is **not** the magnitude boundary of the type:
    /// §4.2 admits no minimum or maximum magnitude, and
    /// [`ExactNumber::from_decimal`] reaches values of any size.
    #[must_use]
    pub fn from_integer(value: i64) -> Self {
        Self(BigRational::from(BigInt::from(value)))
    }

    /// An exact decimal value of unbounded magnitude and unbounded scale.
    ///
    /// The value is `± integer_digits . fraction_digits`, read most-significant
    /// digit first. Both sequences may be empty; two empty sequences are zero.
    ///
    /// This is the constructor that satisfies §4.2's "no minimum or maximum
    /// magnitude" — neither sequence is length-bounded, so `10^1000` and
    /// `10^-1000` are as constructible as `1`. It is also the constructor that
    /// keeps the admissible domain narrow: the denominator is `10^n` by
    /// construction, so the result is always an exact **decimal** rational
    /// (ADR-0005 §7.3).
    ///
    /// Leading and trailing zeroes carry no meaning, because the value does not
    /// retain them: `1`, `1.0` and `1.00` all yield equal values.
    #[must_use]
    pub fn from_decimal(sign: Sign, integer_digits: &[Digit], fraction_digits: &[Digit]) -> Self {
        let mut mantissa = BigInt::from(0);
        for digit in integer_digits.iter().chain(fraction_digits.iter()) {
            mantissa *= BigInt::from(10);
            mantissa += BigInt::from(digit.value());
        }

        let mut denominator = BigInt::from(1);
        for _ in 0..fraction_digits.len() {
            denominator *= BigInt::from(10);
        }

        // The denominator is a power of ten and therefore never zero.
        let magnitude = BigRational::new(mantissa, denominator);

        Self(match sign {
            // No catch-all `_` arm.
            Sign::Positive => magnitude,
            Sign::Negative => -magnitude,
        })
    }

    /// Exact addition — §4.2.
    #[must_use]
    pub fn add(&self, other: &Self) -> Self {
        Self(&self.0 + &other.0)
    }

    /// Exact subtraction — §4.2.
    #[must_use]
    pub fn subtract(&self, other: &Self) -> Self {
        Self(&self.0 - &other.0)
    }

    /// Exact multiplication — §4.2.
    #[must_use]
    pub fn multiply(&self, other: &Self) -> Self {
        Self(&self.0 * &other.0)
    }

    // There is no division, and its absence is a contract requirement rather
    // than an omission. §3.8 excludes division from the admitted expression
    // grammar because it would require deciding divide-by-zero behaviour and a
    // dimension algebra, neither of which any accepted document defines. Its
    // absence is also what keeps the admitted decimal domain closed under
    // arithmetic (ADR-0005 §7.3).
}

// ---------------------------------------------------------------------------
// Ordering.
//
// This is the **exact mathematical order on values**, and it is dimension-free.
//
// It exists because §6.4.3 will need it in Phase 4: the canonical defect
// ordering compares `NUMERIC` literals "by exact rational value, ascending",
// falling back to the dimension symbol only when the values are equal. That key
// is deliberately total across dimensions, because it orders *defects* and
// carries no Strategy semantics.
//
// It is **not** §4.4 comparison, and the two must never be conflated. §4.4
// defines `<`, `<=`, `>`, `>=` "**only** between values of an identical declared
// dimension", and a cross-dimension comparison is a type defect rather than an
// answer. Using this order to decide a §4.4 question would silently return an
// ordering where the contract requires a defect — a wrong verdict, surfacing
// only in Phase 5.
//
// The separation is structural as well as documented: `ExactNumber` holds no
// dimension, so this implementation *cannot* be reached with dimensional
// operands. Phase 2B introduces the dimension-aware operation separately.
//
// ADR-0005 §7.5 is not engaged here. Its prohibition concerns deriving
// authoritative order from the incidental representation of an **opaque**
// domain. Numeric values are not an opaque domain: the contract specifies their
// order mathematically (§4.2, §4.4), and delegating to exact rational
// comparison represents that specified order rather than an encoding artefact.
// ---------------------------------------------------------------------------

impl Ord for ExactNumber {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for ExactNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::Digit::{Eight, Five, Four, Nine, One, Seven, Six, Three, Two, Zero as Oh};
    use super::*;

    /// `± digits . digits`, positive.
    fn decimal(integer_digits: &[Digit], fraction_digits: &[Digit]) -> ExactNumber {
        ExactNumber::from_decimal(Sign::Positive, integer_digits, fraction_digits)
    }

    /// `± digits . digits`, negative.
    fn negative(integer_digits: &[Digit], fraction_digits: &[Digit]) -> ExactNumber {
        ExactNumber::from_decimal(Sign::Negative, integer_digits, fraction_digits)
    }

    /// A digit sequence of `count` nines — an easy way to build a value far
    /// beyond any machine integer.
    fn nines(count: usize) -> Vec<Digit> {
        vec![Nine; count]
    }

    /// Proves the ADR-0005 §6.2 dependencies resolve, link, and deliver *exact*
    /// arithmetic. One third summed three times is exactly one — an identity
    /// binary floating point cannot represent, which is the point of the
    /// exact-rational decision.
    ///
    /// **This is evidence about the dependency, not about admissibility.** It
    /// deliberately reaches past the authoritative constructors to `BigRational`
    /// directly, because one third is **not** an admissible Slice-1 value:
    /// §4.2 admits exact *decimal* rationals, and ADR-0005 §7.3 forbids the
    /// wider representation from expanding what the contract admits. No public
    /// `ExactNumber` constructor can produce this value, and
    /// `the_authoritative_api_cannot_admit_a_non_decimal_rational` is the test
    /// that holds that line.
    #[test]
    fn approved_numeric_dependencies_resolve_and_are_exact() {
        let third = BigRational::new(BigInt::from(1), BigInt::from(3));
        let sum = third.clone() + third.clone() + third.clone();

        assert_eq!(sum, BigRational::from(BigInt::from(1)));
    }

    // -----------------------------------------------------------------------
    // Construction — §4.2.
    // -----------------------------------------------------------------------

    #[test]
    fn zero_is_constructible_and_signless() {
        let empty = decimal(&[], &[]);
        let positive_zero = decimal(&[Oh], &[Oh, Oh]);
        let negative_zero = negative(&[Oh], &[Oh]);

        assert_eq!(empty, ExactNumber::from_integer(0));
        assert_eq!(positive_zero, empty);
        // Negating zero yields zero: sign carries no meaning at zero.
        assert_eq!(negative_zero, empty);
    }

    #[test]
    fn positive_and_negative_integers_are_constructible() {
        assert_eq!(
            decimal(&[One, Two, Three], &[]),
            ExactNumber::from_integer(123)
        );
        assert_eq!(
            negative(&[One, Two, Three], &[]),
            ExactNumber::from_integer(-123)
        );
    }

    #[test]
    fn magnitude_far_beyond_i64_is_constructible() {
        // Sixty nines — roughly 10^60, some forty orders of magnitude past
        // `i64::MAX`. §4.2 admits no maximum magnitude, so this must simply
        // work rather than saturate, wrap, or panic.
        let huge = decimal(&nines(60), &[]);
        let larger = decimal(&nines(61), &[]);
        let negated = negative(&nines(60), &[]);

        assert!(huge < larger);
        assert!(negated < huge);
        assert_eq!(huge.add(&negated), ExactNumber::from_integer(0));
    }

    #[test]
    fn very_small_exact_decimal_magnitudes_are_constructible() {
        // 10^-60. §4.2 admits no minimum magnitude either, and no underflow
        // exists — the value is exact, not a denormal approximation of zero.
        let mut fraction = vec![Oh; 59];
        fraction.push(One);
        let tiny = decimal(&[], &fraction);

        assert_ne!(tiny, ExactNumber::from_integer(0));
        assert!(tiny > ExactNumber::from_integer(0));
        // Exactness at scale: 10^-60 added to itself is 2 x 10^-60, not zero
        // and not a rounded neighbour.
        let mut doubled_fraction = vec![Oh; 59];
        doubled_fraction.push(Two);
        assert_eq!(tiny.add(&tiny), decimal(&[], &doubled_fraction));
    }

    #[test]
    fn decimal_fractional_values_are_exact() {
        // 0.1 + 0.2 == 0.3 exactly. In binary floating point this is the
        // canonical failure; here it is simply true.
        let tenth = decimal(&[Oh], &[One]);
        let fifth = decimal(&[Oh], &[Two]);
        let three_tenths = decimal(&[Oh], &[Three]);

        assert_eq!(tenth.add(&fifth), three_tenths);
    }

    // -----------------------------------------------------------------------
    // Equality — §4.2: representation is not meaning.
    // -----------------------------------------------------------------------

    #[test]
    fn trailing_zeroes_do_not_change_the_value() {
        // `1`, `1.0`, `1.00` and `1e0`. The last needs no parser and no
        // exponent syntax: `1e0` *is* the value one, and the canonical
        // in-memory form of the value one is what every other spelling yields.
        let one = decimal(&[One], &[]);
        let one_point_zero = decimal(&[One], &[Oh]);
        let one_point_zero_zero = decimal(&[One], &[Oh, Oh]);
        let one_e_zero = ExactNumber::from_integer(1);

        assert_eq!(one, one_point_zero);
        assert_eq!(one, one_point_zero_zero);
        assert_eq!(one, one_e_zero);
        assert_eq!(one_point_zero, one_point_zero_zero);
    }

    #[test]
    fn leading_zeroes_do_not_change_the_value() {
        assert_eq!(decimal(&[Oh, Oh, Seven], &[]), ExactNumber::from_integer(7));
    }

    #[test]
    fn equality_is_independent_of_the_construction_path() {
        // Same value, three different digit decompositions and scales.
        let via_integer = ExactNumber::from_integer(25);
        let via_digits = decimal(&[Two, Five], &[]);
        let via_scaled = decimal(&[Two, Five], &[Oh, Oh, Oh]);
        // And by arithmetic rather than construction.
        let via_arithmetic = ExactNumber::from_integer(5).multiply(&ExactNumber::from_integer(5));

        assert_eq!(via_integer, via_digits);
        assert_eq!(via_integer, via_scaled);
        assert_eq!(via_integer, via_arithmetic);
    }

    #[test]
    fn distinct_values_are_not_equal() {
        assert_ne!(decimal(&[One], &[Oh, One]), decimal(&[One], &[]));
        assert_ne!(decimal(&[One], &[]), negative(&[One], &[]));
    }

    // -----------------------------------------------------------------------
    // Ordering — the exact value order, dimension-free (§6.4.3 key).
    // -----------------------------------------------------------------------

    #[test]
    fn ordering_is_by_exact_value() {
        let small = decimal(&[One], &[Oh, One]);
        let middle = decimal(&[One], &[One]);
        let large = decimal(&[Two], &[]);

        assert!(small < middle);
        assert!(middle < large);
        assert!(large > small);
        assert_eq!(middle.cmp(&decimal(&[One], &[One, Oh])), Ordering::Equal);
    }

    #[test]
    fn ordering_is_antisymmetric_and_transitive() {
        let a = negative(&[Four], &[Five]);
        let b = ExactNumber::from_integer(0);
        let c = decimal(&[Six], &[Seven, Eight]);

        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(b.cmp(&a), Ordering::Greater);
        assert_eq!(b.cmp(&c), Ordering::Less);
        // Transitivity.
        assert_eq!(a.cmp(&c), Ordering::Less);
        // Antisymmetry at equality.
        assert_eq!(a.cmp(&a.clone()), Ordering::Equal);
    }

    #[test]
    fn sorting_is_deterministic_and_repeatable() {
        let build = || {
            vec![
                decimal(&[Three], &[]),
                negative(&[One], &[Five]),
                ExactNumber::from_integer(0),
                decimal(&[], &[Oh, Oh, One]),
                decimal(&nines(40), &[]),
            ]
        };

        let mut first = build();
        let mut second = build();
        first.sort();
        second.sort();

        assert_eq!(first, second);
        assert_eq!(first[0], negative(&[One], &[Five]));
        assert_eq!(first[4], decimal(&nines(40), &[]));
        // Sorting the already-sorted sequence changes nothing.
        let mut again = first.clone();
        again.sort();
        assert_eq!(again, first);
    }

    #[test]
    fn ordering_holds_beyond_machine_integer_limits() {
        let beyond = decimal(&nines(30), &[]);
        let beyond_plus_one = beyond.add(&ExactNumber::from_integer(1));

        assert!(beyond < beyond_plus_one);
        assert!(ExactNumber::from_integer(i64::MAX) < beyond);
        assert!(ExactNumber::from_integer(i64::MIN) > negative(&nines(30), &[]));
    }

    // -----------------------------------------------------------------------
    // Arithmetic — §4.2: exact, total, associative, commutative.
    // -----------------------------------------------------------------------

    #[test]
    fn addition_and_subtraction_are_exact() {
        let a = decimal(&[One, Two], &[Three, Four]);
        let b = decimal(&[Five], &[Six, Six]);

        assert_eq!(a.add(&b), decimal(&[One, Eight], &[Oh, Oh]));
        assert_eq!(a.subtract(&b), decimal(&[Six], &[Six, Eight]));
        // Subtraction is addition of the negation.
        assert_eq!(a.subtract(&b), a.add(&negative(&[Five], &[Six, Six])));
    }

    #[test]
    fn multiplication_is_exact() {
        let a = decimal(&[One], &[Five]);
        let b = decimal(&[Two], &[Five]);

        assert_eq!(a.multiply(&b), decimal(&[Three], &[Seven, Five]));
        assert_eq!(
            a.multiply(&negative(&[Two], &[])),
            negative(&[Three], &[Oh])
        );
    }

    #[test]
    fn arithmetic_holds_at_arbitrary_precision() {
        let huge = decimal(&nines(50), &[]);
        let one = ExactNumber::from_integer(1);
        // 999...9 + 1 == 1000...0, exactly, with no overflow.
        let mut rounded = vec![One];
        rounded.extend(vec![Oh; 50]);

        assert_eq!(huge.add(&one), decimal(&rounded, &[]));
        assert_eq!(huge.add(&one).subtract(&one), huge);
    }

    #[test]
    fn addition_and_multiplication_are_commutative() {
        let a = negative(&[Seven], &[Two, Five]);
        let b = decimal(&[One, One], &[Oh, Eight]);

        assert_eq!(a.add(&b), b.add(&a));
        assert_eq!(a.multiply(&b), b.multiply(&a));
    }

    #[test]
    fn addition_and_multiplication_are_associative() {
        let a = decimal(&[One], &[Three]);
        let b = negative(&[Two], &[Seven]);
        let c = decimal(&[], &[Oh, Oh, Nine]);

        assert_eq!(a.add(&b).add(&c), a.add(&b.add(&c)));
        assert_eq!(a.multiply(&b).multiply(&c), a.multiply(&b.multiply(&c)));
    }

    #[test]
    fn the_arithmetic_identities_hold() {
        let value = decimal(&[Four, Two], &[One, Nine]);
        let zero = ExactNumber::from_integer(0);
        let one = ExactNumber::from_integer(1);

        assert_eq!(value.add(&zero), value);
        assert_eq!(value.subtract(&zero), value);
        assert_eq!(value.multiply(&one), value);
        assert_eq!(value.multiply(&zero), zero);
        assert_eq!(value.subtract(&value), zero);
    }

    // -----------------------------------------------------------------------
    // Domain boundary — ADR-0005 §7.3.
    // -----------------------------------------------------------------------

    #[test]
    fn the_authoritative_api_cannot_admit_a_non_decimal_rational() {
        // The proof is structural, so what is asserted here is the *shape* of
        // the domain rather than the rejection of a value: every constructor
        // yields `mantissa / 10^n`, and no public entry point accepts a
        // denominator. `1/3` therefore has no way in — there is no call to
        // write, which is why no runtime rejection path exists for one.
        //
        // What is testable is the consequence: the closest admissible decimals
        // bracket one third without ever equalling it, at any scale.
        let third_of_one = |scale: usize| {
            let mut digits = vec![Three; scale];
            digits.push(Three);
            decimal(&[], &digits)
        };

        let three = ExactNumber::from_integer(3);
        let one = ExactNumber::from_integer(1);

        for scale in [1usize, 5, 20, 60] {
            let approximation = third_of_one(scale);
            // Exactly three times any admissible decimal is strictly below one:
            // the value is never silently rounded up to one third.
            assert!(approximation.multiply(&three) < one);
        }
    }

    #[test]
    fn every_constructed_value_is_an_exact_decimal_rational() {
        // Multiplying a value of scale `n` by 10^n yields an integer — the
        // defining property of a decimal rational. Verified through the public
        // arithmetic rather than by inspecting the representation.
        let scaled = decimal(&[One, Two], &[Three, Four, Five]);
        let thousand = ExactNumber::from_integer(1000);
        let product = scaled.multiply(&thousand);

        assert_eq!(product, ExactNumber::from_integer(12_345));
    }

    #[test]
    fn digits_and_signs_are_exhaustively_matchable() {
        // No catch-all `_` arm: extending either closed category must break
        // here rather than pass silently.
        for sign in [Sign::Positive, Sign::Negative] {
            let named = match sign {
                Sign::Positive => "positive",
                Sign::Negative => "negative",
            };
            assert!(!named.is_empty());
        }

        let all = [Oh, One, Two, Three, Four, Five, Six, Seven, Eight, Nine];
        let value = decimal(&all, &[]);

        // 0123456789 is 123456789.
        assert_eq!(value, ExactNumber::from_integer(123_456_789));
    }
}
