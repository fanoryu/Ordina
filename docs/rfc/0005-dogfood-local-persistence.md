# RFC-0005: Minimum Single-Trader Local Persistence for Dogfood-0

- **Status:** PROPOSED
- **Author:** Principal Engineer
- **Created:** 2026-08-20
- **Decision date:** —
- **Authorizing artifact:** PBA-0176 ("RFC-0005 Dogfood-0 Local-Persistence
  Authoring authorization"), explicit Product Owner direction to pursue Path 1
  (persistence architecture) rather than narrowing Dogfood-0 to a single-session
  no-persistence subset, following the stopped Dogfood-0 authorization cycle
  which found persistence to be an unresolved architecture decision.
- **Originating source:** ADR-0002 §15 ("Later ADR — serialization format";
  "Later ADR — revision/provenance persistence, gated by Q-06"); ADR-0003
  preamble ("storage/persistence … deferred"); RFC-0004 §8 (draft/canonical
  persistence "explicitly out of scope"), §24 (downstream persistence);
  N-22-B-2 independent review (ARR-0060) NOTE 1 (cross-session minter high-water
  carry-forward) and NOTE 2 (real production input source), both carried
  forward as Dogfood-0 concerns.

> **Subordinate to** the Product Constitution **v1.0 — ACTIVE — RATIFIED** and to
> ADR-0002 (ACCEPTED, PBR-0006), ADR-0003 (ACCEPTED, PBR-0007), ADR-0005
> (ACCEPTED), ADR-0007 (ACCEPTED), ADR-0008 (ACCEPTED, PBR-0022), and RFC-0004
> (ACCEPTED, PBR-0021). This RFC may refine, but may not contradict, any of them.
> It is an **engineering / infrastructure** RFC. It proposes a *contract and a
> recommended design* for the smallest local persistence that unblocks Dogfood-0;
> it does not itself select a final byte format, modify `ordina` or
> `ordina-desktop`, or claim acceptance. **Storage identity ≠ semantic identity**
> (ADR-0003): nothing here becomes a second canonical revision-identity encoding.

---

## Summary

Dogfood-0 requires that the Product Owner launch Ordina, load a *real* local
Strategy (not a hard-coded developer fixture), run the already-built
diagnosis/correction loop, save the result, close the application, restart, and
continue — including minting new construct identities that provably cannot
collide with identities minted before the restart. This is impossible today: the
desktop starts from an in-memory fixture, and the N-22-B-2 identity minter's
high-water lives only in one process's memory (ARR-0060 NOTE 1). This RFC
proposes the **minimum single-Trader, single-process, local-machine persistence
contract** that makes that end-to-end loop durable across restarts, and
recommends a concrete minimal design, while deferring the final byte-format
selection and any `ordina`-side serialization capability to the accompanying ADR
and its own bounded implementation authorization.

## Motivation

The problem is concrete and already evidenced by two independent reviews:

- **ARR-0060 NOTE 2 — no real input.** `ordina-desktop`'s binary runs from
  `correction_fixtures` by design; there is no path to a Trader's own Strategy.
  Dogfood-0's purpose is real use, which a fixture-only startup cannot deliver.
- **ARR-0060 NOTE 1 — no cross-session identity continuity.** N-22-B-2 proved
  the monotonic minter is collision-safe *within one live session* because the
  session threads one never-re-seeded counter. Across an application restart the
  counter is gone; and because opaque handles are unreadable through the public
  API, a fresh session cannot re-derive a safe seed by inspecting the reloaded
  corpus. Restart therefore risks minting handles that collide with ones minted
  before it — silently corrupting identity.

Neither is "nice to have": both are the two things that separate a disposable
demo from something the Product Owner can actually dogfood. The stopped Dogfood-0
authorization cycle established that both require persistence, which ADR-0002 §15
and RFC-0004 §24 reserve to later architecture work. This RFC is that work,
scoped to the minimum.

## Constitution Check

**Boundary test (Constitution §2.1).** Persistence is invisible to the trading
decision model; it stores and reloads content the Trader already authored and
the checker already evaluated. It originates no instrument, chooses no
direction, and produces no derived value. Answers:

- **Origination** — unaffected: persistence neither invents nor derives Strategy
  content; it stores what the Trader confirmed and reloads it verbatim.
- **Determinism** — unaffected: no averaging/voting/scoring; reload reconstitutes
  identical canonical content, and revision identity is **recomputed** by
  ADR-0008 (§Detailed Design D-4), never read from storage as authoritative.
- **Provenance** — preserved: lineage identity and the "Why?" path are properties
  of the reloaded Strategy, unchanged by the storage round-trip.
- **Gating** — unaffected: no execution control is touched; Dogfood-0 remains a
  diagnosis/correction surface with no trade-readiness output.
- **Restraint** — unaffected: no outcome is implied or guaranteed.

**Further.** Serves principle of a usable, honest first surface (dogfooding);
touches no C1–C10 trading constraint directly except **C5 reproducibility**,
which it *upholds* by recomputing identity on reload rather than trusting stored
bytes. Touches none of the three validation dimensions' semantics. It changes
**no** status semantics, resolution model, or lifecycle stage. It complies with
`ux-constitution.md` (adds only minimal load/save/error surface, §Detailed Design
E). No answer is "no" or "unclear"; the one genuine open dependency
(§Open Questions OQ-1) is surfaced, not proposed around.

## Detailed Design

### Scope and unit of persistence

Dogfood-0's "corpus" is **one** working Strategy plus the state needed to
continue editing it safely — not a multi-Strategy library, not a database. The
persisted unit is a single local **Dogfood envelope**:

```
DogfoodEnvelope {
    format_version:      u32,             // storage schema version (E)
    checker_context:     PersistedContext,// binding + supported token (B)
    strategy:            Option<Strategy>,// the current canonical Strategy (A)
    mint_high_water:     u64,             // next safe mint handle (C)
}
```

This is a *logical* shape; its concrete byte encoding is deferred to the ADR
(§Alternatives, OQ-1). One envelope lives in one local file at a
Trader-configured path. Single process, single Trader, single writer.

### A. Canonical Strategy / corpus persistence — and the opacity constraint

The load/save cycle must support: start from a real local corpus → load after
startup → diagnose/correct via the closed N-22-A / N-22-B work → save → restart
→ reload → continue.

**The central mechanism question.** Serializing a canonical `Strategy` requires
reading the *values* of its opaque handles (`OccurrenceId`, `LineageId`,
`DimensionSymbol`, `EnumValue`, `VersionToken`, `PrecedenceOrdinal`). The
`ordina` public API **deliberately does not expose those values** — the same
opacity (Slice Contract; ADR-0005 §6.3) that produced ARR-0060 NOTE 1. Therefore
`ordina-desktop` **cannot**, through today's public API, serialize or
deserialize a `Strategy`.

**Recommended resolution (boundary-preserving).** The canonical-Strategy
(de)serialization capability belongs to **`ordina`**, not `ordina-desktop`,
because only `ordina` can read and reconstruct opaque handles *without breaching
the opacity boundary*. `ordina` exposes a narrow, total, deterministic
`Strategy ⇄ bytes` capability; `ordina-desktop` treats the result as an opaque
blob it stores inside the envelope. This keeps every opaque domain opaque to the
desktop, adds no handle-value accessor to the public API, and confines
serialization to the crate that already owns the canonical model. **This RFC does
not implement that capability**; it identifies it as the required mechanism and
defers its exact form to the ADR and a subsequent bounded `ordina`
implementation authorization (OQ-1). An alternative — exposing raw handle values
publicly — is considered and rejected (§Alternatives) because it would breach the
opacity ADR-0005/the Slice Contract deliberately maintain.

**Not a second canonical encoding.** The storage bytes are a *transport/restore*
form, explicitly **not** the ADR-0008 revision-identity preimage and **not** an
ADR-0003 canonicalization. ADR-0003's "semantic identity ≠ serialization
identity" governs: storage may use whatever deterministic encoding the ADR
selects, and it carries no authority over identity (D-4).

### B. CheckerContext input and retention

The Well-Formedness pipeline requires two canonical contract inputs the checker
"never generates, infers or chooses" (`opaque.rs`; RFC-0004 §14.3): the
dimensionless `DimensionBinding` and the supported `VersionToken`. The
implementation **must not** manufacture, infer, or default them (Slice Contract
§2.1/§3.4.1).

**Recommendation.** The envelope carries the context (`PersistedContext`) so a
saved corpus travels with the exact context it was checked under — a single
source of truth, reloaded verbatim. On **first** run (no envelope yet), the
context is **Trader-supplied local configuration** (a value the Product Owner
provides once), never a value the code selects. Persisting the context has the
same opacity constraint as the Strategy (a `VersionToken`/`DimensionSymbol` is an
opaque handle), so it rides the same `ordina`-owned (de)serialization capability
(A/OQ-1). The RFC does not pin the config surface (file field, launch argument);
the ADR/implementation chooses, constrained only by "Trader-supplied, never
manufactured."

### C. Identity-mint high-water continuity

**The high-water MUST be stored explicitly**, as `mint_high_water: u64` in the
envelope. It **cannot** be safely derived from the reloaded corpus, because
deriving it means reading the maximum opaque handle in the Strategy, which the
public API does not expose (A) — and even an `ordina`-side "max handle" helper
would only cover handles *present* in the corpus, missing handles that were
minted and then removed (RFC-0004 §14.2.5: a removed construct's handle is never
reused, so the safe seed must exceed it even though it no longer appears). An
explicit monotonic counter is the only representation that survives removal.

Requirements:

- **Monotonicity.** `mint_high_water` never decreases across saves. Each session
  seeds its `IdentityMinter` from the stored value and, on save, writes back the
  minter's advanced high-water.
- **Atomic pairing (with D).** The corpus and its high-water are one envelope,
  written in one atomic operation, so a reload can never pair a corpus with a
  high-water below some handle it contains or previously minted.
- **Restart behavior.** After restart, the reloaded `mint_high_water` seeds the
  new session's minter; the first mint of the new session is strictly greater
  than every handle any prior session minted into this corpus — closing NOTE 1.
- **Interrupted/failed save.** Guaranteed by D: an interrupted write leaves the
  prior complete envelope intact; a partial envelope is never observed.

This requires **no** weakening of the opaque-ID boundary: the high-water is the
desktop's own monotonic counter (it already owns it in N-22-B-2), merely made
durable. No opaque handle value is read to maintain it.

### D. Persistence consistency / failure model

Single-writer, single-process, local file. The minimum crash model:

- **D-1 Atomic replace.** Write the full new envelope to a temporary file in the
  same directory, flush/sync it, then atomically rename it over the target. A
  crash at any point leaves either the complete prior envelope or the complete
  new one — never a torn mix. (**Platform note / OQ-2:** rename-over-existing is
  atomic on POSIX; on Windows the implementation must use a replace that is
  atomic/`MOVEFILE_REPLACE_EXISTING`-equivalent — `std::fs::rename` replaces an
  existing file on Windows, but the ADR/implementation must verify durability
  semantics on the target platform rather than assume them.)
- **D-2 Whole-envelope invariant.** The saved `{strategy, mint_high_water,
  context, format_version}` is always internally consistent: `mint_high_water`
  strictly exceeds every handle in `strategy` and every handle any prior session
  minted. Because the four fields are one atomic unit, an ordinary interrupted
  write can never produce a corpus + stale-high-water combination that later
  mints a colliding identity — the primary hazard this RFC must foreclose.
- **D-3 Validation on load.** A reloaded envelope whose `format_version` is
  unknown, or whose bytes fail the `ordina` deserialization, is rejected as
  *unusable input* (E) — never silently repaired and never partially applied.
- **D-4 Identity is recomputed, never trusted from storage.** On load, the
  Strategy is reconstituted and its `RevisionIdentity` is **recomputed** via
  ADR-0008 `compute_revision_identity`; storage may cache an identity for display
  only, and any mismatch is treated as corrupt input (D-3), never as authority.
  This preserves ADR-0003 §7 content-addressing and forecloses storage becoming a
  second identity source.

Production database transactions, journaling, and backup/restore are **excluded**
(§Explicit Exclusions); D-1's atomic replace is the whole failure model.

### E. Serialization evolution

The envelope begins with `format_version: u32` (value `1` for Dogfood-0). A
reader rejects an unknown version (D-3) rather than guessing. No migration
infrastructure is built now; a future bump earns a migration only when a second
version exists. `format_version` is **distinct from** ADR-0008's
`scheme_version` (revision identity) and ADR-0004's `WF-1` — conflating them is
forbidden.

### End-to-end sufficiency (the 15-step Dogfood-0 boundary)

1 launch → the binary opens the envelope path; 2 load → deserialize via
`ordina` (A); 3 real context → from the envelope, else first-run Trader config
(B); 4 diagnose → existing N-22-A `view_model`; 5 correct → existing N-22-B-2
loop; 6 fresh re-check → existing `CorrectionSession::submit` (§19); 7
Well-Formed → existing verdict; 8 RevisionIdentity → existing
`compute_revision_identity` (D-4); 9 persist → atomic save of the updated
`{strategy, mint_high_water}` (A/C/D); 10 close; 11 restart; 12 reload → (A);
13 same content + identity → identity **recomputed** equal by ADR-0003 §7 I2
(D-4); 14 edit that mints → seed minter from stored `mint_high_water` (C); 15
no collision → first new mint strictly exceeds the stored high-water, which
already exceeds every prior-session handle (C/D-2). **All 15 steps are supported
by this contract**, contingent only on OQ-1's `ordina` serialization capability.

## Alternatives Considered

1. **Expose raw opaque handle values on `ordina`'s public API; serialize in
   `ordina-desktop`.** Rejected: it breaches the opacity boundary ADR-0005 §6.3
   and the Slice Contract deliberately maintain (opaque domains compared by
   equality only, values never read), to serve a storage convenience. The
   `ordina`-owned capability (A) achieves persistence with the boundary intact.
2. **Derive the high-water from the reloaded corpus instead of storing it.**
   Rejected: impossible through the opaque API, and even an `ordina` "max handle"
   helper would miss minted-then-removed handles (RFC-0004 §14.2.5), reintroducing
   the collision it must prevent. Explicit storage (C) is the only sound option.
3. **A general serialization framework / embedded database (serde-across-the-
   model, SQLite, etc.).** Rejected as vastly over-scoped for one local Strategy;
   it would also pick a broad dependency and format the constitution reserves to a
   deliberate ADR. Dogfood-0 needs one file, one Strategy.
4. **No persistence; narrow Dogfood-0 to a single-session demo (Path 2).**
   Rejected by explicit Product Owner direction: Dogfood-0 must survive restarts.

## What This Makes Harder

- Introduces the first durable on-disk artifact in the project; every future
  format change now has a compatibility surface (mitigated minimally by E).
- Requires a bounded extension of `ordina`'s surface (a Strategy (de)serialization
  capability), enlarging the crate whose dependencies ADR-0005 §6.2 fixes — the
  ADR must decide the capability's shape and any dependency implications.
