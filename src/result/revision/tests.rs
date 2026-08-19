//! Golden / conformance vectors for ADR-0008 §12.
//!
//! Every expected byte string below is **hand-derived from ADR-0008's text**
//! (§2/§2a/§2b/§3/Appendix) via a standalone Python reference implementation
//! that follows only the ADR — it is never produced by calling the encoder
//! under test. That is exactly the §12 item-18 "no shared implementation code"
//! reproducibility check: if the Rust encoder and the independent derivation
//! ever disagree, one of them is wrong, and the test catches it.
//!
//! The CULV boundary table, one BigInt encoding, and one rational encoding
//! were additionally cross-checked by hand against §2a's own worked table
//! before being trusted here.

use num_bigint::BigInt;
use num_rational::BigRational;

use super::*;
use crate::model::condition::{Condition, ConditionClassification};
use crate::model::config::ConfigurationParameter;
use crate::model::declared::{Declared, DeclaredCategory};
use crate::model::derivation::{CandidateDerivation, CandidateRole, DerivationExpression};
use crate::model::direction::{Direction, DirectionRule};
use crate::model::evidence::{EvidenceDeclaration, EvidenceTypeTag};
use crate::model::guard::{Atom, ComparisonOperator, Guard};
use crate::model::literal::Literal;
use crate::model::opaque::{
    DimensionSymbol, EnumValue, LineageId, OccurrenceId, OpaqueValue, PrecedenceOrdinal,
    VersionToken,
};
use crate::model::strategy::Strategy;
use crate::numeric::ExactNumber;

// --- small constructors ----------------------------------------------------

