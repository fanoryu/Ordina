//! Canonical revision-identity computation — ADR-0008 (ACCEPTED, PBR-0022).
//!
//! One public entry point, [`compute_revision_identity`], turns a canonical,
//! Well-Formed [`Strategy`] into a [`RevisionIdentity`]: a BLAKE3-256 digest
//! (ADR-0008 §4, plain unkeyed `hash()`) over an explicitly domain-separated,
//! self-describing canonical TLV preimage (§2, §2a, §2b, §3, Appendix).
//!
//! Everything between those two ends is **implementation-private** (ADR-0008
//! §11): no public type or function for the intermediate byte encoding exists,
//! and none should. The encoder below is module-private; the conformance
//! vectors (§12) reach it only because they are a child module of this one.
//!
//! # What is encoded, and what is never encoded
//!
//! The preimage is built exclusively from fields ADR-0003 §6 classifies **ID**
//! (including §6.1's operative-rule-content default). `occurrence`
//! ([`OccurrenceId`](crate::model::opaque::OccurrenceId)) and any
//! occurrence-derived order are **never walked, on any construct, without
//! exception** — ADR-0007's ruling, applied here, not re-derived. That is
//! enforced structurally rather than by convention: `OccurrenceId` is the one
//! opaque domain with no crate-internal handle accessor, so there is no way to
//! encode it even by mistake.
//!
//! The other five **semantic** opaque domains — `LineageId` in its
//! reference-bearing role, `DimensionSymbol`, `EnumValue`, `VersionToken`,
//! `PrecedenceOrdinal` — *are* walked and encoded (ADR-0008 §1, tag `0x07`).
//!
//! # Independence from Rust source detail
//!
//! No Rust variant name, discriminant, struct field order, memory layout, or
//! host endianness reaches the preimage. Enum spelling is an ADR-fixed
//! semantic tag (§2b, Appendix Table I); field order is the Slice Contract's
//! own §3.x property positions (Appendix Tables B–H); every multi-byte integer
//! is big-endian; unordered collections are sorted by their own encoded member
//! bytes, never by insertion or hash order.

use num_bigint::{BigInt, Sign};
use num_rational::BigRational;

use crate::model::condition::{Condition, ConditionClassification};
use crate::model::config::ConfigurationParameter;
use crate::model::declared::{Declared, DeclaredCategory};
use crate::model::derivation::{CandidateDerivation, CandidateRole, DerivationExpression};
use crate::model::direction::{Direction, DirectionRule};
use crate::model::evidence::{EvidenceDeclaration, EvidenceTypeTag};
use crate::model::guard::{Atom, ComparisonOperator, Guard};
use crate::model::literal::Literal;
use crate::model::strategy::Strategy;
use crate::result::identity::RevisionIdentity;

// ---------------------------------------------------------------------------
// Public surface — ADR-0008 §11, and nothing beyond it.
// ---------------------------------------------------------------------------

/// The error surface of [`compute_revision_identity`] — ADR-0008 §7, §11.
///
/// A closed authoritative category (ADR-0005 §7.4): matching over it is
/// exhaustive and no catch-all `_` arm is permitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RevisionIdentityError {
    /// The `Strategy` is not Well-Formed per ADR-0004's gate — ADR-0003's I9
    /// precondition is unmet, so no revision identity may be produced.
    ///
    /// See [`compute_revision_identity`]'s own documentation for why this
    /// variant is, under the current design, unreachable from a caller that
    /// honours the documented precondition.
    NotWellFormed,
}

