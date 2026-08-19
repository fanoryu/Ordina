//! N-22-A — diagnosis-only desktop presentation surface.
//!
//! Scope: RFC-0003 (ACCEPTED, PBR-0018) §9.1-9.6, §9.11 only, under PBA-0156.
//! No authoring/editing surface, no persistence, no networking — N-22-B
//! (the correction workflow) remains BLOCKED ON N-25 and is not implemented
//! here. The result shown is developer-fixture data (`fixtures`), not
//! Trader-authored content — see `fixtures.rs` for why.

mod app;
mod diagnosis_view;
mod fixtures;
mod view_model;

fn main() -> eframe::Result<()> {
    let result = fixtures::sample_unusable();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Ordina — Well-Formedness Diagnosis (N-22-A)",
        native_options,
        Box::new(move |_cc| Ok(Box::new(app::App::new(&result)))),
    )
}
