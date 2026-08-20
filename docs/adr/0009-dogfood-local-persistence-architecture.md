# ADR-0009: Dogfood Local Persistence Architecture

- **Status:** ACCEPTED
- **Date:** 2026-08-20
- **Decision date:** 2026-08-20
- **Deciders:** Principal Engineer (drafting); **accepted by Product Board**.
- **Accepted by:** Product Board, PBR-0024.
- **Acceptance provenance:** PBA-0180 (acceptance authorization) → PBR-0024
  (Product Board acceptance) → ER-0151 (acceptance synchronization report),
  citing ARR-0062 (independent re-review of the remediated ADR-0009, PASS; 0
  BLOCKER, 0 MAJOR, 0 MINOR, 2 non-defect NOTE). Prior MAJOR-1 / MINOR-1 /
  MINOR-2 all CLOSED; OQ-1 / m-1 / OQ-2 CLOSED, OQ-3 an implementation/UX detail.
  Acceptance covers the persistence **architecture** decided here; persistence
  **implementation** remains a separate, not-yet-granted authorization.
- **Authorizing artifact:** PBA-0178 (ADR-0009 authoring authorization);
  PBA-0179 (bounded remediation authorization); PBA-0180 (acceptance
  authorization).
- **Related:** RFC-0005 (ACCEPTED, PBR-0023 — the Product-level persistence
  contract this ADR realizes; OQ-1/OQ-2 are charged to this ADR, OQ-3 left as an
  implementation/UX detail); ARR-0061 (RFC-0005 independent review, PASS — its
  **MINOR m-1** is charged here and closed in §6); ADR-0002 (ACCEPTED, PBR-0006 —
  §15 "Later ADR — serialization format … persistence" deferral, **now
  discharged** for Dogfood-0 scope; the semantic model is cited, not amended);
  ADR-0003 (ACCEPTED, PBR-0007 — "storage identity ≠ semantic identity"; revision
  identity remains ADR-0003/0008 territory, not persistence); ADR-0004 (ACCEPTED —
  Well-Formedness gate remains authoritative after load); ADR-0005 (ACCEPTED,
  PBR-0013 — §6.2 approved dependencies, §6.3 dependency isolation, §6.4's
  conditional prohibition whose **own escape clause** ("unless a later
  separately-authorized architecture decision permits them") this ADR exercises
  for `serde`, and §7.2 float-free discipline this ADR upholds); ADR-0006
  (ACCEPTED — egui/eframe desktop surface, the persistence caller); ADR-0007
  (ACCEPTED, PBR-0020 — `OccurrenceId` outside revision identity; unaffected —
  persistence preserves it verbatim, revision identity still never walks it);
  ADR-0008 (ACCEPTED, PBR-0022 — revision-identity encoding/digest; its
  implementation-private TLV preimage is **not** reused as, and must not become,
  the persistence format); RFC-0004 (ACCEPTED, PBR-0021 — authoring/translation,
  §14.2.5 removed-handle-non-reuse, §14.3 caller-supplied mint base); N-22-B-1
  (ADR-0008 identity function, `compute_revision_identity`); N-22-B-2 (COMPLETE /
  CLOSED — `CorrectionSession`, `IdentityMinter`; ARR-0060 NOTE 1/NOTE 2, the two
  gaps this persistence closes).

> This ADR decides **one narrow question**: the concrete local-persistence
> architecture RFC-0005 requires for Dogfood-0 — the `ordina`-owned Strategy /
> context serialization boundary, its byte format and dependency posture, the
> safe initial mint-high-water for a first/imported corpus (m-1), and the
> platform-independent durability invariant with its Windows realization (OQ-2).
> It **does not** implement persistence, touch `src/**`, `ordina-desktop/**`,
> tests, or `Cargo`, invent any Product requirement beyond RFC-0005, or reopen
> ADR-0002/0003/0007/0008/RFC-0004. **Storage identity ≠ semantic identity
> (ADR-0003):** nothing here is a second revision-identity encoding; revision
> identity is always **recomputed** by ADR-0008 on load, never read from storage.

---

## Context

RFC-0005 is ACCEPTED at the Product-contract level (PBR-0023): the minimum
Dogfood envelope `{ format_version, checker_context, strategy, mint_high_water }`,
the explicit-monotonic-high-water and whole-envelope-atomic-write invariants, the
Trader-supplied (never-manufactured) checker-context requirement, and the
recompute-not-persist revision-identity rule. It deliberately deferred the
**ADR-level** choices to this ADR: **OQ-1** (the `ordina`-owned `Strategy ⇄
bytes` serialization API, byte format, and dependency posture — including
**m-1**, how a first/imported corpus obtains a safe initial mint high-water when
the desktop cannot read opaque handles) and **OQ-2** (a platform-independent
durability invariant and its Windows atomic-replace realization). **OQ-3** (the
envelope path) is an implementation/UX detail.

What forces the decision now, verified in live source:

- **Opacity is real (`src/model/opaque.rs`).** `OpaqueValue(u64)`'s inner value
  is private; every semantic domain's `identity_handle()` reader is
  `pub(crate)`, not public; `OccurrenceId` has **no** handle accessor at all
  (ADR-0007's structural enforcement). `ordina-desktop` therefore genuinely
  cannot serialize or deserialize a `Strategy` or a `CheckerContext` handle
  through the public API. Only `ordina` can, without breaching the boundary.
- **The minter is desktop-owned and in-memory (`ordina-desktop/src/translate.rs`).**
  `IdentityMinter { next_handle: u64 }` is a **single monotonic counter shared
  across both domains it mints** — `mint_occurrence()` and `mint_lineage()` draw
  from the same `next()` — and it mints **only** `OccurrenceId` and `LineageId`.
  `CorrectionSession::begin(context, strategy, mint_base: u64)` seeds it; the
  doc-comment already states `mint_base` "MUST be above every opaque handle
  already present in the Trader's Strategy corpus." Across a restart the counter
  is gone (ARR-0060 NOTE 1).
- **There is no real input path (`ordina-desktop` runs from `correction_fixtures`).**
  ARR-0060 NOTE 2.
- **`CheckerContext` (`ordina-desktop/src/correction.rs`) is `{ binding:
  DimensionBinding, supported: VersionToken }`** — both wrap `ordina` opaque
  handles the desktop cannot read.

These are exactly the two gaps (NOTE 1, NOTE 2) that separate a demo from
something dogfoodable. ADR-0002 §15 and RFC-0004 §24 reserved the serialization
and persistence architecture to a later ADR; this is that ADR, scoped to the
minimum.

## Decision

We will make **`ordina` the sole owner of Dogfood persistence serialization**,
exposing a narrow, total, deterministic-round-trip **`DogfoodEnvelope ⇄ bytes`**
capability whose concrete format is **JSON produced through `serde` +
`serde_json` over a crate-private serialization schema isolated in a new
`ordina::persistence` module**; `ordina-desktop` performs all file I/O, treating
the serialized bytes as an opaque blob. Every opaque domain stays opaque to the
desktop, no public handle accessor is added, and the persisted bytes are a
transport/restore form that is **never** a revision-identity encoding. The full
architecture follows.

### 1. Serialization ownership and boundary (OQ-1, §4.1)

The (de)serialization capability lives in **`ordina`**, in a new
`ordina::persistence` module — the only place that can read opaque handle values
(`pub(crate) identity_handle()`) and reconstruct handles
(`OpaqueValue::new(u64)` via the public model constructors) **without exposing
raw handles to any external crate**. `ordina-desktop` gains **no** new
handle-value accessor; the desktop-owned raw-serialization alternative is
rejected (§17) because it would require breaching ADR-0005 §6.3 / the Slice
Contract opacity boundary.

The persistence module reads handles through the existing `pub(crate)`
`identity_handle()` accessors and reconstructs them through the existing public
`::new(OpaqueValue::new(h))` constructors — **no change to `src/model/opaque.rs`
or any public opacity surface is required or made.** This is the structural
guarantee that persistence cannot leak handle semantics.

### 2. The logical envelope (§9)

`ordina::persistence` defines the **public** envelope type, carrying exactly the
four RFC-0005 fields, expressed in `ordina`'s own public value types (which the
desktop already holds):

```rust
pub struct DogfoodEnvelope {
    pub format_version: u32,               // storage schema version (§12); value 1
    pub binding:        DimensionBinding,  // checker context, part 1 (§8)
    pub supported:      VersionToken,      // checker context, part 2 (§8)
    pub strategy:       Option<Strategy>,  // the current canonical Strategy
    pub mint_high_water: MintHighWater,    // next safe mint handle, or Exhausted (§7)
}

/// The persisted mint high-water — an **explicit** representation so an exhausted
/// mint domain survives reload as a distinct state, never as a reusable number
/// (§6/§7). `Next(v)` = the next safe handle is `v` (`v` may be `u64::MAX`, the
/// final mintable handle); `Exhausted` = the domain is spent, no further mint may
/// be issued. There is **no** numeric encoding of exhaustion (no sentinel that
/// could be arithmetically transformed back into a handle).
pub enum MintHighWater {
    Next(u64),
    Exhausted,
}
```

- **No `RevisionIdentity` field.** Revision identity is **recomputed** by
  ADR-0008 `compute_revision_identity` after load, never persisted and never read
  from storage as authority (§9, RFC-0005 D-4, ADR-0003 §7). A cached identity is
  **not** persisted for Dogfood-0 — it would be derived data with no benefit and a
  standing corruption/mismatch surface.
- **`checker_context` is committed to the envelope**, not held separately
  (RFC-0005 §B; ARR-0061 NOTE n-1 is a non-blocking location preference, resolved
  here in favour of the single-envelope form so a saved corpus always travels with
  the exact context it was checked under). It is represented as the two public
  values `DimensionBinding` and `VersionToken` — the desktop maps its
  `CheckerContext { binding, supported }` to/from these directly; both are
  `ordina` public types the desktop already uses.
- The envelope type lives in **the `ordina::persistence` module**, not `ordina`'s
  root and not `ordina-desktop`, because only `ordina` can (de)serialize its
  opaque contents. `CheckerContext` itself remains a desktop type (unchanged);
  the desktop performs the trivial field mapping.

### 3. Public API shape (§4.3)

The minimum public surface `ordina::persistence` adds:

```rust
pub fn serialize_envelope(envelope: &DogfoodEnvelope) -> Vec<u8>;

pub fn deserialize_envelope(bytes: &[u8]) -> Result<DogfoodEnvelope, EnvelopeLoadError>;

/// The minimum safe mint high-water derivable from a corpus with no persisted
/// removal history — **fallible**. Semantics (§6/§7):
/// - empty/absent Strategy (no relevant handle present) → `Ok(0)` (the minimum
///   valid next-safe handle);
/// - max present `OccurrenceId`/`LineageId` handle `< u64::MAX` → `Ok(max + 1)`;
/// - max present relevant handle `== u64::MAX` → `Err(Exhausted)` — no safe next
///   handle exists; **never** `u64::MAX + 1` (no wrapping, no saturating).
pub fn minimum_safe_high_water(strategy: &Strategy) -> Result<u64, HighWaterExhausted>;

pub enum EnvelopeLoadError {
    Malformed,                 // not valid JSON / schema; unreadable scalar
    UnsupportedVersion(u32),   // format_version this build does not support
    HighWaterInconsistent,     // stored high_water below the minimum safe value (§7)
    HighWaterExhausted,        // corpus contains a relevant handle == u64::MAX (§6/§7)
    RationalOutOfDomain,       // a persisted ExactNumber is not a valid decimal rational (§4.5)
}

/// The mint domain is exhausted: no handle strictly greater than every present
/// relevant handle exists in `u64`. A distinct, explicit state — never a numeric
/// value, so it can never be transformed back into a reusable handle (§7).
pub struct HighWaterExhausted;
```

- **`serialize_envelope` is total** (returns `Vec<u8>`, no error): every value a
  typed `DogfoodEnvelope` can hold has a JSON representation (§5). (The high-water
  **derivation** is fallible — §6 — but a value already held in the envelope has
  a representation; exhaustion is caught at derivation/seed time, not at
  serialize time.)
- **File I/O stays outside `ordina`.** `serialize_envelope` returns bytes; the
  desktop writes them (§10). `deserialize_envelope` takes bytes; the desktop reads
  them. Raw bytes **are** exposed to the desktop, as an opaque blob it never
  interprets — this is boundary-preserving because the bytes carry no accessor to
  handle *semantics*, only an encoding `ordina` alone reads back.
- The JSON schema DTO and all encoding mechanics are **module-private**; no
  intermediate encoding type is public (mirrors ADR-0008 §11's minimal-surface
  discipline).

### 4. Concrete storage format — JSON via `serde` + `serde_json` (OQ-1, §4.2)

**Selected: JSON, produced by `serde_json` over a crate-private `#[derive(Serialize,
Deserialize)]` schema (a "persistence DTO") that mirrors the envelope and every
`Strategy` construct, defined inside `ordina::persistence`.** The persistence
module converts `DogfoodEnvelope` ⇄ DTO internally (reading/reconstructing opaque
handles as raw `u64` only within the crate) and calls `serde_json` on the DTO.

Why JSON meets every RFC-0005 requirement:

- **Deterministic-enough round-trip.** The requirement is *content* round-trip
  (save → load reconstitutes the identical logical `DogfoodEnvelope`), **not**
  byte-canonical determinism (that is ADR-0008's job, not persistence's). JSON
  round-trips every value the DTO holds exactly: `u64` handles as JSON integers,
  booleans, arrays, and tagged objects for the model's sum types.
- **Every canonical `Strategy` field preserved.** The DTO mirrors the whole model
  — including PX/CX content and `OccurrenceId` — because persistence stores the
  *entire* Strategy, unlike ADR-0008's identity preimage which walks only
  ID-classified content. Each `Declared`/`DeclaredCategory` state is an explicit
  tagged variant; no field is omitted.
- **Opaque identities preserved exactly.** Each handle is stored as its raw `u64`
  (`identity_handle()`), reconstructed via `OpaqueValue::new`. No reminting, no
  aliasing, no reinterpretation (§5).
- **`ExactNumber` preserved without floats** (round-trip fully specified in §4.5).
  `ExactNumber`/`BigRational` is serialized as an explicit
  `{ "num": "<decimal>", "den": "<decimal>" }` pair of **arbitrary-precision
  base-10 integer strings** (numerator signed, denominator strictly positive) —
  never `f32`/`f64` (upholds ADR-0005 §7.2; the numeric crates' `serde` feature
  stays **disabled**, §4.4). Reading uses the existing `pub(crate) as_ratio()`;
  reconstruction and its fail-closed domain validation are specified in §4.5.
- **Versioned** (§12), **rejects malformed input safely** (§13 — `serde_json`
  typed deserialization fails closed by construction; a truncated/invalid document
  yields `EnvelopeLoadError::Malformed`, never a partial or defaulted value),
  **local-only**, and **not a second ADR-0008 canonical encoding** (JSON of the
  full Strategy is structurally unlike ADR-0008's ID-only TLV preimage; revision
  identity is recomputed, §9).

**Enum / sum-type stability.** Model enums and the DTO's tagged variants use
**explicit `#[serde(rename = "…")]` string tags**, not Rust variant identifiers,
so a later Rust-side rename with no semantic change does not silently invalidate
an existing saved file. (This is a forward-compat robustness rule for the format,
distinct from ADR-0008 §2b's identity requirement; here the motive is
save-file stability, not revision identity.)

Human-inspectability is a deliberate secondary benefit: a Product Owner
dogfooding the first surface can read the one local file, which aids trust and
debugging at zero cost for a single small Strategy.

### 4.4 Dependency posture (OQ-1)

Two production dependencies are added **to `ordina` only**: **`serde`** (with the
`derive` feature) and **`serde_json`**. Justification and governance:

- **ADR-0005 §6.4's prohibition is conditional and self-referential.** §6.4 states
  the listed features "must remain disabled **unless a later separately-authorized
  architecture decision permits them**," and its `serde` rationale is precisely
  "Serialization format remains deferred (ADR-0002 §15)." ADR-0009 **is** that
  later separately-authorized decision (PBA-0178 explicitly authorizes deciding
  the format and dependency posture), and it **is** the discharge of ADR-0002
  §15's deferral. Permitting `serde` here therefore operates **within** ADR-0005's
  own stated framework — it does **not** reopen or contradict ADR-0005, and no
  STOP condition is triggered.
- **Scope is contained (ADR-0005 §6.3 isolation).** `serde`/`serde_json` are used
  **only** inside `ordina::persistence`, never on the authoritative checker path,
  and the numeric crates' own `serde` feature remains **disabled** (§4.2 encodes
  `ExactNumber` by hand as decimal strings). §6.4's specific concern — the `serde`
  feature of `num-bigint`/`num-rational` entering the authoritative path — is
  therefore **untouched**; those features stay off.
