//! Admitted canonical construct inventory — Slice Contract §3.
//!
//! Phase 1 makes every §3 construct representable. It implements **no**
//! Well-Formedness check, no defect, and no evaluation: those arrive in later
//! phases of `docs/engineering/slice-1-implementation-plan.md`.

pub mod condition;
pub mod config;
pub mod derivation;
pub mod direction;
pub mod evidence;
pub mod guard;
pub mod literal;
pub mod opaque;
pub mod strategy;
