//! Ordina — Slice 1: the deterministic Strategy Well-Formedness checker.
//!
//! Behaviour is governed by the **First Runnable Slice Contract** (ACCEPTED),
//! which is authoritative. This crate realizes that contract under ADR-0005
//! (ACCEPTED); where the two could ever be read as differing, the contract
//! governs.
//!
//! **Slice 1 is built one authorized phase at a time.** The semantic
//! foundation is in place: the §3 construct model and the five opaque domains
//! (`model`), the exact dimension-free numeric value layer (`numeric`), the
//! dimension-aware equality, comparison and dimension primitives
//! (`semantics::comparison`, `semantics::dimension`), the §4.10 per-reference
//! constraint-domain algebra — contradiction, overlap and subsumption
//! (`semantics::constraint`), and the §6 result and defect model — the binary
//! verdict, deterministic defect identity, and the total canonical defect
//! ordering (`result`).
//!
//! On that foundation the first mandatory check now runs: `checks` implements
//! **check 1** (required-property completeness, `MISSING_REQUIRED_PROPERTY`) as
//! pure per-construct defect production over the reconciled model. It is **not**
//! a running checker: no other mandatory check, no prerequisite gating, no
//! Direction logic, no verdict, and no checker orchestration exists yet. The
//! authoritative, always-current phase status lives in
//! `docs/engineering/slice-1-implementation-plan.md`, not in this comment.

// ADR-0005 §7.7 — no `unsafe` is authorized or required for Slice 1.
#![forbid(unsafe_code)]
// ADR-0005 §7.2 — the primary control against floating point is structural: no
// authoritative semantic type or field is `f32`/`f64`, so a float has no type
// to inhabit. These lints are defense in depth on top of that, and they are
// only enforced when `cargo clippy` actually runs — `cargo test` alone does not
// establish compliance with the float policy.
#![deny(clippy::float_arithmetic)]
#![deny(clippy::float_cmp)]

pub mod checks;
pub mod model;
pub mod numeric;
pub mod persistence;
pub mod result;
pub mod semantics;
