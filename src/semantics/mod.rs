//! Constraint-domain semantics — Slice Contract §4.10.
//!
//! Phase 3 introduces the per-reference constraint domain of a guard and the
//! three relations built on it: contradiction, overlap and subsumption. These
//! are decided by comparison of declared literals — no solver, prover, or
//! constraint library is involved (ADR-0005 §6.2).
//!
//! Phase 0: intentionally empty.