/// Compute a Strategy's canonical revision identity — ADR-0008.
///
/// A **pure function of canonical `Strategy` content**. Nothing else
/// participates: no ambient state, no clock, no allocator address, no host
/// endianness, no configuration. Two conforming implementations — on different
/// architectures, different compilers, or in different languages entirely —
/// given the same canonical content produce byte-identical preimages and
/// therefore identical digests (ADR-0003 I8, ADR-0008 §8).
///
/// # Well-Formedness is a caller-established precondition
///
/// The caller must have already established, via ADR-0004's Well-Formedness
/// checker, that `strategy` is Well-Formed **before** calling this function.
/// This function does **not** perform Well-Formedness checking itself.
/// ADR-0003 §5 fixes the ordering `Well-Formed → canonicalize → identity` and
/// frames the gate as something this computation *depends on*, not something it
/// *owns* ("this ADR only depends on the gate, invariant I9"); ADR-0008 §1
/// likewise takes it as given that "by the time a `Strategy` reaches this
/// ADR's hashing step it has already passed ADR-0003 §5's pipeline."
///
/// Calling this on a Strategy that has not passed that gate is a violation of
/// the caller's contract. The result in that case is still deterministic —
/// this function is total over every state the typed model can hold (ADR-0008
/// §2a, §7) — but it is not a meaningful revision identity, and ADR-0003 I9
/// does not admit it as one.
///
/// # Why [`RevisionIdentityError::NotWellFormed`] is never actually returned
///
/// ADR-0008 §7 fixes `NotWellFormed` as the complete and correct error surface
/// for this function, and §11 fixes the `Result` signature; both are honoured
/// here. But Well-Formedness is a **whole-Strategy, sixteen-check, ADR-0004
/// orchestration property**, not a locally-decidable structural one — a
/// Strategy can be structurally complete and still fail check 4, 8, 9, 11, or
/// 13B. This function therefore has no way to *independently* detect a
/// non-Well-Formed input, and deliberately does not manufacture a partial,
/// weaker check that would look like a gate while catching only some of what
/// ADR-0004 catches. Re-running the checker internally is likewise excluded:
/// it would make an external gate an internal one and contradict ADR-0003 §5's
/// ordering.
///
/// So the variant exists for API-contract completeness and future-proofing,
/// exactly matching ADR-0008 §7's stated surface, and is **unreachable from a
/// caller obeying the precondition above**. This is documented rather than
/// disguised behind an invented check: a fake gate would be worse than an
/// honest absence, because it would let a caller believe the precondition was
/// being enforced.
///
/// # Errors
///
/// Returns [`RevisionIdentityError::NotWellFormed`] if the `Strategy` is not
/// Well-Formed. Per the section above, the current implementation never
/// reaches this path — the precondition is the caller's to establish.
pub fn compute_revision_identity(
    strategy: &Strategy,
) -> Result<RevisionIdentity, RevisionIdentityError> {
    let preimage = revision_preimage(strategy);
    // ADR-0008 §4: plain unkeyed BLAKE3, 256-bit output, over the whole
    // domain-separated preimage as a single input. No key, no derive-key
    // context — domain separation is the §3 preimage prefix alone.
    let digest = *blake3::hash(&preimage).as_bytes();
    Ok(RevisionIdentity::from_parts(SCHEME_VERSION, digest))
}

// ---------------------------------------------------------------------------
// §3 — domain separation.
// ---------------------------------------------------------------------------

/// ADR-0008 §3: the fixed 16-byte ASCII identity-domain separator.
const DOMAIN: &[u8; 16] = b"ordina-rev-ident";

/// ADR-0008 §3/§6: this ADR's scheme version, carried both inside the preimage
/// and alongside the digest in [`RevisionIdentity`]'s own value (§5).
const SCHEME_VERSION: u8 = 0x01;

/// The full digest preimage — ADR-0008 §3:
/// `DOMAIN (16) || SCHEME_VERSION (1) || ROOT_FRAME (tag 0x0A)`.
///
/// No length prefix separates the parts: the first two are fixed-width and
/// fixed-position, and the root frame is self-delimiting by §2's framing rules.
fn revision_preimage(strategy: &Strategy) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(DOMAIN);
    out.push(SCHEME_VERSION);
    out.extend_from_slice(&encode_strategy(strategy));
    out
}

// ---------------------------------------------------------------------------
// §2 — the tag alphabet.
//
// Every value, at every nesting depth, is the same `(tag: u8, payload)` shape.
// ---------------------------------------------------------------------------

const TAG_ABSENT: u8 = 0x01;
const TAG_U64: u8 = 0x02;
const TAG_I64: u8 = 0x03;
const TAG_RATIONAL: u8 = 0x04;
const TAG_STRING: u8 = 0x05;
const TAG_BOOL: u8 = 0x06;
const TAG_OPAQUE: u8 = 0x07;
const TAG_SEQUENCE: u8 = 0x08;
const TAG_COLLECTION: u8 = 0x09;
const TAG_RECORD: u8 = 0x0A;
const TAG_CATEGORY_ONE: u8 = 0x0B;
const TAG_CATEGORY_MULTIPLE: u8 = 0x0C;
const TAG_CATEGORY_OUT_OF_DOMAIN: u8 = 0x0D;
const TAG_LITERAL: u8 = 0x0E;
const TAG_DERIVATION: u8 = 0x0F;
const TAG_BIGINT: u8 = 0x10;
const TAG_ENUM: u8 = 0x11;

/// Appendix Table A — record-type tags (tag `0x0A`'s leading byte).
const RECORD_STRATEGY: u8 = 1;
const RECORD_CONDITION: u8 = 2;
const RECORD_CONFIGURATION_PARAMETER: u8 = 3;
const RECORD_CANDIDATE_DERIVATION: u8 = 4;
const RECORD_DIRECTION_RULE: u8 = 5;
const RECORD_EVIDENCE_DECLARATION: u8 = 6;
const RECORD_GUARD: u8 = 7;
const RECORD_ATOM: u8 = 8;

