//! Declaration presence and category-declaration state — Slice Contract §3.
//!
//! # Why a declared property is not simply its value
//!
//! The Slice Contract assigns **explicit defect semantics** to declarations that
//! are missing or malformed. Check 1 reports an absent unconditional property
//! (`MISSING_REQUIRED_PROPERTY`); check 15 reports an absent operative lineage
//! identity (`MISSING_OPERATIVE_PROVENANCE`); check 16 reports an absent
//! semantic-model version; checks 3 and 10 report a classification or Direction
//! outcome that is **zero, multiple, or out-of-domain**; check 6 reports an
//! evidence-declaration property that is present where the declared type forbids
//! it, or absent where the type requires it.
//!
//! A check cannot detect what cannot be constructed. Modelling a mandatory
//! property as its bare value therefore does not make a Strategy safer — it
//! deletes a mandatory check's entire defect population and would make the
//! checker silently unable to reach a verdict the contract requires. The two
//! types below exist solely so those states stay **representable**.
//!
//! # What these types are not
//!
//! They are **not** validation, and Phase 1 implements none. `Declared::Absent`
//! is not a defect; it is the *shape* a defect will later be reported about.
//! Nothing here decides which check owns a state, when a state is faulty, or how
//! a defect is produced or ordered.
//!
//! They are also **not** a general optionality mechanism. `Option<T>` is
//! deliberately absent from this module and from the model: the contract's
//! absence states are named, load-bearing, and owned by specific checks, and
//! collapsing them into the host language's ordinary optionality would lose
//! exactly the distinction the contract draws. Where §3 admits no absence — a
//! grammar-level fragment such as a guard atom's operands — the property stays a
//! bare value and absence remains unrepresentable by construction.

/// The declaration state of a single mandatory property — Slice Contract §3.
///
/// A closed authoritative category (ADR-0005 §7.4): matching over it is
/// exhaustive and no catch-all `_` arm is permitted.
///
/// Use this for a property whose **absence** the contract makes a defect
/// condition and whose value, when present, is drawn from an open or opaque
/// domain — a lineage identity, a dimension symbol, a version token, a bound, a
/// guard, an expression. For a property drawn from a **closed** named category,
/// where the contract additionally defines "multiple" and "out-of-domain"
/// faults, use [`DeclaredCategory`] instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Declared<T> {
    /// The declaration is absent from the canonical input.
    ///
    /// This is the state check 1, check 15 and check 16 exist to find. It is
    /// **not** a default, a null, or a placeholder for a value: no value is
    /// implied, and none may be invented (ADR-0002 §10b — the input is already
    /// canonical, so defaults are already resolved explicitly).
    Absent,
    /// The declaration is present and carries this value.
    ///
    /// "Present" says nothing about "valid". A present bound may still be
    /// inverted (check 4) and a present guard may still be contradictory
    /// (check 11); those are later phases' judgements.
    Present(T),
}

impl<T> Declared<T> {
    /// A present declaration.
    #[must_use]
    pub const fn present(value: T) -> Self {
        Self::Present(value)
    }

    /// Whether the declaration is absent.
    #[must_use]
    pub const fn is_absent(&self) -> bool {
        matches!(*self, Self::Absent)
    }

    /// Whether the declaration is present.
    #[must_use]
    pub const fn is_present(&self) -> bool {
        matches!(*self, Self::Present(_))
    }
}

/// The declaration state of a property drawn from a **closed** category —
/// Slice Contract §3.5, §3.7, §3.10.
///
/// A closed authoritative category (ADR-0005 §7.4): matching over it is
/// exhaustive and no catch-all `_` arm is permitted.
///
/// The contract states four distinct outcomes for such a property, and this type
/// keeps all four apart. §3.10 prohibits "zero or more than one classification"
/// and "any classification value outside the two named"; §3.7 prohibits the same
/// three shapes for a Direction outcome; §3.5 and §5.1.1 make "declared type is
/// not one of `NUMERIC` / `BOOLEAN` / `ENUM`" a check-6 violation. Checks 3, 10
/// and 6 each report one defect per construct per violation, so each shape must
/// be constructible.
///
/// # Why `OutOfDomain` carries no payload
///
/// An out-of-domain declaration is a value the closed category does not contain,
/// and by construction the typed model has no such value to hold. The contract
/// does not need one either: checks 3 and 10 emit exactly one defect per
/// construct for the out-of-domain case and never report the offending value, and
/// check 6 discriminates by the §3.5 **property**, never by content (§5.1.1). The
/// contract-significant fact is therefore *that the declaration was out of
/// domain*, and this variant records exactly that.
///
/// Giving the variant a payload would require inventing an untyped value tree, a
/// foreign-content representation, or an arbitrary string inside the authoritative
/// model — which §4 of the governing authorization forbids, and which would
/// duplicate at the wrong layer the external shape/interpretation boundary that
/// owns `UNKNOWN_CONSTRUCT`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclaredCategory<T> {
    /// No declaration at all — the "zero" case of §3.7 and §3.10, and check 1's
    /// business where the property is unconditional (§3.5).
    Absent,
    /// Exactly one declaration, drawn from the closed category. The only
    /// well-formed shape.
    One(T),
    /// More than one declaration — the "more than one" case of §3.7 and §3.10.
    ///
    /// Two members are held **structurally** rather than in a bare sequence, so
    /// that "multiple" cannot silently encode zero or one. An ambiguous encoding
    /// would reintroduce the very collapse this type exists to prevent.
    Multiple {
        first: T,
        second: T,
        /// Any further declarations beyond the first two.
        rest: Vec<T>,
    },
    /// A declaration outside the closed category — the "out-of-domain" case.
    /// See the type-level note above for why it carries nothing.
    OutOfDomain,
}

