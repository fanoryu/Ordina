//! Dogfood-0 local-file persistence integration — the minimum `ordina-desktop`
//! surface over the accepted `ordina::persistence` capability (ADR-0009 §10/§11/
//! §14; implemented under PBA-0181).
//!
//! `ordina` owns serialization; this module owns only **file I/O**, treating the
//! serialized envelope as an opaque byte blob. Saving is atomic and non-torn: the
//! full envelope is written to a temporary file in the **same directory**,
//! flushed to durable storage (`sync_all`), then atomically renamed over the
//! target — so after any interrupted save the next launch observes either the
//! complete previous envelope or the complete new one, never a torn mixture
//! (ADR-0009 §10). The envelope path is caller-supplied (OQ-3 stays minimal): no
//! global location is hard-coded here.
//!
//! Windows: `std::fs::rename` replaces an existing destination; the exact
//! mechanism is std/OS/version dependent (ADR-0009 §11), and the non-torn
//! invariant is what this relies on, verified by the tests below on the host.

use std::fs;
use std::io::{self, Write};
use std::path::Path;

use ordina::persistence::{
    self, DogfoodEnvelope, EnvelopeLoadError, FORMAT_VERSION, MintHighWater,
};

use crate::correction::{CheckerContext, CorrectionSession};

/// Why loading a persisted envelope from a local file failed.
#[derive(Debug)]
pub enum LoadError {
    /// The file could not be read (missing, permissions, …). A missing file is
    /// the caller's cue to treat this as a first run (ADR-0009 §13), so callers
    /// typically match `NotFound` before treating other I/O errors as fatal.
    Io(io::Error),
    /// The bytes were present but not a usable envelope (ADR-0009 §13).
    Envelope(EnvelopeLoadError),
}

impl From<EnvelopeLoadError> for LoadError {
    fn from(e: EnvelopeLoadError) -> Self {
        LoadError::Envelope(e)
    }
}

/// Build the persisted envelope for a session's current committed state
/// (ADR-0009 §2): its checker context, its Strategy, and its mint high-water.
#[must_use]
pub fn envelope_of(session: &CorrectionSession) -> DogfoodEnvelope {
    let context = session.context();
    DogfoodEnvelope {
        format_version: FORMAT_VERSION,
        binding: context.binding,
        supported: context.supported,
        strategy: Some(session.strategy().clone()),
        mint_high_water: session.high_water(),
    }
}

/// Atomically save a session's committed state to `path` (ADR-0009 §10/§11).
///
/// Same-directory temp file → `sync_all` → atomic replace-existing rename. The
/// target is left complete-old or complete-new, never torn.
///
/// # Errors
///
/// Propagates the underlying [`io::Error`] if the temp write, flush, or rename
/// fails; on failure the prior envelope at `path` is left intact.
pub fn save_session(session: &CorrectionSession, path: &Path) -> io::Result<()> {
    let bytes = persistence::serialize_envelope(&envelope_of(session));
    atomic_write(path, &bytes)
}

/// Write `bytes` to `path` atomically: temp file in the same directory, flushed
/// and synced, then renamed over the target (ADR-0009 §10).
///
/// # Errors
///
/// Propagates any I/O error; the target is never left torn.
pub fn atomic_write(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let dir = path.parent().filter(|p| !p.as_os_str().is_empty());
    let file_name = path.file_name().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "envelope path has no file name",
        )
    })?;

    // A temp name in the SAME directory, so the rename is a same-volume metadata
    // operation (atomic replace), not a cross-volume copy.
    let mut tmp_name = std::ffi::OsString::from(".");
    tmp_name.push(file_name);
    tmp_name.push(".tmp");
    let tmp_path = match dir {
        Some(d) => d.join(&tmp_name),
        None => Path::new(&tmp_name).to_path_buf(),
    };

    {
        let mut tmp = fs::File::create(&tmp_path)?;
        tmp.write_all(bytes)?;
        tmp.flush()?;
        // Flush file contents to durable storage BEFORE the replace (§10) so the
        // new bytes are on disk before the target is repointed.
        tmp.sync_all()?;
    }

    // Atomic replace-existing rename (§10/§11). On Windows this replaces an
    // existing destination; the non-torn property is what we rely on.
    match fs::rename(&tmp_path, path) {
        Ok(()) => Ok(()),
        Err(e) => {
            // Best-effort cleanup of the orphan temp; the target is untouched.
            let _ = fs::remove_file(&tmp_path);
            Err(e)
        }
    }
}

/// Load an envelope from `path` (ADR-0009 §13). A successful decode does **not**
/// imply Well-Formedness — the caller runs the checker (via [`resume_session`]).
///
/// # Errors
///
/// [`LoadError::Io`] if the file cannot be read (match `NotFound` for first-run),
/// or [`LoadError::Envelope`] if the bytes are not a usable envelope.
pub fn load_envelope(path: &Path) -> Result<DogfoodEnvelope, LoadError> {
    let bytes = fs::read(path).map_err(LoadError::Io)?;
    Ok(persistence::deserialize_envelope(&bytes)?)
}

/// Resume a correction session from a loaded envelope (ADR-0009 §15 steps 3–6).
///
/// Restores the checker context and Strategy and seeds the minter from the
/// persisted high-water. Returns `None` for an envelope with no Strategy (the
/// caller establishes first-run inputs instead). An exhausted corpus still
/// resumes for diagnosis; the minter refuses further minting.
#[must_use]
pub fn resume_session(envelope: DogfoodEnvelope) -> Option<CorrectionSession> {
    let strategy = envelope.strategy?;
    let context = CheckerContext {
        binding: envelope.binding,
        supported: envelope.supported,
    };
    Some(CorrectionSession::resume(
        context,
        strategy,
        envelope.mint_high_water,
    ))
}