// ---------------------------------------------------------------------------
// §2a — Canonical Unsigned Length Varint (CULV).
// ---------------------------------------------------------------------------

/// Encode a non-negative integer, given as its **minimal big-endian magnitude
/// bytes**, as a CULV octet sequence — ADR-0008 §2a.
///
/// Base-128: seven payload bits per octet, bit 7 the continuation bit (`1` =
/// another octet follows). Groups are emitted **most-significant first**,
/// composing with this ADR's uniformly big-endian convention. Zero is exactly
/// one octet, `0x00`. The encoding is minimal: the leading 7-bit group is
/// never all-zero unless it is the only group.
///
/// Taking the magnitude as bytes rather than as a `u64` is what keeps this
/// genuinely unbounded: §2a fixes no maximum octet count, and a caller with an
/// arbitrarily large length has an arbitrarily long byte slice for it. In
/// practice every call site's length is a `usize` byte count, which
/// [`culv_encode`] handles via [`u64::to_be_bytes`] — but the grammar below is
/// defined by ADR-0008, not by any platform width.
fn culv_encode_magnitude(magnitude_be: &[u8]) -> Vec<u8> {
    // Strip leading zero bytes; an all-zero (or empty) input is the value zero.
    let significant = match magnitude_be.iter().position(|byte| *byte != 0) {
        Some(first) => &magnitude_be[first..],
        None => return vec![0x00],
    };

    // Bit length of the value, then the number of 7-bit groups needed.
    let leading_zeros = significant[0].leading_zeros();
    let bit_length = (significant.len() as u64) * 8 - u64::from(leading_zeros);
    let group_count = bit_length.div_ceil(7);

    let mut out = Vec::with_capacity(group_count as usize);
    for group in (0..group_count).rev() {
        // Bits [group*7, group*7+7) of the value, counted from the LSB.
        let payload = extract_seven_bits(significant, group * 7);
        let continuation = if group == 0 { 0x00 } else { 0x80 };
        out.push(payload | continuation);
    }
    out
}

/// The seven bits of `magnitude_be` starting `offset` bits above the least
/// significant bit, as the low seven bits of the returned byte.
fn extract_seven_bits(magnitude_be: &[u8], offset: u64) -> u8 {
    let mut value: u8 = 0;
    for bit in 0..7u64 {
        if bit_at(magnitude_be, offset + bit) {
            value |= 1 << bit;
        }
    }
    value
}

/// Whether the bit `index` places above the least significant bit is set.
fn bit_at(magnitude_be: &[u8], index: u64) -> bool {
    let byte_from_end = index / 8;
    if byte_from_end >= magnitude_be.len() as u64 {
        return false;
    }
    let byte = magnitude_be[magnitude_be.len() - 1 - byte_from_end as usize];
    byte & (1 << (index % 8)) != 0
}

/// CULV for a length that fits a `u64` — the shape every current call site has.
fn culv_encode(value: u64) -> Vec<u8> {
    culv_encode_magnitude(&value.to_be_bytes())
}

/// Decode one CULV value from the front of `input` — ADR-0008 §2a.
///
/// Returns the decoded magnitude's minimal big-endian bytes and the number of
/// octets consumed. Rejects, rather than silently accepting:
///
/// - a **non-canonical** encoding (a redundant leading all-zero 7-bit group
///   followed by a continuation-bit-`1` octet), and
/// - a **truncated** sequence (one that ends without a continuation-bit-`0`
///   octet).
///
/// The decoder exists for the §12 item 9b conformance vector and for the
/// grammar's own boundary tests; the production path only ever encodes.
#[cfg(test)]
fn culv_decode(input: &[u8]) -> Result<(Vec<u8>, usize), CulvError> {
    let mut groups: Vec<u8> = Vec::new();
    let mut consumed = 0usize;
    let mut terminated = false;

    for byte in input {
        consumed += 1;
        groups.push(byte & 0x7F);
        if byte & 0x80 == 0 {
            terminated = true;
            break;
        }
    }

    if !terminated {
        // §2a: "an octet sequence that ends without ever emitting a
        // continuation bit of 0 ... is invalid and must be rejected."
        return Err(CulvError::Truncated);
    }
    if groups.len() > 1 && groups[0] == 0 {
        // §2a minimality: a leading all-zero payload group followed by a
        // continuation-bit-1 octet is a redundant leading-zero group.
        return Err(CulvError::NonCanonical);
    }

    // Reassemble the 7-bit groups (MSB-group-first) into big-endian bytes.
    let mut bits: Vec<bool> = Vec::with_capacity(groups.len() * 7);
    for group in &groups {
        for bit in (0..7).rev() {
            bits.push(group & (1 << bit) != 0);
        }
    }
    while !bits.len().is_multiple_of(8) {
        bits.insert(0, false);
    }
    let mut bytes: Vec<u8> = bits
        .chunks(8)
        .map(|chunk| {
            chunk
                .iter()
                .fold(0u8, |acc, bit| (acc << 1) | u8::from(*bit))
        })
        .collect();
    while bytes.len() > 1 && bytes[0] == 0 {
        bytes.remove(0);
    }
    Ok((bytes, consumed))
}

