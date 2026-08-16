//! The mandatory Well-Formedness checks — Slice Contract §5.
//!
//! Phase 5 implements the 15 applicable mandatory checks in dependency order;
//! Phase 6 adds per-unit-of-work prerequisite gating and `not-evaluated`
//! metadata (Slice Contract §5.2). Checks 7 and 17 are out of Slice-1 scope
//! because their constructs are not admitted.
//!
//! **Phase 5 is built one slice per authorization.** Each implemented check is
//! pure defect production over the reconciled Revision-4 model: it anchors each
//! defect on the offending construct's occurrence identity, reports **only** its
//! own accepted population, and returns a deterministic `Vec<DefectCore>`. No
//! prerequisite gating, not-evaluated record, verdict, or canonical sorting is
//! implemented yet (those are Phase 6).
//!
//! - [`required_property`] — **check 1** (`MISSING_REQUIRED_PROPERTY`).
//! - [`operative_provenance`] — **check 15** (`MISSING_OPERATIVE_PROVENANCE`).
//! - [`semantic_model_version`] — **check 16** semantic-model version half
//!   (`UNSUPPORTED_SEMANTIC_MODEL_VERSION`); `UNKNOWN_CONSTRUCT` is unrepresentable
//!   in the typed model and is not produced here.
//! - [`reference_integrity`] — **check 2** (`DUPLICATE_LINEAGE_IDENTITY`,
//!   `UNRESOLVED_REFERENCE`, `TYPE_MISMATCH`, `ENUM_VALUE_NOT_DECLARED`).
//! - [`direction_coverage`] — **check 8** (`DIRECTION_COVERAGE_INCOMPLETE`,
//!   `MULTIPLE_TOTAL_DIRECTION_RULES`, `TOTAL_RULE_NOT_LOWEST_PRECEDENCE`).
//! - [`direction_determinism`] — **check 10** (`INVALID_DIRECTION_OUTCOME`).
//! - [`condition_classification`] — **check 3** (`INVALID_CONDITION_CLASSIFICATION`).
//! - [`configuration_bounds`] — **check 4** (`INVERTED_CONFIGURATION_BOUNDS`,
//!   `CONFIGURATION_BOUND_DIMENSION_MISMATCH`).
//! - [`configuration_position`] — **check 5** (`CONFIGURATION_IN_STRUCTURAL_POSITION`).
//! - [`evidence_integrity`] — **check 6** (`INVALID_EVIDENCE_DECLARATION`).

pub mod condition_classification;
pub mod configuration_bounds;
pub mod configuration_position;
pub mod direction_coverage;
pub mod direction_determinism;
pub mod evidence_integrity;
pub mod operative_provenance;
pub mod reference_integrity;
pub mod required_property;
pub mod semantic_model_version;
