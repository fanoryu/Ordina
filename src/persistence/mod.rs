//! Dogfood-0 local persistence — the `ordina`-owned `DogfoodEnvelope ⇄ bytes`
//! capability (RFC-0005, ADR-0009; implemented under PBA-0181).
//!
//! `ordina` is the sole owner of Dogfood persistence serialization because only
//! it can read and reconstruct opaque handles without breaching the opacity
//! boundary (ADR-0005 §6.3 / the Slice Contract). This module exposes a narrow,
//! total-serialize / fallible-deserialize capability over one logical
//! [`DogfoodEnvelope`]; `ordina-desktop` performs all file I/O and treats the
//! bytes as an opaque blob. The bytes are a **transport/restore** form, never a
//! revision-identity canonicalization — revision identity is recomputed on load
//! via ADR-0008 (`compute_revision_identity`), never read from storage.
//!
//! # What is (and is not) here
//!
//! - The concrete format is JSON via `serde`/`serde_json` over a **crate-private
//!   DTO** ([`dto`]); no opaque-handle value and no intermediate encoding type
//!   is public. `ExactNumber` is transported as decimal numerator/denominator
//!   strings, never a float (ADR-0005 §7.2).
//! - The mint high-water is an explicit [`MintHighWater`] (`Next`/`Exhausted`),
//!   so an exhausted mint domain survives reload as a distinct state, never a
//!   reusable number (ADR-0009 §6/§7). [`minimum_safe_high_water`] is fallible:
//!   it never evaluates `u64::MAX + 1`.
//! - Decode success does **not** imply Well-Formedness: after a successful
//!   [`deserialize_envelope`], the caller runs the existing checker, which
//!   remains authoritative (ADR-0009 §13).

mod dto;

#[cfg(test)]
mod tests;

use crate::model::declared::Declared;
use crate::model::derivation::DerivationExpression;
use crate::model::guard::Guard;
use crate::model::opaque::VersionToken;
use crate::model::opaque::{LineageId, OpaqueValue};
use crate::model::strategy::Strategy;
use crate::semantics::dimension::DimensionBinding;

use dto::{EnvelopeDto, MintHighWaterDto, StrategyDto};

/// The current Dogfood persistence schema version (ADR-0009 §12). A reader that
/// does not recognise a stored `format_version` rejects it — it never guesses.
pub const FORMAT_VERSION: u32 = 1;

/// The logical, persisted Dogfood unit — exactly the four RFC-0005 fields
/// (ADR-0009 §2). It carries **no** `RevisionIdentity`: identity is recomputed
/// on load (ADR-0009 §9). `binding` + `supported` are the checker context
/// (ADR-0009 §8), expressed as `ordina` public value types the desktop holds.
#[derive(Debug, Clone)]
pub struct DogfoodEnvelope {
    /// Storage schema version (ADR-0009 §12); [`FORMAT_VERSION`] when written.
    pub format_version: u32,
    /// Checker context, part 1 — the distinguished dimensionless binding.
    pub binding: DimensionBinding,
    /// Checker context, part 2 — the supported semantic-model version token.
    pub supported: VersionToken,
    /// The current canonical Strategy, if any.
    pub strategy: Option<Strategy>,
    /// The next safe mint handle, or the terminal `Exhausted` state (§7).
    pub mint_high_water: MintHighWater,
}

/// The persisted mint high-water (ADR-0009 §7). `Next(v)` is the next safe mint
/// handle (`v` may be `u64::MAX`, the final mintable handle); `Exhausted` is the
/// terminal state — the domain is spent and no further mint may be issued. There
/// is **no** numeric encoding of exhaustion, so reload can never transform it
/// back into a reusable handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MintHighWater {
    /// The next safe mint handle.
    Next(u64),
    /// The mint domain is spent; no further mint may be issued.
    Exhausted,
}

impl MintHighWater {
    /// The seed value for a fresh minter, or `None` if the domain is exhausted.
    #[must_use]
    pub const fn seed(self) -> Option<u64> {
        match self {
            MintHighWater::Next(v) => Some(v),
            MintHighWater::Exhausted => None,
        }
    }

    /// Issue the next handle and advance, using **checked** arithmetic — never
    /// wrapping, never saturating into reuse (ADR-0009 §7). Returns the issued
    /// handle and the successor state: after issuing `u64::MAX` the successor is
    /// [`MintHighWater::Exhausted`]. Returns `None` when already exhausted.
    #[must_use]
    pub fn issue(self) -> Option<(u64, MintHighWater)> {
        match self {
            MintHighWater::Next(v) => {
                let next = match v.checked_add(1) {
                    Some(n) => MintHighWater::Next(n),
                    None => MintHighWater::Exhausted, // v == u64::MAX: last handle
                };
                Some((v, next))
            }
            MintHighWater::Exhausted => None,
        }
    }
}