/// Why a CULV octet sequence was rejected — ADR-0008 §2a.
#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CulvError {
    /// A redundant leading all-zero 7-bit group — not the minimal encoding.
    NonCanonical,
    /// The sequence ended without a continuation-bit-`0` octet.
    Truncated,
}

// ---------------------------------------------------------------------------
// §2 / §2a — scalar frames.
// ---------------------------------------------------------------------------

/// Tag `0x01` — an absent declaration. Zero-length payload.
fn frame_absent() -> Vec<u8> {
    vec![TAG_ABSENT]
}

/// Tag `0x02` — a bounded unsigned integer, 8-byte big-endian.
///
/// Defined by ADR-0008 §2 and implemented here for completeness of the tag
/// alphabet; no field in the current model is encoded through it (every
/// integer-shaped identity-bearing value is an opaque-domain reference, tag
/// `0x07`, or an exact rational, tag `0x04`).
#[expect(
    dead_code,
    reason = "ADR-0008 §2 tag defined; no current model field uses it"
)]
fn frame_u64(value: u64) -> Vec<u8> {
    let mut out = vec![TAG_U64];
    out.extend_from_slice(&value.to_be_bytes());
    out
}

/// Tag `0x03` — a bounded signed integer, 8-byte big-endian two's complement.
///
/// Defined by ADR-0008 §2; unused by the current model for the same reason as
/// tag `0x02`. Signedness is carried by the **tag**, never inferred from range.
#[expect(
    dead_code,
    reason = "ADR-0008 §2 tag defined; no current model field uses it"
)]
fn frame_i64(value: i64) -> Vec<u8> {
    let mut out = vec![TAG_I64];
    out.extend_from_slice(&value.to_be_bytes());
    out
}

/// Tag `0x05` — a UTF-8 string, NFC-normalized (Unicode 15.0.0), 4-byte
/// big-endian byte-length prefix.
///
/// Defined by ADR-0008 §2/§2c/§9 and reserved for future identity-bearing
/// free-text content. **No field in the current model is free text** — every
/// string-shaped value is an opaque-domain reference (tag `0x07`) or a
/// closed-category enum (tag `0x11`) — so this path is never taken today, and
/// consequently no normalization is applied here: applying NFC would require a
/// Unicode dependency that ADR-0005 §6.2's closed dependency set does not
/// admit and that nothing currently exercises. A future model revision adding
/// genuine free-text ID content must add that normalization step (and its
/// approved dependency) before using this frame.
#[expect(
    dead_code,
    reason = "ADR-0008 §2c tag reserved; no current model field is free text"
)]
fn frame_string(value: &str) -> Vec<u8> {
    let bytes = value.as_bytes();
    let mut out = vec![TAG_STRING];
    out.extend_from_slice(&(bytes.len() as u32).to_be_bytes());
    out.extend_from_slice(bytes);
    out
}

/// Tag `0x06` — a boolean, exactly one payload byte, `0x00` or `0x01`.
fn frame_bool(value: bool) -> Vec<u8> {
    vec![TAG_BOOL, u8::from(value)]
}

/// Tag `0x07` — an opaque-domain reference, 8-byte big-endian `u64` of the
/// referenced value's underlying handle (ADR-0008 §1, §2).
///
/// Reached only from the **five semantic** opaque domains. `OccurrenceId` has
/// no handle accessor and therefore cannot reach this function at all.
fn frame_opaque(handle: u64) -> Vec<u8> {
    let mut out = vec![TAG_OPAQUE];
    out.extend_from_slice(&handle.to_be_bytes());
    out
}

/// Tag `0x08` — an ordered sequence: 4-byte big-endian element count, then each
/// element's own frame in declared order.
///
/// Defined by ADR-0008 §2 and reserved: **no field in the current model is
/// contract-declared order-significant**, so every collection today takes tag
/// `0x09` instead. Kept so a future order-significant field has an encoding
/// already defined.
#[expect(
    dead_code,
    reason = "ADR-0008 §2 tag reserved; no current field is order-significant"
)]
fn frame_sequence(elements: &[Vec<u8>]) -> Vec<u8> {
    let mut out = vec![TAG_SEQUENCE];
    out.extend_from_slice(&(elements.len() as u32).to_be_bytes());
    for element in elements {
        out.extend_from_slice(element);
    }
    out
}

