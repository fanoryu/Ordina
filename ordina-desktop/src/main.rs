//! N-22-B-2 — the desktop-local correction loop (PBA-0168).
//!
//! A thin binary over the `ordina_desktop` library: it builds a
//! [`CorrectionSession`] over developer-fixture data and hands it to the egui
//! shell. All correction logic lives in the library's pure modules.
//!
//! Scope: RFC-0004 §10/§11/§12/§14/§16/§17/§19 — canonical Strategy → editable
//! draft hydration, explicit Trader confirmation, deterministic draft →
//! Strategy translation with identity minting/preservation, and full fresh
//! Well-Formedness re-check integrated with the N-22-B-1 revision-identity API.
//! A **bounded** first Trader editing surface (semantic-model version +
//! conditions), not a complete generic authoring product. No persistence,
//! networking, accounts, or packaging (PBA-0168's explicit exclusions).

use ordina_desktop::app::App;
use ordina_desktop::correction::CorrectionSession;
use ordina_desktop::correction_fixtures;

fn main() -> eframe::Result<()> {
    // A Strategy that is Unusable for one correctable reason (an absent
    // semantic-model version), so the surface opens on a real correction task.
    let context = correction_fixtures::checker_context();
    let strategy = correction_fixtures::unusable_missing_version();
    let session = CorrectionSession::begin(context, strategy, correction_fixtures::MINT_BASE);

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Ordina — Strategy Correction (N-22-B-2)",
        native_options,
        Box::new(move |_cc| Ok(Box::new(App::new(session)))),
    )
}
