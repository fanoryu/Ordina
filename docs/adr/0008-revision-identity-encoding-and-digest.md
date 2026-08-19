# ADR-0008: Revision-Identity Encoding and Digest

- **Status:** ACCEPTED
- **Date:** 2026-08-20
- **Accepted by:** Product Board, PBR-0022
- **Decision date:** 2026-08-20
- **Deciders:** Principal Engineer; accepted by Product Board
- **Related:** ADR-0002 (ACCEPTED, PBR-0006 — semantic model, cited, not
  amended); ADR-0003 (ACCEPTED, PBR-0007 — §5 pipeline ordering, §6 ID/CX/PX/INV
  classification, §7 revision-identity invariants I1–I10, §10 configuration-vs-
  revision set, §13 threat table, §16 deferred decisions — all cited, none
  reopened); ADR-0004 (ACCEPTED — Well-Formedness gate, I9 precondition,
  unaffected); ADR-0005 (ACCEPTED — Rust toolchain, dependency-approval
  discipline this ADR's digest-crate choice must respect, §7.2 float-free
  discipline, §7.3 arbitrary-precision `BigRational` domain, §7.4 closed-
  category discipline, §7.5 anti-incidental-order principle); ADR-0007
  (ACCEPTED — `OccurrenceId`/order excluded from revision identity, unaffected,
  applied here); RFC-0004 (ACCEPTED, PBR-0021 — authoring/translation semantics
  and revision-identity-bearing convergence, unaffected); PBA-0169 (drafting
  authorization); PBA-0170 (first bounded remediation authorization); PBA-0171
  (second bounded remediation authorization); PBA-0172 (third bounded
  remediation authorization, closing the sole finding from the independent
  final re-review of the second remediation); ARR-0058 (independent final
  re-review, PASS — zero BLOCKER/MAJOR/MINOR/NOTE); PBA-0173 (acceptance
  authorization), PBR-0022 (Product Board acceptance), ER-0144 (acceptance
  synchronization report)
- **Review lineage:** PBA-0169 → drafting → independent review (FAIL, 2
  BLOCKER, 2 MAJOR) → PBA-0170 → first remediation → independent re-review
  (FAIL, 1 BLOCKER, 1 MAJOR) → PBA-0171 → second remediation → independent
  re-review (FAIL, 1 BLOCKER) → PBA-0172 → third remediation → ARR-0058
  (independent final re-review, PASS; zero BLOCKER/MAJOR/MINOR/NOTE) →
  PBA-0173 → PBR-0022 (Product Board acceptance)

- **Remediation (PBA-0170):** the first remediation cycle addressed 2 BLOCKER
  (missing canonical field/tag registry; fixed-width rational encoding
  incompatible with arbitrary-precision `BigRational`) and 2 MAJOR
  (Rust-variant-name-dependent enum spelling; `RevisionIdentity` scheme/version
  not externally distinguishable) findings from the drafted version's
  independent review.

