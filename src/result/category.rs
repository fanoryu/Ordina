//! Defect categories and their ADR-0004 check numbers — Slice Contract §6.4.1.
//!
//! §6.4.1 declares the **complete** set of 26 defect categories admitted by
//! Slice 1, each with an explicit ordinal that **is** the §6.4 key-3 order, and
//! each owned by one ADR-0004 mandatory check. Both the category ordinal and the
//! check number are declared here **explicitly**, never derived from a variant's
//! name or its position in a Rust `enum` (ADR-0005 §7.5): a reordering of the
//! source would not change the authoritative order, because the order lives in
//! [`DefectCategory::ordinal`] and [`DefectCategory::check`], not in the layout.

use core::cmp::Ordering;

/// An ADR-0004 mandatory-check number, as referenced by §6.4.1.
///
/// Only the checks that own a Slice-1 defect category appear. `13A`/`13B` and
/// `14A`/`14B` are **distinct** checks. The §6.4 key-1 order ("check number,
/// ascending") is given by [`CheckNumber::sort_key`], declared explicitly rather
/// than taken from variant layout. A closed category, no catch-all arm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckNumber {
    /// Check 1 — required properties present.
    C1,
    /// Check 2 — reference / type resolution.
    C2,
    /// Check 3 — condition classification.
    C3,
    /// Check 4 — configuration bounds.
    C4,
    /// Check 5 — configuration in structural position.
    C5,
    /// Check 6 — evidence declaration validity.
    C6,
    /// Check 8 — Direction coverage.
    C8,
    /// Check 9 — Direction precedence validity.
    C9,
    /// Check 10 — Direction outcome validity.
    C10,
    /// Check 11 — unreachable operative rules.
    C11,
    /// Check 12 — contradictory operative rules.
    C12,
    /// Check 13A — derivation form.
    C13A,
    /// Check 13B — candidate derivation presence.
    C13B,
    /// Check 14A — derivation dimensional consistency.
    C14A,
    /// Check 14B — candidate dimensional divergence.
    C14B,
    /// Check 15 — operative provenance.
    C15,
    /// Check 16 — construct / semantic-model recognition.
    C16,
}

impl CheckNumber {
    /// The §6.4 key-1 sort position, ascending. Declared explicitly so the
    /// order does not depend on variant layout.
    #[must_use]
    pub const fn sort_key(self) -> u8 {
        // No catch-all `_` arm: a new check must be placed here deliberately.
        match self {
            Self::C1 => 1,
            Self::C2 => 2,
            Self::C3 => 3,
            Self::C4 => 4,
            Self::C5 => 5,
            Self::C6 => 6,
            Self::C8 => 7,
            Self::C9 => 8,
            Self::C10 => 9,
            Self::C11 => 10,
            Self::C12 => 11,
            Self::C13A => 12,
            Self::C13B => 13,
            Self::C14A => 14,
            Self::C14B => 15,
            Self::C15 => 16,
            Self::C16 => 17,
        }
    }
}

impl Ord for CheckNumber {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sort_key().cmp(&other.sort_key())
    }
}

impl PartialOrd for CheckNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A Slice-1 defect category — §6.4.1.
///
/// The complete admitted set; a category outside it cannot arise because §5.1 is
/// exhaustive. Adding or removing one is a checker-semantics version change
/// (§6.2). A closed authoritative category, matched exhaustively with no
/// catch-all arm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefectCategory {
    MissingRequiredProperty,
    DuplicateLineageIdentity,
    UnresolvedReference,
    TypeMismatch,
    EnumValueNotDeclared,
    InvalidConditionClassification,
    InvertedConfigurationBounds,
    ConfigurationBoundDimensionMismatch,
    ConfigurationInStructuralPosition,
    InvalidEvidenceDeclaration,
    DirectionCoverageIncomplete,
    MultipleTotalDirectionRules,
    TotalRuleNotLowestPrecedence,
    DirectionPrecedenceTie,
    InvalidDirectionOutcome,
    UnreachableContradictoryGuard,
    UnreachableSubsumedRule,
    ContradictoryConditionGuard,
    InvalidDerivationForm,
    MissingCandidateDerivation,
    DuplicateCandidateDerivation,
    DimensionMismatch,
    CandidateDimensionDivergence,
    MissingOperativeProvenance,
    UnsupportedSemanticModelVersion,
    UnknownConstruct,
}

impl DefectCategory {
    /// The §6.4.1 ordinal — the §6.4 key-3 order, ascending. Declared here, not
    /// derived from the variant name or its layout position.
    #[must_use]
    pub const fn ordinal(self) -> u8 {
        // No catch-all `_` arm (ADR-0005 §7.4): a category change must break here.
        match self {
            Self::MissingRequiredProperty => 1,
            Self::DuplicateLineageIdentity => 2,
            Self::UnresolvedReference => 3,
            Self::TypeMismatch => 4,
            Self::EnumValueNotDeclared => 5,
            Self::InvalidConditionClassification => 6,
            Self::InvertedConfigurationBounds => 7,
            Self::ConfigurationBoundDimensionMismatch => 8,
            Self::ConfigurationInStructuralPosition => 9,
            Self::InvalidEvidenceDeclaration => 10,
            Self::DirectionCoverageIncomplete => 11,
            Self::MultipleTotalDirectionRules => 12,
            Self::TotalRuleNotLowestPrecedence => 13,
            Self::DirectionPrecedenceTie => 14,
            Self::InvalidDirectionOutcome => 15,
            Self::UnreachableContradictoryGuard => 16,
            Self::UnreachableSubsumedRule => 17,
            Self::ContradictoryConditionGuard => 18,
            Self::InvalidDerivationForm => 19,
            Self::MissingCandidateDerivation => 20,
            Self::DuplicateCandidateDerivation => 21,
            Self::DimensionMismatch => 22,
            Self::CandidateDimensionDivergence => 23,
            Self::MissingOperativeProvenance => 24,
            Self::UnsupportedSemanticModelVersion => 25,
            Self::UnknownConstruct => 26,
        }
    }