fn occ(handle: u64) -> OccurrenceId {
    OccurrenceId::new(OpaqueValue::new(handle))
}
fn lineage(handle: u64) -> LineageId {
    LineageId::new(OpaqueValue::new(handle))
}
fn dim(handle: u64) -> DimensionSymbol {
    DimensionSymbol::new(OpaqueValue::new(handle))
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn integer_literal(value: i64, dimension: u64) -> Literal {
    Literal::Numeric {
        value: ExactNumber::from_integer(value),
        dimension: dim(dimension),
    }
}

// ===========================================================================
// §2a — CULV, tested as a pure length primitive (§12 item 9a).
// ===========================================================================

#[test]
fn culv_matches_the_adr_worked_table_exactly() {
    // §2a's own boundary table, hand-transcribed. These are the primary test
    // of the grammar's single-octet ↔ multi-octet transitions.
    let table: [(u64, &str); 9] = [
        (0, "00"),
        (1, "01"),
        (127, "7f"),
        (128, "8100"),
        (255, "817f"),
        (256, "8200"),
        (4_294_967_295, "8fffffff7f"),
        (4_294_967_296, "9080808000"),
        (18_446_744_073_709_551_615, "81ffffffffffffffff7f"),
    ];
    for (value, expected) in &table {
        assert_eq!(hex(&culv_encode(*value)), *expected, "CULV({value})");
    }
    // u64::MAX + 1 == 2^64: a 9-byte magnitude 01 00..00 (65 bits).
    let two_pow_64: BigInt = BigInt::from(1u8) << 64u32;
    let (_sign, magnitude) = two_pow_64.to_bytes_be();
    assert_eq!(
        hex(&culv_encode_magnitude(&magnitude)),
        "82808080808080808000"
    );
}

#[test]
fn culv_zero_is_exactly_one_octet() {
    assert_eq!(culv_encode(0), vec![0x00]);
    // A magnitude of all-zero bytes (or empty) is also the value zero.
    assert_eq!(culv_encode_magnitude(&[0, 0, 0]), vec![0x00]);
    assert_eq!(culv_encode_magnitude(&[]), vec![0x00]);
}

#[test]
fn culv_round_trips_and_is_minimal() {
    for value in [
        0u64,
        1,
        127,
        128,
        255,
        256,
        16_384,
        u32::MAX as u64,
        u64::MAX,
    ] {
        let encoded = culv_encode(value);
        let (magnitude, consumed) = culv_decode(&encoded).expect("canonical");
        assert_eq!(consumed, encoded.len());
        let round = if magnitude == [0u8] {
            0
        } else {
            let mut acc = 0u64;
            for byte in &magnitude {
                acc = (acc << 8) | u64::from(*byte);
            }
            acc
        };
        assert_eq!(round, value, "CULV round-trip for {value}");
    }
}

#[test]
fn culv_rejects_non_canonical_leading_zero_group() {
    // §12 item 9b: 0x80 0x01 is a redundant leading all-zero group encoding of
    // 1, whose canonical form is the single octet 0x01. It must be rejected.
    assert_eq!(culv_decode(&[0x80, 0x01]), Err(CulvError::NonCanonical));
    // 0x00 alone (zero) is canonical and must still be accepted.
    assert_eq!(culv_decode(&[0x00]), Ok((vec![0u8], 1)));
}

#[test]
fn culv_rejects_a_truncated_sequence() {
    // §2a: a sequence that never emits a continuation-bit-0 octet is truncated.
    assert_eq!(culv_decode(&[0x81]), Err(CulvError::Truncated));
    assert_eq!(culv_decode(&[0x81, 0x80]), Err(CulvError::Truncated));
    assert_eq!(culv_decode(&[]), Err(CulvError::Truncated));
}

#[test]
fn culv_grammar_is_total_beyond_u64_without_allocating_the_magnitude() {
    // §12 item 9d: a length value beyond u64::MAX is encodable at the
    // length-prefix grammar level alone. Encode 2^64 as a *length* directly.
    let beyond_u64: BigInt = BigInt::from(1u8) << 64u32;
    let (_sign, magnitude) = beyond_u64.to_bytes_be();
    let culv = culv_encode_magnitude(&magnitude);
    // Non-empty, self-delimiting (final octet has continuation bit clear).
    assert!(culv.last().is_some_and(|b| b & 0x80 == 0));
    assert!(culv.iter().rev().skip(1).all(|b| b & 0x80 != 0));
}

// ===========================================================================
// §2a — BigInt (tag 0x10).
// ===========================================================================

#[test]
fn bigint_canonical_vectors() {
    let cases: [(i64, &str); 5] = [
        (0, "1000"),
        (1, "10010101"),
        (-1, "10020101"),
        (255, "100101ff"),
        (256, "1001020100"),
    ];
    for (value, expected) in cases {
        assert_eq!(
            hex(&frame_bigint(&BigInt::from(value))),
            expected,
            "BigInt({value})"
        );
    }
}

#[test]
fn bigint_beyond_64_bits_is_not_truncated() {
    // §12 item 9: a magnitude exceeding 64 bits, distinct from any value
    // sharing its low 64 bits.
    let two_pow_64: BigInt = BigInt::from(1u8) << 64u32;
    assert_eq!(hex(&frame_bigint(&two_pow_64)), "100109010000000000000000");
    assert_eq!(
        hex(&frame_bigint(&(-two_pow_64.clone()))),
        "100209010000000000000000"
    );
    // Its low 64 bits are all zero, but it is not zero and not equal to 0's frame.
    assert_ne!(frame_bigint(&two_pow_64), frame_bigint(&BigInt::from(0)));
}

#[test]
fn bigint_magnitude_exceeding_255_bytes_crosses_the_culv_octet_boundary() {
    // §12 item 9c: a 256-byte magnitude forces the CULV byte-count itself to
    // become a two-octet value (256 -> 0x82 0x00).
    let magnitude_256_bytes = BigInt::from(1u8) << (255u32 * 8); // top byte 0x01, 256 bytes total
    let frame = frame_bigint(&magnitude_256_bytes);
    // tag 0x10, sign 0x01, then CULV(256) == 82 00, then 256 magnitude bytes.
    assert_eq!(&frame[..4], &[0x10, 0x01, 0x82, 0x00]);
    assert_eq!(frame.len(), 2 + 2 + 256);
    assert_eq!(frame[4], 0x01); // most-significant magnitude byte
    assert!(frame[5..].iter().all(|b| *b == 0)); // remaining 255 bytes zero
}

// ===========================================================================
// §2a — Rational (tag 0x04).
// ===========================================================================

#[test]
fn rational_normalizes_equal_values_to_identical_bytes() {
    // §12 item 11: 1/2, 2/4, -1/-2 all reduce to numerator 1, denominator 2.
    let half = BigRational::new(BigInt::from(1), BigInt::from(2));
    let two_quarters = BigRational::new(BigInt::from(2), BigInt::from(4));
    let neg_over_neg = BigRational::new(BigInt::from(-1), BigInt::from(-2));
    assert_eq!(hex(&frame_rational(&half)), "041001010110010102");
    assert_eq!(frame_rational(&half), frame_rational(&two_quarters));
    assert_eq!(frame_rational(&half), frame_rational(&neg_over_neg));
}

#[test]
fn rational_zero_is_zero_over_one() {
    let zero = BigRational::new(BigInt::from(0), BigInt::from(1));
    assert_eq!(hex(&frame_rational(&zero)), "04100010010101");
}

#[test]
fn rational_negative_numerator_carries_the_sign() {
    // §12 item 10: a negative numerator gets sign octet 2; the denominator
    // stays positive (sign octet 1).
    let neg = BigRational::new(BigInt::from(-3), BigInt::from(4));
    assert_eq!(hex(&frame_rational(&neg)), "041002010310010104");
}

// ===========================================================================
// §2 — enum semantic tags (§12 item 12).
// ===========================================================================

#[test]
fn enum_semantic_tags_are_pinned_independent_of_rust_spelling() {
    // These bytes are Appendix Table I's, not Rust's. A hypothetical rename of
    // any variant below MUST NOT change these expected values — that is the
    // whole point of §2b, and this test would fail if the mapping drifted.
    assert_eq!(
        hex(&frame_condition_classification(
            &ConditionClassification::Blocking
        )),
        "1100"
    );
    assert_eq!(
        hex(&frame_condition_classification(
            &ConditionClassification::Temporary
        )),
        "1101"
    );
    assert_eq!(hex(&frame_direction(&Direction::Buy)), "1100");
    assert_eq!(hex(&frame_direction(&Direction::Sell)), "1101");
    assert_eq!(
        hex(&frame_comparison_operator(ComparisonOperator::Less)),
        "1100"
    );
    assert_eq!(
        hex(&frame_comparison_operator(ComparisonOperator::LessOrEqual)),
        "1101"
    );
    assert_eq!(
        hex(&frame_comparison_operator(ComparisonOperator::Greater)),
        "1102"
    );
    assert_eq!(
        hex(&frame_comparison_operator(
            ComparisonOperator::GreaterOrEqual
        )),
        "1103"
    );
    assert_eq!(
        hex(&frame_comparison_operator(ComparisonOperator::Equal)),
        "1104"
    );
    assert_eq!(
        hex(&frame_comparison_operator(ComparisonOperator::NotEqual)),
        "1105"
    );
    assert_eq!(
        hex(&frame_evidence_type_tag(&EvidenceTypeTag::Numeric)),
        "1100"
    );
    assert_eq!(
        hex(&frame_evidence_type_tag(&EvidenceTypeTag::Boolean)),
        "1101"
    );
    assert_eq!(
        hex(&frame_evidence_type_tag(&EvidenceTypeTag::Enum)),
        "1102"
    );
    assert_eq!(hex(&frame_candidate_role(&CandidateRole::Entry)), "1100");
    assert_eq!(hex(&frame_candidate_role(&CandidateRole::StopLoss)), "1101");
    assert_eq!(
        hex(&frame_candidate_role(&CandidateRole::TakeProfit)),
        "1102"
    );
}

// ===========================================================================
// §2 — unordered collections (§12 item 8).
// ===========================================================================

#[test]
fn unordered_collection_is_permutation_invariant() {
    let one = frame_opaque(1);
    let two = frame_opaque(2);
    let three = frame_opaque(3);
    let a = frame_collection(vec![three.clone(), one.clone(), two.clone()]);
    let b = frame_collection(vec![one, two, three]);
    assert_eq!(a, b);
    assert_eq!(
        hex(&a),
        "0900000003070000000000000001070000000000000002070000000000000003"
    );
}

#[test]
fn unordered_collection_deduplicates_identical_frames() {
    // ADR-0003 §6: a repeated identical canonical frame contributes once; the
    // count prefix is the post-dedup count.
    let dd = frame_collection(vec![frame_opaque(5), frame_opaque(5)]);
    assert_eq!(hex(&dd), "0900000001070000000000000005");
}

// ===========================================================================
// §2 — declaration-state distinctions (§12 item 13).
// ===========================================================================

#[test]
fn absent_present_empty_and_out_of_domain_are_distinct_frames() {
    let absent = frame_declared::<u8>(&Declared::Absent, |_| unreachable!());
    let present_empty = frame_declared(&Declared::Present(Vec::<EnumValue>::new()), |values| {
        frame_collection(
            values
                .iter()
                .map(|v| frame_opaque(v.identity_handle()))
                .collect(),
        )
    });
    let out_of_domain =
        frame_declared_category::<u8>(&DeclaredCategory::OutOfDomain, |_| unreachable!());
    assert_eq!(hex(&absent), "01");
    assert_eq!(hex(&present_empty), "0900000000");
    assert_eq!(hex(&out_of_domain), "0d");
    assert_ne!(absent, present_empty);
    assert_ne!(absent, out_of_domain);
    assert_ne!(present_empty, out_of_domain);
}

// ===========================================================================
// §2 — nested record / tagged-union framing (§12 item 14).
// ===========================================================================

#[test]
fn multiply_derivation_nests_through_0f_0e_without_ambiguity() {
    // A Multiply nesting a boxed sub-expression and a scalar literal.
    let expr = DerivationExpression::Multiply {
        operand: Box::new(DerivationExpression::EvidenceReference(lineage(7))),
        scalar: integer_literal(3, 1),
    };
    let frame = frame_derivation(&expr);
    // 0f 05 | (0f 01 | opaque 7) | (0e 00 | rational 3/1 | opaque dim 1)
    let expected = concat_hex(&[
        "0f05",
        "0f01",
        "070000000000000007",
        "0e00",
        "041001010310010101", // rational 3/1
        "070000000000000001", // dimension 1
    ]);
    assert_eq!(hex(&frame), expected);
}

fn concat_hex(parts: &[&str]) -> String {
    parts.concat()
}

// ===========================================================================
// The golden fixture Strategy and its end-to-end vectors.
// ===========================================================================

/// The minimal complete Strategy used for the end-to-end golden vector.
///
/// Its content mirrors, field for field, the fixture in the independent Python
/// derivation that produced [`GOLDEN_PREIMAGE_HEX`]. Occurrence identities are
/// arbitrary and deliberately *not* mirrored — they are never encoded.
fn golden_strategy() -> Strategy {
    Strategy::new(
        occ(1),
        lineage(1),
        VersionToken::new(OpaqueValue::new(1)),
        vec![EvidenceDeclaration::numeric(occ(100), lineage(100), dim(1))],
        vec![ConfigurationParameter::new(
            occ(200),
            lineage(200),
            dim(1),
            integer_literal(1, 1),
            integer_literal(10, 1),
            true,
        )],
        vec![Condition::new(
            occ(300),
            lineage(300),
            ConditionClassification::Blocking,
            Guard::from_atoms(vec![Atom::new(
                lineage(100),
                ComparisonOperator::Greater,
                integer_literal(50, 1),
            )]),
        )],
        vec![DirectionRule::new(
            occ(400),
            lineage(400),
            PrecedenceOrdinal::new(OpaqueValue::new(9)),
            Guard::always_applicable(),
            Direction::Buy,
        )],
        vec![CandidateDerivation::new(
            occ(500),
            lineage(500),
            CandidateRole::Entry,
            DerivationExpression::EvidenceReference(lineage(100)),
        )],
    )
}

/// The full domain-separated preimage of [`golden_strategy`], hand-derived by
/// the independent Python reference (never by this crate's encoder).
const GOLDEN_PREIMAGE_HEX: &str = "6f7264696e612d7265762d6964656e74010a010000000701070000000000000001020700000000000000010309000000010a060000000401070000000000000064020b11000307000000000000000104010409000000010a0300000005010700000000000000c803070000000000000001040e00041001010110010101070000000000000001050e00041001010a100101010700000000000000010606010509000000010a02000000030107000000000000012c020b1100030a07000000010109000000010a080000000301070000000000000064021102030e000410010132100101010700000000000000010609000000010a05000000040107000000000000019002070000000000000009030a0700000001010900000000040b11000709000000010a0400000003010700000000000001f4021100030f01070000000000000064";

#[test]
fn end_to_end_preimage_matches_the_independent_derivation() {
    // §12 item 18 + item 15: the whole preimage, DOMAIN || 0x01 || ROOT, equals
    // the byte string an independent implementation of the ADR produced.
    let preimage = revision_preimage(&golden_strategy());
    assert_eq!(hex(&preimage), GOLDEN_PREIMAGE_HEX);
    // §12 item 15: the DOMAIN || SCHEME_VERSION prefix is present verbatim.
    assert!(hex(&preimage).starts_with("6f7264696e612d7265762d6964656e7401"));
}

#[test]
fn end_to_end_digest_and_identity_match_blake3_of_the_independent_preimage() {
    // Assert the final digest AND the final RevisionIdentity — not the digest
    // alone (§12 header). The digest is BLAKE3 of the *hand-derived* preimage,
    // so this does not merely re-hash the encoder's own output.
    let independent_preimage = decode_hex(GOLDEN_PREIMAGE_HEX);
    let expected_digest = *blake3::hash(&independent_preimage).as_bytes();

    let identity = compute_revision_identity(&golden_strategy()).expect("well-formed fixture");
    assert_eq!(identity.scheme_version(), 0x01);
    assert_eq!(identity.digest(), &expected_digest);

    // §12 item 1: recomputation from identical content is identical.
    let again = compute_revision_identity(&golden_strategy()).expect("well-formed fixture");
    assert_eq!(identity, again);
}

fn decode_hex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("hex"))
        .collect()
}

