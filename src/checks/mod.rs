//! The mandatory Well-Formedness checks — Slice Contract §5.
//! !
//! Phase 5 implements the 15 applicable mandatory checks in dependency order;
//! Phase 6 adds per-unit-of-work prerequisite gating and `not-evaluated`
//! metadata (Slice Contract §5.2). Checks 7 and 17 are out of Slice-1 scope
//! because their constructs are not admitted.
//! !
//! **Phase 5 is built one slice per authorization.** Each implemented check is
//! pure defect production over the reconciled Revision-4 model: it anchors each
//! defect on the offending construct's occurrence identity, reports **only** its
//! own accepted population, and returns a deterministic `Vec<DefectCore>`. No
//! prerequisite gating, not-evaluated record, verdict, or canonical sorting is
//! implemented yet (those are Phase 6).
//! !
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
//! - [`guard_analysis`] — shared §4.10 guard-analysis helper, consumed directly
//!   by checks 9, 11, and 12 only.
//! - [`direction_conflict`] — **check 9** (`DIRECTION_PRECEDENCE_TIE`).
//! - [`unreachable_rules`] — **check 11** (`UNREACHABLE_CONTRADICTORY_GUARD`,
//!   `UNREACHABLE_SUBSUMED_RULE`).
//! - [`contradictory_rules`] — **check 12** (`CONTRADICTORY_CONDITION_GUARD`).
//! - [`derivation_form`] — **check 13A** (`INVALID_DERIVATION_FORM`); its runtime
//!   population is structurally zero under the closed `DerivationExpression` model
//!   (PBA-0126 / ER-0104), and the check is realized explicitly and exhaustively so
//!   a future grammar extension is forced through its match.
//! - [`candidate_derivation_presence`] — **check 13B** (`MISSING_CANDIDATE_DERIVATION`,
//!   `DUPLICATE_CANDIDATE_DERIVATION`); Strategy-level candidate-role completeness and
//!   uniqueness — counts occurrences of each closed [`CandidateRole`] and emits one
//!   defect per missing or duplicated role, anchored on the envelope occurrence
//!   identity with the ROLE discriminator (§6.4.2 kind 3). One check per increment.

pub mod candidate_derivation_presence;
pub mod condition_classification;
pub mod configuration_bounds;
pub mod configuration_position;
pub mod contradictory_rules;
pub mod derivation_form;
pub mod direction_conflict;
pub mod direction_coverage;
pub mod direction_determinism;
pub mod evidence_integrity;
pub mod guard_analysis;
pub mod operative_provenance;
pub mod reference_integrity;
pub mod required_property;
pub mod semantic_model_version;
pub mod unreachable_rules;
