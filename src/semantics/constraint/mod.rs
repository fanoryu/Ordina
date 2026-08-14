//! Per-reference constraint domains — Slice Contract §4.10.
//!
//! §4.10 builds overlap, contradiction and subsumption on **one** construction:
//! the *constraint domain* `D(G, e)` of a guard `G` for a declared evidence item
//! `e` — the set of values of `e` the atoms of `G` referencing `e` permit. This
//! module provides that construction and the relations over it, **per
//! reference**, for each admitted evidence type:
//!
//! * `NUMERIC` → [`numeric::NumericDomain`] — an interval (open/closed/point)
//!   minus a finite set of excluded points;
//! * `BOOLEAN` → [`boolean::BooleanDomain`] — a subset of `{true, false}`;
//! * `ENUM` → [`enumerated::EnumDomain`] — a subset of the declared value set.
//!
//! Because a guard is a conjunction of **single-reference** atoms, the states it
//! admits are the product of its per-reference domains (§4.10). That product
//! structure is why the three relations are decidable **per reference**, and why
//! this module needs no solver, prover, decision procedure, or constraint
//! library — every operation is an ordinary comparison of declared literals
//! (§4.10 numeric-domain note). The guard-level relations (overlap for *every*
//! `e`, subsumption for *every* `e`) quantify these per-reference primitives over
//! a Strategy's references; that quantification, which needs the declaration
//! context, belongs to the mandatory checks (checks 9, 11, 12), not here.
//!
//! # The three relations, per reference
//!
//! * **Contradiction** — the domain is **empty** (`is_empty`). A contradictory
//!   guard has some reference whose domain is empty (§4.10).
//! * **Overlap** — two domains **intersect** to a non-empty domain (`overlaps`).
//!   Guards overlap iff every reference's domains overlap.
//! * **Subsumption** — one domain **contains** the other (`contains`). `G2` is
//!   subsumed by `G1` iff `D(G2, e) ⊆ D(G1, e)` for every `e`.
//!
//! # Scope
//!
//! These are semantic primitives, not checks: nothing here produces a defect,
//! reads a Strategy, or decides which check owns a result. Numeric values are
//! the exact, dimension-free Phase-2A [`ExactNumber`]; a numeric constraint
//! domain is per-reference, so all its atoms share one declared dimension and
//! the interval algebra compares values alone.
//!
//! [`ExactNumber`]: crate::numeric::ExactNumber

pub mod boolean;
pub mod enumerated;
pub mod numeric;

/// The two operators §3.6 admits for `BOOLEAN` and `ENUM` referents.
///
/// A closed category (ADR-0005 §7.4). `BOOLEAN`/`ENUM` atoms admit only equality
/// and inequality (§3.6); the ordering operators are a §4.1 type defect and are
/// caught by check 2 before any constraint domain is built. The boolean and enum
/// builders therefore take this two-variant operator, so an ill-typed operator
/// has no representation to enter through.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EqualityOperator {
    Equal,
    NotEqual,
}
