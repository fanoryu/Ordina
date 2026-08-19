//! Ordina desktop — the diagnosis surface (N-22-A) and the local correction
//! loop (N-22-B-2).
//!
//! The correction loop's logic is a pure, egui-free library — `draft`,
//! `hydrate`, `translate`, and `correction` — so RFC-0004 §16/§17/§19 behavior
//! is unit-testable without a running GUI (mirroring how `view_model` already
//! isolates N-22-A's presentation logic from `diagnosis_view`'s rendering). The
//! `app`/`correction_view`/`diagnosis_view` modules are the thin egui shell over
//! that library; `correction_fixtures` supplies developer sample data for the
//! binary and the tests.
//!
//! Scope is RFC-0003 §9 (diagnosis) and RFC-0004's authoring/correction contract
//! only, under PBA-0156 / PBA-0168 — no persistence, networking, accounts, or
//! packaging.

pub mod app;
pub mod correction;
pub mod correction_fixtures;
pub mod correction_view;
pub mod diagnosis_view;
pub mod draft;
pub mod hydrate;
pub mod translate;
pub mod view_model;
