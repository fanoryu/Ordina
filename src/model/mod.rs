//! Admitted canonical construct inventory — Slice Contract §3.
//!
//! Phase 1 makes every §3 construct representable. It implements **no**
//! Well-Formedness check, no defect, and no evaluation: those arrive in later
//! phases of `docs/engineering/slice-1-implementation-plan.md`.
//!
//! # Representability is the exit criterion, in both directions
//!
//! Two obligations pull against each other here, and both are load-bearing.
//!
//! * A shape the contract assigns **explicit defect semantics** must be
//!   **constructible**, or the check that owns it has nothing to find and the
//!   checker cannot reach the verdict the contract requires. [`declared`] exists
//!   for exactly that: absent, multiple and out-of-domain declarations are named
//!   states, not accidents.
//! * A shape the contract does **not admit at all** — a grammar-level form such
//!   as `expr * expr`, or an operand where the grammar requires a literal — is
//!   made **unconstructible**, because no check owns it and nothing downstream
//!   would ever reject it.
//!
//! The dividing line is whether the Slice Contract names a defect condition for
//! the shape. Where it does, the model represents it; where it does not, the
//! type system forbids it. Neither obligation may be traded for the other.

pub mod condition;
pub mod config;
pub mod declared;
pub mod derivation;
pub mod direction;
pub mod evidence;
pub mod guard;
pub mod literal;
pub mod opaque;
pub mod strategy;
