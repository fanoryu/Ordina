# ADR-0008: Revision-Identity Encoding and Digest

- **Status:** PROPOSED
- **Date:** 2026-08-19
- **Deciders:** Principal Engineer
- **Related:** ADR-0002 (ACCEPTED, PBR-0006 — semantic model, cited, not
  amended); ADR-0003 (ACCEPTED, PBR-0007 — §5 pipeline ordering, §6 ID/CX/PX/INV
  classification, §7 revision-identity invariants I1–I10, §13 threat table, §16
  deferred decisions — all cited, none reopened); ADR-0004 (ACCEPTED — Well-
  Formedness gate, I9 precondition, unaffected); ADR-0005 (ACCEPTED — Rust
  toolchain, dependency-approval discipline this ADR's digest-crate choice must
  respect); ADR-0007 (ACCEPTED — `OccurrenceId`/order excluded from revision
  identity, unaffected, applied here); RFC-0004 (ACCEPTED, PBR-0021 —
  authoring/translation semantics and revision-identity-bearing convergence,
  unaffected); PBA-0169 (drafting authorization)

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
stopped. PBA-0169 authorizes this ADR to close exactly that gap.

What this ADR does **not** revisit: whether a property is ID/CX/PX/INV (ADR-0003
§6, closed), what the revision-identity invariants require semantically
(ADR-0003 §7, closed), or whether `OccurrenceId`/its order participates in
revision identity (ADR-0007, closed — it does not). This ADR is purely the
mechanical bridge from "canonical ID-bearing content, as ADR-0003 §6 already
classifies it" to "deterministic identity bytes."

## Decision

We will compute revision identity as a **BLAKE3-256 digest over an explicitly
domain-separated, self-describing canonical binary encoding (a purpose-built
TLV framing defined in full below, not CBOR or any other general-purpose
format) of exactly the ADR-0003 §6 ID-classified content of a Well-Formed
Strategy**, exposed through one minimal public function,
`compute_revision_identity(&Strategy) -> Result<RevisionIdentity, RevisionIdentityError>`.

### 1. Revision-identity input model