- **Second remediation (PBA-0171):** independent re-review of the first
  remediation found 1 new BLOCKER (Appendix Table D's `ConfigurationParameter`
  field-tag positions contradicted the Slice Contract's own confirmed §3.9
  positions) and 1 new MAJOR (the BigInt magnitude-length field's fixed `u32`
  width contradicted Slice Contract §4.2's explicitly unbounded magnitude,
  despite this ADR's own totality claim) — both remediated in this revision.
  Appendix Table B (`Strategy`) was independently re-verified against
  `required_property.rs`'s `check_1` and confirmed correct, unchanged.

- **Third remediation (PBA-0172):** independent final re-review of the second
  remediation found the "fixed 4-byte `u32`" defect had been replaced with a
  structurally identical defect at a different scale — a single fixed byte
  `L` (`0 < L ≤ 255`) still capped the length-of-magnitude value, falsifying
  the ADR's own "for any reason, ever" totality claim. §2a's length grammar is
  replaced outright with a Canonical Unsigned Length Varint (CULV) — a
  base-128, continuation-bit, self-delimiting encoding with no fixed-width
  field of any kind, fully specified normatively rather than referenced
  externally. Tag `0x10` is re-stated completely; a totality proof is added
  distinguishing semantic encodability from practical resource availability.

## Context

ADR-0003 defines revision identity architecturally — a deterministic,
content-addressed function of a Strategy's canonical, ID-classified semantic
content (§7, I1–I8) — but explicitly defers the concrete mechanics needed to
implement it. §16 ("Deferred Decisions") names "concrete serialization format;
concrete hash/digest algorithm" outright. §13's threat table states identity
input domain separation is "an implementation note; not decided here." ADR-0002,
ADR-0004, and ADR-0005 each independently corroborate the same deferral;
ADR-0005 §6's cross-ADR impact table states plainly: "Canonicalization and
revision identity untouched and not computed by Slice 1; hash algorithm remains
unselected."

That deferral was appropriate when written — Slice 1 had not yet reached a
point where any code needed to compute a revision identity. It has now: N-22-B-1
(PBA-0168) requires a public `ordina` function that takes a valid canonical
`Strategy` and returns a `RevisionIdentity`, and no accepted document specifies
how to turn canonical semantic content into deterministic identity bytes. A
prior N-22-B-1 implementation attempt (PBA-0168 cycle) correctly identified this
as a genuine, unresolved architecture gap rather than inventing an answer, and
stopped. PBA-0169 authorized this ADR to close exactly that gap; a first
drafted version failed independent review (2 BLOCKER, 2 MAJOR); PBA-0170
authorizes this bounded remediation.

What this ADR does **not** revisit: whether a property is ID/CX/PX/INV (ADR-0003
§6, closed), what the revision-identity invariants require semantically
(ADR-0003 §7, closed), or whether `OccurrenceId`/its order participates in
revision identity (ADR-0007, closed — it does not). This ADR is purely the
mechanical bridge from "canonical ID-bearing content, as ADR-0003 §6 already
classifies it" to "deterministic identity bytes."

## Decision

We will compute revision identity as a **BLAKE3-256 digest, plain unkeyed hash
mode**, over an explicitly domain-separated, self-describing canonical binary
encoding (a purpose-built TLV framing, fully specified below and in the
Appendix, not CBOR or any other general-purpose format) of exactly the
ADR-0003 §6 ID-classified content of a Well-Formed Strategy, exposed through
one minimal public function,
`compute_revision_identity(&Strategy) -> Result<RevisionIdentity, RevisionIdentityError>`,
where **`RevisionIdentity`'s logical value is `{ scheme_version: u8, digest: [u8; 32] }`**
— the scheme version travels alongside the digest, not only inside it.

### 1. Revision-identity input model

The identity input is built exclusively from fields ADR-0003 §6 classifies
**ID** (including the §6.1 operative-rule-content default), after §6's
canonicalization rules have already been applied to the in-memory `Strategy`
(CX collapsed, PX excluded, INV never present in a Well-Formed Strategy per
ADR-0004's gate, I9). **`occurrence` (`OccurrenceId`) and any occurrence-derived
order are explicitly never walked, on every construct, without exception** —
ADR-0007's ruling, applied here, not re-derived.

Every other opaque-domain scalar the model uses (`LineageId` in its
reference-bearing role, `DimensionSymbol`, `EnumValue`, `VersionToken`,
`PrecedenceOrdinal`) **is** walked and encoded, distinct from `OccurrenceId`.
This is a deliberate, authority-grounded distinction, not an oversight: Revision
4 §C.1 names `OccurrenceId` as a **sixth**, addressing/conformance-only opaque
domain, explicitly separate from the other five "**semantic** opaque domains"
(`src/model/opaque.rs`'s own module doc) that ADR-0003 §6's classification
table treats as ordinary ID or CX-cosmetic content. By the time a `Strategy`
reaches this ADR's hashing step it has already passed ADR-0003 §5's
`Well-Formed → canonicalize → identity` pipeline — canonicalization (Slice
Contract §3.3.1: "both the values of an opaque domain and a deterministic
total order over them arrive as part of the already-canonical input") is
already responsible for making these five domains' handles a **stable,
deterministic function of confirmed semantic content** before this ADR ever
sees them; this ADR's job is only to encode whatever canonical bytes are
already there, faithfully and without further interpretation, exactly as it
already does for every other ID-classified field. Reopening how canonicalization
achieves that stability is ADR-0003/RFC-0004 territory, not this ADR's.

Because `Declared<T>` and `DeclaredCategory<T>` already distinguish
present/absent/multiple/out-of-domain per Slice Contract §3 and ADR-0002 §10b,
the encoding (§2, Appendix) represents each declaration state as an explicit
tag — never an implicit default, and never by omitting the field.

The complete construct/field mapping is in the **Appendix** below, derived
directly from the live `src/model/**` types and, where the Slice Contract or
its check implementations already assign a stable per-construct property
position (confirmed directly in source: CandidateDerivation §3.8 "lineage 1,
role 2, expression 3"; Condition §3.10 "position 1 (lineage)... position 2
(classification)... position 3 (guard)"; Strategy's semantic-model-version at
"position 2"), that **existing** position numbering is reused as the field tag
— this ADR invents no new numbering where the contract already has one, and
follows the same declared-field-order convention for the remaining fields
whose numeric position is not separately quoted in a check's own doc comment.

### 2. Canonical byte/structural representation — TLV framing

The encoding is a **self-describing, purpose-built canonical binary framing**,
specified completely enough — together with the Appendix's per-construct
tables — that two independent conforming implementations produce byte-identical
preimages without sharing code. Every value is encoded as `(tag: u8, payload)`:

| Tag | Meaning | Payload |
|---|---|---|
| `0x01` | Absent (`Declared::Absent` / `DeclaredCategory::Absent`) | none — zero-length |
| `0x02` | Unsigned integer (bounded, e.g. count/length fields) | 8-byte big-endian `u64` |
| `0x03` | Signed integer (bounded) | 8-byte big-endian two's-complement `i64` |
| `0x04` | Rational (exact, dimension-free `ExactNumber`/`BigRational`) | numerator: tag `0x10` canonical-BigInt frame (signed); denominator: tag `0x10` canonical-BigInt frame, sign fixed positive; both **already reduced to lowest terms** (§2a) |
| `0x05` | UTF-8 string (opaque-domain rendering used only where §2c requires it; not used for enum spelling — see §2b) | 4-byte big-endian length-prefix (`u32`, bytes, not chars) + UTF-8 bytes, **NFC-normalized, Unicode 15.0 tables** (§9) before length is computed |
| `0x06` | Boolean | 1 byte, `0x00`/`0x01` only — any other byte value is a framing error |
| `0x07` | Opaque-domain reference (`LineageId` reference-bearing role, `DimensionSymbol`, `EnumValue`, `VersionToken`, `PrecedenceOrdinal` — the model's five "semantic opaque domains," never `OccurrenceId`) | 8-byte big-endian `u64` of the referenced value's underlying `OpaqueValue` (§1's canonical-input justification) |
| `0x08` | Ordered sequence (only where a construct's own field is contract-declared order-significant; none currently exist in the model — reserved for future use per §10's Deferred Decisions) | 4-byte big-endian element count (`u32`) + each element's own tagged frame, declared order preserved |
| `0x09` | Unordered collection (every current `Vec<T>` field in the model — `evidence_declarations`, `configuration_parameters`, `conditions`, `direction_rules`, `candidate_derivations`, `declared_values`, `Guard::atoms` — none is contract-named as order-significant; §3.6's guard is an explicitly commutative conjunction, and no other collection's own §3.x text assigns meaning to Vec-insertion order, so ADR-0003 §6's default "Unordered collections — CX→ID(members)" row governs all of them) | 4-byte big-endian element count (`u32`) + each already-canonicalized member's own tagged frame bytes, **then the resulting frames sorted ascending by unsigned byte-lexicographic order of the frame bytes themselves**, then concatenated — deduplicated per ADR-0003 §6 (a repeated identical frame after canonicalization contributes once) |
| `0x0A` | Record (a construct with named ID fields) | 1-byte **record-type tag** (Appendix Table A) + 4-byte big-endian field count (`u32`) + for each field, in the **fixed §3.x property-position order** (Appendix Tables B–H; never Rust declaration order, memory layout, or serde order): a 1-byte **field-position tag** (Appendix, reused directly from the Slice Contract's own property positions where quoted in source, declared-order-consistent otherwise) followed by that field's own tagged value frame |
| `0x0B` | `DeclaredCategory::One(T)` | the wrapped value's own tagged frame |
| `0x0C` | `DeclaredCategory::Multiple{first, second, rest}` | `first`'s frame + `second`'s frame + `rest`'s tag-`0x09` unordered-collection frame (rest is itself an unordered `Vec<T>` beyond the first two — Slice Contract §3.7/§3.10 assign it no order) |
| `0x0D` | `DeclaredCategory::OutOfDomain` | none — zero-length |
| `0x0E` | `Literal` tagged union | 1-byte variant tag (`0`=`Numeric{value, dimension}` → tag-`0x04` frame + tag-`0x07` frame; `1`=`Boolean(bool)` → tag-`0x06` frame; `2`=`Enum(EnumValue)` → tag-`0x07` frame) |
| `0x0F` | `DerivationExpression` tagged union (recursive) | 1-byte variant tag (`0`=`Literal(Literal)`→ tag-`0x0E` frame; `1`=`EvidenceReference(LineageId)`→ tag-`0x07` frame; `2`=`ConfigurationReference(LineageId)`→ tag-`0x07` frame; `3`=`Add{left,right}`→ two nested tag-`0x0F` frames; `4`=`Subtract{left,right}`→ two nested tag-`0x0F` frames; `5`=`Multiply{operand,scalar}`→ one nested tag-`0x0F` frame + one tag-`0x0E` frame) |
| `0x10` | Canonical arbitrary-precision `BigInt` (§2a) | 1-byte sign (`0`=zero, `1`=positive, `2`=negative) + [if non-zero] magnitude byte-count as a **CULV** (§2a — no fixed-width field, genuinely unbounded) + minimal big-endian magnitude bytes (no leading zero byte) |
| `0x11` | Closed-category enum (Appendix Table I; every model enum — `ConditionClassification`, `Direction`, `ComparisonOperator`, `EvidenceTypeTag`, `CandidateRole`) | 1-byte **ADR-fixed semantic tag** (§2b) — never a Rust discriminant, never a variant-name string |

Rules that eliminate ambiguity across the list above, applying uniformly:

- **Integer width/endianness:** all multi-byte fixed-width integers are
  big-endian; `u64`/`i64` fixed 8 bytes, length/count prefixes fixed 4 bytes —
  never variable-length/LEB128 for these bounded fields (unbounded values use
  tag `0x10` instead, never a fixed-width tag).
- **Signedness:** `u64` (tag `0x02`) and `i64` (tag `0x03`) are always
  distinguished by tag, never inferred from value range; tag `0x10`'s sign is
  an explicit leading byte, never two's-complement.
- **String encoding:** UTF-8 only, NFC-normalized against a pinned Unicode
  version (§9), length in bytes. Enum spelling never uses tag `0x05` — see
  tag `0x11`/§2b.
- **Length prefixes:** always present for variable-length payloads — fixed
  4-byte `u32` counts for `0x05`, `0x08`, `0x09`, `0x0A`'s field count (these
  remain bounded: no construct has anywhere near `2^32` fields, evidence
  declarations, or collection members, so a fixed 4-byte count introduces no
  real ceiling); `0x10`'s magnitude instead uses the **self-delimiting
  arbitrary-precision length prefix** defined in §2a, deliberately not a fixed
  width, because the magnitude domain itself (Slice Contract §4.2) is
  explicitly unbounded. Fixed-width payloads (`0x02`, `0x03`, `0x06`, `0x07`,
  `0x11`) carry no length prefix at all.
- **Sequence boundaries:** always an explicit `u32` count, never a sentinel/
  terminator byte.
- **Nested value framing:** every value, at any nesting depth, is the same
  `(tag, payload)` shape — no separate "top-level" rule, and every recursive
  type (`DerivationExpression`) nests the identical outer framing.
- **Absence/presence markers:** `Declared::Absent`/`DeclaredCategory::Absent`
  is tag `0x01`, zero-length; a `Declared`-present field's frame is the
  value's own tag directly (no extra wrapper); `DeclaredCategory`'s three
  further states use tags `0x01`/`0x0B`/`0x0C`/`0x0D` as listed — all four
  states are always structurally distinguishable, and an ID-classified field
  is never omitted from its record frame under any declaration state.
- **Map/set ordering:** unordered collections (tag `0x09`) are sorted by the
  byte-lexicographic order of their own already-encoded member frames — a pure
  function of member content, never insertion/hash/comparator order.
- **Field omission:** no ID-classified field may be omitted from a record
  frame (tag `0x0A`) under any circumstance, including when its state is
  `Absent`/`OutOfDomain` (still an explicit tag). PX/CX-collapsed-away content
  is never part of a record frame — it was never ID-classified.
- **Record-type / field-position collision safety:** a record's type tag
  (Appendix Table A) and its fields' position tags occupy distinct,
  independently-scoped byte positions inside tag `0x0A`'s own payload — a
  field-position byte is never read as an outer value tag and vice versa, so
  no cross-namespace collision is possible by construction.
- **Frame self-delimitation prevents concatenation ambiguity:** because every
  variable-length payload carries its own explicit length (element count for
  sequences/collections, byte length for strings and BigInt magnitudes, field
  count for records) rather than relying on a following tag to imply where it
  ends, two structurally distinct trees (e.g., a two-element sequence `[A, BC]`
  vs. a differently-partitioned `[AB, C]`, where `A`/`B`/`C` are themselves
  framed values) can never produce identical byte streams: each element's own
  length prefix fixes exactly where it ends, so the partition itself is part
  of the encoded bytes, not inferred from content.

#### 2a. Canonical arbitrary-precision integer and rational encoding (BLOCKER-2)

**Historical note (not normative):** an earlier revision of this section used
a fixed 4-byte `u32` magnitude-byte-length, then (second remediation) a
single fixed byte `L` (`0 < L ≤ 255`) bounding the byte-length of the
length-of-magnitude value. Independent re-review found the second scheme
still finite — merely a much larger, but still fixed, ceiling — falsifying
this ADR's own totality claim. **Neither design is normative below.** The
definition in this section replaces both outright with a length encoding
that has no fixed-width field anywhere in its own structure.

**Canonical Unsigned Length Varint (CULV)** — the length primitive used
everywhere a length/count needs to be genuinely unbounded (here, `0x10`'s
magnitude-byte-length). Base-128, continuation-bit, big-endian-digit-order,
fully self-contained — no external LEB128/protobuf specification is assumed
or required; every rule is stated here in full:

- Every non-negative integer `n` is encoded as one or more octets. Each octet
  carries **7 payload bits** in its low-order bits; **bit 7 (the high bit) is
  the continuation bit**: `1` means "another octet follows," `0` means "this
  is the final octet."
- The integer's binary representation is split into 7-bit groups; groups are
  emitted **most-significant group first** (unlike LEB128's traditional
  least-significant-first order — chosen here specifically so no reader
  needs to buffer the whole varint before knowing its numeric value's
  leading digits, and so it composes uniformly with this ADR's existing
  big-endian convention for every other multi-byte field). Every octet
  except the last has its continuation bit set to `1`; the last has it set
  to `0`.
- **Zero** has exactly one encoding: a single octet `0x00` (continuation bit
  `0`, all seven payload bits `0`).
- **Minimality (canonical form):** the encoding uses the fewest octets that
  can represent `n` — equivalently, the first (most-significant) emitted
  7-bit group is never `0000000` unless it is also the only group (i.e., `n
  = 0`). A decoder encountering a leading all-zero payload group followed by
  a continuation bit of `1` (a redundant leading-zero group) has received a
  **non-canonical, invalid** encoding and must reject it — this is the
  varint-level analogue of `0x10`'s own "no leading zero byte" magnitude
  rule, applied one level down.
- **Termination:** an octet sequence that ends without ever emitting a
  continuation bit of `0` (i.e., truncated input) is invalid and must be
  rejected — never interpreted as a partial or default value.
- **No fixed maximum octet count.** Nothing in this grammar states or implies
  an upper bound on how many 7-bit groups a CULV may contain — an
  arbitrarily large `n` is represented by an arbitrarily long (but always
  finite, for any finite `n`) sequence of continuation-bit-`1` octets
  followed by exactly one continuation-bit-`0` octet. This is the property
  that makes CULV genuinely unbounded, as opposed to §2a's two prior,
  rejected designs, both of which capped the *number of length-describing
  bytes* at a fixed constant.

**Concrete boundary examples**, worked through explicitly:

| `n` | Binary | 7-bit groups (MSB-first) | CULV octets (hex) |
|---|---|---|---|
| `0` | `0` | `0000000` | `00` |
| `1` | `1` | `0000001` | `01` |
| `127` | `1111111` | `1111111` | `7F` |
| `128` | `10000000` | `0000001`, `0000000` | `81 00` |
| `255` | `11111111` | `0000001`, `1111111` | `81 7F` |
| `256` | `100000000` | `0000010`, `0000000` | `82 00` |
| `u32::MAX` (`4294967295`) | 32 ones | `0001111`, `1111111`, `1111111`, `1111111`, `1111111` | `8F FF FF FF 7F` |
| `u32::MAX + 1` (`4294967296`, i.e. `2^32`) | `1` followed by 32 zeros (33 bits) | padded to 35 bits (`001` + 32 zeros); groups `0010000`, then four `0000000` | `90 80 80 80 00` |
| `u64::MAX` (`18446744073709551615`) | 64 ones | padded to 70 bits (`000000` + 64 ones); groups `0000001`, then nine `1111111` | `81 FF FF FF FF FF FF FF FF 7F` |
| `u64::MAX + 1` (i.e. `2^64`) | `1` followed by 64 zeros (65 bits) | padded to 70 bits (`00000` + 65 bits); groups `0000010`, then nine `0000000` | `82 80 80 80 80 80 80 80 80 00` |

(Every row above was hand-decomposed into 7-bit groups directly from its
binary form, per the grammar stated above — pad the value's own binary
representation on the left with the minimum number of zero bits needed to
reach a multiple of 7, split into 7-bit groups most-significant first, then
emit one octet per group with the continuation bit set on every group except
the last. No row required consulting an external spec to derive; each is
independently reproducible from this paragraph alone.)

This grammar extends to **every finite non-negative integer** without
introducing another fixed outer bound, because nothing above names a maximum
octet count — the encoding procedure (split into 7-bit groups, emit
most-significant-first, set continuation bits, stop after the last group) is
defined purely in terms of `n`'s own binary length, which is finite for every
finite `n` and has no ceiling the grammar itself imposes.

**Tag `0x10` (`BigInt`), fully re-stated:**

1. **Sign octet:** `0` = zero, `1` = strictly positive, `2` = strictly
   negative — never inferred from magnitude, never two's-complement.
2. **Zero** has exactly one representation: sign octet `0`, nothing further
   (no length, no magnitude bytes).
3. **Magnitude** (non-zero only): the magnitude's **minimal** big-endian byte
   representation (no leading zero byte — the most-significant byte is
   always non-zero) is computed first; its byte **count** is then encoded as
   a single **CULV** value (above); the CULV octets are emitted, followed
   immediately by that many magnitude bytes.
4. **No fixed-width field appears anywhere in this frame** — not for the
   sign (a single octet, but a closed 3-value enumeration, not a length),
   not for the magnitude-byte-count (CULV, unbounded), not for the magnitude
   itself (exactly as long as its own minimal representation requires).

**Totality proof:**

- Every finite `BigInt` has a well-defined sign (zero/positive/negative) and,
  if non-zero, a well-defined absolute value.
- Every finite non-negative integer (the absolute value's magnitude) has a
  finite minimal big-endian byte representation — a standard, total function
  of the integer's own bit length.
- That minimal byte representation has a finite byte **count** — itself a
  finite non-negative integer.
- CULV encodes **every** finite non-negative integer, by construction (no
  step above depends on the integer's size; the procedure only depends on
  its finiteness).
- Therefore every finite `BigInt` has a complete, finite, well-defined tag
  `0x10` frame. **Every finite `BigInt` value representable by the accepted
  model has a finite canonical encoding.**

This is a claim about **semantic encodability**, not about practical
resource availability: an implementation may exhaust memory or time
attempting to construct or hash an absurdly large value, exactly as it could
for any other unboundedly large in-memory structure anywhere in the crate —
that is an ordinary runtime/environmental failure mode, not a Product/model
representability limit, and this ADR makes no claim, and needs to make none,
about what happens when a real machine runs out of resources (§7).

Tag `0x04` (`Rational`, `ExactNumber`/`BigRational`) is two `0x10` frames —
numerator (signed) and denominator (sign byte always `1`, i.e., strictly
positive) — under two additional **normalization requirements**, both already
guaranteed by `num_rational::Ratio`'s own construction discipline (confirmed
directly against `src/numeric/mod.rs`: `ExactNumber` wraps `BigRational`, whose
crate-level invariant keeps every value in reduced form with a positive
denominator) and restated here as this ADR's own normative requirement so the
encoding does not merely inherit but **requires** it independent of any one
crate's implementation choice:

- **Denominator strictly positive, never zero** — a zero denominator is not a
  rational and has no encoding under this tag; `ExactNumber`'s own type cannot
  hold one.
- **Lowest terms:** `gcd(|numerator|, denominator) = 1` before encoding. Any
  two mathematically equal rationals — `1/2`, `2/4`, `-1/-2` — reduce to the
  identical canonical pair (numerator `1`, denominator `2`; the third example's
  double negation cancels under the sign-normalization rule below) and
  therefore produce byte-identical frames.
- **Sign normalization:** the sign lives entirely on the numerator (denominator
  is always positive), so `-1/-2` first normalizes to `1/2` before the lowest-
  terms reduction above — eliminating the only remaining source of non-
  uniqueness for equivalent rationals.
- **Zero** is canonically `0/1`: numerator's `0x10` frame is the zero encoding
  above, denominator's `0x10` frame is sign octet `1`, CULV magnitude
  byte-count `01` (a single octet, value `1`), magnitude byte `0x01`.

No truncation, no platform-width dependency, and no overflow path exists for
any value the current `BigInt`/`BigRational` domain accepts — this closes
BLOCKER-2 completely: the encoding is total and injective over that entire
domain, not merely over values that happen to fit in 64 bits.

#### 2b. Stable semantic enum constants, independent of Rust source (MAJOR-1)

Tag `0x11` replaces the drafted version's "Rust variant name via UTF-8 string"
rule outright — that rule is withdrawn; it violated I4 by exactly the same
reasoning the drafted ADR itself used to reject numeric Rust discriminants (a
source identifier is as much a compiler/refactor-exposed accident as a
discriminant value). Every enum participating in revision-identity-bearing
content instead gets an **explicit, ADR-fixed, single-byte semantic tag**,
enumerated in full in Appendix Table I. The mapping is normative; the
associated Rust variant name is documentary only, and a future Rust-side
rename of any variant — with no semantic change — **must not** alter these
tag values or, therefore, any computed revision identity. `EnumValue` itself
(Slice Contract §3.5's `ENUM` evidence-domain payload) is **not** encoded via
tag `0x11` — it is an open, caller-supplied opaque domain (not a Rust `enum`
at all), and continues to use tag `0x07` per §1's opaque-domain-reference rule.

#### 2c. What tag `0x05` (UTF-8 string) is actually for

After §2b's withdrawal of Rust-variant-name spelling, tag `0x05` is reserved
for future identity-bearing free-text content — none exists in the current
model (every current string-shaped value is either an opaque-domain reference,
tag `0x07`, or a closed-category enum, tag `0x11`). It remains fully specified
(§9) so that a future model addition with genuine free-text ID content has an
unambiguous encoding already defined, without requiring a new ADR merely to
add a string field.

### 3. Domain separation

The full digest preimage grammar, unambiguous and fully self-delimiting:

```
DOMAIN (16 bytes, fixed) || SCHEME_VERSION (1 byte, fixed) || ROOT_FRAME (tag 0x0A record, self-delimiting per §2)
```

`DOMAIN` is the fixed ASCII byte string `b"ordina-rev-ident"` — exactly 16
bytes (`o-r-d-i-n-a---r-e-v---i-d-e-n-t`, counted), no padding required or
applied at that exact length. `SCHEME_VERSION` is `0x01` for this ADR's
scheme. Both components are fixed-width and placed at the **front** of the
preimage, before any Strategy content, so no possible Strategy-content byte
sequence can be crafted to collide with another Ordina identity domain's
preimage by omission of a separator — the separator is unconditionally present
and fixed-position, and `ROOT_FRAME` (the Strategy's own tag-`0x0A` record
frame, per Appendix Table A entry 1) is self-delimiting by §2's framing rules,
so no length prefix is needed between `SCHEME_VERSION` and `ROOT_FRAME` either.

`SCHEME_VERSION` inside the preimage governs the **encoding and digest
mechanics** as one bound pair — this ADR does not separately version encoding
and digest algorithm, since a change to either one changes the preimage
grammar as a whole. A future ADR introducing a materially different scheme
(different framing rules, different digest algorithm, or both) increments
`SCHEME_VERSION`; see §6.

### 4. Concrete digest algorithm — BLAKE3-256, plain hash mode

**Selected: BLAKE3, plain unkeyed hash mode, 256-bit (32-byte) output.**
Explicitly: standard BLAKE3 `hash()` over the full domain-separated preimage
(§3) as a single input; **no secret key**, **no derive-key context string**,
**no keyed mode**. Domain separation comes exclusively from the ADR-defined
`DOMAIN || SCHEME_VERSION` preimage prefix (§3), not from BLAKE3's own
keying/context mechanisms — this keeps the scheme implementable against any
conforming BLAKE3 library that exposes the standard unkeyed hash function
(present in every mainstream BLAKE3 implementation, Rust or otherwise),
without depending on a specific library's keyed-API surface. BLAKE3's internal
Merlo/tree-chunking behavior for large inputs is **not** a design choice this
ADR makes — it is fixed by BLAKE3's own published specification for the
`hash()` function and is identical across all conforming implementations by
definition; this ADR does not need to (and does not) restate it.

**Alternatives rejected:**

- **SHA-256** — cryptographically sound and more universally implemented
  outside Rust, but materially slower than BLAKE3 for the repeated,
  potentially large-Strategy hashing this feature implies, and offers no
  countervailing benefit here: this is a pure content-addressing use, not an
  interop requirement with an external system that mandates SHA-2. Rejected
  on performance with no offsetting advantage.
- **SHA-512/256** — same interoperability profile as SHA-256, no performance
  advantage over BLAKE3, and less common in general tooling despite its
  length-extension-resistance benefit — a benefit this domain-separated,
  fixed-preimage-structure design does not need (the preimage is never
  attacker-extensible; it is fully determined by canonical Strategy content).
  Rejected as strictly dominated by BLAKE3 for this use.
- **BLAKE2b/BLAKE2s** — reasonable general alternatives, but BLAKE3 supersedes
  both with better throughput and a simpler, more heavily audited single
  reference specification and a mature `RustCrypto`-independent `blake3` crate
  with a stable 1.x API. Rejected in favor of the newer, faster, equally
  well-specified successor.

BLAKE3 has a stable, published specification (the BLAKE3 paper and reference
implementation), a mature, widely-used Rust crate (`blake3`, pure-Rust core,
no C dependency, `no_std`-capable), and is already the digest of choice in
several high-integrity Rust ecosystems (e.g., `cargo`'s own registry index),
giving strong ecosystem-maturity and auditability grounding without importing
unfamiliar cryptographic risk. Using only its plain unkeyed hash mode (never
keyed/derive-key) additionally means no key-management or context-string
architecture question is introduced by this ADR at all.

### 5. Digest width and `RevisionIdentity` representation (MAJOR-2)

`RevisionIdentity`'s **logical value is now `{ scheme_version: u8, digest:
[u8; 32] }`**, not a bare 32-byte blob and not the drafted version's bare
`u64`-based `OpaqueValue` wrapper — both are superseded by this remediation.
The scheme version travels **alongside** the digest as part of the type's own
value, distinct from `SCHEME_VERSION`'s role *inside* the hashed preimage
(§3): the preimage copy exists so the version participates in domain
separation and cannot be stripped without changing the digest; the value-level
copy exists so a caller or persistence layer can determine which scheme
produced any given `RevisionIdentity` **without recomputing or re-hashing
anything** — directly closing the drafted version's external-distinguishability
gap.

- **Equality:** two `RevisionIdentity` values are equal iff both
  `scheme_version` and `digest` are equal — a `SCHEME_VERSION = 0x01` digest
  and a hypothetical future `SCHEME_VERSION = 0x02` digest are **never** equal
  even if their raw 32-byte digest fields happen to collide, because equality
  is defined over the pair, not the digest alone. This is the concrete answer
  to "may old and new scheme identities ever compare equal": **no, never**,
  by construction of the equality relation itself, independent of any
  digest-collision property.
  - This does not resolve the (astronomically unlikely, per §7) case of two
    values under the **same** scheme version colliding on their digest bytes —
    that remains governed by BLAKE3's collision-resistance property, as
    already stated in the drafted version and unchanged here.
- **Ordering/comparison:** none is defined, matching `RevisionIdentity`'s
  existing precedent (`VersionToken`'s own doc: "the contract supplies no
  order... this type deliberately has no `Ord` implementation") — a content-
  addressed identity has no meaningful order, and none is introduced.
- **Opaque public surface, otherwise unchanged:** `RevisionIdentity` remains
  opaque, `Clone`/`Debug`/`PartialEq`/`Eq`; no public byte accessor is required
  beyond what conformance-vector testing needs internally. This is a change to
  the type's internal representation width (from a bare `u64` to
  `{scheme_version, digest}`), not to its public API contract.

### 6. Algorithm/version binding and cross-version behavior

`SCHEME_VERSION` (§3) is carried both inside the digest preimage and,
per §5, alongside the resulting digest in `RevisionIdentity`'s own value — so
an algorithm or framing change is automatically and irreversibly identity-
forking, both cryptographically (the preimage changes) and structurally (the
value's own `scheme_version` field changes and participates in equality). It
cannot silently re-identify an existing revision. A future encoding or
digest-algorithm change is a new, superseding ADR that increments
`SCHEME_VERSION` (or introduces a new `DOMAIN` if the change is significant
enough to warrant a wholly separate identity namespace) — never a silent
reinterpretation of existing bytes under this ADR's `SCHEME_VERSION = 0x01`.
No digest computed under one scheme version is ever reinterpreted as belonging
to another; §5's equality rule enforces this at the type level, not merely as
a documentation convention.

### 7. Collision / error-handling assumptions

BLAKE3 is treated as collision-resistant per its published security claims
(128-bit collision resistance at 256-bit output) — consistent with ADR-0003
§13's "Semantically-different identity collision" threat-property requirement
("collision-resistant content-addressing"). No application-level collision
detection/handling is introduced; this matches ADR-0003's own posture.
`compute_revision_identity` returns `Result<..., RevisionIdentityError>`
**only** for genuinely malformed input — specifically, a `Strategy` that is
not Well-Formed per ADR-0004's gate (I9's precondition). With this remediation's
self-delimiting arbitrary-precision framing (§2a), the encoding is total over
the entire `BigInt`/`BigRational` domain unconditionally — there is no
numeric-overflow **semantic** error path, and none is added, because none is
needed: no `BigInt` value the typed model can construct is excluded by the
encoding, for any magnitude. This is distinct from **ordinary resource
exhaustion** (a process running out of memory or time while encoding an
adversarially, computation-terminating-but-absurdly large magnitude) — that
is a property of the executing machine, not of the identity scheme, and this
ADR does not and must not turn it into a Product-level numeric limit by
inventing an error variant for it; an implementation encountering resource
exhaustion fails the way any Rust program does under memory/time exhaustion
(allocation failure, abort), which is outside `RevisionIdentityError`'s scope
entirely, exactly as it would be for any other unbounded allocation anywhere
in the crate. `RevisionIdentityError`'s single variant, `NotWellFormed`,
remains the complete and correct error surface for this function — no
"unsupported scheme/model state" or "malformed internal state" case exists
separately, because `compute_revision_identity` only ever receives a value
the typed `Strategy` model already constructed, and every state that model
can hold has a defined encoding (§2a) once the WF precondition is satisfied.

### 8. Deterministic cross-platform behavior

Every rule in §2/§2a/§2b was chosen specifically to eliminate
platform/architecture-dependent variation: fixed big-endian byte order
(independent of host endianness), no floating-point anywhere in the preimage
(ADR-0005 §7.2's float-free discipline extends here — `ExactNumber` is exact
rational, never `f32`/`f64`), no pointer/memory-layout-derived bytes, no
hash-map iteration order anywhere in the framing (tag `0x09`'s content-sort
rule instead), no Rust-compiler-chosen discriminants or identifiers anywhere
(tag `0x11`'s ADR-fixed semantic tags), and pinned-Unicode-version NFC string
normalization (§9). Two conforming implementations on different architectures,
different Rust compiler versions, or even different languages entirely, given
the same canonical Strategy content, must produce byte-identical preimages
and therefore identical digests — the ADR-0003 I8 reproducibility requirement,
made concrete.

### 9. Unicode normalization

Under the current model, **no identity-bearing field is free-text UTF-8**
(§2c) — tag `0x05` is defined but currently unused; every present string-like
value is either an opaque-domain reference (tag `0x07`) or a closed-category
enum (tag `0x11`), neither of which involves Unicode normalization at all.
Accepted authority does not currently require NFC normalization of anything,
because nothing identity-bearing is free text yet. §2's tag `0x05` definition
retains an explicit, concrete normalization rule anyway — **NFC, Unicode
version 15.0.0** (the version current at this ADR's drafting; a future
Unicode-version bump is a wording-only amendment to this ADR's own text,
tracked the same way as any other ADR correction, and does **not** require a
`SCHEME_VERSION` bump on its own since no field currently exercises this path)
— purely so a future model revision that adds genuine free-text ID content has
an unambiguous, pre-specified rule ready, rather than requiring a new ADR
merely to add a string field. This is narrower than the drafted version's
implicit "NFC applies to everything string-shaped" framing (which conflated
enum spelling, now separately resolved by §2b, with genuine free text) and
does not itself constitute a wider semantic decision — no accepted authority
is reinterpreted, and no currently-encoded value is affected by this section
at all.

### 10. Upgrade / version-migration semantics

An algorithm or encoding change is **never** a silent recomputation. §6/§5
together mean: (a) existing revision identities computed under
`SCHEME_VERSION = 0x01` remain valid and unchanged forever — this ADR does not
invalidate anything, since nothing has been computed under it yet; (b) a
future encoding/digest change requires a new ADR that explicitly states
whether it is additive (a new `SCHEME_VERSION` value, same `DOMAIN`,
coexisting) or a breaking supersession (new `DOMAIN`), and in either case must
state explicitly whether previously-computed identities remain citable as
historical record or are declared a distinct identity generation; (c) two
identities under different scheme versions are **never** equal, by §5's
equality rule, regardless of their raw digest bytes; (d) this ADR does not
design persistence/migration *machinery* — no schema, no batch-recompute tool,
no storage format — only the identity-semantics rule that recomputation is
never silent and never in-place.

### 11. Public API consequence

The minimum public surface needed by N-22-B-1:

```rust
pub fn compute_revision_identity(strategy: &Strategy) -> Result<RevisionIdentity, RevisionIdentityError>;

pub enum RevisionIdentityError {
    NotWellFormed,
}

// RevisionIdentity's logical value (§5) — internal representation, exact
// derive/accessor surface left to the implementation cycle:
// { scheme_version: u8, digest: [u8; 32] }
```

living in `ordina::result` (alongside the existing `RevisionIdentity` type) or
a new `ordina::identity` module — the exact module placement is an
implementation-cycle decision, not an architecture one. The TLV encoding
mechanics of §2/Appendix are **implementation-private** — no public type or
function for the intermediate byte encoding is required or should be exposed;
only the final `RevisionIdentity` value and the one entry-point function are
public. This keeps the public surface minimal, matching PBA-0168's "minimum
public API" scope and RFC-0004 §14's existing identity-API discipline.

### 12. Conformance vectors required before implementation acceptance

Before N-22-B-1 implementation may be considered closed, deterministic golden
vectors must exist and pass — checked in, covering **both the full encoded
preimage bytes and the final `RevisionIdentity` value**, not the digest alone
— for at minimum:

1. Identical canonical Strategy content, computed twice → identical preimage
   bytes and identical `RevisionIdentity`.
2. One ID-bearing scalar difference (e.g., a changed condition threshold) →
   different preimage bytes, different `RevisionIdentity`.
3. One CX-equivalent input after canonicalization (e.g., differing numeric
   representation of the same rational value before reduction) → **same**
   preimage bytes.
4. A PX field difference (e.g., differing authoring timestamp/tool metadata,
   differing in-bounds configuration value) → **same** `RevisionIdentity`
   (PX content is never walked at all, so this reduces to "PX-only diffs
   produce identical preimages").
5. An INV/not-Well-Formed case → `RevisionIdentityError::NotWellFormed`
   (never a computed identity).
6. Two Strategies differing **only** in `OccurrenceId` values → **same**
   `RevisionIdentity`.
7. Two Strategies differing **only** in occurrence-handle order/assignment →
   **same** `RevisionIdentity`.
8. An unordered collection with its member `Vec` permuted (canonical content
   identical) → **same** `RevisionIdentity`.
9. **BigInt magnitude exceeding 64 bits** (numerator or denominator) →
   correctly encoded per §2a, no truncation, distinct from any other value
   that happens to share its low 64 bits.
9a. **CULV length-primitive golden vectors, standalone:** exact octet
    sequences for `0`, `1`, `127`, `128`, `255`, `256`, `u32::MAX`,
    `u32::MAX + 1`, `u64::MAX`, `u64::MAX + 1` — §2a's own worked table —
    tested as a pure length-encoder/decoder unit, independent of any BigInt
    that happens to have that many magnitude bytes; this is the primary test
    of the grammar's boundary transitions (single-octet ↔ multi-octet, and
    each subsequent octet-count boundary).
9b. **Non-canonical CULV rejected:** an artificially-constructed encoding
    with a redundant leading all-zero 7-bit group followed by a
    continuation-bit-`1` octet (e.g., `0x80 0x01` as an encoding of `1`,
    where the canonical form is the single octet `0x01`) → decoding fails,
    never silently accepted as an alternate valid encoding of the same
    value.
9c. **BigInt magnitude exceeding 255 bytes:** a real, concretely
    constructible vector (a 256+ byte magnitude is a few hundred bytes of
    test fixture, not a multi-gigabyte allocation) proving the CULV-prefixed
    magnitude framing crosses the single-octet-length-count boundary
    (magnitude byte-count itself becomes a two-octet CULV value) correctly.
9d. **Architecture-level totality, not an empirical large-allocation test:**
    §2a's grammar is proven total by construction — CULV encodes every
    finite non-negative integer with no fixed maximum octet count, so a
    magnitude requiring a length value beyond `u32::MAX` or `u64::MAX` bytes
    is provable **at the length-prefix grammar level alone** (encode that
    literal length value with CULV per 9a's worked table, using it as the
    byte-count prefix in a conceptual frame) without allocating or hashing
    the corresponding multi-gigabyte-or-larger magnitude payload; this ADR
    does not require, and implementation must not require, a literal
    absurdly-large fixture to demonstrate the grammar handles arbitrarily
    large magnitudes — the general case follows from §2a's totality proof,
    not from additional empirical scale.
10. **Negative `BigInt`** in a rational numerator → correct sign-byte framing,
    distinct from its positive counterpart.
10a. **`ConfigurationParameter` field-position correctness:** a Strategy
    containing at least one declared `ConfigurationParameter` → the field
    frames use positions `1, 3, 4, 5, 6` (never `2`, per corrected Table D),
    verified against golden preimage bytes.
11. **Rational normalization:** `1/2`, `2/4`, and `-1/-2` all produce the
    identical canonical `0x04` frame (numerator `1`, denominator `2`).
12. **Enum canonical semantic tag independent of Rust spelling:** a vector
    whose expected bytes are pinned to Appendix Table I's tag values directly,
    with an explicit comment noting that a hypothetical Rust variant rename
    must not change this test's expected bytes.
13. **Absent vs. present-empty vs. out-of-domain** distinguished: a
    `Declared::Absent` field, a `Declared::Present` field holding an empty
    `Vec`, and (where applicable) a `DeclaredCategory::OutOfDomain` state all
    produce distinct, non-colliding frames.
14. **Nested record framing:** a `CandidateDerivation` holding a `Multiply`
    `DerivationExpression` (itself nesting a `Literal` and a boxed
    sub-expression) round-trips through the full tag-`0x0A`/`0x0F`/`0x0E`
    nesting without ambiguity.
15. `SCHEME_VERSION = 0x01` preimage prefix present verbatim in every vector's
    expected bytes (`DOMAIN || 0x01 || ...`).
16. **Cross-scheme-version non-equality:** given the same digest bytes
    hypothetically paired with two different `scheme_version` values, the two
    resulting `RevisionIdentity` values are unequal (a direct test of §5's
    equality rule, not dependent on any real second scheme existing yet).
17. An empty/minimal valid Strategy (if ADR-0004's WF gate admits one) → a
    stable, reproducible identity, not a panic or error.
18. The same vectors re-computed via a from-scratch, independent
    re-implementation of §2/§2a/§2b/Appendix's framing (e.g., a small
    standalone Python/Rust script following only this ADR's text, not the
    `ordina` crate's own code) → byte-identical preimages — the actual test of
    §8's "no shared implementation code" reproducibility claim, now achievable
    given the Appendix's complete field-tag registry.

## Consequences

### Positive

- Closes the encoding-completeness (BLOCKER-1), arbitrary-precision-numeric
  (BLOCKER-2), Rust-identifier-independence (MAJOR-1), and scheme-
  distinguishability (MAJOR-2) gaps the independent review found, using a
  fully specified, auditable, dependency-light (`blake3` crate only) scheme.
- Fully deterministic and platform-independent by construction (§2/§2a/§8).
- Explicit domain separation (§3) forecloses cross-domain digest confusion by
  design, not by convention.
- Versioned at both the preimage and value level (§5, §6, §10) — a future
  algorithm change is structurally forced to be non-silent, and its scheme is
  now recoverable from any stored/transmitted `RevisionIdentity` without
  recomputation.
- Minimal public API (§11) — no encoding internals leak into `ordina`'s public
  surface.
- The Appendix's field/tag registry is derived from, not invented independent
  of, the Slice Contract's own existing property-position numbering, so it
  extends rather than duplicates existing authority.

### Negative

- A purpose-built TLV framing (rather than adopting an existing standard like
  CBOR) means this ADR itself, together with its Appendix, is now the sole
  specification of the byte format — any ambiguity or omission is a defect in
  this document, not something an external spec can be blamed for or
  corrected via upstream fix. Mitigated by the conformance-vector requirement
  (§12's item 18 specifically tests independent-reimplementation fidelity).
- BLAKE3, while mature, is younger and less universally implemented outside
  Rust/C than SHA-2 — a future cross-language reproduction (e.g., a Python
  verifier) depends on BLAKE3 library availability in that language, which is
  good but not as universal as SHA-256. Accepted trade-off given the
  performance and specification-quality benefit.
- `RevisionIdentity`'s representation is now a structured pair rather than a
  bare digest, a larger and slightly more complex internal type than the
  drafted version proposed — an accepted trade-off for closing MAJOR-2's
  external-distinguishability gap.

### Neutral / accepted trade-offs

- The TLV framing, and its Appendix, are verbose relative to a hand-rolled ad
  hoc format, by design — verbosity here is legibility and unambiguity, not
  accidental bloat.
- `RevisionIdentity`'s internal representation must widen from a bare `u64`
  wrapper to a `{scheme_version, digest}` pair; this is a breaking change to
  that type's internal layout (not its public API, which remains opaque) and
  is scoped to the implementation cycle, not decided further here.

## Reversibility

**Costly, not effectively permanent.** ADR-0003 §14 itself already frames
revision-identity choices as high-stakes ("changing it later would invalidate
authored Strategies' identities"), which is exactly why §5/§6/§10 above bind
an explicit version into both the preimage and the value itself: a future
change to this ADR's digest or encoding choice is a new, superseding ADR with
an explicit, non-silent versioning consequence, not a silent break. Nothing
computed under this ADR is retroactively invalidated by that future ADR unless
that future ADR explicitly says so.

## Alternatives Rejected

- **CBOR (deterministic/canonical subset, e.g., RFC 8949 §4.2's "Core
  Deterministic Encoding")** — a reasonable, standards-based alternative,
  genuinely considered. Rejected in favor of the purpose-built TLV framing
  because: (a) CBOR's generality (arbitrary map key ordering rules, indefinite-
  length items, multiple numeric representations for the same value) requires
  *more* specification text to pin down to Ordina's exact needs than a minimal
  purpose-built framing does, working against the "eliminate ambiguity"
  requirement rather than for it; (b) pulling in a CBOR crate as a production
  dependency would need its own ADR-0005 §6.2-style dependency-approval
  argument this cycle is not scoped to make; (c) nothing in Ordina's identity
  model needs CBOR's self-describing-to-arbitrary-consumers property — the
  only consumer of this byte format is `ordina` itself, so a minimal
  purpose-built framing scoped exactly to the Strategy construct shapes is
  strictly simpler to audit. Revisit if a future cross-system interop
  requirement emerges that CBOR would materially help with.
- **JSON** — rejected outright: no canonical JSON form is stable enough across
  implementations (numeric representation, key ordering, whitespace) without
  itself reinventing a large fraction of a canonical-encoding specification,
  at which point a purpose-built binary framing is simpler and smaller.
- **Rust `Debug`/`Hash`/native memory layout** — rejected: none of these are
  specified independent of the Rust compiler and this specific type
  definition; violates I4 (independence from implementation/tool accident)
  and I8 (cross-implementation reproducibility) outright.
- **`bincode`/serde-derived encoding** — rejected: `bincode`'s wire format is
  tied to the exact Rust struct definition and its derive-generated field
  order, meaning a refactor that reorders struct fields (with no semantic
  change) would silently change revision identity — directly violating I4. A
  purpose-built framing with its own explicit, ADR-fixed field order
  (Appendix Tables B–H, sourced from the Slice Contract's own property
  positions) avoids this coupling entirely.
- **Rust enum variant names as canonical spelling (the drafted version's
  original choice)** — rejected during this remediation: identical failure
  mode to `bincode`'s field-order coupling, applied to enum identifiers
  instead. See §2b.
- **Fixed-width `i64`/`u64` rational framing (the drafted version's original
  choice)** — rejected during the first remediation: incompatible with the
  accepted, deliberately unbounded `BigInt`/`BigRational` domain (ADR-0005
  §7.3). See §2a.
- **Fixed 4-byte `u32` magnitude-byte-length (the first remediation's own
  interim choice)** — rejected during the second remediation: still a
  fixed-width ceiling (~4.29 GiB), just a larger one than `i64`/`u64`; still
  formally incompatible with Slice Contract §4.2's "no minimum or maximum
  magnitude," and still contradicted this ADR's own totality claim.
- **Single fixed byte `L` (`0 < L ≤ 255`) bounding a length-of-magnitude
  value (the second remediation's own interim choice)** — rejected during
  this third remediation: independent re-review found this scheme, while a
  vast improvement in scale, was structurally the identical defect as the
  `u32` design above — a fixed-width outer field still capped what the
  encoding could represent, only at `2^(255·8)` bytes instead of `2^32`
  bytes, and the ADR's own "for any reason, ever" totality claim remained
  false. Replaced by §2a's Canonical Unsigned Length Varint (CULV), which has
  no fixed-width field anywhere in its own structure — the defect pattern of
  "relocate the ceiling to a larger constant" is retired for good by this
  design, which contains no constant to relocate.
- **Existing external varint specifications (protobuf varint / LEB128),
  referenced but not normatively adopted** — considered, but this ADR
  specifies CULV's grammar completely and self-containedly in §2a rather than
  incorporating an external specification by reference, per PBA-0172's
  explicit requirement that "no protobuf/LEB128 dependency may be assumed
  unless the ADR normatively selects and fully pins that external format."
  CULV is deliberately MSB-group-first (the reverse of traditional LEB128's
  LSB-group-first convention) specifically so it composes uniformly with
  this ADR's existing big-endian convention for every other multi-byte field
  — adopting LEB128 verbatim would have introduced the one byte-order
  inconsistency in an otherwise uniformly big-endian document.

---

## Appendix — Canonical field/tag registry (normative, closes BLOCKER-1)

This appendix is normative and part of the Decision. Every table below is
derived from the live `src/model/**` types read fresh during this remediation
cycle and from ADR-0003 §6's classification table (including the §6.1
operative-rule-content default, which places condition classification,
Direction logic, and candidate-value derivations unambiguously in ID). No
field's classification was invented: every entry traces either to an explicit
ADR-0003 §6 table row or to §6.1's default. `occurrence: OccurrenceId` is
omitted from every table below by design (§1) — it is never encoded, on any
construct, without exception.

### Table A — Record-type tags (tag `0x0A`'s leading byte)

| Type tag | Rust type | ADR-0003 §6 basis |
|---|---|---|
| `1` | `Strategy` | envelope; §6.1 default + explicit ID rows below |
| `2` | `Condition` | "operative rule-content default" (§6.1: condition classification) |
| `3` | `ConfigurationParameter` | "Configuration structure"/"Configuration bounds" rows — ID |
| `4` | `CandidateDerivation` | §6.1 default (candidate-value derivations) |
| `5` | `DirectionRule` | §6.1 default (Direction logic) |
| `6` | `EvidenceDeclaration` | "Required evidence declarations" row — ID |
| `7` | `Guard` | composed of Atom members; conjunction, unordered (§2 tag `0x09` rule) |
| `8` | `Atom` | grammar-level fragment of an ID-classified Guard; its own fields are ID by the same default |

### Table B — `Strategy` (type tag `1`), field positions

**Verified (re-review, independent re-confirmation) — outcome A, confirmed
correct.** Directly quoted, not inferred, from `src/checks/required_property.rs`'s
`check_1` (the envelope-level check-1 function) and its own doc comment:
*"Check 1 over a whole Strategy — ... The envelope (§3.4) owns the absence of
its five required collections (positions 3–7); positions 1 (lineage) and 2
(semantic-model version) are not check 1's (checks 15 and 16)."* The function
body itself assigns the five collections' positions explicitly and in this
exact order: `evidence_declarations` → 3, `configuration_parameters` → 4,
`conditions` → 5, `direction_rules` → 6, `candidate_derivations` → 7 — matching
this table without alteration.

| Position (field tag) | Field | Frame |
|---|---|---|
| `1` | `lineage` | tag `0x01` (absent) or tag `0x07` (present) |
| `2` | `semantic_model_version` | tag `0x01` or tag `0x07` (`VersionToken`) |
| `3` | `evidence_declarations` | tag `0x01` or tag `0x09` of type-tag-`6` records |
| `4` | `configuration_parameters` | tag `0x01` or tag `0x09` of type-tag-`3` records |
| `5` | `conditions` | tag `0x01` or tag `0x09` of type-tag-`2` records |
| `6` | `direction_rules` | tag `0x01` or tag `0x09` of type-tag-`5` records |
| `7` | `candidate_derivations` | tag `0x01` or tag `0x09` of type-tag-`4` records |

### Table C — `Condition` (type tag `2`), field positions

Confirmed source: `src/checks/required_property.rs` — "Position 1 (lineage)
... position 2 (classification) ... position 3 (guard)."

| Position | Field | Frame |
|---|---|---|
| `1` | `lineage` | tag `0x01` or `0x07` |
| `2` | `classification` | tag `0x01`/`0x0B`/`0x0C`/`0x0D` wrapping tag `0x11` (`ConditionClassification`, Table I) |
| `3` | `guard` | tag `0x01` or type-tag-`7` record |

### Table D — `ConfigurationParameter` (type tag `3`), field positions

**Corrected (second remediation) — the prior revision of this table used
plain declared-struct-order and was wrong.** Directly quoted, not inferred,
from `src/checks/required_property.rs`'s `check_configuration_parameter` and
its own doc comment: *"Check 1 for one configuration parameter — §3.9
positions 1, 3, 4, 5, 6 are check-1-owned. Position 2 (declared type) is
NUMERIC-only and structurally implied (§3.9); its absence is unrepresentable
and is never manufactured."* The function body assigns `lineage` → 1,
`dimension` → 3, `lower_bound` → 4, `upper_bound` → 5, `adjustable` → 6,
explicitly skipping 2. Independently corroborated by
`src/checks/configuration_bounds.rs`: *"§3.9 positions: lower bound = 4,
upper bound = 5."* Position 2 is **reserved**, not reused, not renumbered,
and not collapsed — no live field occupies it, because Slice 1 admits only
`NUMERIC` configuration parameters (`src/model/config.rs`'s own doc: "the
declared type is implied by the struct rather than carried as a field"), so
the declared-type property §3.9 names exists at the contract level but has no
`Declared`-wrapped representation to encode.

| Position | Field | Frame |
|---|---|---|
| `1` | `lineage` | tag `0x01` or `0x07` |
| `2` | *(reserved — structurally-implied `NUMERIC` declared type; no live field; never emitted)* | — |
| `3` | `dimension` | tag `0x01` or `0x07` (`DimensionSymbol`) |
| `4` | `lower_bound` | tag `0x01` or `0x0E` (`Literal`) |
| `5` | `upper_bound` | tag `0x01` or `0x0E` |
| `6` | `adjustable` | tag `0x01` or `0x06` (bool) — declared structural flag, not an in-bounds value; "Configuration structure" row, ID |

Because position `2` has no corresponding live field, `ConfigurationParameter`'s
tag-`0x0A` record frame emits exactly **5** field-position/value pairs (`1`,
`3`, `4`, `5`, `6`) — never 6, and never a placeholder frame for position `2`
itself; the "field count" prefix in tag `0x0A`'s framing (§2) is `5` for this
record type, matching the number of fields actually present in the model, not
the highest position number named by the contract. This is not an
inconsistency: the record's own field-count prefix bounds what an
implementation must iterate, and the *position tags* it iterates over are a
sparse subset of the contract's full numbering — exactly analogous to how
`Condition`, `DirectionRule`, and `EvidenceDeclaration`'s own field counts
already equal their live field counts, not their highest contract position.

### Table E — `CandidateDerivation` (type tag `4`), field positions

Confirmed source: `src/model/derivation.rs` — "property positions (lineage 1,
role 2, expression 3)."

| Position | Field | Frame |
|---|---|---|
| `1` | `lineage` | tag `0x01` or `0x07` |
| `2` | `role` | tag `0x01` or `0x11` (`CandidateRole`, Table I) |
| `3` | `expression` | tag `0x01` or `0x0F` (`DerivationExpression`) |

### Table F — `DirectionRule` (type tag `5`), field positions

Confirmed source: `src/model/direction.rs`/`required_property.rs` —
"Position 1 (lineage)... position 4 (outcome)"; positions 2/3 follow declared
struct order (`precedence`, `guard`) consistent with check 8's own stated
prerequisite pairing.

| Position | Field | Frame |
|---|---|---|
| `1` | `lineage` | tag `0x01` or `0x07` |
| `2` | `precedence` | tag `0x01` or `0x07` (`PrecedenceOrdinal`) |
| `3` | `guard` | tag `0x01` or type-tag-`7` record |
| `4` | `outcome` | tag `0x01`/`0x0B`/`0x0C`/`0x0D` wrapping tag `0x11` (`Direction`, Table I) |

### Table G — `EvidenceDeclaration` (type tag `6`), field positions

Confirmed source: `src/checks/required_property.rs` — "type stays position 2";
positions 1/3/4 follow declared struct order.

| Position | Field | Frame |
|---|---|---|
| `1` | `lineage` | tag `0x01` or `0x07` |
| `2` | `declared_type` | tag `0x01`/`0x0B`/`0x0C`/`0x0D` wrapping tag `0x11` (`EvidenceTypeTag`, Table I) |
| `3` | `declared_dimension` | tag `0x01` or `0x07` |
| `4` | `declared_values` | tag `0x01` or tag `0x09` (unordered `Vec<EnumValue>`, each member tag `0x07`) |

### Table H — `Guard` (type tag `7`) and `Atom` (type tag `8`)

`Guard` has one field, `atoms` (position `1`): tag `0x09` unordered collection
of type-tag-`8` `Atom` records (§3.6 — a conjunction is order-independent).

`Atom`'s three fields — `evidence` (position `1`, `LineageId` reference, tag
`0x07`), `operator` (position `2`, `ComparisonOperator`, tag `0x11`, Table I),
`literal` (position `3`, tag `0x0E`) — are all unconditional grammar-level
fields (never `Declared`-wrapped in the live type), so their frame is always
the value's own tag directly, never tag `0x01`.

### Table I — Closed-category enum semantic tags (tag `0x11`'s payload byte; closes MAJOR-1)

| Enum | Rust variant (documentary only) | ADR-fixed semantic tag |
|---|---|---|
| `ConditionClassification` | `Blocking` | `0` |
| `ConditionClassification` | `Temporary` | `1` |
| `Direction` | `Buy` | `0` |
| `Direction` | `Sell` | `1` |
| `ComparisonOperator` | `Less` | `0` |
| `ComparisonOperator` | `LessOrEqual` | `1` |
| `ComparisonOperator` | `Greater` | `2` |
| `ComparisonOperator` | `GreaterOrEqual` | `3` |
| `ComparisonOperator` | `Equal` | `4` |
| `ComparisonOperator` | `NotEqual` | `5` |
| `EvidenceTypeTag` | `Numeric` | `0` |
| `EvidenceTypeTag` | `Boolean` | `1` |
| `EvidenceTypeTag` | `Enum` | `2` |
| `CandidateRole` | `Entry` | `0` |
| `CandidateRole` | `StopLoss` | `1` |
| `CandidateRole` | `TakeProfit` | `2` |

These tags are normative and immutable once this ADR is Accepted — a future
addition to any of these closed categories (itself requiring its own
governed model-evolution decision per ADR-0002 §10b, since these are all
"closed authoritative categor[ies]" per ADR-0005 §7.4) appends a new tag
value; existing tags are never renumbered.

### PX/INV exclusion note

No field in the current `ordina::model` shape is classified PX or INV. In-
bounds configuration values (PX per ADR-0003 §6.2) are not represented
anywhere in `ConfigurationParameter` or elsewhere in `Strategy` — they belong
to a future Decision-level type outside the Strategy model entirely, so no
exclusion mechanic is needed for them in this registry today. If a future
model revision adds such a field to `Strategy` or any construct, the §6.1
classification-closure gate governs it, and this Appendix must be extended
(a new, later ADR revision or superseding ADR, per §6) before that field may
participate in — or be correctly excluded from — revision identity.

### Registry completeness audit (second remediation)

Re-derived directly from live `src/model/**` during this remediation cycle,
covering every reachable type: `Strategy`, `Condition`, `ConfigurationParameter`,
`CandidateDerivation`, `DirectionRule`, `EvidenceDeclaration`, `Guard`, `Atom`,
`Literal`, `DerivationExpression`, and the five closed enums.

- **Live reachable record/struct types examined:** 8 (`Strategy`, `Condition`,
  `ConfigurationParameter`, `CandidateDerivation`, `DirectionRule`,
  `EvidenceDeclaration`, `Guard`, `Atom`) + 2 tagged unions (`Literal`,
  `DerivationExpression`) + 5 closed enums.
- **Live fields examined (record types, `occurrence` counted separately
  below):** Strategy 7 (`lineage`, `semantic_model_version`, and 5
  collections) + Condition 3 + ConfigurationParameter 5 (position `2` has no
  field — see Table D) + CandidateDerivation 3 + DirectionRule 4 +
  EvidenceDeclaration 4 + Guard 1 (`atoms`) + Atom 3 (`evidence`, `operator`,
  `literal` — grammar-level fields, never `occurrence`-anchored) = **30**.
- **Revision-bearing fields:** 30 (all — no PX/INV field exists in the
  current model, per the note above).
- **Excluded PX/INV fields:** 0.
- **`OccurrenceId` exclusions:** 6 — one `occurrence` field on each of
  `Strategy`, `Condition`, `ConfigurationParameter`, `CandidateDerivation`,
  `DirectionRule`, `EvidenceDeclaration` (the six occurrence-anchored
  constructs); `Guard` and `Atom` are grammar-level fragments with no
  `occurrence` field of their own, so no exclusion applies to them because
  there is nothing there to exclude.
- **Appendix-mapped fields:** 30 of 30.
- **Missing mappings:** 0.
- **Duplicate mappings:** 0.
- **Reserved (contract-named, no live field) positions:** 1 —
  `ConfigurationParameter` position `2` (Table D).
- **Contradictory mappings:** 0 (Table D's prior contradiction, found by
  independent re-review, is corrected above; Table B was independently
  re-verified and confirmed correct, not contradicted).

---

> ADRs are immutable once Accepted. If the decision changes, write a new ADR
> and mark this one Superseded. Never edit the reasoning of a past decision —
> the record of why we were wrong is the most valuable part of the archive.
