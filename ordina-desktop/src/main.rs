//! N-22-B-2 — the desktop-local correction loop (PBA-0168), with Dogfood-0
//! local persistence wired in (PBA-0182).
//!
//! A thin binary over the `ordina_desktop` library. It resolves the initial
//! [`CorrectionSession`] from a **caller-supplied local envelope path** (the
//! first command-line argument — OQ-3 stays minimal, no file picker):
//!
//! - **path given, file exists** → load and resume the persisted envelope
//!   through `ordina::persistence` (never falling back to the developer fixture;
//!   a load failure is surfaced and aborts, per PBA-0182 §3/§8);
//! - **path given, file absent** → first-run seed from the existing developer
//!   fixture/context (PBA-0182 §4), saved to that path on the first save;
//! - **no path** → developer/dev-mode fixture only, no persistence (unchanged
//!   N-22-B-2 behaviour).
//!
//! All correction and persistence logic lives in the library's pure modules;
//! this binary only resolves startup state and hands the session to the shell.

use std::path::PathBuf;
use std::process::ExitCode;

use ordina_desktop::app::App;
use ordina_desktop::correction::CorrectionSession;
use ordina_desktop::{correction_fixtures, persist};

/// A first-run / dev-mode session seeded from the developer fixture (PBA-0182
/// §4): a Strategy that is Unusable for one correctable reason, so the surface
/// opens on a real correction task.
fn fixture_session() -> CorrectionSession {
    let context = correction_fixtures::checker_context();
    let strategy = correction_fixtures::unusable_missing_version();
    CorrectionSession::begin(context, strategy, correction_fixtures::MINT_BASE)
}

fn run() -> Result<(CorrectionSession, Option<PathBuf>), String> {
    let path: Option<PathBuf> = std::env::args().nth(1).map(PathBuf::from);
    match path {
        Some(p) if p.exists() => {
            // A real persisted envelope: load it, never substitute the fixture.
            let envelope = persist::load_envelope(&p)
                .map_err(|e| format!("failed to load envelope at {}: {e:?}", p.display()))?;
            let session = persist::resume_session(envelope)
                .ok_or_else(|| format!("envelope at {} has no Strategy to resume", p.display()))?;
            Ok((session, Some(p)))
        }
        Some(p) => {
            // First run at a new path: seed from the developer fixture; the real
            // envelope is written to `p` on the first save.
            Ok((fixture_session(), Some(p)))
        }
        None => {
            // Dev mode: no persistence path, fixture only.
            Ok((fixture_session(), None))
        }
    }
}

fn main() -> ExitCode {
    let (session, envelope_path) = match run() {
        Ok(v) => v,
        Err(message) => {
            // Surface the load failure clearly; do NOT fall back to fixture data
            // when a real path was supplied (PBA-0182 §8).
            eprintln!("Ordina: {message}");
            return ExitCode::from(2);
        }
    };

    let native_options = eframe::NativeOptions::default();
    let result = eframe::run_native(
        "Ordina — Strategy Correction (N-22-B-2)",
        native_options,
        Box::new(move |_cc| Ok(Box::new(App::new(session, envelope_path)))),
    );
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Ordina: {e}");
            ExitCode::FAILURE
        }
    }
}
