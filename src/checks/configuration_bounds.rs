//! Check 4 — configuration-bound validity (Slice Contract §5.1 check 4, §4.11).
//!
//! A parameter's bounds define a closed interval `[lower, upper]` in the
//! parameter's declared dimension. §4.11 makes it **valid iff** `lower <= upper`
//! (equality permitted) **and** both bounds share the parameter's dimension —
//! two independent conjuncts, hence two categories:
//!
//! - `INVERTED_CONFIGURATION_BOUNDS` — `lower > upper` by exact numeric value
//!   (§4.2). **One per parameter** (a single interval relation); anchored on the
//!   parameter occurrence (Revision 4 §C) with the `NONE` discriminator (one such
//!   defect per parameter, so key 2 already distinguishes it).
//! - `CONFIGURATION_BOUND_DIMENSION_MISMATCH` — a bound whose declared dimension
//!   differs from the parameter's declared dimension. **One per offending bound**
//!   (lower and/or upper), anchored on the parameter occurrence with the
//!   **`PROPERTY`** discriminator carrying the offending bound's §3.9 property
//!   position (lower = 4, upper = 5). PROPERTY is what §6.4.2 assigns check 4, and
//!   it is exactly what keeps two same-parameter dimension defects distinct.
//!
//! **Presence vs. validity.** Absence of the dimension / lower bound / upper bound
//! / adjustable flag is check 1's (§3.9 positions 3/4/5/6); check 4 never
//! re-reports it. Check 4 validates only the **meaning of sufficiently-present
//! bounds**: a predicate is evaluated only where its operands are determinate —
//! `lower > upper` only when both bounds are present NUMERIC literals, and a bound
//! dimension mismatch only when that bound is a present NUMERIC literal and the
//! parameter's dimension is present. A non-numeric or absent operand leaves the
//! predicate intrinsically undefined and yields no defect. This is intrinsic
//! predicate computability, **not** §5.2 prerequisite gating (no NotEvaluatedRecord,
//! no suppression). No numeric or dimensional algebra is reimplemented here — the
//! comparison uses the accepted exact `ExactNumber` order and exact
//! `DimensionSymbol` equality.
//!
//! Defect production only; deterministic.

use crate::model::config::ConfigurationParameter;
use crate::model::declared::Declared;
use crate::model::literal::Literal;
use crate::model::opaque::DimensionSymbol;
use crate::model::strategy::Strategy;
use crate::numeric::ExactNumber;
use crate::result::category::DefectCategory;
use crate::result::discriminator::{Discriminator, PropertyPosition};
use crate::result::identity::DefectCore;

/// The exact numeric value of a bound, if it is a present NUMERIC literal.
/// A `BOOLEAN`/`ENUM` bound has no value to order and no dimension to compare;
/// an absent bound is check 1's. Either way the check-4 predicate is undefined.
fn numeric_value(bound: &Declared<Literal>) -> Option<&ExactNumber> {
    match bound {
        Declared::Present(Literal::Numeric { value, .. }) => Some(value),
        Declared::Present(Literal::Boolean(_) | Literal::Enum(_)) | Declared::Absent => None,
    }
}

/// The declared dimension of a bound, if it is a present NUMERIC literal.
fn numeric_dimension(bound: &Declared<Literal>) -> Option<&DimensionSymbol> {
    match bound {
        Declared::Present(Literal::Numeric { dimension, .. }) => Some(dimension),
        Declared::Present(Literal::Boolean(_) | Literal::Enum(_)) | Declared::Absent => None,
    }
}

/// Check 4 for one configuration parameter.
fn check_parameter(parameter: &ConfigurationParameter) -> Vec<DefectCore> {
    let mut defects = Vec::new();

    // INVERTED_CONFIGURATION_BOUNDS — lower > upper by exact value. Computable
    // only when both bounds are present NUMERIC literals (§4.11 interval).
    if let (Some(lower), Some(upper)) = (
        numeric_value(&parameter.lower_bound),
        numeric_value(&parameter.upper_bound),
    ) && lower > upper
    {
        defects.push(DefectCore::new(
            parameter.occurrence,
            DefectCategory::InvertedConfigurationBounds,
            Discriminator::None,
        ));
    }

    // CONFIGURATION_BOUND_DIMENSION_MISMATCH — one per offending bound whose
    // dimension differs from the parameter's declared dimension. Computable only
    // when the parameter dimension and the bound's own dimension are present.
    if let Declared::Present(parameter_dimension) = &parameter.dimension {
        // §3.9 positions: lower bound = 4, upper bound = 5.
        for (bound, position) in [(&parameter.lower_bound, 4), (&parameter.upper_bound, 5)] {
            if let Some(bound_dimension) = numeric_dimension(bound)
                && bound_dimension != parameter_dimension
            {
                defects.push(DefectCore::new(
                    parameter.occurrence,
                    DefectCategory::ConfigurationBoundDimensionMismatch,
                    Discriminator::Property(PropertyPosition::new(position)),
                ));
            }
        }
    }

    defects
}

