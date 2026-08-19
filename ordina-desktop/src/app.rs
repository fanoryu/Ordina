//! N-22-B-2 top-level application state — the desktop correction loop
//! (PBA-0168).
//!
//! Extends N-22-A's diagnosis-only surface into RFC-0004's full local
//! correction workflow: a Trader is shown a diagnosis and may edit, confirm,
//! submit, and re-check the Strategy (§16/§17/§19). The correction logic itself
//! lives entirely in the pure `correction`/`draft`/`hydrate`/`translate`
//! modules; this struct only owns the [`CorrectionSession`] across frames and
//! delegates rendering to `correction_view` (which reuses N-22-A's
//! `diagnosis_view` unchanged for the diagnosis portion).

use eframe::egui;

use crate::correction::CorrectionSession;

pub struct App {
    session: CorrectionSession,
    /// The most recent Trader-facing status line (last action's outcome).
    last_message: String,
}

impl App {
    #[must_use]
    pub fn new(session: CorrectionSession) -> Self {
        Self {
            session,
            last_message: String::new(),
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        crate::correction_view::show(ui, &mut self.session, &mut self.last_message);
    }
}