/// The mint domain is exhausted: no handle strictly greater than every present
/// relevant handle exists in `u64` (ADR-0009 §6/§7). A distinct, explicit
/// outcome — never a numeric value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HighWaterExhausted;

/// Why a persisted envelope could not be loaded (ADR-0009 §13). Every variant is
/// a fail-closed rejection of unusable input; none is a silent repair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnvelopeLoadError {
    /// Not valid JSON / schema, or an unreadable scalar (fails closed).
    Malformed,
    /// A `format_version` this build does not support.
    UnsupportedVersion(u32),
    /// The stored high-water is below the minimum safe value for the corpus.
    HighWaterInconsistent,
    /// The corpus is already exhausted (a relevant handle equals `u64::MAX`)
    /// while the stored high-water contradicts it — never turned into a
    /// reusable value.
    HighWaterExhausted,
    /// A persisted `ExactNumber` is not a valid decimal rational (§4.5).
    RationalOutOfDomain,
}

/// Serialize an envelope to bytes (ADR-0009 §3) — **total**: every value a typed
/// [`DogfoodEnvelope`] can hold has a JSON representation.
#[must_use]
pub fn serialize_envelope(envelope: &DogfoodEnvelope) -> Vec<u8> {
    let dto = EnvelopeDto {
        format_version: envelope.format_version,
        binding: envelope.binding.dimensionless().identity_handle(),
        supported: envelope.supported.identity_handle(),
        strategy: envelope.strategy.as_ref().map(StrategyDto::from_model),
        mint_high_water: match envelope.mint_high_water {
            MintHighWater::Next(v) => MintHighWaterDto::Next(v),
            MintHighWater::Exhausted => MintHighWaterDto::Exhausted,
        },
    };
    // The DTO is plain data with no map-with-non-string-keys and no fallible
    // custom serializer, so JSON serialization cannot fail; serialize is total.
    serde_json::to_vec(&dto).expect("DogfoodEnvelope DTO is always serializable")
}

/// Deserialize an envelope from bytes (ADR-0009 §3), failing closed on any
/// malformed / unsupported / inconsistent input (§13). A successful decode does
/// **not** imply Well-Formedness — the caller still runs the checker (§13).
///
/// # Errors
///
/// [`EnvelopeLoadError`] on malformed bytes, an unsupported `format_version`, an
/// out-of-domain `ExactNumber`, or a high-water inconsistent with the corpus.
pub fn deserialize_envelope(bytes: &[u8]) -> Result<DogfoodEnvelope, EnvelopeLoadError> {
    let dto: EnvelopeDto =
        serde_json::from_slice(bytes).map_err(|_| EnvelopeLoadError::Malformed)?;

    if dto.format_version != FORMAT_VERSION {
        return Err(EnvelopeLoadError::UnsupportedVersion(dto.format_version));
    }

    let strategy = match dto.strategy {
        Some(s) => Some(s.into_model()?),
        None => None,
    };

    let mint_high_water = match dto.mint_high_water {
        MintHighWaterDto::Next(v) => MintHighWater::Next(v),
        MintHighWaterDto::Exhausted => MintHighWater::Exhausted,
    };

    // Load-time high-water consistency (ADR-0009 §7).
    if let Some(strategy) = &strategy {
        match minimum_safe_high_water(strategy) {
            Ok(minimum) => match mint_high_water {
                MintHighWater::Next(v) if v < minimum => {
                    return Err(EnvelopeLoadError::HighWaterInconsistent);
                }
                // Next(v) with v >= minimum, or Exhausted (accepted as-is; it only
                // ever forbids minting, which is always safe): both admitted.
                _ => {}
            },
            Err(HighWaterExhausted) => {
                // The corpus itself contains u64::MAX. A stored Next(_) contradicts
                // that (would permit a colliding/overflowing mint); only Exhausted
                // is coherent.
                if let MintHighWater::Next(_) = mint_high_water {
                    return Err(EnvelopeLoadError::HighWaterExhausted);
                }
            }
        }
    }

    Ok(DogfoodEnvelope {
        format_version: dto.format_version,
        binding: DimensionBinding::new(crate::model::opaque::DimensionSymbol::new(
            OpaqueValue::new(dto.binding),
        )),
        supported: VersionToken::new(OpaqueValue::new(dto.supported)),
        strategy,
        mint_high_water,
    })
}

