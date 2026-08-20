# ADR-0009: Dogfood Local Persistence Architecture

- **Status:** PROPOSED
- **Date:** 2026-08-20
- **Deciders:** Principal Engineer (drafting). **Not yet accepted** — acceptance
  is a separate Product Owner / Product Board cycle (CLAUDE.md §Hard-stops(6)).
- **Authorizing artifact:** PBA-0178 (ADR-0009 authoring authorization).
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
    pub format_version: u32,             // storage schema version (§12); value 1
    pub binding:        DimensionBinding,// checker context, part 1 (§8)
    pub supported:      VersionToken,    // checker context, part 2 (§8)
    pub strategy:       Option<Strategy>,// the current canonical Strategy
    pub mint_high_water: u64,            // next safe mint handle (§7)
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
/// removal history — max over every `OccurrenceId` and `LineageId` handle
/// present in `strategy`, plus one; `0` for an empty/absent Strategy. See §6.
pub fn minimum_safe_high_water(strategy: &Strategy) -> u64;

pub enum EnvelopeLoadError {
    Malformed,                 // not valid JSON / schema; unreadable scalar
    UnsupportedVersion(u32),   // format_version this build does not support
    HighWaterInconsistent,     // stored high_water < minimum_safe_high_water(strategy)
}
```

- **`serialize_envelope` is total** (returns `Vec<u8>`, no error): every value a
  typed `DogfoodEnvelope` can hold has a JSON representation (§5).
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
- **`ExactNumber` preserved without floats.** `ExactNumber`/`BigRational` is
  serialized as an explicit `{ "num": "<decimal>", "den": "<decimal>" }` pair of
  **arbitrary-precision decimal strings** (numerator signed, denominator positive)
  — never `f32`/`f64` (upholds ADR-0005 §7.2; the numeric crates' `serde` feature
  stays **disabled**, §4.4).
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

`ordina::persistence::minimum_safe_high_water(strategy)` returns
`max(every OccurrenceId handle, every LineageId handle) + 1`, or `0` for an
empty/absent Strategy. The scan happens **inside `ordina`** (only it can read
handles); the desktop never inspects a handle. Case analysis:

- **Case A — new envelope from newly-authored local state.** The desktop owns the
  minter from a known base (e.g. `0`); on save it writes back the minter's current
  `next_handle`, which already exceeds every handle it minted. `mint_high_water`
  is the minter's own state — trivially safe. `minimum_safe_high_water` is not
  needed here (it would agree, since the minter minted every present handle).
- **Case B — reload of a persisted Dogfood envelope with explicit high-water.**
  The stored `mint_high_water` is **authoritative** and seeds the new session's
  minter. It already accounts for **removed** handles (a removed construct's
  handle is never reused, RFC-0004 §14.2.5), because it was carried forward
  monotonically, not re-derived from present handles. This is the normal steady
  state.
- **Case C — first import of a Strategy that contains identities but has no
  persisted Dogfood high-water.** The desktop calls
  `minimum_safe_high_water(strategy)` and seeds the minter with it. This is
  **safe** because a fresh import has **no Ordina-side mint/removal history**: the
  handles present in the imported corpus **are** the complete knowable mint
  history, so `max-present + 1` cannot collide with anything (there is nothing
  removed-but-higher that the corpus does not itself contain).
- **Case D — imported corpus has high opaque handles unknown to the desktop.**
  Identical to Case C: the scan runs **inside `ordina`**, which can read the high
  handles; the desktop still never inspects a handle. `max-present + 1` is safe
  regardless of how large the handles are.
- **Case E — removed / high historical identities exist beyond currently-present
  handles.** Such history can only exist if the corpus was **previously edited
  inside an Ordina session**, which by RFC-0005's contract persisted an explicit
  monotonic `mint_high_water` — i.e. **Case B**, where the stored high-water
  already covers removed handles. A corpus that genuinely carries removal history
  **but no persisted high-water** presents history that is **not derivable from
  the corpus itself**; `minimum_safe_high_water` cannot and must not invent it.

**Chosen mechanism and its exact limitation.** For a first/imported corpus with no
persisted high-water, the seed is `minimum_safe_high_water(strategy)` =
max-present-handle-over-{occurrence,lineage} + 1. This is **not** chosen "because
it is convenient" (PBA-0178's caution): it is the **minimum sound** seed for a
corpus **with no removal history**, and the only removal history that can exist
carries its own persisted high-water (Case B/E) and takes that path instead.
Scanning occurs **inside `ordina`**; it includes exactly the two minted domains
(occurrence, lineage); removed handles are **out of a first import's knowable
scope** by construction. **Stated limitation:** first-import from a bare corpus
**cannot** reconstruct removed-but-higher historical handles it does not contain.
Where a caller has out-of-band knowledge of such history, the import path **must**
accept an **explicitly supplied high-water alongside the corpus** (the desktop
passes that value as `mint_base` instead of the scan result); the ADR requires the
import surface to permit this explicit override. Dogfood-0's normal loop never hits
this (it always reloads a Case-B envelope); it exists only for the corner case of
importing a corpus whose removal history lives outside the file.

### 7. Persistent high-water representation (§7)

- **Logical type:** `u64` (`mint_high_water`).
- **Semantics:** the **next** safe mint handle — strictly greater than every
  handle any session minted into this corpus.
- **Monotonicity:** never decreases across saves; each session seeds its
  `IdentityMinter` from the stored value and, on save, writes back the minter's
  advanced `next_handle`.
- **Equality / ordering:** only **ordering** is operative (the seed must strictly
  exceed every prior handle); equality carries no special meaning.
- **Relationship to identity domains:** governs the **minter's** domains
  (`OccurrenceId` ∪ `LineageId`) via the single shared counter; **one** shared
  value suffices (§6). It has **no** relationship to `DimensionSymbol`/`EnumValue`/
  `VersionToken`/`PrecedenceOrdinal`, which are never minted.
- **Reload behaviour:** the stored value seeds the new minter (Case B); the first
  new mint is strictly greater than every prior-session handle — closing
  ARR-0060 NOTE 1.
- **Update behaviour:** after each successful mint the minter advances; the
  advanced value is written back on the next save.
- **Prohibition:** never decremented, reset below a real prior mint, or reused;
  load-time validation rejects a stored `mint_high_water <
  minimum_safe_high_water(strategy)` as `EnvelopeLoadError::HighWaterInconsistent`
  (RFC-0005 R-P3, D-3).

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

Production database transactions, journaling, and backup/restore are **excluded**
(§19); this atomic-replace-with-pre-fsync is the whole failure model.

### 11. Windows requirement (§11)

**Architecture requirement (what MUST hold):** on every supported platform,
replacing the target must be an **atomic replace-existing** that leaves the target
as either the complete old file or the complete new file, with the new bytes
already durably flushed before the replace (§10).

**Implementation choice (what realizes it):** `std::fs::rename`. On Windows,
`std::fs::rename` maps to `MoveFileExW` with `MOVEFILE_REPLACE_EXISTING |
MOVEFILE_WRITE_THROUGH`, which — **unlike POSIX-only assumptions** — **does replace
an existing destination** and, for a same-directory (same-volume, NTFS) move,
performs the replace atomically at the filesystem-metadata level. Standard rename
semantics are therefore **sufficient** on the supported Windows/NTFS local target;
the ADR does **not** need to pin a Win32 API in the architecture.

- The implementation cycle **must verify** (not assume) this durability behaviour
  on the supported Windows target, per RFC-0005 R-P2 / OQ-2. If verification shows
  plain `std::fs::rename` insufficient on a supported target, the minimum
  compatible strengthening is `ReplaceFileW` (or `MoveFileExW` with
  `MOVEFILE_WRITE_THROUGH`) via a thin platform shim — an **implementation**
  detail, adopted only if verification requires it, not pinned as architecture
  here.
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
| Invalid high-water (non-`u64`) | `EnvelopeLoadError::Malformed`. |
| High-water inconsistent with corpus (`< minimum_safe_high_water`) | `EnvelopeLoadError::HighWaterInconsistent`. |
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
6. restore safe mint high-water → stored `mint_high_water` (Case B), or
   `minimum_safe_high_water`/explicit import value (Case C/D/E) (§6/§7);
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
- `mint_high_water` round-trips exactly.

**Restart**
- save → reload → same logical Strategy;
- recomputed `RevisionIdentity` **unchanged** across the round-trip (ADR-0003 §7
  I2; identity recomputed, not read).

**Mint continuity**
- mint → save → restart → mint again → **no collision**;
- mint highest identity → **remove** it → save → restart → mint → **no reuse**
  (exercises Case B / RFC-0004 §14.2.5 removed-handle non-reuse).

**First / import**
- imported corpus (identities present, no persisted high-water) obtains a safe
  seed via `minimum_safe_high_water`; first mint does not collide (Case C/D);
- explicit-import-high-water override accepted where supplied (Case E).

**Failure**
- interrupted save preserves the previous valid envelope (temp + pre-fsync +
  atomic replace);
- malformed / truncated / empty load fails closed (`Malformed`);
- unknown version rejected (`UnsupportedVersion`);
- high-water inconsistent with corpus rejected (`HighWaterInconsistent`);
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
  `serialize_envelope`, `deserialize_envelope`, `minimum_safe_high_water`,
  `EnvelopeLoadError`. Plus a **`pub(crate)`** (not public) `OccurrenceId`
  handle accessor for transport (§5).
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
- Adds a `pub(crate)` `OccurrenceId` transport accessor (mitigated: crate-internal,
  identity encoder still never calls it, ADR-0007 preserved — §5).

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

> ADRs are immutable once Accepted. This ADR is **PROPOSED** — it is not yet
> accepted, implements nothing, and changes no `src/**`, `ordina-desktop/**`,
> test, or `Cargo` file. If the decision changes before acceptance it is edited as
> a draft; once accepted it becomes immutable and any change is a new, superseding
> ADR. The record of why we chose this architecture is the archive's value.