- **No broad framework adoption.** `serde` + `serde_json` are single-purpose,
  ubiquitous, mature (MIT OR Apache-2.0), and used exactly once, for one local
  file. No async runtime, ORM, database, or network dependency is introduced. The
  `Cargo.toml` must declare the feature set explicitly (matching ADR-0005 §6.4's
  "state the feature set explicitly rather than inherit it" discipline) when the
  subsequent implementation cycle adds them.

The implementation cycle (a separate authorization) makes the actual `Cargo`
change; this ADR decides only that these two dependencies, so scoped, are the
approved posture.

### 4.5 ExactNumber reconstruction and domain validation (MINOR-1)

`ExactNumber` (`src/numeric/mod.rs`) is exact, arbitrary-precision, and
**decimal-only** by construction: its constructors are `from_integer(i64)` and
`from_decimal(sign, integer_digits, fraction_digits)`, the latter producing a
denominator that is a power of ten, so a non-terminating rational such as `1/3`
**cannot** be built (ADR-0005 §7.3). Its only value reader is `pub(crate)
as_ratio() -> &BigRational`. Persistence must round-trip it exactly without
broadening that semantic domain and without any floating point.

- **Reading** uses the existing `pub(crate) as_ratio()`; the DTO stores the
  reduced rational's numerator and denominator as base-10 integer strings (§4.2).
