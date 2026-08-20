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

/// Resolve the initial [`CorrectionSession`] from an already-parsed optional
/// envelope path — the pure startup decision, separated from `std::env::args()`
/// so it is directly testable (the PBA-0183 regression-coverage remediation of
/// the startup branch, especially the guarantee that a supplied but unusable
/// envelope surfaces an error and never silently falls back to the fixture).
/// CLI parsing stays in [`run`]; the branch semantics here are unchanged.
fn resolve_startup(path: Option<PathBuf>) -> Result<(CorrectionSession, Option<PathBuf>), String> {
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

fn run() -> Result<(CorrectionSession, Option<PathBuf>), String> {
    let path: Option<PathBuf> = std::env::args().nth(1).map(PathBuf::from);
    resolve_startup(path)
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

#[cfg(test)]
mod startup_tests {
    //! Regression coverage for the startup-resolution branch (PBA-0183). These
    //! exercise the pure [`resolve_startup`] seam directly — no process-global
    //! `std::env::args()` — so the load / first-run / no-fixture-fallback
    //! guarantees are automatically verified. Behaviour is unchanged; only the
    //! testable seam is new.

    use super::{fixture_session, resolve_startup};
    use std::fs;
    use std::path::{Path, PathBuf};

    use ordina::persistence::{self, DogfoodEnvelope, FORMAT_VERSION, MintHighWater};
    use ordina_desktop::correction::CorrectionSession;
    use ordina_desktop::{correction_fixtures, persist};

    fn temp_path(name: &str) -> PathBuf {
        let mut p = std::env::temp_dir();
        p.push(format!("ordina-startup-{}-{name}.json", std::process::id()));
        p
    }

    /// A genuine, loadable envelope whose Strategy is **Well-Formed** (semantic
    /// model version *present*) — deliberately distinct from the developer
    /// fixture (version *absent*), so a fixture fallback is detectable.
    fn save_valid_present_version_envelope(path: &Path) {
        let context = correction_fixtures::checker_context();
        let strategy = correction_fixtures::clean_strategy();
        assert!(
            strategy.semantic_model_version.is_present(),
            "test premise: the saved envelope's version is present"
        );
        let session = CorrectionSession::begin(context, strategy, correction_fixtures::MINT_BASE);
        persist::save_session(&session, path).expect("atomic save of a valid envelope");
    }

    /// 1. An existing valid envelope loads and resumes the persisted session, the
    ///    supplied path is retained, and there is no fixture fallback.
    #[test]
    fn existing_valid_envelope_loads_and_resumes_without_fixture_fallback() {
        let path = temp_path("valid");
        let _ = fs::remove_file(&path);
        save_valid_present_version_envelope(&path);

        let (session, retained) =
            resolve_startup(Some(path.clone())).expect("valid envelope loads");

        // The persisted (Well-Formed) Strategy was resumed, distinguishable from
        // the developer fixture, which is Unusable (version absent).
        assert!(
            session.strategy().semantic_model_version.is_present(),
            "resumed Strategy must be the persisted one, not the version-absent fixture"
        );
        assert!(
            fixture_session()
                .strategy()
                .semantic_model_version
                .is_absent(),
            "premise: the developer fixture is version-absent, so fallback would be visible"
        );
        assert_eq!(retained, Some(path.clone()), "supplied path is retained");

        let _ = fs::remove_file(&path);
    }

    /// 2. An absent supplied path uses the existing first-run seed, retains the
    ///    path for the first Save, and does not itself create/write the envelope.
    #[test]
    fn absent_path_uses_first_run_seed_and_does_not_write() {
        let path = temp_path("absent");
        let _ = fs::remove_file(&path);
        assert!(!path.exists(), "premise: the supplied path does not exist");

        let (session, retained) = resolve_startup(Some(path.clone())).expect("first-run seed");

        // The first-run seed is the developer fixture (version absent).
        assert!(
            session.strategy().semantic_model_version.is_absent(),
            "absent path must seed the first-run developer fixture"
        );
        assert_eq!(
            retained,
            Some(path.clone()),
            "path retained for the first Save"
        );
        assert!(
            !path.exists(),
            "startup resolution must not create or write the envelope"
        );
    }

    /// 3. A malformed existing envelope returns `Err`, never falls back to the
    ///    fixture, and leaves the malformed file byte-identical and unmodified.
    #[test]
    fn malformed_envelope_errors_without_fallback_and_preserves_file() {
        let path = temp_path("malformed");
        let malformed = b"{ this is not a valid dogfood envelope ";
        fs::write(&path, malformed).expect("seed a malformed file");
        let before = fs::read(&path).unwrap();

        let result = resolve_startup(Some(path.clone()));
        assert!(
            result.is_err(),
            "a malformed supplied envelope must surface an error, never a fixture session"
        );

        let after = fs::read(&path).unwrap();
        assert_eq!(
            after, before,
            "the malformed file must be left byte-unchanged"
        );
        assert_eq!(after, malformed, "the malformed file must be unmodified");

        let _ = fs::remove_file(&path);
    }

    /// Trivial-after-extraction: no CLI path → developer fixture / dev mode, with
    /// no persistence path threaded.
    #[test]
    fn no_path_is_developer_fixture_dev_mode() {
        let (session, retained) = resolve_startup(None).expect("dev-mode fixture");
        assert!(
            session.strategy().semantic_model_version.is_absent(),
            "no-path dev mode is the developer fixture"
        );
        assert_eq!(retained, None, "dev mode threads no persistence path");
    }

    /// Trivial-after-extraction: an envelope with an unsupported format version
    /// returns `Err` (no fixture fallback), and the file is left unmodified.
    #[test]
    fn unsupported_format_version_errors_without_fallback() {
        let path = temp_path("badversion");
        let context = correction_fixtures::checker_context();
        let envelope = DogfoodEnvelope {
            format_version: FORMAT_VERSION + 1,
            binding: context.binding,
            supported: context.supported,
            strategy: Some(correction_fixtures::clean_strategy()),
            mint_high_water: MintHighWater::Next(correction_fixtures::MINT_BASE),
        };
        let bytes = persistence::serialize_envelope(&envelope);
        fs::write(&path, &bytes).unwrap();
        let before = fs::read(&path).unwrap();

        assert!(
            resolve_startup(Some(path.clone())).is_err(),
            "an unsupported format version must surface an error"
        );
        assert_eq!(
            fs::read(&path).unwrap(),
            before,
            "the file must be unmodified"
        );

        let _ = fs::remove_file(&path);
    }

    /// Trivial-after-extraction: a decodable envelope carrying no Strategy returns
    /// `Err` ("no Strategy to resume"), never a fixture fallback.
    #[test]
    fn envelope_without_strategy_errors_without_fallback() {
        let path = temp_path("nostrategy");
        let context = correction_fixtures::checker_context();
        let envelope = DogfoodEnvelope {
            format_version: FORMAT_VERSION,
            binding: context.binding,
            supported: context.supported,
            strategy: None,
            mint_high_water: MintHighWater::Next(correction_fixtures::MINT_BASE),
        };
        let bytes = persistence::serialize_envelope(&envelope);
        fs::write(&path, &bytes).unwrap();

        assert!(
            resolve_startup(Some(path.clone())).is_err(),
            "an envelope with no Strategy must surface an error, not a fixture session"
        );

        let _ = fs::remove_file(&path);
    }
}