/// The minimum safe mint high-water for a corpus with no persisted removal
/// history (ADR-0009 §6) — **fallible**, never `u64::MAX + 1`.
///
/// Returns `Ok(0)` when no relevant handle is present, `Ok(max + 1)` when the
/// maximum present `OccurrenceId`/`LineageId` handle is `< u64::MAX`, and
/// `Err(HighWaterExhausted)` when that maximum equals `u64::MAX` — no handle
/// strictly greater exists, so no safe next handle exists.
///
/// The minter mints only `OccurrenceId` and `LineageId`, from a single shared
/// counter, so scanning those two domains and using one shared high-water is
/// correct; the other opaque domains are never minted (distinct newtypes a
/// minted handle can never collide with).
///
/// # Errors
///
/// [`HighWaterExhausted`] when the corpus already contains a relevant handle
/// equal to `u64::MAX`.
pub fn minimum_safe_high_water(strategy: &Strategy) -> Result<u64, HighWaterExhausted> {
    let mut max: Option<u64> = None;
    let mut see = |h: u64| {
        max = Some(match max {
            Some(m) => m.max(h),
            None => h,
        });
    };
    walk_relevant_handles(strategy, &mut see);

    match max {
        None => Ok(0),
        Some(m) => m.checked_add(1).ok_or(HighWaterExhausted), // m == u64::MAX → Err
    }
}

/// Visit every mint-domain (`OccurrenceId` ∪ `LineageId`) handle in `strategy`.
fn walk_relevant_handles(strategy: &Strategy, see: &mut impl FnMut(u64)) {
    see(strategy.occurrence.identity_handle());
    see_lineage(&strategy.lineage, see);

    for e in present_slice(&strategy.evidence_declarations) {
        see(e.occurrence.identity_handle());
        see_lineage(&e.lineage, see);
    }
    for c in present_slice(&strategy.configuration_parameters) {
        see(c.occurrence.identity_handle());
        see_lineage(&c.lineage, see);
        see_bound(&c.lower_bound, see);
        see_bound(&c.upper_bound, see);
    }
    for c in present_slice(&strategy.conditions) {
        see(c.occurrence.identity_handle());
        see_lineage(&c.lineage, see);
        if let Declared::Present(g) = &c.guard {
            see_guard(g, see);
        }
    }
    for d in present_slice(&strategy.direction_rules) {
        see(d.occurrence.identity_handle());
        see_lineage(&d.lineage, see);
        if let Declared::Present(g) = &d.guard {
            see_guard(g, see);
        }
    }
    for c in present_slice(&strategy.candidate_derivations) {
        see(c.occurrence.identity_handle());
        see_lineage(&c.lineage, see);
        if let Declared::Present(e) = &c.expression {
            see_expression(e, see);
        }
    }
}

fn present_slice<T>(d: &Declared<Vec<T>>) -> &[T] {
    match d {
        Declared::Present(v) => v,
        Declared::Absent => &[],
    }
}

fn see_lineage(d: &Declared<LineageId>, see: &mut impl FnMut(u64)) {
    if let Declared::Present(l) = d {
        see(l.identity_handle());
    }
}

fn see_bound(d: &Declared<crate::model::literal::Literal>, see: &mut impl FnMut(u64)) {
    if let Declared::Present(l) = d {
        see_literal(l, see);
    }
}

fn see_literal(l: &crate::model::literal::Literal, see: &mut impl FnMut(u64)) {
    // Only LineageId/OccurrenceId are mint-domain handles; a literal carries
    // neither, so nothing here contributes to the high-water. (DimensionSymbol /
    // EnumValue are never minted.) Present for completeness of the walk shape.
    let _ = (l, see);
}

fn see_guard(g: &Guard, see: &mut impl FnMut(u64)) {
    for a in &g.atoms {
        see(a.evidence.identity_handle());
    }
}

fn see_expression(e: &DerivationExpression, see: &mut impl FnMut(u64)) {
    match e {
        DerivationExpression::Literal(_) => {}
        DerivationExpression::EvidenceReference(l)
        | DerivationExpression::ConfigurationReference(l) => see(l.identity_handle()),
        DerivationExpression::Add { left, right }
        | DerivationExpression::Subtract { left, right } => {
            see_expression(left, see);
            see_expression(right, see);
        }
        DerivationExpression::Multiply { operand, scalar } => {
            see_expression(operand, see);
            let _ = scalar;
        }
    }
}