#[test]
fn configuration_parameter_uses_positions_1_3_4_5_6_never_2() {
    // §12 item 10a: exactly five field-position/value pairs, positions
    // 1, 3, 4, 5, 6, with no position 2. The record's field count is 5.
    let parameter = ConfigurationParameter::new(
        occ(200),
        lineage(200),
        dim(1),
        integer_literal(1, 1),
        integer_literal(10, 1),
        true,
    );
    let frame = encode_configuration_parameter(&parameter);
    // 0x0A | type 3 | count 00000005 | (1 ..)(3 ..)(4 ..)(5 ..)(6 ..)
    assert_eq!(&frame[..2], &[TAG_RECORD, RECORD_CONFIGURATION_PARAMETER]);
    assert_eq!(&frame[2..6], &[0, 0, 0, 5]);
    // The five position tags actually emitted, in order.
    let positions = field_positions(&frame[6..]);
    assert_eq!(positions, vec![1, 3, 4, 5, 6]);
    assert!(!positions.contains(&2));
}

/// Recover the top-level field-position tags from a record's field region by
/// walking each field's self-delimiting frame. Test-only structural helper.
fn field_positions(mut fields: &[u8]) -> Vec<u8> {
    let mut positions = Vec::new();
    while !fields.is_empty() {
        positions.push(fields[0]);
        let consumed = 1 + frame_len(&fields[1..]);
        fields = &fields[consumed..];
    }
    positions
}

