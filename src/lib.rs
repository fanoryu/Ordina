//! Ordina — Slice 1: the deterministic Strategy Well-Formedness checker.
//!
//! Behaviour is governed by the **First Runnable Slice Contract** (ACCEPTED),
//! which is authoritative. This crate realizes that contract under ADR-0005
//! (ACCEPTED); where the two could ever be read as differing, the contract
//! governs.
//!
//! **Phase 0 — bootstrap only.** No mandatory Well-Formedness check, defect
//! semantics, canonical ordering, constraint-domain algebra, Direction logic,
//! prerequisite gating, or verdict logic is implemented yet. The modules below
//! exist to fix the architecture boundaries; they are filled in by later
//! phases of `docs/engineering/slice-1-implementation-plan.md`.

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
pub mod result;
pub mod semantics;