/// Tag `0x09` — an unordered collection (ADR-0008 §2, ADR-0003 §6's
/// "Unordered collections — CX→ID(members)" row).
///
/// Members are encoded first, then **sorted ascending by unsigned
/// byte-lexicographic order of the encoded frames themselves**, then
/// deduplicated (a repeated identical canonical frame contributes once), then
/// counted and concatenated. The order is a pure function of member content —
/// never insertion, hash, or comparator order.
///
/// The count prefix is the count of frames actually emitted, i.e. after
/// deduplication.
fn frame_collection(mut members: Vec<Vec<u8>>) -> Vec<u8> {
    members.sort_unstable();
    members.dedup();

    let mut out = vec![TAG_COLLECTION];
    out.extend_from_slice(&(members.len() as u32).to_be_bytes());
    for member in &members {
        out.extend_from_slice(member);
    }
    out
}

/// Tag `0x0A` — a record: 1-byte record-type tag (Appendix Table A), 4-byte
/// big-endian field count, then for each field its 1-byte **position tag**
/// (Appendix Tables B–H) followed by that field's own tagged frame.
///
/// Fields are passed in fixed §3.x property-position order — never Rust
/// declaration order, memory layout, or serde order. No ID-classified field is
/// ever omitted, including when its state is absent or out-of-domain: the
/// absent state is an explicit tag, not a missing field.
///
/// The field count is the number of fields the model actually has, which for
/// `ConfigurationParameter` is 5 and not 6 — Appendix Table D's position `2` is
/// contract-named but has no live field, so the emitted position tags are a
/// sparse subset (`1, 3, 4, 5, 6`) of the contract's numbering.
fn frame_record(record_type: u8, fields: &[(u8, Vec<u8>)]) -> Vec<u8> {
    let mut out = vec![TAG_RECORD, record_type];
    out.extend_from_slice(&(fields.len() as u32).to_be_bytes());
    for (position, value) in fields {
        out.push(*position);
        out.extend_from_slice(value);
    }
    out
}

/// Tag `0x11` — a closed-category enum's ADR-fixed semantic tag (§2b, Appendix
/// Table I).
///
/// **Never** a Rust discriminant and never a variant-name string: a future
/// Rust-side rename with no semantic change must not alter these bytes.
fn frame_enum(semantic_tag: u8) -> Vec<u8> {
    vec![TAG_ENUM, semantic_tag]
}

/// Tag `0x10` — a canonical arbitrary-precision `BigInt` (ADR-0008 §2a).
///
/// `sign (0 = zero, 1 = positive, 2 = negative)` then, for a non-zero value, the
/// magnitude's byte count as a **CULV** followed by that many minimal
/// big-endian magnitude bytes (most-significant byte never zero). Zero has
/// exactly one representation: the sign octet `0`, nothing further.
///
/// No fixed-width field appears anywhere in the frame, so the encoding is total
/// over the whole `BigInt` domain — there is no magnitude this can fail on and
/// no numeric-overflow error path (§7).
fn frame_bigint(value: &BigInt) -> Vec<u8> {
    let (sign, magnitude) = value.to_bytes_be();
    match sign {
        // No catch-all `_` arm (ADR-0005 §7.4).
        Sign::NoSign => vec![TAG_BIGINT, 0],
        Sign::Plus | Sign::Minus => {
            let sign_octet: u8 = if matches!(sign, Sign::Plus) { 1 } else { 2 };
            // `to_bytes_be` yields the minimal big-endian magnitude for a
            // non-zero value: the most-significant byte is never zero.
            debug_assert!(magnitude.first().is_some_and(|byte| *byte != 0));
            let mut out = vec![TAG_BIGINT, sign_octet];
            out.extend_from_slice(&culv_encode(magnitude.len() as u64));
            out.extend_from_slice(&magnitude);
            out
        }
    }
}

/// Tag `0x04` — an exact rational (ADR-0008 §2a): the numerator's signed
/// tag-`0x10` frame followed by the denominator's tag-`0x10` frame, whose sign
/// octet is always `1`.
///
/// The two normalization requirements are applied **here**, explicitly, rather
/// than inherited: §2a restates lowest terms and numerator-carried sign as this
/// ADR's own normative requirement "so the encoding does not merely inherit but
/// requires it independent of any one crate's implementation choice."
/// Reconstructing the ratio through `BigRational::new` re-establishes both, so
/// `1/2`, `2/4` and `-1/-2` all reduce to the identical canonical pair and
/// therefore produce byte-identical frames.
fn frame_rational(value: &BigRational) -> Vec<u8> {
    let normalized = BigRational::new(value.numer().clone(), value.denom().clone());
    debug_assert!(*normalized.denom() > BigInt::from(0));

    let mut out = vec![TAG_RATIONAL];
    out.extend_from_slice(&frame_bigint(normalized.numer()));
    out.extend_from_slice(&frame_bigint(normalized.denom()));
    out
}