/// The byte length of one self-delimiting frame at the front of `bytes`.
fn frame_len(bytes: &[u8]) -> usize {
    let tag = bytes[0];
    match tag {
        TAG_ABSENT | TAG_CATEGORY_OUT_OF_DOMAIN => 1,
        TAG_BOOL => 2,
        TAG_ENUM => 2,
        TAG_OPAQUE => 1 + 8,
        TAG_U64 | TAG_I64 => 1 + 8,
        TAG_BIGINT => {
            let sign = bytes[1];
            if sign == 0 {
                2
            } else {
                let (count, consumed) = culv_decode(&bytes[2..]).expect("canonical");
                let magnitude = be_to_usize(&count);
                2 + consumed + magnitude
            }
        }
        TAG_RATIONAL => {
            let num = frame_len(&bytes[1..]);
            let den = frame_len(&bytes[1 + num..]);
            1 + num + den
        }
        TAG_STRING => {
            let len = u32::from_be_bytes(bytes[1..5].try_into().unwrap()) as usize;
            1 + 4 + len
        }
        TAG_LITERAL => match bytes[1] {
            0 => {
                let rat = frame_len(&bytes[2..]);
                let dimn = frame_len(&bytes[2 + rat..]);
                2 + rat + dimn
            }
            1 => 2 + frame_len(&bytes[2..]),
            2 => 2 + frame_len(&bytes[2..]),
            other => panic!("unexpected literal variant {other}"),
        },
        TAG_DERIVATION => {
            let payload = &bytes[2..];
            match bytes[1] {
                0 => 2 + frame_len(payload),
                1 | 2 => 2 + frame_len(payload),
                3 | 4 => {
                    let left = frame_len(payload);
                    let right = frame_len(&payload[left..]);
                    2 + left + right
                }
                5 => {
                    let operand = frame_len(payload);
                    let scalar = frame_len(&payload[operand..]);
                    2 + operand + scalar
                }
                other => panic!("unexpected derivation variant {other}"),
            }
        }
        TAG_CATEGORY_ONE => 1 + frame_len(&bytes[1..]),
        TAG_CATEGORY_MULTIPLE => {
            let first = frame_len(&bytes[1..]);
            let second = frame_len(&bytes[1 + first..]);
            let rest = frame_len(&bytes[1 + first + second..]);
            1 + first + second + rest
        }
        TAG_SEQUENCE | TAG_COLLECTION => {
            let count = u32::from_be_bytes(bytes[1..5].try_into().unwrap()) as usize;
            let mut offset = 5;
            for _ in 0..count {
                offset += frame_len(&bytes[offset..]);
            }
            offset
        }
        TAG_RECORD => {
            let count = u32::from_be_bytes(bytes[2..6].try_into().unwrap()) as usize;
            let mut offset = 6;
            for _ in 0..count {
                offset += 1; // position tag
                offset += frame_len(&bytes[offset..]);
            }
            offset
        }
        other => panic!("unexpected tag {other:#x}"),
    }
}