- **Reconstruction** requires a **new crate-internal** constructor — the minimum
  mechanism, spelled here by semantics (the exact name is an implementation-cycle
  choice), e.g.:

  ```rust
  // in `ordina::numeric`, pub(crate) only — the external ExactNumber API stays
  // opaque/minimal and unchanged; ADR-0005 §6.3's public boundary is untouched.
  pub(crate) fn from_ratio_exact(num: BigInt, den: BigInt)
      -> Result<ExactNumber, RationalDomainError>;
  ```

  It is `pub(crate)`, used **only** by `ordina::persistence` — exactly the
  boundary-preserving pattern §5 uses for the `OccurrenceId` transport accessor.
  No public numeric API is added.
- **Fail-closed domain validation** — reconstruction returns an error (surfaced as
  `EnvelopeLoadError::RationalOutOfDomain`, §13) unless **all** hold:
  - the denominator string parses to a `BigInt` and is **strictly positive**
    (`> 0`; a `0` or negative denominator is rejected);
  - the numerator string parses to a `BigInt` (sign permitted on the numerator
    only);
  - the reduced rational lies in `ExactNumber`'s **decimal** domain — i.e. the
    denominator of the value in lowest terms has the form `2^a · 5^b` (equivalently,
    the value is a **terminating** decimal). A value such as `1/3` (denominator `3`)
    is **rejected**, not coerced;
  - reconstruction is **exact** — the reconstructed `BigRational` equals the
    persisted value with **no precision loss** and **no floating-point** step
    anywhere.
