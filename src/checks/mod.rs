//! The mandatory Well-Formedness checks — Slice Contract §5.
//!
//! Phase 5 implements the 15 applicable mandatory checks in dependency order;
//! Phase 6 adds per-unit-of-work prerequisite gating and `not-evaluated`
//! metadata (Slice Contract §5.2). Checks 7 and 17 are out of Slice-1 scope
//! because their constructs are not admitted.
//!
//! **Phase 5A — check 1 only.** [`required_property`] implements check 1
//! (`MISSING_REQUIRED_PROPERTY`) as pure defect production over the reconciled
//! Revision-4 model: it anchors each defect on the offending construct's
//! occurrence identity and reports **only** the check-1-owned unconditional
//! properties. No other mandatory check, no prerequisite gating, no verdict, and
//! no canonical sorting is implemented yet.

pub mod required_property;