fn be_to_usize(bytes: &[u8]) -> usize {
    bytes.iter().fold(0usize, |acc, b| (acc << 8) | *b as usize)
}

// ===========================================================================
// §12 items 2/3/4/6/7 — identity sensitivity and insensitivity.
// ===========================================================================

#[test]
fn an_id_bearing_scalar_change_changes_the_identity() {
    // §12 item 2: a changed condition threshold (50 -> 51) changes preimage and
    // identity.
    let base = golden_strategy();
    let mut changed = golden_strategy();
    changed.conditions = Declared::Present(vec![Condition::new(
        occ(300),
        lineage(300),
        ConditionClassification::Blocking,
        Guard::from_atoms(vec![Atom::new(
            lineage(100),
            ComparisonOperator::Greater,
            integer_literal(51, 1),
        )]),
    )]);
    assert_ne!(revision_preimage(&base), revision_preimage(&changed));
    assert_ne!(
        compute_revision_identity(&base).unwrap(),
        compute_revision_identity(&changed).unwrap()
    );
}

#[test]
fn changing_only_occurrence_ids_leaves_the_identity_unchanged() {
    // §9 item (a) / §12 item 6: changing only OccurrenceId values leaves the
    // identity unchanged, because occurrence is never walked.
    let base = golden_strategy();
    let mut reminted = golden_strategy();
    // Re-mint the envelope occurrence and one child occurrence to arbitrary
    // new handles; nothing else changes.
    reminted.occurrence = occ(9_999);
    if let Declared::Present(conditions) = &mut reminted.conditions {
        conditions[0].occurrence = occ(8_888);
    }
    assert_eq!(revision_preimage(&base), revision_preimage(&reminted));
    assert_eq!(
        compute_revision_identity(&base).unwrap(),
        compute_revision_identity(&reminted).unwrap()
    );
}

