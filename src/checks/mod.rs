//! The mandatory Well-Formedness checks — Slice Contract §5.
//!
//! Phase 5 implements the 15 applicable mandatory checks in dependency order;
//! Phase 6 adds per-unit-of-work prerequisite gating and `not-evaluated`
//! metadata (Slice Contract §5.2). Checks 7 and 17 are out of Slice-1 scope
//! because their constructs are not admitted.
//!
//! Phase 0: intentionally empty.