- **Accepted** (exact round-trip, no loss): huge positive numerator, huge negative
  numerator, huge denominator, any exact terminating-decimal value, and zero
  (canonically `0/1`).
- **Rejected** (fail closed): `1/3` (and any non-terminating rational), denominator
  `0`, malformed sign, non-numeric integer strings, and any otherwise-invalid
  representation.

**Persistence reconstruction does not broaden `ExactNumber`'s semantic domain:**
the loader admits exactly the values the type's own constructors can produce, and
rejects everything else as unusable input — it never manufactures an out-of-domain
value (ADR-0005 §7.2/§7.3 upheld).

### 5. Opaque-identity round-trip (§5)

Every identity-bearing domain is persisted as its raw `u64` handle and restored
to the same domain newtype:

| Domain | Persisted as | Restored via |
|---|---|---|
| `OccurrenceId` | `u64` (module reads it through a `pub(crate)` accessor added **inside `ordina`**, mirroring the five semantic domains — see note) | `OccurrenceId::new(OpaqueValue::new(h))` |
| `LineageId` | `u64` (`identity_handle()`) | `LineageId::new(OpaqueValue::new(h))` |
| `DimensionSymbol` | `u64` (`identity_handle()`) | `DimensionSymbol::new(OpaqueValue::new(h))` |
| `EnumValue` | `u64` (`identity_handle()`) | `EnumValue::new(OpaqueValue::new(h))` |
| `VersionToken` | `u64` (`identity_handle()`) | `VersionToken::new(OpaqueValue::new(h))` |
| `PrecedenceOrdinal` | `u64` (`identity_handle()`) | `PrecedenceOrdinal::new(OpaqueValue::new(h))` |