// ---------------------------------------------------------------------------
// §2 — declaration-state frames.
// ---------------------------------------------------------------------------

/// A [`Declared`] field's frame: tag `0x01` when absent, otherwise the value's
/// own tag directly (no extra wrapper) — ADR-0008 §2's absence/presence rule.
fn frame_declared<T>(declared: &Declared<T>, encode: impl Fn(&T) -> Vec<u8>) -> Vec<u8> {
    match declared {
        // No catch-all `_` arm (ADR-0005 §7.4).
        Declared::Absent => frame_absent(),
        Declared::Present(value) => encode(value),
    }
}

/// A [`DeclaredCategory`] field's frame — ADR-0008 §2 tags `0x01`/`0x0B`/
/// `0x0C`/`0x0D`.
///
/// All four states are structurally distinguishable, and an ID-classified field
/// is never omitted under any of them. `Multiple`'s `rest` is itself an
/// unordered `Vec<T>` (§3.7/§3.10 assign it no order) and so takes tag `0x09`.
fn frame_declared_category<T>(
    declared: &DeclaredCategory<T>,
    encode: impl Fn(&T) -> Vec<u8>,
) -> Vec<u8> {
    match declared {
        // No catch-all `_` arm (ADR-0005 §7.4).
        DeclaredCategory::Absent => frame_absent(),
        DeclaredCategory::One(value) => {
            let mut out = vec![TAG_CATEGORY_ONE];
            out.extend_from_slice(&encode(value));
            out
        }
        DeclaredCategory::Multiple {
            first,
            second,
            rest,
        } => {
            let mut out = vec![TAG_CATEGORY_MULTIPLE];
            out.extend_from_slice(&encode(first));
            out.extend_from_slice(&encode(second));
            out.extend_from_slice(&frame_collection(rest.iter().map(&encode).collect()));
            out
        }
        DeclaredCategory::OutOfDomain => vec![TAG_CATEGORY_OUT_OF_DOMAIN],
    }
}

// ---------------------------------------------------------------------------
// §2b / Appendix Table I — closed-category enum semantic tags.
//
// These values are NORMATIVE and IMMUTABLE. They are the ADR's, not Rust's: a
// variant rename here changes nothing below, and must not.
// ---------------------------------------------------------------------------

fn frame_condition_classification(value: &ConditionClassification) -> Vec<u8> {
    frame_enum(match value {
        // No catch-all `_` arm (ADR-0005 §7.4).
        ConditionClassification::Blocking => 0,
        ConditionClassification::Temporary => 1,
    })
}

fn frame_direction(value: &Direction) -> Vec<u8> {
    frame_enum(match value {
        Direction::Buy => 0,
        Direction::Sell => 1,
    })
}

fn frame_comparison_operator(value: ComparisonOperator) -> Vec<u8> {
    frame_enum(match value {
        ComparisonOperator::Less => 0,
        ComparisonOperator::LessOrEqual => 1,
        ComparisonOperator::Greater => 2,
        ComparisonOperator::GreaterOrEqual => 3,
        ComparisonOperator::Equal => 4,
        ComparisonOperator::NotEqual => 5,
    })
}

fn frame_evidence_type_tag(value: &EvidenceTypeTag) -> Vec<u8> {
    frame_enum(match value {
        EvidenceTypeTag::Numeric => 0,
        EvidenceTypeTag::Boolean => 1,
        EvidenceTypeTag::Enum => 2,
    })
}

fn frame_candidate_role(value: &CandidateRole) -> Vec<u8> {
    frame_enum(match value {
        CandidateRole::Entry => 0,
        CandidateRole::StopLoss => 1,
        CandidateRole::TakeProfit => 2,
    })
}

// ---------------------------------------------------------------------------
// §2 — tagged unions.
// ---------------------------------------------------------------------------

/// Tag `0x0E` — a [`Literal`], as a 1-byte variant tag plus its payload frames
/// (ADR-0008 §2).
///
/// `Literal::Enum`'s payload is an `EnumValue`, which is an **open,
/// caller-supplied opaque domain** — not a Rust `enum` — so it takes tag `0x07`
/// and never tag `0x11` (§2b's closing paragraph).
fn frame_literal(value: &Literal) -> Vec<u8> {
    match value {
        // No catch-all `_` arm (ADR-0005 §7.4).
        Literal::Numeric { value, dimension } => {
            let mut out = vec![TAG_LITERAL, 0];
            out.extend_from_slice(&frame_rational(value.as_ratio()));
            out.extend_from_slice(&frame_opaque(dimension.identity_handle()));
            out
        }
        Literal::Boolean(flag) => {
            let mut out = vec![TAG_LITERAL, 1];
            out.extend_from_slice(&frame_bool(*flag));
            out
        }
        Literal::Enum(enum_value) => {
            let mut out = vec![TAG_LITERAL, 2];
            out.extend_from_slice(&frame_opaque(enum_value.identity_handle()));
            out
        }
    }
}