#[test]
fn reminting_all_occurrence_ids_leaves_the_identity_unchanged() {
    // §9 item (b): re-mint every occurrence id throughout an otherwise-identical
    // Strategy; the identity is unchanged.
    let base = golden_strategy();
    let reminted = Strategy::new(
        occ(70_001),
        lineage(1),
        VersionToken::new(OpaqueValue::new(1)),
        vec![EvidenceDeclaration::numeric(
            occ(70_100),
            lineage(100),
            dim(1),
        )],
        vec![ConfigurationParameter::new(
            occ(70_200),
            lineage(200),
            dim(1),
            integer_literal(1, 1),
            integer_literal(10, 1),
            true,
        )],
        vec![Condition::new(
            occ(70_300),
            lineage(300),
            ConditionClassification::Blocking,
            Guard::from_atoms(vec![Atom::new(
                lineage(100),
                ComparisonOperator::Greater,
                integer_literal(50, 1),
            )]),
        )],
        vec![DirectionRule::new(
            occ(70_400),
            lineage(400),
            PrecedenceOrdinal::new(OpaqueValue::new(9)),
            Guard::always_applicable(),
            Direction::Buy,
        )],
        vec![CandidateDerivation::new(
            occ(70_500),
            lineage(500),
            CandidateRole::Entry,
            DerivationExpression::EvidenceReference(lineage(100)),
        )],
    );
    assert_eq!(
        compute_revision_identity(&base).unwrap(),
        compute_revision_identity(&reminted).unwrap()
    );
}