/// Whether a loaded high-water still permits minting (i.e. is not exhausted).
#[must_use]
pub fn can_mint(high_water: MintHighWater) -> bool {
    high_water.seed().is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::correction_fixtures;
    use ordina::result::compute_revision_identity;

    fn temp_path(name: &str) -> std::path::PathBuf {
        let mut p = std::env::temp_dir();
        p.push(format!("ordina-dogfood-{}-{name}.json", std::process::id()));
        p
    }

    #[test]
    fn save_load_resume_round_trip_preserves_identity_and_high_water() {
        let context = correction_fixtures::checker_context();
        let strategy = correction_fixtures::unusable_missing_version();
        let session = CorrectionSession::begin(context, strategy, correction_fixtures::MINT_BASE);

        let before = compute_revision_identity(session.strategy());

        let path = temp_path("roundtrip");
        save_session(&session, &path).expect("atomic save");
        let envelope = load_envelope(&path).expect("load");
        let resumed = resume_session(envelope).expect("has strategy");

        let after = compute_revision_identity(resumed.strategy());
        assert_eq!(
            before.is_ok(),
            after.is_ok(),
            "well-formedness class changed across persistence"
        );
        if let (Ok(a), Ok(b)) = (before, after) {
            assert_eq!(
                a, b,
                "recomputed RevisionIdentity changed across persistence"
            );
        }
        // High-water continuity: the resumed minter seeds at the same base.
        assert_eq!(resumed.high_water(), session.high_water());

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn atomic_save_replaces_existing_and_is_not_torn() {
        let path = temp_path("replace");
        atomic_write(&path, b"{\"first\":true}").unwrap();
        atomic_write(&path, b"{\"second\":true}").unwrap();
        let back = fs::read(&path).unwrap();
        // Complete-new, never a torn mixture of old and new.
        assert_eq!(back, b"{\"second\":true}");
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn missing_file_is_an_io_error_for_first_run() {
        let path = temp_path("does-not-exist");
        let _ = fs::remove_file(&path);
        match load_envelope(&path) {
            Err(LoadError::Io(e)) => assert_eq!(e.kind(), io::ErrorKind::NotFound),
            other => panic!("expected NotFound Io error, got {other:?}"),
        }
    }

    /// MINOR-1 (ADR-0009 §16 "interrupted save preserves the previous valid
    /// envelope"): a save that fails **before** the atomic replace must leave the
    /// prior destination byte-unchanged and still loadable — never torn.
    ///
    /// Deterministic failure injection without weakening production logic: the
    /// temp path `atomic_write` uses is `<dir>/.<name>.tmp`; pre-creating a
    /// **directory** at exactly that path makes the temp `File::create` fail, so
    /// the replace never runs and the target is untouched. No production seam is
    /// added — the real `atomic_write` is exercised end to end.
    #[test]
    fn interrupted_save_preserves_the_previous_valid_envelope() {
        // A private working directory so the temp-name collision is isolated.
        let dir = {
            let mut d = std::env::temp_dir();
            d.push(format!("ordina-dogfood-{}-interrupted", std::process::id()));
            d
        };
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let target = dir.join("envelope.json");

        // 1. A valid initial destination envelope (a real serialized envelope).
        let context = correction_fixtures::checker_context();
        let strategy = correction_fixtures::unusable_missing_version();
        let session = CorrectionSession::begin(context, strategy, correction_fixtures::MINT_BASE);
        save_session(&session, &target).expect("initial save");
        let original = fs::read(&target).unwrap();
        // It is a genuinely loadable envelope.
        assert!(load_envelope(&target).is_ok());

        // 2. Force the next save's temp-file creation to fail: occupy the exact
        //    temp path (`.envelope.json.tmp`) with a directory.
        let tmp_collision = dir.join(".envelope.json.tmp");
        fs::create_dir(&tmp_collision).unwrap();

        // 3. The save must return Err (temp create fails before any replace).
        let err = atomic_write(&target, b"{\"torn\":true}").unwrap_err();
        assert!(
            matches!(
                err.kind(),
                io::ErrorKind::AlreadyExists
                    | io::ErrorKind::PermissionDenied
                    | io::ErrorKind::Other
            ) || err.raw_os_error().is_some(),
            "expected a create failure, got {err:?}"
        );

        // 4-6. The prior destination is byte-unchanged, still loads, not torn.
        let after = fs::read(&target).unwrap();
        assert_eq!(
            after, original,
            "prior envelope was mutated by a failed save"
        );
        assert!(
            load_envelope(&target).is_ok(),
            "prior envelope no longer loads after a failed save"
        );

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn exhausted_session_refuses_minting_without_panic() {
        // A resumed corpus at the exhaustion boundary must report it cannot mint;
        // the UI gate (correction_view) keys off exactly this to refuse editing.
        assert!(!can_mint(MintHighWater::Exhausted));
        assert!(can_mint(MintHighWater::Next(0)));
        assert!(can_mint(MintHighWater::Next(u64::MAX)));
    }

    #[test]
    fn save_advances_and_reload_preserves_high_water_continuity() {
        let context = correction_fixtures::checker_context();
        let strategy = correction_fixtures::unusable_missing_version();
        let session = CorrectionSession::begin(context, strategy, correction_fixtures::MINT_BASE);
        let path = temp_path("continuity");

        save_session(&session, &path).unwrap();
        let reloaded = resume_session(load_envelope(&path).unwrap()).unwrap();
        // The persisted high-water seeds the resumed session identically — no
        // hidden in-memory state is required across the save/reload boundary.
        assert_eq!(reloaded.high_water(), session.high_water());
        assert!(can_mint(reloaded.high_water()));

        let _ = fs::remove_file(&path);
    }
}