    /// The ADR-0004 check that owns this category — the §6.4 key-1 source
    /// (§6.4.1 table).
    #[must_use]
    pub const fn check(self) -> CheckNumber {
        // No catch-all `_` arm: every category maps to exactly one check.
        match self {
            Self::MissingRequiredProperty => CheckNumber::C1,
            Self::DuplicateLineageIdentity
            | Self::UnresolvedReference
            | Self::TypeMismatch
            | Self::EnumValueNotDeclared => CheckNumber::C2,
            Self::InvalidConditionClassification => CheckNumber::C3,
            Self::InvertedConfigurationBounds | Self::ConfigurationBoundDimensionMismatch => {
                CheckNumber::C4
            }
            Self::ConfigurationInStructuralPosition => CheckNumber::C5,
            Self::InvalidEvidenceDeclaration => CheckNumber::C6,
            Self::DirectionCoverageIncomplete
            | Self::MultipleTotalDirectionRules
            | Self::TotalRuleNotLowestPrecedence => CheckNumber::C8,
            Self::DirectionPrecedenceTie => CheckNumber::C9,
            Self::InvalidDirectionOutcome => CheckNumber::C10,
            Self::UnreachableContradictoryGuard | Self::UnreachableSubsumedRule => CheckNumber::C11,
            Self::ContradictoryConditionGuard => CheckNumber::C12,
            Self::InvalidDerivationForm => CheckNumber::C13A,
            Self::MissingCandidateDerivation | Self::DuplicateCandidateDerivation => {
                CheckNumber::C13B
            }
            Self::DimensionMismatch => CheckNumber::C14A,
            Self::CandidateDimensionDivergence => CheckNumber::C14B,
            Self::MissingOperativeProvenance => CheckNumber::C15,
            Self::UnsupportedSemanticModelVersion | Self::UnknownConstruct => CheckNumber::C16,
        }
    }
}

impl Ord for DefectCategory {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ordinal().cmp(&other.ordinal())
    }
}

impl PartialOrd for DefectCategory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The 26 categories in declared order — the fixture for exhaustiveness and
    /// ordering tests.
    const ALL: [DefectCategory; 26] = [
        DefectCategory::MissingRequiredProperty,
        DefectCategory::DuplicateLineageIdentity,
        DefectCategory::UnresolvedReference,
        DefectCategory::TypeMismatch,
        DefectCategory::EnumValueNotDeclared,
        DefectCategory::InvalidConditionClassification,
        DefectCategory::InvertedConfigurationBounds,
        DefectCategory::ConfigurationBoundDimensionMismatch,
        DefectCategory::ConfigurationInStructuralPosition,
        DefectCategory::InvalidEvidenceDeclaration,
        DefectCategory::DirectionCoverageIncomplete,
        DefectCategory::MultipleTotalDirectionRules,
        DefectCategory::TotalRuleNotLowestPrecedence,
        DefectCategory::DirectionPrecedenceTie,
        DefectCategory::InvalidDirectionOutcome,
        DefectCategory::UnreachableContradictoryGuard,
        DefectCategory::UnreachableSubsumedRule,
        DefectCategory::ContradictoryConditionGuard,
        DefectCategory::InvalidDerivationForm,
        DefectCategory::MissingCandidateDerivation,
        DefectCategory::DuplicateCandidateDerivation,
        DefectCategory::DimensionMismatch,
        DefectCategory::CandidateDimensionDivergence,
        DefectCategory::MissingOperativeProvenance,
        DefectCategory::UnsupportedSemanticModelVersion,
        DefectCategory::UnknownConstruct,
    ];

    #[test]
    fn ordinals_are_one_through_twenty_six_in_declared_order() {
        for (index, category) in ALL.iter().enumerate() {
            let expected = u8::try_from(index + 1).expect("26 fits in u8");
            assert_eq!(category.ordinal(), expected);
        }
    }

    #[test]
    fn ordinals_are_distinct_and_total() {
        for (i, a) in ALL.iter().enumerate() {
            for (j, b) in ALL.iter().enumerate() {
                assert_eq!(a.cmp(b), i.cmp(&j));
            }
        }
    }

    #[test]
    fn category_check_mapping_matches_6_4_1() {
        use CheckNumber::{
            C1, C2, C3, C4, C5, C6, C8, C9, C10, C11, C12, C13A, C13B, C14A, C14B, C15, C16,
        };
        let expected = [
            C1, C2, C2, C2, C2, C3, C4, C4, C5, C6, C8, C8, C8, C9, C10, C11, C11, C12, C13A, C13B,
            C13B, C14A, C14B, C15, C16, C16,
        ];
        for (category, check) in ALL.iter().zip(expected.iter()) {
            assert_eq!(category.check(), *check);
        }
    }

    #[test]
    fn check_numbers_sort_ascending_with_sub_checks_in_sequence() {
        use CheckNumber::{C11, C12, C13A, C13B, C14A, C14B};
        assert!(C11 < C12);
        assert!(C12 < C13A);
        assert!(C13A < C13B);
        assert!(C13B < C14A);
        assert!(C14A < C14B);
    }
}