- Commits Dogfood-0 to an `ordina`-owned serialization boundary; a later
  networked/multi-Trader story (explicitly out of scope) may need to revisit it.

## Risks

- **R-P1 (new).** A serialization capability on `ordina` could, if carelessly
  specified, leak opaque-handle semantics or drift toward a second canonical
  encoding. *Mitigation:* the ADR must specify it as transport-only, total,
  deterministic, and identity-neutral (D-4), reviewed independently.
- **R-P2 (new).** Platform atomic-replace assumptions (D-1) may not hold on
  Windows without care. *Mitigation:* OQ-2; the implementation must verify, not
  assume.
- **R-P3 (new).** A stored high-water that is ever reset or hand-edited below a
  real prior mint reintroduces collisions. *Mitigation:* monotonic write-back
  (C), whole-envelope atomicity (D-2), and load-time validation (D-3).

## Open Questions

- **OQ-1 (blocks the ADR, not this RFC).** The exact `ordina`-owned
  `Strategy ⇄ bytes` (de)serialization capability — its API shape, whether it also
  serializes the `PersistedContext` opaque handles, its dependency posture
  (ADR-0005 §6.2), and the concrete byte format (a bespoke deterministic encoding
  vs. an approved serialization library) — is **undecided** and must be settled by
  the accompanying ADR plus a subsequent bounded `ordina` implementation
  authorization. This RFC establishes *that such a capability is required and where
  it must live*, not *what it is*. It does **not** invent it.
- **OQ-2.** The platform-correct atomic-replace primitive on Windows (D-1).
- **OQ-3.** Whether the envelope path is a fixed per-user location or a
  Trader-supplied argument — a UX/config detail deferred to the ADR/implementation,
  not an architecture question.
- **NOTE 3 (unchanged).** Cross-session independently-authored new-content §13
  convergence remains forward-looking; nothing here resolves or requires it.

This RFC does not block on OQ-2/OQ-3 (implementation details). OQ-1 is the
substantive architecture decision the ADR must make; this RFC frames it and
recommends the boundary-preserving direction (A), without deciding it.