The identity input is built exclusively from fields ADR-0003 §6 classifies
**ID** (including the §6.1 operative-rule-content default), after §6's
canonicalization rules have already been applied to the in-memory `Strategy`
(CX collapsed, PX excluded, INV never present in a Well-Formed Strategy per
ADR-0004's gate, I9). Concretely, for the current `ordina::model::strategy::
Strategy` shape, the identity input walks: `semantic_model_version`,
`evidence_declarations`, `configuration_parameters` (declared structure/bounds
only — in-bounds values are PX, §6.2, excluded), `conditions`,
`direction_rules`, `candidate_derivations`, each construct's `lineage` where
present (ADR-0003 §9 — lineage is CX-cosmetic/ID-reference-bearing per row
"Rule/node identifiers"; only its reference-bearing role enters identity, never
a bare cosmetic rename), and the canonicalization-version constant this ADR's
own §8 below binds. **`occurrence` (`OccurrenceId`) and any occurrence-derived
order are explicitly never walked** — ADR-0007's ruling, applied here, not
re-derived.

Because `Declared<T>` already distinguishes present/absent/default per §9-§10
of RFC-0004 and ADR-0002 §10b, the encoding (§2 below) represents each `Declared`
field as an explicit presence tag — never an implicit default.

### 2. Canonical byte/structural representation — TLV framing

The encoding is a **self-describing, purpose-built canonical binary framing**,
specified completely enough that two independent conforming implementations
produce byte-identical preimages without sharing code. Every value is encoded
as `(tag: u8, payload)`:

| Tag | Meaning | Payload |
|---|---|---|
| `0x01` | Absent (declared-absent `Declared<T>`) | none — zero-length |
| `0x02` | Unsigned integer | 8-byte big-endian `u64` |
| `0x03` | Signed integer | 8-byte big-endian two's-complement `i64` |
| `0x04` | Rational (exact, dimension-free numeric value, ADR-0005 §7.1's `num-rational` domain) | numerator: tag `0x03` frame, denominator: tag `0x02` frame (always positive, always in lowest terms — the crate's own canonical reduced form) |
| `0x05` | UTF-8 string (enum spelling, token, opaque-domain rendering) | 4-byte big-endian length-prefix (`u32`, bytes, not chars) + UTF-8 bytes, **NFC-normalized** before length is computed |
| `0x06` | Boolean | 1 byte, `0x00`/`0x01` only — any other byte value is a framing error |
| `0x07` | Fixed-size opaque identity reference (`LineageId` in its reference-bearing role only) | 8-byte big-endian `u64` of the referenced `LineageId`'s underlying `OpaqueValue` |
| `0x08` | Ordered sequence (semantically-ordered collections, ADR-0003 §6 "Sequence/Timing/Confirmation" row) | 4-byte big-endian element count (`u32`) + each element's own tagged frame, in the Strategy's own declared order, unchanged |
| `0x09` | Unordered collection (ADR-0003 §6 "Unordered collections — CX→ID(members)") | 4-byte big-endian element count (`u32`) + each already-canonicalized member's own tagged frame bytes, **then the resulting frames sorted ascending by unsigned byte-lexicographic order of the frame bytes themselves**, then concatenated — deduplicated per ADR-0003 §6 (a repeated identical frame after canonicalization contributes once) |
| `0x0A` | Record (a construct with named ID fields — Condition, DirectionRule, ConfigurationParameter, CandidateDerivation, EvidenceDeclaration, etc.) | 4-byte big-endian field count (`u32`) + for each field, in the **fixed declaration order of that Rust struct's ID-classified fields as this ADR's own conformance vectors enumerate per type** (never iteration/insertion/hash order): a 1-byte field-tag constant (this ADR's conformance appendix assigns one small stable `u8` per named field, per struct type) followed by that field's own tagged value frame |

Rules that eliminate ambiguity across the list above, applying uniformly:

- **Integer width/endianness:** all multi-byte integers are big-endian, fixed
  8 bytes for `u64`/`i64` and 4 bytes for length/count prefixes — never
  variable-length/LEB128, to keep framing trivially re-parseable and auditable.
- **Signedness:** `u64` (tag `0x02`) and `i64` (tag `0x03`) are always
  distinguished by tag, never inferred from value range.
- **String encoding:** UTF-8 only, NFC-normalized, length in bytes.
- **Length prefixes:** always present for variable-length payloads (`0x05`,
  `0x08`, `0x09`, `0x0A`); fixed-width payloads (`0x02`, `0x03`, `0x06`,
  `0x07`) carry none.
- **Enum/tag representation:** every enum (`DefectCategory`, condition kind,
  Direction-role kind, etc.) is represented as its ADR-0003 §6 "canonical
  spelling" — this ADR mandates that canonical spelling be each enum's declared
  Rust variant name, ASCII, encoded via tag `0x05` — never a numeric
  discriminant (numeric discriminants are an implementation accident of one
  compiler's memory layout, not authored semantics, and are explicitly
  forbidden here to satisfy I4's independence-from-representation-accident
  requirement).
- **Sequence boundaries:** always an explicit `u32` count, never a sentinel/
  terminator byte, eliminating any ambiguity from a payload value that happens
  to collide with a terminator.
- **Nested value framing:** every value, at any nesting depth, is the same
  `(tag, payload)` shape — there is no separate "top-level" framing rule.
- **Absence/presence markers:** `Declared::Absent` is tag `0x01` with a
  zero-length payload; there is no separate "null" tag and no implicit
  omission — a field that is ID-classified and absent is always present in
  the encoded frame as an explicit absence marker, never skipped.
- **Map/set ordering:** unordered collections (tag `0x09`) are sorted by the
  byte-lexicographic order of their own already-encoded member frames — a
  pure function of the member content, never of insertion, hash, or any
  external comparator — so permutation of the source `Vec`/collection can
  never change the encoded bytes.
- **Field omission:** no ID-classified field may be omitted from a record
  frame (tag `0x0A`) under any circumstance; a record with a `Declared`-absent
  ID field still emits that field's tag byte and an absence-marker payload.
  PX/CX-collapsed-away fields are never part of a record frame at all — they
  were never ID-classified to begin with.

### 3. Domain separation

The full digest preimage is:

```
DOMAIN_TAG (16 bytes, ASCII, NUL-padded) || VERSION (1 byte) || encoded record frame
```

`DOMAIN_TAG` is the fixed ASCII string `b"ordina-rev-ident"` (16 bytes exactly,
no NUL padding needed at that length). `VERSION` is `0x01` for this ADR's
scheme. Both are placed at the **front** of the preimage, before any Strategy
content, so no possible Strategy-content byte sequence can be crafted to
collide with another Ordina identity domain's preimage by omission of a
separator — the separator is unconditionally present and fixed-position.

### 4. Concrete digest algorithm — BLAKE3-256

**Selected: BLAKE3, 256-bit (32-byte) output**, applied to the full
domain-separated preimage above.

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
unfamiliar cryptographic risk.

### 5. Digest width and output representation

**32 bytes (256 bits)**, BLAKE3's native output width — no truncation, no
extension. `RevisionIdentity`'s existing internal representation
(`RevisionIdentity(OpaqueValue)`, where `OpaqueValue` is presently a bare
`u64`) is **insufficient** to carry a 32-byte digest and must be widened as
part of implementation (not this ADR) to hold the full 32 bytes — this ADR
does not redesign `OpaqueValue`'s general shape, only states that
`RevisionIdentity`'s internal payload width must accommodate 32 bytes.
`RevisionIdentity`'s public surface should remain otherwise unchanged
(opaque, comparable, `Clone`/`Debug`, no public byte accessor required beyond
what conformance-vector testing needs internally).

### 6. Algorithm/version binding

`VERSION` (§3) is carried inside the digest preimage itself, so an algorithm
or framing change is automatically and irreversibly identity-forking — it
cannot silently re-identify an existing revision, satisfying ADR-0003 I6's
canonicalization-version-binding invariant extended to this ADR's own
encoding/digest layer. A future encoding or digest-algorithm change is a new,
superseding ADR that increments `VERSION` (or introduces a new `DOMAIN_TAG` if
the change is significant enough to warrant a wholly separate identity
namespace) — never a silent reinterpretation of existing bytes under this
ADR's `VERSION = 0x01`.

### 7. Collision / error-handling assumptions

BLAKE3 is treated as collision-resistant per its published security claims
(128-bit collision resistance at 256-bit output) — consistent with ADR-0003
§13's "Semantically-different identity collision" threat-property requirement
("collision-resistant content-addressing"). No application-level collision
detection/handling is introduced; this matches ADR-0003's own posture (collision
resistance is an assumed cryptographic property, not something Slice 1 code
verifies at runtime). `compute_revision_identity` returns `Result<...,
RevisionIdentityError>` **only** for genuinely malformed input — specifically,
a `Strategy` that is not Well-Formed per ADR-0004's gate (I9's precondition) —
never for an encoding failure, since the encoding in §2 is total over any
value the typed `Strategy`/construct model can hold.

### 8. Deterministic cross-platform behavior

Every rule in §2 was chosen specifically to eliminate platform/architecture-
dependent variation: fixed big-endian byte order (independent of host
endianness), no floating-point anywhere in the preimage (ADR-0005 §7.2's
float-free discipline extends here — the crate's own dimension-free numeric
value type, tag `0x04`, is exact rational, never `f32`/`f64`), no pointer/
memory-layout-derived bytes, no hash-map iteration order anywhere in the
framing (tag `0x09`'s content-sort rule instead), and NFC string
normalization to remove text-encoding ambiguity. Two conforming
implementations on different architectures, different Rust compiler versions,
or even different languages entirely, given the same canonical Strategy
content, must produce byte-identical preimages and therefore identical
digests — this is the ADR-0003 I8 reproducibility requirement, made concrete.

### 9. Upgrade / version-migration semantics

An algorithm or encoding change is **never** a silent recomputation. §6's
`VERSION` byte means: (a) existing revision identities computed under
`VERSION = 0x01` remain valid and unchanged forever — this ADR does not
invalidate anything, since nothing has been computed under it yet; (b) a
future encoding/digest change requires a new ADR that explicitly states
whether it is additive (a new `VERSION` value, same `DOMAIN_TAG`, coexisting)
or a breaking supersession (new `DOMAIN_TAG`), and in either case must state
explicitly whether previously-computed identities remain citable as historical
record or are declared a distinct identity generation; (c) this ADR does not
design persistence/migration *machinery* — no schema, no batch-recompute tool,
no storage format — only the identity-semantics rule that recomputation is
never silent and never in-place.

### 10. Public API consequence

The minimum public surface needed by N-22-B-1:

```rust
pub fn compute_revision_identity(strategy: &Strategy) -> Result<RevisionIdentity, RevisionIdentityError>;

pub enum RevisionIdentityError {
    NotWellFormed,
}
```

living in `ordina::result` (alongside the existing `RevisionIdentity` type) or
a new `ordina::identity` module — the exact module placement is an
implementation-cycle decision, not an architecture one. The TLV encoding
mechanics of §2 are **implementation-private** — no public type or function
for the intermediate byte encoding is required or should be exposed; only the
final `RevisionIdentity` value and the one entry-point function are public.
This keeps the public surface minimal, matching PBA-0168's "minimum public API"
scope and RFC-0004 §14's existing identity-API discipline.

### 11. Conformance vectors required before implementation acceptance

Before N-22-B-1 implementation may be considered closed, deterministic golden
vectors must exist and pass covering at minimum:

1. Identical canonical Strategy content, computed twice → identical
   `RevisionIdentity`.
2. One ID-bearing scalar difference (e.g., a changed condition threshold) →
   different `RevisionIdentity`.
3. One CX-equivalent input (e.g., differing field-declaration order, differing
   numeric representation of the same rational value, differing whitespace/
   presentation in any string field) after canonicalization → **same**
   `RevisionIdentity`.
4. A PX field difference (e.g., differing authoring timestamp/tool metadata,
   differing in-bounds configuration value) → **same** `RevisionIdentity`.
5. An INV/unknown-construct case → `RevisionIdentityError::NotWellFormed`
   (never a computed identity) — exercised via ADR-0004's existing WF gate,
   not a new INV-handling code path in this function.
6. Two Strategies differing **only** in `OccurrenceId` values (all else
   identical) → **same** `RevisionIdentity`.
7. Two Strategies differing **only** in occurrence-handle order/assignment →
   **same** `RevisionIdentity`.
8. An unordered collection with its member `Vec` permuted (source order
   scrambled, canonical content identical) → **same** `RevisionIdentity`.
9. A semantically-ordered collection (Sequence/Timing/Confirmation) with its
   order changed → **different** `RevisionIdentity`.
10. An empty/minimal valid Strategy (if ADR-0004's WF gate admits one) →
    a stable, reproducible identity, not a panic or error.
11. The same vectors re-computed via a from-scratch, independent
    re-implementation of §2's framing (e.g., a small standalone Python/Rust
    script following only this ADR's text, not the `ordina` crate's own code)
    → byte-identical preimages — the actual test of §8's "no shared
    implementation code" reproducibility claim.

## Consequences

### Positive

- Closes the one remaining concrete-mechanics gap blocking N-22-B-1, using a
  fully specified, auditable, dependency-light (`blake3` crate only) scheme.
- Fully deterministic and platform-independent by construction (§2, §8).
- Explicit domain separation (§3) forecloses cross-domain digest confusion by
  design, not by convention.
- Versioned from day one (§6, §9) — a future algorithm change is structurally
  forced to be non-silent.
- Minimal public API (§10) — no encoding internals leak into `ordina`'s public
  surface.

### Negative

- A purpose-built TLV framing (rather than adopting an existing standard like
  CBOR) means this ADR itself is now the sole specification of the byte
  format — any ambiguity or omission in §2 is a defect in this document,
  not something an external spec can be blamed for or corrected via upstream
  fix. Mitigated by the conformance-vector requirement (§11's item 11
  specifically tests independent-reimplementation fidelity).
- BLAKE3, while mature, is younger and less universally implemented outside
  Rust/C than SHA-2 — a future cross-language reproduction (e.g., a Python
  verifier) depends on BLAKE3 library availability in that language, which is
  good but not as universal as SHA-256. Accepted trade-off given the
  performance and specification-quality benefit.

### Neutral / accepted trade-offs

- The TLV framing is verbose relative to a hand-rolled ad hoc format, by
  design — verbosity here is legibility and unambiguity, not accidental bloat.
- `RevisionIdentity`'s internal representation must widen from a bare `u64`
  wrapper to a 32-byte payload; this is a breaking change to that type's
  internal layout (not its public API, which remains opaque) and is scoped to
  the implementation cycle, not decided further here.

## Reversibility

**Costly, not effectively permanent.** ADR-0003 §14 itself already frames
revision-identity choices as high-stakes ("changing it later would invalidate
authored Strategies' identities"), which is exactly why §6/§9 above bind an
explicit version into every computed identity: a future change to this ADR's
digest or encoding choice is a new, superseding ADR with an explicit,
non-silent versioning consequence, not a silent break. Nothing computed under
this ADR is retroactively invalidated by that future ADR unless that future
ADR explicitly says so.

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
  purpose-built framing scoped exactly to the seven Strategy construct shapes
  is strictly simpler to audit. Revisit if a future cross-system interop
  requirement emerges that CBOR would materially help with.
- **JSON** — rejected outright per §2's explicit prohibition: no canonical
  JSON form is stable enough across implementations (numeric representation,
  key ordering, whitespace) without itself reinventing a large fraction of a
  canonical-encoding specification, at which point a purpose-built binary
  framing is simpler and smaller.
- **Rust `Debug`/`Hash`/native memory layout** — rejected: none of these are
  specified independent of the Rust compiler and this specific type
  definition; violates I4 (independence from implementation/tool accident)
  and I8 (cross-implementation reproducibility) outright. Not seriously
  considered as a candidate, only named here because the authorizing PBA
  required this document to explicitly rule them out.
- **`bincode`/serde-derived encoding** — rejected: `bincode`'s wire format is
  tied to the exact Rust struct definition and its derive-generated field
  order, meaning a refactor that reorders struct fields (with no semantic
  change) would silently change revision identity — directly violating I4.
  A purpose-built framing with its own explicit, ADR-fixed field order (§2,
  tag `0x0A`) avoids this coupling entirely.

---

> ADRs are immutable once Accepted. If the decision changes, write a new ADR
> and mark this one Superseded. Never edit the reasoning of a past decision —
> the record of why we were wrong is the most valuable part of the archive.