#[test]
fn an_id_classified_operative_change_does_change_identity() {
    // §9 item (c): a genuinely ID-classified operative change — a Direction
    // outcome Buy -> Sell — DOES change the identity.
    let base = golden_strategy();
    let mut changed = golden_strategy();
    changed.direction_rules = Declared::Present(vec![DirectionRule::new(
        occ(400),
        lineage(400),
        PrecedenceOrdinal::new(OpaqueValue::new(9)),
        Guard::always_applicable(),
        Direction::Sell,
    )]);
    assert_ne!(
        compute_revision_identity(&base).unwrap(),
        compute_revision_identity(&changed).unwrap()
    );
}

#[test]
fn collection_member_permutation_leaves_the_identity_unchanged() {
    // §12 item 8: permuting a Vec whose canonical content is identical leaves
    // the identity unchanged. Two evidence declarations, two orders.
    let build = |first_handle: u64, second_handle: u64| {
        let mut strategy = golden_strategy();
        strategy.evidence_declarations = Declared::Present(vec![
            EvidenceDeclaration::numeric(occ(100), lineage(first_handle), dim(1)),
            EvidenceDeclaration::numeric(occ(101), lineage(second_handle), dim(1)),
        ]);
        strategy
    };
    let a = build(100, 200);
    let b = build(200, 100);
    assert_eq!(
        compute_revision_identity(&a).unwrap(),
        compute_revision_identity(&b).unwrap()
    );
}

#[test]
fn an_empty_minimal_strategy_yields_a_stable_identity_not_a_panic() {
    // §12 item 17: a wholly-undeclared envelope is representable and yields a
    // stable, reproducible identity. Its preimage matches the independent
    // derivation.
    let bare = Strategy::wholly_undeclared(occ(7));
    let preimage = revision_preimage(&bare);
    assert_eq!(
        hex(&preimage),
        "6f7264696e612d7265762d6964656e74010a01000000070101020103010401050106010701"
    );
    let identity = compute_revision_identity(&bare).expect("total over all typed states");
    assert_eq!(
        identity,
        compute_revision_identity(&Strategy::wholly_undeclared(occ(42))).unwrap()
    );
}

// ===========================================================================
// §5 — RevisionIdentity value semantics (§12 item 16).
// ===========================================================================

#[test]
fn identities_with_different_scheme_versions_are_never_equal() {
    // §12 item 16 / §5: the same 32 digest bytes under two different
    // scheme_version values produce unequal RevisionIdentity values.
    let digest = [0xABu8; 32];
    let v1 = RevisionIdentity::from_parts(0x01, digest);
    let v2 = RevisionIdentity::from_parts(0x02, digest);
    assert_ne!(v1, v2);
    // Same version, same digest — equal.
    assert_eq!(v1, RevisionIdentity::from_parts(0x01, digest));
}