/// Check 4 over a whole Strategy. Returns a deterministic `Vec<DefectCore>`.
#[must_use]
pub fn check_4(strategy: &Strategy) -> Vec<DefectCore> {
    let mut defects = Vec::new();
    let Declared::Present(parameters) = &strategy.configuration_parameters else {
        return defects;
    };
    for parameter in parameters {
        defects.extend(check_parameter(parameter));
    }
    defects
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::opaque::{LineageId, OccurrenceId, OpaqueValue};

    fn occ(handle: u64) -> OccurrenceId {
        OccurrenceId::new(OpaqueValue::new(handle))
    }

    fn lineage(handle: u64) -> LineageId {
        LineageId::new(OpaqueValue::new(handle))
    }

    fn dim(handle: u64) -> DimensionSymbol {
        DimensionSymbol::new(OpaqueValue::new(handle))
    }

    /// A NUMERIC bound literal with the given value and dimension.
    fn bound(value: i64, dimension: u64) -> Literal {
        Literal::Numeric {
            value: ExactNumber::from_integer(value),
            dimension: dim(dimension),
        }
    }

    /// A parameter (dimension 1) with the given occurrence and bounds.
    fn parameter(
        occ_id: u64,
        lower: Declared<Literal>,
        upper: Declared<Literal>,
    ) -> ConfigurationParameter {
        ConfigurationParameter {
            occurrence: occ(occ_id),
            lineage: Declared::Present(lineage(occ_id)),
            dimension: Declared::Present(dim(1)),
            lower_bound: lower,
            upper_bound: upper,
            adjustable: Declared::Present(true),
        }
    }

    fn strategy_with(configuration_parameters: Declared<Vec<ConfigurationParameter>>) -> Strategy {
        Strategy {
            occurrence: occ(1),
            lineage: Declared::Present(lineage(1000)),
            semantic_model_version: Declared::Absent,
            evidence_declarations: Declared::Present(vec![]),
            configuration_parameters,
            conditions: Declared::Present(vec![]),
            direction_rules: Declared::Present(vec![]),
            candidate_derivations: Declared::Present(vec![]),
        }
    }

    fn categories(defects: &[DefectCore]) -> Vec<DefectCategory> {
        defects.iter().map(DefectCore::category).collect()
    }

    fn property_position(discriminator: &Discriminator) -> u32 {
        // PropertyPosition is opaque; bracket it via Ord.
        match discriminator {
            Discriminator::Property(p) => (0..16)
                .find(|c| *p == PropertyPosition::new(*c))
                .expect("position within range"),
            _ => panic!("expected a PROPERTY discriminator"),
        }
    }

    #[test]
    fn a_well_formed_interval_is_no_defect() {
        let defects = check_4(&strategy_with(Declared::Present(vec![parameter(
            10,
            Declared::Present(bound(1, 1)),
            Declared::Present(bound(10, 1)),
        )])));
        assert!(defects.is_empty());
    }

    #[test]
    fn an_equal_bound_pair_is_valid() {
        // §4.11: equality permitted — a pinned parameter is well-formed.
        let defects = check_4(&strategy_with(Declared::Present(vec![parameter(
            10,
            Declared::Present(bound(5, 1)),
            Declared::Present(bound(5, 1)),
        )])));
        assert!(defects.is_empty());
    }

    #[test]
    fn a_reversed_pair_is_inverted_bounds_on_the_parameter() {
        let defects = check_4(&strategy_with(Declared::Present(vec![parameter(
            10,
            Declared::Present(bound(10, 1)),
            Declared::Present(bound(1, 1)),
        )])));
        assert_eq!(
            categories(&defects),
            vec![DefectCategory::InvertedConfigurationBounds]
        );
        assert_eq!(defects[0].offending(), occ(10));
        assert!(matches!(defects[0].discriminator(), Discriminator::None));
    }

    #[test]
    fn a_lower_bound_dimension_mismatch_is_property_position_four() {
        // Lower bound dimension 2 ≠ parameter dimension 1.
        let defects = check_4(&strategy_with(Declared::Present(vec![parameter(
            10,
            Declared::Present(bound(1, 2)),
            Declared::Present(bound(10, 1)),
        )])));
        assert_eq!(
            categories(&defects),
            vec![DefectCategory::ConfigurationBoundDimensionMismatch]
        );
        assert_eq!(defects[0].offending(), occ(10));
        assert_eq!(property_position(defects[0].discriminator()), 4);
    }

    #[test]
    fn an_upper_bound_dimension_mismatch_is_property_position_five() {
        let defects = check_4(&strategy_with(Declared::Present(vec![parameter(
            10,
            Declared::Present(bound(1, 1)),
            Declared::Present(bound(10, 2)),
        )])));
        assert_eq!(
            categories(&defects),
            vec![DefectCategory::ConfigurationBoundDimensionMismatch]
        );
        assert_eq!(property_position(defects[0].discriminator()), 5);
    }

    #[test]
    fn both_bounds_mismatching_yield_two_distinct_dimension_defects() {
        // Both bounds dimension 2 ≠ parameter dimension 1 ⇒ two defects,
        // distinguished only by the PROPERTY position (4 then 5).
        let defects = check_4(&strategy_with(Declared::Present(vec![parameter(
            10,
            Declared::Present(bound(1, 2)),
            Declared::Present(bound(10, 2)),
        )])));
        assert_eq!(
            categories(&defects),
            vec![
                DefectCategory::ConfigurationBoundDimensionMismatch,
                DefectCategory::ConfigurationBoundDimensionMismatch,
            ]
        );
        assert_eq!(property_position(defects[0].discriminator()), 4);
        assert_eq!(property_position(defects[1].discriminator()), 5);
        // Distinct under §6.4 despite the shared occurrence and category.
        assert_ne!(defects[0].cmp(&defects[1]), core::cmp::Ordering::Equal);
    }

    #[test]
    fn inverted_and_dimension_mismatch_co_occur_per_violation() {
        // lower=10 (dim 2) > upper=1 (dim 1), parameter dim 1: the pair is
        // inverted AND the lower bound's dimension mismatches. Two independent
        // §4.11 conjuncts ⇒ both defects, category-ordered (7 before 8).
        let defects = check_4(&strategy_with(Declared::Present(vec![parameter(
            10,
            Declared::Present(bound(10, 2)),
            Declared::Present(bound(1, 1)),
        )])));
        assert_eq!(
            categories(&defects),
            vec![
                DefectCategory::InvertedConfigurationBounds,
                DefectCategory::ConfigurationBoundDimensionMismatch,
            ]
        );
        assert_eq!(property_position(defects[1].discriminator()), 4);
    }

    #[test]
    fn an_absent_bound_is_check_1s_not_re_reported() {
        // Missing upper bound is check 1's; check 4 cannot compare and stays
        // silent — it never re-reports the absence.
        let defects = check_4(&strategy_with(Declared::Present(vec![parameter(
            10,
            Declared::Present(bound(10, 1)),
            Declared::Absent,
        )])));
        assert!(defects.is_empty());
    }

    #[test]
    fn an_absent_parameter_dimension_suppresses_only_the_dimension_predicate() {
        // Parameter dimension absent (check 1's): the dimension predicate is not
        // computable, but the value order still is — an inverted pair is still
        // caught, and no dimension defect is manufactured.
        let param = ConfigurationParameter {
            occurrence: occ(10),
            lineage: Declared::Present(lineage(10)),
            dimension: Declared::Absent,
            lower_bound: Declared::Present(bound(10, 2)),
            upper_bound: Declared::Present(bound(1, 3)),
            adjustable: Declared::Present(true),
        };
        let defects = check_4(&strategy_with(Declared::Present(vec![param])));
        assert_eq!(
            categories(&defects),
            vec![DefectCategory::InvertedConfigurationBounds]
        );
    }

    #[test]
    fn a_non_numeric_bound_is_not_a_manufactured_defect() {
        // A BOOLEAN lower bound has no value to order and no dimension to compare;
        // the check-4 predicate is intrinsically undefined ⇒ no defect.
        let param = ConfigurationParameter {
            occurrence: occ(10),
            lineage: Declared::Present(lineage(10)),
            dimension: Declared::Present(dim(1)),
            lower_bound: Declared::Present(Literal::Boolean(true)),
            upper_bound: Declared::Present(bound(10, 1)),
            adjustable: Declared::Present(true),
        };
        assert!(check_4(&strategy_with(Declared::Present(vec![param]))).is_empty());
    }

    #[test]
    fn an_absent_configuration_collection_is_check_1s() {
        assert!(check_4(&strategy_with(Declared::Absent)).is_empty());
    }
}
