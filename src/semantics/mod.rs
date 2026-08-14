//! Dimension-aware semantic relations — Slice Contract §4.3, §4.4, §4.6.
//!
//! This module is the home of the Slice-1 semantic relations that reason about
//! **dimensions and typed literals** (Phase 2B, F-1): equality (§4.3),
//! comparison (§4.4) and the dimensional rules (§4.6). It sits above the
//! dimension-free exact value layer (`numeric`, Phase 2A, closed) and the
//! construct model (`model`, Phase 1); the dependency direction is
//! `semantics → model → numeric`.
//!
//! These are **semantic primitives**, not checks. A later phase's mandatory
//! checks — check 2's typing, check 14's dimensional consistency — compose them.
//! Nothing here produces a defect, a verdict, or a canonical ordering, and
//! nothing here is a checker.
//!
//! Two contract properties shape the API:
//!
//! * **Outcomes are closed and named, never `bool` or `Ordering`.** §4.3 makes a
//!   cross-dimension equality a *type defect* rather than `false`, and §4.4
//!   leaves ordering *undefined* on some domains; a two-valued answer cannot
//!   carry that (see [`comparison`]). This is also why `Literal` implements none
//!   of Rust's equality or ordering traits (B-4).
//! * **`DIMENSIONLESS` is supplied canonical input, never manufactured.** §2.1 of
//!   the accepted contract makes the distinguished dimensionless symbol's
//!   identity part of the input; [`dimension::DimensionBinding`] carries it and
//!   is passed per invocation (B-1).
//!
//! §4.10 constraint-domain semantics (contradiction, overlap, subsumption) are
//! **Phase 3**, not implemented here.

pub mod comparison;
pub mod dimension;
