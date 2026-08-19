//! N-22-A top-level application state — diagnosis-only (PBA-0156).
//!
//! There is exactly one state in N-22-A's scope: a Trader is shown the
//! diagnosis for one already-computed `WellFormednessResult`. A "re-check"
//! or "awaiting re-check" state is deliberately **not** modeled here — RFC-
//! 0003 §9.7 stage 3's re-submission only makes sense once an edit surface
//! exists to re-submit *from*, and that surface is N-22-B, blocked on N-25
//! (see PBA-0156's explicit exclusions). Adding an unreachable state now
//! would be inventing behavior this authorization does not cover.

use eframe::egui;
use ordina::result::WellFormednessResult;

use crate::view_model::DiagnosisViewModel;

pub struct App {
    view_model: DiagnosisViewModel,
}

impl App {
    #[must_use]
    pub fn new(result: &WellFormednessResult) -> Self {
        Self {
            view_model: DiagnosisViewModel::from_result(result),
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        crate::diagnosis_view::show(ui, &self.view_model);
    }
}