/// Tag `0x0F` — a [`DerivationExpression`], recursively (ADR-0008 §2).
///
/// Every nesting level uses the identical outer framing; there is no separate
/// top-level rule.
fn frame_derivation(value: &DerivationExpression) -> Vec<u8> {
    match value {
        // No catch-all `_` arm (ADR-0005 §7.4).
        DerivationExpression::Literal(literal) => {
            let mut out = vec![TAG_DERIVATION, 0];
            out.extend_from_slice(&frame_literal(literal));
            out
        }
        DerivationExpression::EvidenceReference(lineage) => {
            let mut out = vec![TAG_DERIVATION, 1];
            out.extend_from_slice(&frame_opaque(lineage.identity_handle()));
            out
        }
        DerivationExpression::ConfigurationReference(lineage) => {
            let mut out = vec![TAG_DERIVATION, 2];
            out.extend_from_slice(&frame_opaque(lineage.identity_handle()));
            out
        }
        DerivationExpression::Add { left, right } => {
            let mut out = vec![TAG_DERIVATION, 3];
            out.extend_from_slice(&frame_derivation(left));
            out.extend_from_slice(&frame_derivation(right));
            out
        }
        DerivationExpression::Subtract { left, right } => {
            let mut out = vec![TAG_DERIVATION, 4];
            out.extend_from_slice(&frame_derivation(left));
            out.extend_from_slice(&frame_derivation(right));
            out
        }
        DerivationExpression::Multiply { operand, scalar } => {
            let mut out = vec![TAG_DERIVATION, 5];
            out.extend_from_slice(&frame_derivation(operand));
            out.extend_from_slice(&frame_literal(scalar));
            out
        }
    }
}

// ---------------------------------------------------------------------------
// Appendix Tables A–H — per-construct record frames.
//
// Every table below emits its fields in the Slice Contract's own §3.x property
// positions, and every one of them omits `occurrence` (ADR-0007).
// ---------------------------------------------------------------------------

/// Appendix Table H — `Atom` (type tag `8`).
///
/// All three fields are unconditional grammar-level fields (never
/// `Declared`-wrapped in the live type), so each frame is the value's own tag
/// directly and never tag `0x01`. `Atom` has no `occurrence` field of its own.
fn encode_atom(atom: &Atom) -> Vec<u8> {
    frame_record(
        RECORD_ATOM,
        &[
            (1, frame_opaque(atom.evidence.identity_handle())),
            (2, frame_comparison_operator(atom.operator)),
            (3, frame_literal(&atom.literal)),
        ],
    )
}

/// Appendix Table H — `Guard` (type tag `7`), one field `atoms` at position 1.
///
/// §3.6's conjunction is explicitly commutative, so the atoms are an unordered
/// collection (tag `0x09`). `Guard` has no `occurrence` field of its own.
fn encode_guard(guard: &Guard) -> Vec<u8> {
    frame_record(
        RECORD_GUARD,
        &[(
            1,
            frame_collection(guard.atoms.iter().map(encode_atom).collect()),
        )],
    )
}

/// Appendix Table G — `EvidenceDeclaration` (type tag `6`), positions 1–4.
fn encode_evidence_declaration(declaration: &EvidenceDeclaration) -> Vec<u8> {
    frame_record(
        RECORD_EVIDENCE_DECLARATION,
        &[
            (
                1,
                frame_declared(&declaration.lineage, |lineage| {
                    frame_opaque(lineage.identity_handle())
                }),
            ),
            (
                2,
                frame_declared_category(&declaration.declared_type, frame_evidence_type_tag),
            ),
            (
                3,
                frame_declared(&declaration.declared_dimension, |dimension| {
                    frame_opaque(dimension.identity_handle())
                }),
            ),
            (
                4,
                frame_declared(&declaration.declared_values, |values| {
                    frame_collection(
                        values
                            .iter()
                            .map(|value| frame_opaque(value.identity_handle()))
                            .collect(),
                    )
                }),
            ),
        ],
    )
}