**`OccurrenceId` note — a bounded, boundary-preserving implementation
consequence.** `OccurrenceId` today has **no** handle accessor, because ADR-0007
places it outside ADR-0003 §6's taxonomy so revision identity never walks it —
the *absence* is the structural enforcement of "never encoded for identity."
Persistence, however, must round-trip the whole Strategy including its occurrence
handles (they are canonical input, RFC-0005 §A). The implementation cycle will
therefore add a **`pub(crate)`** occurrence-handle accessor used **only** by
`ordina::persistence`. This does **not** weaken ADR-0007: (a) it stays
`pub(crate)`, invisible to every external crate, so opacity to `ordina-desktop`
is intact; (b) ADR-0008's revision-identity encoder still **never** calls it —
`OccurrenceId` remains outside the identity preimage, unchanged; (c) it serves
transport only, exactly as `identity_handle()` serves the five semantic domains
for ADR-0008 without making them public. The ADR-0007 invariant ("occurrence
identity carries no product semantics and is never walked for identity") is
preserved verbatim; only a crate-internal transport reader is added.

Round-trip guarantees: **exact** handle values, **no reminting** on load (handles
are restored, not regenerated), **no aliasing** (distinct handles stay distinct;
distinct domains stay distinct newtypes), **no semantic reinterpretation**, **no
`OccurrenceId` → revision-identity coupling** (§9), **no weakening of ADR-0007**.

### 6. Initial / imported-corpus mint high-water — m-1 closure (REQUIRED)

**Hard invariant (RFC-0005 D-2, restated):** the first subsequent mint after any
load/import must be strictly greater than every handle already present in — or
previously minted into — the loaded corpus, including handles no longer visible
in the current construct set when a historical high-water is known.

The desktop minter (`translate.rs`) mints **only** `OccurrenceId` and `LineageId`,
from a **single shared `u64` counter**. Therefore a safe seed need only exceed the
maximum handle across **those two domains** (the other four opaque domains are
caller-supplied context/draft values, never minted, and are distinct newtypes a
minted handle can never collide with). **One shared high-water is sufficient**;
per-domain high-waters are not required (§7), because the minter itself is a
single counter.

`ordina::persistence::minimum_safe_high_water(strategy)` is **fallible** (§3): it
returns `Ok(max(every OccurrenceId handle, every LineageId handle) + 1)` when that
maximum is `< u64::MAX`, `Ok(0)` for an empty/absent Strategy, and
`Err(HighWaterExhausted)` when the maximum present relevant handle **equals**
`u64::MAX` — because no handle strictly greater than `u64::MAX` exists, so there is
**no safe next handle** and the `+1` must **never** be evaluated (no wrapping, no
saturating). The scan happens **inside `ordina`** (only it can read handles); the
desktop never inspects a handle. Case analysis:

- **Case A — fresh corpus, no identities.** No relevant handle present →
  `minimum_safe_high_water` = `Ok(0)`, the minimum valid next-safe handle. The
  desktop seeds the minter at `0`; on save it writes back the minter's state as
  `MintHighWater::Next(next_handle)` (or `Exhausted`, Case E).
- **Case B — imported corpus with max `< u64::MAX`.** `minimum_safe_high_water` =
  `Ok(max + 1)`; the desktop seeds the minter with it. Safe because a fresh import
  has **no Ordina-side mint/removal history**: the handles present **are** the
  complete knowable mint history, so `max + 1` collides with nothing (there is
  nothing removed-but-higher the corpus does not itself contain).
- **Case C — imported corpus containing `u64::MAX`.** `minimum_safe_high_water` =
  `Err(HighWaterExhausted)` — the domain is spent; **no** identity may be minted.
  The desktop surfaces exhaustion and does **not** open an editing session that
  would mint (diagnosis/read is still possible; the corpus is loadable, only
  further minting is refused). No wraparound to `0`, no reuse, no silent collision.
- **Case D — persisted envelope whose high-water is already exhausted.** The
  reloaded `mint_high_water` is `MintHighWater::Exhausted`; the session refuses to
  mint further. Because exhaustion is an explicit tagged state, reload cannot
  transform it into a reusable numeric value (§7). A stored `Next(u64::MAX)` seeds
  the minter for exactly **one** final mint (issuing `u64::MAX`), after which the
  minter is `Exhausted` and the next save persists `Exhausted`.
- **Case E — minter reaches `u64::MAX` during a live session.** The mint that
  issues `u64::MAX` is the last permitted; the minter must then transition to
  `Exhausted` rather than advancing `next_handle` to `u64::MAX + 1` (which would
  overflow). Any subsequent mint attempt **fails closed** — no identity is issued
  after exhaustion unless a later architecture decision changes the identity
  domain. This is the live-session analogue of the derivation rule above; the
  implementation must use checked arithmetic (never `+=` that can wrap) at the
  minter's advance step.

**Removed / high historical identities** (the former Case E concern) are unchanged:
such history can only exist if the corpus was previously edited inside an Ordina
session, which persisted an explicit monotonic `mint_high_water` (`Next(v)` or
`Exhausted`) — the reload path (Case D / §7) — where the stored high-water already
covers removed handles (RFC-0004 §14.2.5). A bare import that genuinely carries
removal history **but no persisted high-water** presents history **not derivable
from the corpus itself**; `minimum_safe_high_water` cannot and must not invent it,
and the import surface must accept an explicitly supplied high-water alongside the
corpus (below).

**Chosen mechanism and its exact limitation.** For a first/imported corpus with no
persisted high-water, the seed is `minimum_safe_high_water(strategy)` =
`Ok(max-present-handle-over-{occurrence,lineage} + 1)` when that maximum is
`< u64::MAX`, and `Err(HighWaterExhausted)` at `u64::MAX` (above). This is **not**
chosen "because it is convenient" (PBA-0178's caution): it is the **minimum sound**
seed for a corpus **with no removal history**, and the only removal history that can
exist carries its own persisted high-water (Case D / §7) and takes that path
instead. Scanning occurs **inside `ordina`**; it includes exactly the two minted
domains (occurrence, lineage); removed handles are **out of a first import's
knowable scope** by construction. **Stated limitation:** first-import from a bare
corpus
**cannot** reconstruct removed-but-higher historical handles it does not contain.
Where a caller has out-of-band knowledge of such history, the import path **must**
accept an **explicitly supplied high-water alongside the corpus** (the desktop
passes that value as `mint_base` instead of the scan result); the ADR requires the
import surface to permit this explicit override. Dogfood-0's normal loop never hits
this (it always reloads a Case-B envelope); it exists only for the corner case of
importing a corpus whose removal history lives outside the file.

### 7. Persistent high-water representation (§7)

- **Logical type:** `MintHighWater` (`Next(u64) | Exhausted`, §2) — **not** a bare
  `u64`. The explicit `Exhausted` variant is the minimum representation that lets
  an exhausted mint domain survive reload as a distinct state, never as a numeric
  value that could be arithmetically transformed back into a reusable handle.
- **Semantics:** `Next(v)` = the **next** safe mint handle is `v`, strictly greater
  than every handle any session minted into this corpus (`v` may be `u64::MAX`, the
  final mintable handle); `Exhausted` = the domain is spent, no further mint may be
  issued (§6 Cases C–E).
- **Monotonicity:** never decreases across saves; each session seeds its
  `IdentityMinter` from the stored value and, on save, writes back the minter's
  advanced state (`Next(next_handle)`, or `Exhausted` once `u64::MAX` has been
  issued). The advance step uses **checked** arithmetic — never `+=` that can wrap;
  advancing past `u64::MAX` yields `Exhausted`, never `0`.
- **No overflow / no wrap / no saturate:** the `+1` is evaluated **only** when the
  current value is `< u64::MAX`; there is no code path that computes `u64::MAX + 1`.
- **Equality / ordering:** only **ordering** is operative for `Next(v)` (the seed
  must strictly exceed every prior handle); `Exhausted` is a terminal state, not an
  orderable value; equality carries no special meaning.
- **Relationship to identity domains:** governs the **minter's** domains
  (`OccurrenceId` ∪ `LineageId`) via the single shared counter; **one** shared
  value suffices (§6). It has **no** relationship to `DimensionSymbol`/`EnumValue`/
  `VersionToken`/`PrecedenceOrdinal`, which are never minted.
- **Reload behaviour:** a stored `Next(v)` seeds the new minter (reload, Case D;
  first-import Cases A–B instead seed from the derived `minimum_safe_high_water`); the
  first new mint is strictly greater than every prior-session handle — closing
  ARR-0060 NOTE 1. A stored `Exhausted` refuses further minting (Case D).
- **Update behaviour:** after each successful mint the minter advances (checked);
  the advanced state is written back on the next save.
- **Prohibition and load-time validation:** never decremented, reset below a real
  prior mint, or reused. Load-time validation, using `minimum_safe_high_water`:
  - a stored `Next(v)` with `v` below the minimum safe value for the corpus →
    `EnvelopeLoadError::HighWaterInconsistent` (RFC-0005 R-P3, D-3);
  - a corpus whose present handles are already exhausted
    (`minimum_safe_high_water` returns `Err`) while the stored high-water is a
    `Next(_)` that contradicts it → `EnvelopeLoadError::HighWaterExhausted`
    (reload cannot turn exhaustion into a reusable value);
  - a stored `Exhausted` is accepted as-is and forbids further minting.

### 8. `CheckerContext` persistence (§8)

The two canonical contract inputs the WF pipeline "never generates, infers, or
chooses" — `DimensionBinding` (its distinguished `DimensionSymbol`) and the
supported `VersionToken` — are persisted as the envelope's `binding` and
`supported` fields (§2), as their raw opaque handles (§5).

- **Caller-supplied initially:** on **first** run (no envelope) the context is
  **Trader-supplied local configuration** — never manufactured, inferred, or
  defaulted by the code (Slice Contract §2.1/§3.4.1). This ADR does **not** pin
  the first-run config surface (file field vs. launch argument) — that is
  implementation/UX (§14), constrained only by "Trader-supplied, never
  manufactured."
- **Exact round-trip across restart:** handles are stored and restored exactly
  (§5), so the reloaded corpus is checked under the identical context.
- **Mismatch / corruption:** an unreadable/absent context field fails
  deserialization closed (`EnvelopeLoadError::Malformed`); it is never defaulted
  (§13).
- **Location:** part of the **same** `DogfoodEnvelope` (§2), so a saved corpus and
  its context are one atomic unit (ARR-0061 NOTE n-1 resolved in favour of the
  single envelope). No separate local representation is introduced.
- **No UI is designed here.**

### 9. Envelope definition, and identity is never persisted (§9)

The envelope contains **exactly**: `format_version` (§12), the checker context
(`binding`, `supported`, §8), `Option<Strategy>`, and `mint_high_water` (§7). It
does **not** contain a `RevisionIdentity`.

- **RevisionIdentity is not semantic authority from persistence.** On load, the
  Strategy is reconstituted and its `RevisionIdentity` is **recomputed** via
  ADR-0008 `compute_revision_identity` (the existing `CorrectionSession`
  `revision_identity()` path). Storage carries no identity authority (RFC-0005
  D-4; ADR-0003 §7).
- **No persisted identity cache.** Dogfood-0 does not persist a derived identity;
  it is unnecessary and would create a standing mismatch surface. (If a future
  display-only cache were ever justified, ADR-0008 §5's `{scheme_version, digest}`
  value would be the shape — but it is **out of scope** here.)
- Preferring **not** to persist derived identity keeps ADR-0003 §7
  content-addressing the single source of identity truth.

### 10. Durability / atomicity — OQ-2 (§10)

**Architecture invariant (platform-independent):**

> After any interrupted save, the next launch observes **either** the complete
> previous valid envelope **or** the complete new envelope — never a torn mixture
> of Strategy and mint-high-water state.

Realization (single-writer, single-process, local file):

- **Temp file in the same directory.** The desktop writes the full new serialized
  bytes to a temporary file **in the same directory** as the target (same
  filesystem, so the replace is a metadata operation, not a cross-volume copy).
- **Flush before replace.** The temp file's contents are flushed and synced to
  durable storage (`File::sync_all` — `fsync`) **before** the replace, so the
  new bytes are on disk before the target is repointed. This ordering is the
  **architectural** durability requirement (not merely a temp-file existence
  requirement).
- **Atomic replace.** The temp file is atomically renamed over the target
  (`std::fs::rename`). A crash at any point leaves the target as either the
  complete old envelope (rename not yet effected) or the complete new one (rename
  effected) — never a partial write to the target itself.
- **Whole-envelope consistency.** Because `{format_version, context, strategy,
  mint_high_water}` are one serialized unit written atomically, an interrupted
  write can never pair a corpus with a stale high-water (RFC-0005 D-2) — the
  primary hazard this forecloses.
- **Cleanup / orphans.** A leftover temp file from a crashed prior save is
  **ignored** on load (the target is authoritative) and safely overwritten by the
  next save; it is never read as an envelope. Load reads only the caller-named
  target path (§14).
- **Failure reporting.** A save that cannot complete (disk full, permission)
  surfaces an I/O error to the desktop **without** having replaced the target —
  the prior envelope remains intact and observable. `ordina` reports load failures
  as `EnvelopeLoadError` (§13); the desktop reports I/O failures from its own file
  operations.

**Scope of the guarantee — atomic/non-torn visibility, not strongest power-loss
durability (NOTE-1).** The invariant above is a **non-torn visibility** guarantee:
the next launch sees a *complete* envelope — old or new — never a torn mixture.
This is exactly RFC-0005 D-1/D-2's requirement, and the atomic replace-existing
rename provides it. It is deliberately **distinct** from a *strongest post-power-loss
durability* guarantee (that the newest committed save, specifically, always survives
a power loss rather than the still-valid previous one). On some POSIX filesystems
the latter additionally requires an `fsync` of the **containing directory** after
the rename so the directory entry itself is durable; Dogfood-0 does **not** promise
that stronger property and **must not** be read as promising it — losing the last
save to the still-complete previous envelope is within contract. An implementation
**may** add a parent-directory `fsync` for extra robustness, but it is **not
required** to make any claim in this ADR true, and adding it does **not** upgrade
Dogfood-0's durability product contract. Production database transactions,
journaling, and backup/restore remain **excluded** (§19); this
atomic-replace-with-pre-fsync is the whole failure model.

### 11. Windows requirement (§11)

**Architecture requirement (what MUST hold):** on every supported platform,
replacing the target must be an **atomic replace-existing** that leaves the target
as either the complete old file or the complete new file, with the new bytes
already durably flushed before the replace (§10).

**Implementation choice (what realizes it):** a **replace-existing rename** that
satisfies the invariant above — expected to be `std::fs::rename`, which **unlike
POSIX-only assumptions does replace an existing destination on Windows**. The ADR
does **not** assert, and does **not** rely on, any specific Win32 mapping: the
exact mechanism `std::fs::rename` uses on Windows is **std-library/OS/version
dependent** (current Rust std, for example, prefers `SetFileInformationByHandle`
with `FileRenameInfoEx` where available and falls back to `MoveFileExW` — details
that are internal to std and not guaranteed as a stable contract), so pinning a
particular flag combination would be a factual overstatement. What the architecture
requires is only the **replace-existing, non-torn** property; the concrete Windows
realization is an implementation choice.

- The implementation cycle **must verify** (not assume) that the chosen mechanism
  provides the replace-existing / non-torn durability behaviour on the supported
  Windows target, per RFC-0005 R-P2 / OQ-2 — rather than relying on any assumed
  std-internal mapping.
- If verification shows the default `std::fs::rename` insufficient on a supported
  target, a proven replace-existing mechanism (for example `ReplaceFileW`, or
  `MoveFileExW` with `MOVEFILE_WRITE_THROUGH`, via a thin platform shim) may be
  adopted — an **implementation** detail chosen only if verification requires it,
  **not** pinned as architecture here.
- Non-local / network filesystems that cannot provide atomic replace-existing are
  **out of scope** (Dogfood-0 is local-only, §19).

### 12. Format / schema versioning (§12)

- **Representation:** `format_version: u32`, the first field of the envelope.
- **Initial value:** `1` for Dogfood-0.
- **Unknown-version behaviour:** a reader that does not support the stored
  `format_version` returns `EnvelopeLoadError::UnsupportedVersion(v)` — it never
  guesses or partially applies. Dogfood-0 supports exactly version `1`.
- **Distinct from three other version concepts** (conflation forbidden):
  - **ADR-0008 `scheme_version`** (revision-identity encoding scheme; value `0x01`)
    — governs identity bytes, not storage.
  - **Strategy semantic-model version** (the `VersionToken` / check 16 concern) —
    a *content* value inside the Strategy/context, not the storage schema.
  - **ADR-0004 Well-Formedness version (`WF-1`)** — the checker contract version.
  `format_version` versions **only the on-disk envelope encoding**. No migration
  infrastructure is built now (§19); a future bump earns a migration only when a
  second version exists.

### 13. Corruption / invalid-load semantics (§13)

Deterministic, fail-closed behaviour — **no silently repaired data**:

| Condition | Behaviour |
|---|---|
| Missing file | Desktop-level (it owns I/O): treated as **first run** (no envelope) — establish Trader-supplied context and an empty/new corpus (§15). Not an `ordina` error. |
| Empty file | `deserialize_envelope` → `EnvelopeLoadError::Malformed`. |
| Truncated file | `EnvelopeLoadError::Malformed` (`serde_json` fails closed). |
| Malformed format | `EnvelopeLoadError::Malformed`. |
| Unknown format version | `EnvelopeLoadError::UnsupportedVersion(v)`. |
| Invalid opaque-identity representation (non-`u64`, missing field) | `EnvelopeLoadError::Malformed`. |
| Invalid high-water (neither a `Next(u64)` nor `Exhausted`) | `EnvelopeLoadError::Malformed`. |
| High-water `Next(v)` below the corpus minimum safe value | `EnvelopeLoadError::HighWaterInconsistent`. |
| Corpus already exhausted (present handle `== u64::MAX`) while stored high-water is a contradicting `Next(_)` | `EnvelopeLoadError::HighWaterExhausted` (§7) — never turned into a reusable value. |
| Persisted `ExactNumber` out of the decimal domain (`1/3`, denominator `0`/negative, malformed sign/digits) | `EnvelopeLoadError::RationalOutOfDomain` (§4.5). |
| Malformed `CheckerContext` field | `EnvelopeLoadError::Malformed`. |
| Deserialized Strategy that is **not** Well-Formed | **Not a load error.** Decode success ≠ Well-Formedness. |

**Decode success does not imply Well-Formedness.** After a successful
`deserialize_envelope`, the existing WF pipeline (`CheckerContext::check`) runs
and remains **authoritative** — a reloaded corpus may be `Unusable`, which is a
valid diagnosis state, not a load failure (RFC-0005 D-3; ADR-0004 gate intact).
Nothing is silently repaired.

### 14. File-path boundary — OQ-3 disposition (§14)

**OQ-3 is an implementation / UX-config detail, not architecture** (RFC-0005 §OQ-3,
ARR-0061). The architecture requires only:

- the **caller (desktop) supplies** the local envelope path;
- **no hard-coded global location** is required by the architecture;
- `ordina` performs **no** file I/O and knows **no** path — it only
  serializes/deserializes bytes (§3).

Whether the path is a fixed per-user location or a launch argument is deferred to
the implementation/UX cycle. No path-picker UX is designed here.

### 15. Dogfood-0 integration consequence (§15)

Accepted ADR-0009 unblocks the implementation cycle to realize the 15-step loop
(RFC-0005 end-to-end), each step now backed by a concrete decision:

1. launch → desktop opens the caller-supplied envelope path (§14);
2. open path → read bytes (desktop I/O);
3. load or first-run → `deserialize_envelope` (§3), else Trader-supplied context +
   empty corpus (§8, §13 missing-file);
4. restore `CheckerContext` → from `binding`/`supported` (§8);
5. restore `Strategy` → from the envelope (§4/§5);
6. restore safe mint high-water → stored `mint_high_water` on reload (Case D), or
   `minimum_safe_high_water`/explicit import value on first import (Cases A–C),
   **failing closed on exhaustion** (`HighWaterExhausted`, Cases C–E) (§6/§7);
7. run current diagnosis → existing `CheckerContext::check` (N-22-A);
8. edit/correct → existing N-22-B-2 `CorrectionSession` loop;
9. fresh whole-Strategy WF re-check → existing `submit` (§19 of RFC-0004);
10. compute RevisionIdentity → existing `compute_revision_identity` (N-22-B-1 /
    ADR-0008), **recomputed**, never persisted (§9);
11. save complete envelope atomically → `serialize_envelope` + atomic replace
    (§10/§11);
12. close; 13 restart; 14 reload → `deserialize_envelope` (§3);
15. continue minting collision-free → first mint strictly exceeds stored
    high-water, which exceeds every prior-session handle (§6/§7).

**Remaining decisions before implementation:** none at the **architecture** level
— OQ-1, m-1, and OQ-2 are resolved here; OQ-3 is an implementation/UX detail.
Implementation itself requires a **separate bounded implementation authorization**
(it touches `src/**`, `ordina-desktop/**`, tests, and `Cargo`), plus independent
review and acceptance of this ADR first.

### 16. Conformance / test requirements (§16)

Implementation-reviewable requirements the implementation cycle must satisfy:

**Serialization round-trip**
- Strategy content round-trips exactly (every construct, every `Declared`/
  `DeclaredCategory` state, `ExactNumber` via decimal-string num/den).
- Opaque identities round-trip exactly (all six domains, §5).
- `CheckerContext` (`binding`, `supported`) round-trips exactly.
- `mint_high_water` round-trips exactly, **including the `Exhausted` variant** (a
  saved `Exhausted` reloads as `Exhausted`, never as a number).

**ExactNumber round-trip (§4.5)**
- valid exact values round-trip with **no precision loss**: huge positive
  numerator, huge negative numerator, huge denominator, an exact terminating
  decimal, and zero (`0/1`);
- **out-of-domain / malformed input is rejected** (`RationalOutOfDomain`): `1/3`
  (non-terminating), denominator `0`, negative denominator, malformed sign, and
  non-numeric integer strings;
- no floating-point step appears on any path.

**Restart**
- save → reload → same logical Strategy;
- recomputed `RevisionIdentity` **unchanged** across the round-trip (ADR-0003 §7
  I2; identity recomputed, not read).

**Mint continuity**
- mint → save → restart → mint again → **no collision**;
- mint highest identity → **remove** it → save → restart → mint → **no reuse**
  (exercises reload Case D / RFC-0004 §14.2.5 removed-handle non-reuse).

**Mint exhaustion boundary (MAJOR-1, §6/§7)**
- corpus with max relevant handle = `u64::MAX - 1` → `minimum_safe_high_water` =
  `Ok(u64::MAX)`; one final mint issues `u64::MAX`, then the minter is `Exhausted`;
- corpus with max relevant handle = `u64::MAX` → `minimum_safe_high_water` =
  `Err(HighWaterExhausted)`; **no** mint is issued; **no** wrap to `0`;
- persisted `Exhausted` state reloads as `Exhausted` and forbids further minting;
- **no-wrap guarantee:** there is no input for which the derivation or the minter
  advance produces `0` (or any prior handle) from a `u64::MAX` boundary.

**First / import**
- imported corpus (identities present, max `< u64::MAX`, no persisted high-water)
  obtains a safe seed via `minimum_safe_high_water`; first mint does not collide
  (Cases B);
- imported corpus containing `u64::MAX` fails closed (Case C);
- explicit-import-high-water override accepted where supplied (removed-history
  corner case).

**Failure**
- interrupted save preserves the previous valid envelope (temp + pre-fsync +
  atomic replace);
- malformed / truncated / empty load fails closed (`Malformed`);
- unknown version rejected (`UnsupportedVersion`);
- high-water below the corpus minimum rejected (`HighWaterInconsistent`);
- out-of-domain persisted rational rejected (`RationalOutOfDomain`, §4.5);
- corpus/high-water exhaustion rejected/handled fail-closed (`HighWaterExhausted`);
- a decoded-but-not-Well-Formed Strategy is a valid `Unusable` diagnosis, **not**
  a load error.

**Cross-platform**
- persistence format independent of Rust memory layout / endianness / native
  width (JSON integers and decimal strings, not native encodings);
- Windows durability behaviour is testable (temp-in-same-dir + fsync + atomic
  replace verified on the supported Windows target, §11).

### 17. Alternatives Considered

- **Desktop-owned serialization with public raw-handle access.** Rejected: adding
  a public handle-value accessor breaches ADR-0005 §6.3 / the Slice Contract
  opacity boundary (the exact boundary ARR-0061 verified is real), to serve a
  storage convenience. The `ordina`-owned capability achieves persistence with the
  boundary intact.
- **`ordina`-owned serialization.** **Selected** (§1) — the only boundary-preserving
  owner.
- **JSON.** **Selected** (§4) — self-describing, field-name-keyed (robust to
  field reordering, unlike order-coupled formats), fails closed on malformed
  input by construction, human-inspectable, mature single-purpose dependency.
- **Deterministic / custom binary framing (à la ADR-0008 TLV).** Rejected for
  persistence: ADR-0008's framing is identity-specific (ID-only content,
  byte-canonical, excludes `OccurrenceId`/PX) and must **not** be reused as
  persistence (which needs the *whole* Strategy). Hand-writing a *complete*-model
  binary (de)serializer duplicates what `serde_json` does reliably, concentrates
  fail-closed-parsing risk in bespoke code, and buys byte-determinism that
  persistence does not need (that is identity's requirement, not transport's).
  Boring beats bespoke here.
- **CBOR (via `ciborium` or similar).** Rejected: more compact but not
  human-inspectable, adds a less-familiar dependency with no offsetting benefit
  for one small local file; JSON is the more boring, more universal choice.
- **`serde`-derived binary (`bincode`).** Rejected for the same reason ADR-0008
  rejected it: the wire format is coupled to Rust struct field **order**, so a
  harmless refactor could invalidate existing saved files — a forward-compat
  hazard JSON's field-**name** keying avoids.
- **Separate Strategy / high-water files.** Rejected: two files cannot be replaced
  atomically together, reintroducing the corpus + stale-high-water pairing hazard
  (RFC-0005 D-2). **One atomic envelope** (§10) is required.
- **One atomic envelope.** **Selected** (§2/§10).
- **Deriving high-water from the current corpus instead of storing it.** Rejected
  as the steady-state mechanism: it misses minted-then-removed handles (RFC-0004
  §14.2.5). Explicit persisted high-water (§7) is authoritative; the corpus scan
  (`minimum_safe_high_water`) is used **only** for a first import with no removal
  history (§6, Case C/D), where it is provably sufficient.
- **Explicit persisted high-water.** **Selected** (§7).

### 18. Architecture consequences (§18)

- **Public API additions:** `ordina::persistence` with `DogfoodEnvelope`,
  `MintHighWater` (`Next(u64) | Exhausted`, §2/§7), `serialize_envelope`,
  `deserialize_envelope`, the **fallible** `minimum_safe_high_water -> Result<u64,
  HighWaterExhausted>` (§3/§6), and `EnvelopeLoadError` (now including
  `HighWaterExhausted` and `RationalOutOfDomain`). Plus two **`pub(crate)`** (not
  public) transport helpers used only by `ordina::persistence`: an `OccurrenceId`
  handle accessor (§5) and an `ExactNumber` reconstruction constructor with
  fail-closed decimal-domain validation (§4.5). No public opacity or numeric API
  surface changes.
- **Dependency additions:** `serde` (derive) + `serde_json`, isolated to
  `ordina::persistence`, numeric-crate `serde` features remaining off (§4.4).
- **Persistence-format stability commitment:** `format_version 1` / JSON with
  explicit string enum tags; a breaking change bumps the version and is
  non-silently rejected by older readers (§12).
- **Forward-compatibility limitation:** no migration machinery; Dogfood-0 supports
  exactly version `1` and rejects others.
- **Dogfood-only scope:** single-Trader, single-process, single-writer,
  local-machine (§19).
- **No network / multi-user guarantees.**
- **No second revision-identity encoding** — revision identity stays ADR-0008,
  recomputed on load (§9).

## Consequences

### Positive

- Closes ARR-0060 NOTE 1 (cross-session mint continuity) and NOTE 2 (real input),
  the two gaps blocking Dogfood-0, with the opacity boundary intact.
- Resolves OQ-1 (owner + format + dependency + opaque round-trip), m-1 (safe
  initial/imported high-water), and OQ-2 (durability invariant + Windows
  realization) completely enough for implementation to proceed without inventing
  architecture.
- Boring, minimal, isolated: one local JSON file, two single-purpose dependencies
  confined to one module, `ordina`'s public opacity surface unchanged.

### Negative

- Introduces the project's first production `serde` usage and first durable
  on-disk artifact — a compatibility surface (mitigated by §12 versioning and
  §4's field-name/string-tag stability).
- Adds two `pub(crate)` transport helpers — an `OccurrenceId` handle accessor (§5)
  and an `ExactNumber` reconstruction constructor (§4.5) — both crate-internal;
  the identity encoder still never calls the `OccurrenceId` accessor (ADR-0007
  preserved) and the external numeric/opacity APIs are unchanged.

### Neutral / accepted trade-offs

- JSON is larger and slower than a bespoke binary — irrelevant for one small local
  Strategy, and it buys inspectability and fail-closed parsing.
- `CheckerContext` is committed to the envelope rather than stated as a bare
  requirement (ARR-0061 NOTE n-1) — chosen so context always travels with the
  corpus it was checked under.

## Reversibility

**Costly, not permanent.** The persisted format is versioned (§12); a future
format change is a new `format_version` and a superseding ADR, non-silently
rejected by older readers, never an in-place reinterpretation. The `serde`
dependency and the `ordina::persistence` module are isolated (ADR-0005 §6.3), so
replacing the format later touches one module. Nothing here is load-bearing for
revision identity, which remains ADR-0008's and is recomputed, so a persistence
change never disturbs identity.

## Alternatives Rejected

See §17 (each with its reason). Principally: desktop-owned raw-handle
serialization (breaches opacity), custom/CBOR/`bincode` formats (bespoke burden,
non-inspectability, or field-order fragility with no offsetting benefit),
separate files (non-atomic), and corpus-derived steady-state high-water (misses
removed handles).

---

> ADRs are immutable once Accepted. This ADR is **ACCEPTED** (Product Board,
> PBR-0024; acceptance provenance in the header) — it is now immutable: any change
> is a new, superseding ADR, never an edit to this reasoning. Acceptance decides
> the persistence **architecture** only; it implements nothing and changes no
> `src/**`, `ordina-desktop/**`, test, or `Cargo` file — persistence
> implementation remains a separate, not-yet-granted authorization. The record of
> why we chose this architecture is the archive's value.