impl<T> DeclaredCategory<T> {
    /// The single well-formed shape: exactly one declaration.
    #[must_use]
    pub const fn one(value: T) -> Self {
        Self::One(value)
    }

    /// Two or more declarations.
    #[must_use]
    pub const fn multiple(first: T, second: T, rest: Vec<T>) -> Self {
        Self::Multiple {
            first,
            second,
            rest,
        }
    }
}

// No summarizing accessor is offered, and that is deliberate (ARR-0015 M-1).
//
// A count, a validity predicate, or any similar convenience necessarily maps
// two of the four states onto one answer — `One` and `OutOfDomain` both being
// "one declaration" is the paradigm case. That collapse is exactly the failure
// ARR-0014 MAJOR-1 was raised about, reintroduced one layer up: a later gate
// written as "exactly one declaration, therefore well-formed" would silently
// admit the out-of-domain state that checks 3, 6 and 10 exist to report.
//
// Consumers match the four variants exhaustively instead. That is more verbose
// at each site and it is meant to be: this model's discipline is to make the
// wrong thing unrepresentable rather than to rely on a rule a reader has to
// remember.

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Category {
        A,
        B,
    }

    #[test]
    fn absence_and_presence_are_distinct_states() {
        let absent: Declared<u8> = Declared::Absent;
        let present = Declared::present(7u8);

        assert!(absent.is_absent());
        assert!(!absent.is_present());
        assert!(present.is_present());
        assert_ne!(absent, present);
    }

    #[test]
    fn a_declared_property_is_exhaustively_matchable() {
        // No catch-all `_` arm (ADR-0005 §7.4): a third declaration state must
        // break this match rather than pass silently.
        let declared = Declared::present(1u8);
        let described = match declared {
            Declared::Absent => "absent",
            Declared::Present(_) => "present",
        };

        assert_eq!(described, "present");
    }

    /// A distinct label for each of the four authoritative states.
    ///
    /// Total and **injective**: no two variants share a label, so unlike a
    /// count this cannot be used to conflate `One` with `OutOfDomain`. It is
    /// test-local and is not part of the public API.
    fn describe(value: &DeclaredCategory<Category>) -> &'static str {
        match value {
            // No catch-all `_` arm (ADR-0005 §7.4).
            DeclaredCategory::Absent => "absent",
            DeclaredCategory::One(_) => "one",
            DeclaredCategory::Multiple { .. } => "multiple",
            DeclaredCategory::OutOfDomain => "out-of-domain",
        }
    }

    #[test]
    fn all_four_category_declaration_states_are_distinct() {
        let absent: DeclaredCategory<Category> = DeclaredCategory::Absent;
        let one = DeclaredCategory::one(Category::A);
        let multiple = DeclaredCategory::multiple(Category::A, Category::B, vec![]);
        let out_of_domain: DeclaredCategory<Category> = DeclaredCategory::OutOfDomain;

        // Each value lands on its own arm — the distinction is structural, and
        // in particular `One` and `OutOfDomain` do not answer alike.
        assert_eq!(describe(&absent), "absent");
        assert_eq!(describe(&one), "one");
        assert_eq!(describe(&multiple), "multiple");
        assert_eq!(describe(&out_of_domain), "out-of-domain");

        // The point of the type: no two of these collapse into one another.
        assert_ne!(absent, one);
        assert_ne!(absent, out_of_domain);
        assert_ne!(one, out_of_domain);
        assert_ne!(one, multiple);
        assert_ne!(multiple, out_of_domain);
    }

    #[test]
    fn multiple_cannot_encode_fewer_than_two_declarations() {
        // `Multiple` holds its first two members structurally, so the smallest
        // representable "multiple" is genuinely two. Reading the members back
        // is what proves the arity — a count would only have asserted it.
        let smallest = DeclaredCategory::multiple(Category::A, Category::B, vec![]);
        let larger = DeclaredCategory::multiple(Category::A, Category::B, vec![Category::A]);

        match smallest {
            DeclaredCategory::Multiple {
                first,
                second,
                rest,
            } => {
                assert_eq!(first, Category::A);
                assert_eq!(second, Category::B);
                assert!(rest.is_empty());
            }
            DeclaredCategory::Absent | DeclaredCategory::One(_) | DeclaredCategory::OutOfDomain => {
                panic!("constructed a multiple declaration")
            }
        }

        match larger {
            DeclaredCategory::Multiple {
                first,
                second,
                rest,
            } => {
                assert_eq!(first, Category::A);
                assert_eq!(second, Category::B);
                assert_eq!(rest, vec![Category::A]);
            }
            DeclaredCategory::Absent | DeclaredCategory::One(_) | DeclaredCategory::OutOfDomain => {
                panic!("constructed a multiple declaration")
            }
        }
    }

    #[test]
    fn a_category_declaration_is_exhaustively_matchable() {
        // No catch-all `_` arm (ADR-0005 §7.4).
        let declared = DeclaredCategory::one(Category::B);
        let described = match declared {
            DeclaredCategory::Absent => "absent",
            DeclaredCategory::One(_) => "one",
            DeclaredCategory::Multiple { .. } => "multiple",
            DeclaredCategory::OutOfDomain => "out-of-domain",
        };

        assert_eq!(described, "one");
    }
}