/// Appendix Table D — `ConfigurationParameter` (type tag `3`).
///
/// Positions `1, 3, 4, 5, 6` — **never `2`**. Position 2 is the
/// structurally-implied `NUMERIC` declared type: contract-named at §3.9 but
/// with no live field to encode, because Slice 1 admits only `NUMERIC`
/// configuration parameters. It is reserved, not reused, not renumbered, and
/// not collapsed, and the record's field count is therefore **5**.
///
/// This mirrors `check_configuration_parameter`'s own position assignment
/// exactly (`lineage → 1, dimension → 3, lower_bound → 4, upper_bound → 5,
/// adjustable → 6`, skipping 2), independently corroborated by
/// `configuration_bounds.rs` ("lower bound = 4, upper bound = 5").
fn encode_configuration_parameter(parameter: &ConfigurationParameter) -> Vec<u8> {
    frame_record(
        RECORD_CONFIGURATION_PARAMETER,
        &[
            (
                1,
                frame_declared(&parameter.lineage, |lineage| {
                    frame_opaque(lineage.identity_handle())
                }),
            ),
            (
                3,
                frame_declared(&parameter.dimension, |dimension| {
                    frame_opaque(dimension.identity_handle())
                }),
            ),
            (4, frame_declared(&parameter.lower_bound, frame_literal)),
            (5, frame_declared(&parameter.upper_bound, frame_literal)),
            (
                6,
                frame_declared(&parameter.adjustable, |flag| frame_bool(*flag)),
            ),
        ],
    )
}

/// Appendix Table C — `Condition` (type tag `2`), positions 1–3.
fn encode_condition(condition: &Condition) -> Vec<u8> {
    frame_record(
        RECORD_CONDITION,
        &[
            (
                1,
                frame_declared(&condition.lineage, |lineage| {
                    frame_opaque(lineage.identity_handle())
                }),
            ),
            (
                2,
                frame_declared_category(&condition.classification, frame_condition_classification),
            ),
            (3, frame_declared(&condition.guard, encode_guard)),
        ],
    )
}

/// Appendix Table F — `DirectionRule` (type tag `5`), positions 1–4.
fn encode_direction_rule(rule: &DirectionRule) -> Vec<u8> {
    frame_record(
        RECORD_DIRECTION_RULE,
        &[
            (
                1,
                frame_declared(&rule.lineage, |lineage| {
                    frame_opaque(lineage.identity_handle())
                }),
            ),
            (
                2,
                frame_declared(&rule.precedence, |precedence| {
                    frame_opaque(precedence.identity_handle())
                }),
            ),
            (3, frame_declared(&rule.guard, encode_guard)),
            (4, frame_declared_category(&rule.outcome, frame_direction)),
        ],
    )
}

/// Appendix Table E — `CandidateDerivation` (type tag `4`), positions 1–3.
fn encode_candidate_derivation(derivation: &CandidateDerivation) -> Vec<u8> {
    frame_record(
        RECORD_CANDIDATE_DERIVATION,
        &[
            (
                1,
                frame_declared(&derivation.lineage, |lineage| {
                    frame_opaque(lineage.identity_handle())
                }),
            ),
            (2, frame_declared(&derivation.role, frame_candidate_role)),
            (3, frame_declared(&derivation.expression, frame_derivation)),
        ],
    )
}

/// Appendix Table B — `Strategy` (type tag `1`), positions 1–7: the root frame.
///
/// The five collections' positions (`evidence_declarations → 3`,
/// `configuration_parameters → 4`, `conditions → 5`, `direction_rules → 6`,
/// `candidate_derivations → 7`) are `check_1`'s own, in `check_1`'s own order;
/// `lineage → 1` and `semantic_model_version → 2` are the positions its doc
/// comment names as checks 15's and 16's.
///
/// `strategy.occurrence` is not read here, or anywhere below it (ADR-0007).
fn encode_strategy(strategy: &Strategy) -> Vec<u8> {
    frame_record(
        RECORD_STRATEGY,
        &[
            (
                1,
                frame_declared(&strategy.lineage, |lineage| {
                    frame_opaque(lineage.identity_handle())
                }),
            ),
            (
                2,
                frame_declared(&strategy.semantic_model_version, |token| {
                    frame_opaque(token.identity_handle())
                }),
            ),
            (
                3,
                frame_declared(&strategy.evidence_declarations, |declarations| {
                    frame_collection(
                        declarations
                            .iter()
                            .map(encode_evidence_declaration)
                            .collect(),
                    )
                }),
            ),
            (
                4,
                frame_declared(&strategy.configuration_parameters, |parameters| {
                    frame_collection(
                        parameters
                            .iter()
                            .map(encode_configuration_parameter)
                            .collect(),
                    )
                }),
            ),
            (
                5,
                frame_declared(&strategy.conditions, |conditions| {
                    frame_collection(conditions.iter().map(encode_condition).collect())
                }),
            ),
            (
                6,
                frame_declared(&strategy.direction_rules, |rules| {
                    frame_collection(rules.iter().map(encode_direction_rule).collect())
                }),
            ),
            (
                7,
                frame_declared(&strategy.candidate_derivations, |derivations| {
                    frame_collection(
                        derivations
                            .iter()
                            .map(encode_candidate_derivation)
                            .collect(),
                    )
                }),
            ),
        ],
    )
}

#[cfg(test)]
mod tests;
