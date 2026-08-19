# ADR-0006: Desktop GUI Toolkit Selection

- **Status:** PROPOSED
- **Date:** 2026-08-19
- **Deciders:** Principal Engineer; to be accepted by Product Board
- **Related:** RFC-0003 (ACCEPTED, PBR-0018), PBD-0008 (Q-06 platform/form-factor
  ruling), ADR-0005 (ACCEPTED, immutable), PBA-0154 (drafting authorization),
  ER-0128 (drafting report)
- **Review lineage:** PBD-0008 → GUI-toolkit determination (read-only) → PBA-0154
  → ADR-0006 (this document) → *(independent review, not yet scheduled)* →
  *(Product Board acceptance, not yet scheduled)*
- **Governed by:** Product Constitution v1.0 — ACTIVE — RATIFIED

> This ADR decides **which toolkit renders Ordina's desktop presentation
> surface**, not what that surface must show or how it must behave — RFC-0003
> already fixed that. It decides for the **initial desktop surface only**. It
> selects **no** packaging/distribution model, no persistence, no networking, no
> accounts, and it does **not** authorize N-22 implementation. It does not
> resolve Q-08, N-24, N-25, or F-8.

---

## 1. Context

PBD-0008 (explicit Product Owner decision, 2026-08-19) resolved Q-06: Ordina's
initial target platform/form factor is a **local, single-Trader desktop
presentation surface, running in-process against the existing Rust core, with no
server-side persistence and no multi-user/external-beta assumptions at this
stage.** PBD-0008 explicitly left the GUI framework/toolkit undecided.

RFC-0003 (ACCEPTED, PBR-0018) defines the platform-agnostic interaction contract
that surface must realize: a categorically distinct `UNUSABLE` presentation
(§9.1), the complete defect and not-evaluated sets always reachable, never
drip-fed (§9.2, §9.5), presentation order that is identity-derived and stable
though not necessarily canonical (§9.3), a Trader-facing defect description that
never discards the underlying `DefectIdentity` (§9.4), navigable blocking links
(§9.6), a three-stage diagnosis → authorship → re-submission flow that never
presumes a fix worked before a fresh full re-check returns (§9.7), absolute
prohibitions on silent repair (§9.9) and state-label ambiguity (§9.10), and
`ux-constitution.md`'s accessibility/progressive-disclosure/forbidden-surface
constraints (§9.12). RFC-0003 selects no framework itself (§5 Non-Goals) and
treats `WellFormednessResult`/`DefectIdentity`/`NotEvaluatedRecord`/canonical
ordering as Slice 1's frozen, consumed contract (§8).

ADR-0005 (ACCEPTED, immutable) scoped the Rust core to Slice 1 only: "no UI, no
networking, no deployment target, no process model... Such execution targets may
be added by **a future decision**." `Cargo.toml` presently declares a library
crate only, with exactly two approved production dependencies (`num-bigint`,
`num-rational`; ADR-0005 §6.2), and §6.4 grounds the exclusion of `serde`,
`rand`, `rand_core`, `quickcheck`, and `arbitrary` in the ADR itself, not in
upstream defaults. This ADR is the "future decision" ADR-0005 anticipated: it
adds the project's first binary/UI/process-model surface and its first
production dependencies beyond that closed set.

A prior read-only GUI-toolkit determination (2026-08-19, authorized informally
as decision-support ahead of PBA-0154) surveyed Tauri, egui/eframe, iced, and
Slint against PBD-0008's in-process constraint, RFC-0003's requirements, and
ADR-0005's dependency-minimization posture, and recommended egui/eframe. PBA-0154
authorized drafting this ADR to formally evaluate and either adopt or reject that
recommendation.

Nothing here is chosen for general framework merit. The decision is driven by
which toolkit best satisfies PBD-0008's in-process, no-server, no-new-runtime
constraint and RFC-0003's presentation contract, at the lowest new dependency
and architectural-commitment cost consistent with ADR-0005's posture.

## 2. Decision Drivers

Derived from accepted authority, cited so the decision is traceable rather than
preferential:

- **In-process, no server-side persistence, no multi-user/external-beta
  assumptions** (PBD-0008).
- **A real Trader-facing surface must exist** — RFC-0003 exists specifically to
  define what a Trader is shown; a toolkit that cannot render §9.1–§9.11's
  contract does not satisfy N-22's purpose (RFC-0003 §9).
- **Identity-derived, stable, never-incidental presentation ordering** (RFC-0003
  §9.3) — the toolkit's state model must support deterministic re-ordering
  driven by `DefectIdentity`, not by iteration/insertion order.
- **No presumed-fixed state before a fresh full re-check returns** (RFC-0003
  §9.7 stage 3) — the toolkit's state model must make "stale optimistic UI"
  avoidable, not merely possible to avoid by discipline.
- **No silent repair, no state-label ambiguity** (RFC-0003 §9.9, §9.10) — the
  toolkit must not encourage implicit-apply UI patterns (default-selected
  suggestions, single-click "accept all") as its idiomatic style.
- **Dependency-graph minimization and no compensating abstraction for language
  unfamiliarity** — the general posture ADR-0005 §9/§10 established for Slice 1
  and, absent a reason to depart, the default posture for this ADR too.
- **No `serde`-class serialization boundary reintroduced without a documented
  reason** — ADR-0005 §6.4 grounds the Slice-1 exclusion of `serde` in the ADR
  itself; an architecture that requires it for the UI layer reopens that
  concern and must justify doing so, not merely inherit it by toolkit choice.

Ordina's desktop surface has no accounts, no broker/market-data integration, no
persistence layer, and no multi-user requirement at this stage (PBD-0008,
`user-zero.md` §5, `system-context.md`). The decision is therefore constrained
by RFC-0003's presentation contract and ADR-0005's dependency posture, not by
scale or integration requirements.

## 3. Considered Options

### 3.1 egui/eframe

A pure-Rust, immediate-mode GUI library (`egui`) with a windowing/application
harness (`eframe`). View code calls directly into the existing Rust library —
no IPC boundary, no serialization layer, no second language runtime.

- **Fit with PBD-0008:** Direct. "Running in-process against the existing Rust
  core" describes eframe's normal usage exactly — a native binary that links
  the `ordina` library crate and calls its functions from the render loop.
- **Fit with RFC-0003:** Workable. `WellFormednessResult` and its contents can
  be held in application state and rendered each frame; presentation ordering
  (§9.3) is computed once per fresh result and cached, not recomputed
  incidentally from a HashMap. The three-stage flow (§9.7) requires explicit
  discipline — immediate-mode UIs redraw every frame from current state, so the
  "don't presume fixed" rule must be enforced by the application's own state
  machine (e.g., a `Diagnosing | AwaitingRecheck | Diagnosing(new_result)`
  enum), not by anything the framework itself prevents. This is achievable but
  is not free by construction.
- **Dependency/build impact:** Smallest of the four. Adds `egui` and `eframe`
  (plus their transitive rendering backend, typically `wgpu` or a software
  rasterizer) as new production dependencies. No second language toolchain, no
  webview runtime, no DSL compiler.
- **Accessibility:** Weakest of the four historically. `egui`'s `AccessKit`
  integration (screen-reader/assistive-technology support) exists and is
  actively developed but is materially less mature than a browser-based
  (Tauri) or long-established native-widget toolkit's accessibility story.
  `ux-constitution.md` imposes no explicit assistive-technology requirement
  today, so this is not a blocking defect against current accepted authority,
  but it is a real, documented gap this ADR does not resolve.
- **Testability / RFC-0003 conformance:** View logic is ordinary Rust and unit-
  testable like any other module in this codebase. There is no existing
  UI-conformance-testing pattern in this repository (Slice 1's conformance
  suite tests the library only); a future N-22 implementation cycle would need
  to design one regardless of toolkit.
- **Reversibility:** High. No IPC boundary or DSL to unwind; migrating to a
  different toolkit later means rewriting view code against the same core
  library, not discarding a parallel language stack.
- **New architectural commitment:** Smallest — one native crate family,
  in-process, no new language runtime.

### 3.2 iced

A pure-Rust, Elm-architecture (Model-Update-View) GUI library, native rendering
via `wgpu` or `tiny-skia`.

- **Fit with PBD-0008:** Direct, same in-process model as egui.
- **Fit with RFC-0003:** Strong, arguably stronger than egui's. The explicit
  Model-Update-View discipline is a structural match for §9.7/§9.9's "state
  changes only through an explicit, confirmed action, never presumed" rule —
  a re-check result arriving is naturally modeled as one `Message` variant that
  replaces state atomically, and the "still diagnosing, awaiting re-check"
  distinction is a natural `enum` variant of the `Model`, not a convention the
  application must separately enforce.
- **Dependency/build impact:** Comparable to egui — pure Rust, native rendering
  backend, no second language runtime, no webview.
- **Accessibility:** Improving via the same `AccessKit` foundation egui uses;
  comparable maturity, marginally less community volume than egui's.
- **Testability / RFC-0003 conformance:** The `Update` function (pure state
  transition, no rendering) is unusually straightforward to unit-test in
  isolation — arguably an advantage over egui's immediate-mode style, where
  view and state-mutation are less separated.
- **Reversibility:** High, same profile as egui.
- **New architectural commitment:** Small, comparable to egui.

### 3.3 Tauri

A Rust backend paired with a system WebView frontend (HTML/CSS/JS), communicating
over a command/IPC boundary.

- **Fit with PBD-0008:** Weak. Architecturally a client/frontend split even
  though both halves run on one machine — the Rust core and the UI communicate
  across a serialized IPC boundary, not through direct in-process calls. This
  sits in tension with PBD-0008's explicit "running in-process" framing.
- **Fit with RFC-0003:** Workable in principle (any Turing-complete UI layer
  can implement the contract), but the IPC boundary adds an extra place where
  §9.9's "no silent repair" and §9.3's "identity-derived order" guarantees must
  be independently re-verified on both sides of the boundary.
- **Dependency/build impact:** Largest. Requires a JS/TS frontend toolchain
  (npm ecosystem) alongside Cargo, a webview runtime dependency (WebView2 on
  Windows), and `tauri-bundler` for packaging. The command/IPC layer
  conventionally depends on `serde` for the Rust↔JS boundary — precisely the
  dependency category ADR-0005 §6.4 grounds its Slice-1 exclusion in. Adopting
  Tauri would reopen that exact question for the UI layer without ADR-0005
  itself having anticipated or endorsed it.
- **Accessibility:** Strongest of the four — inherits the OS webview's HTML
  accessibility tree (screen readers, standard keyboard navigation) largely for
  free, since HTML/ARIA accessibility tooling is far more mature than any
  native Rust GUI's current state.
- **Testability / RFC-0003 conformance:** UI logic is split across two
  languages (Rust backend, JS/TS frontend), requiring two separate test
  toolchains and complicating single-repository conformance testing.
- **Reversibility:** Lowest of the four. Migrating off Tauri later means
  discarding an entire JS/webview layer and its own component/state
  architecture, not swapping one Rust crate for another.
- **New architectural commitment:** Largest — a second language runtime, a new
  IPC boundary, a new serialization dependency, a new bundler toolchain.

### 3.4 Slint

A Rust backend with UI structure declared in a separate `.slint` domain-specific
language, compiled at build time.

- **Fit with PBD-0008:** Workable — Rust-side logic remains in-process, but the
  UI's structural definition lives outside Rust entirely.
- **Fit with RFC-0003:** Workable — property bindings between Rust state and
  `.slint`-declared UI can express the contract, but tracing *why* a given
  render happened (relevant to §9.4's "stable, Trader-facing description" and
  §9.3's ordering guarantee) means reasoning across two source-file types
  instead of one.
- **Dependency/build impact:** Medium — adds Slint's own compiler/build-time
  code-generation step in addition to Cargo, and a new `.slint` source-file
  type this repository's existing tooling (rustfmt, clippy, the conformance
  test harness) does not cover.
- **Accessibility:** Improving, actively worked on; maturity between egui/iced
  and Tauri at present, without a definitive advantage over either pure-Rust
  option.
- **Testability / RFC-0003 conformance:** Rust-side logic is testable
  normally; `.slint`-side behavior needs Slint's own, less mature test tooling
  — a second testing discipline this repository does not currently need for
  any other surface.
- **Reversibility:** Comparable to Tauri's — migrating off Slint means
  discarding the `.slint` files and their structural relationship to the Rust
  state, a similar-shaped cost to discarding a webview frontend, though smaller
  in absolute scope.
- **New architectural commitment:** Medium — a new DSL and its own compiler
  toolchain. Additionally, Slint is dual-licensed (GPLv3 or a commercial
  license) — a licensing dimension none of the other three candidates carry,
  and one this ADR does not have the authority to evaluate or resolve (it is
  a legal/licensing question, not an architecture question, and is flagged as
  an unresolved architectural question in §8 below rather than decided here).

## 4. Decision

**We will adopt `egui`/`eframe` as Ordina's initial desktop GUI toolkit.**

This affirms the prior read-only determination's recommendation. Rationale, in
order of weight:

1. **Best fit with PBD-0008's in-process constraint at the lowest new
   architectural cost.** egui and iced are equally strong on this driver; egui
   is chosen over iced as the *first* toolkit because it reaches a working,
   RFC-0003-conformant surface with less new structural ceremony for an initial,
   single-screen "show diagnosis, navigate, re-check" surface — not because iced
   is a worse fit. iced's Model-Update-View architecture is, if anything, a
   *better* structural match for RFC-0003 §9.7/§9.9's state-transition
   discipline, and remains available as a future migration target if the
   presentation layer's complexity grows past what egui's immediate-mode style
   comfortably expresses (§7, Reversibility).
2. **Avoids reopening the `serde`/IPC-boundary question ADR-0005 §6.4
   deliberately closed for Slice 1.** Tauri's conventional architecture would
   require re-litigating that exclusion for the UI layer without a documented
   reason to do so; egui requires no such boundary.
3. **Smallest new dependency and toolchain surface**, consistent with
   ADR-0005's general posture of dependency-graph minimization and avoiding
   compensating abstraction.
4. **No new non-Rust source-file type or build-time DSL compiler**, unlike
   Slint — keeping the existing `rustfmt`/`clippy`/toolchain-pin discipline
   (ADR-0005 §5) applicable to the entire codebase without carve-outs, and
   avoiding Slint's unresolved dual-license question entirely.

This decision selects **egui/eframe as a library/framework choice only**. It
does not select a packaging or distribution model (§6), does not add the
dependency to `Cargo.toml` (§9), and does not authorize any `src/**` or
`tests/**` change (§9).

## 5. Consequences

### Positive

- Ordina gains a viable, minimal-dependency path to a working RFC-0003-
  conformant presentation surface without reopening ADR-0005's `serde`/IPC
  exclusion.
- View code integrates directly with the existing Rust core; no new
  serialization contract or cross-language boundary needs to be designed,
  documented, or kept in sync with `WellFormednessResult`/`DefectIdentity`
  changes.
- The dependency graph remains auditable at the same scale ADR-0005 established
  for Slice 1 — one new native crate family, no new language runtime.

### Negative

- egui's accessibility maturity (via `AccessKit`) is the weakest of the four
  candidates surveyed. No accepted authority currently mandates a specific
  assistive-technology conformance level, so this is not a violation of any
  accepted requirement — but it is a real gap a future accessibility-focused
  cycle may need to revisit, potentially at nontrivial cost if it requires a
  toolkit change.
- Immediate-mode redraw-every-frame semantics mean RFC-0003 §9.7 stage 3's "do
  not presume fixed before a fresh result returns" guarantee must be actively
  enforced by the application's own state machine; egui does not prevent a
  poorly structured implementation from violating it the way iced's
  Model-Update-View discipline more naturally would.
- No existing UI-conformance-testing pattern exists in this repository for any
  toolkit; adopting egui does not reduce that gap, and a future N-22
  implementation cycle must still design one.

### Neutral / accepted trade-offs

- iced was a close second and remains the natural escalation path if the
  presentation layer's state-management needs outgrow egui's immediate-mode
  style (§7).
- Packaging/distribution (how the resulting binary is signed, installed, or
  updated) is explicitly deferred (§6) and carries its own cost/complexity
  profile independent of this decision.

## 6. Packaging / Distribution — Explicitly Out of Scope

This ADR selects a **toolkit**, not a packaging or distribution model. How the
resulting binary is built for release, signed, installed, or updated (a single
unbundled executable, an installer, an auto-update mechanism, code-signing
credentials, per-OS distribution channels) is a **separate decision**, not
addressed here, and not implied by this toolkit choice. It is a downstream
concern of a future N-22 implementation-scoping cycle, not this ADR.

## 7. Reversibility

**Cheap.** egui/eframe requires no IPC boundary, no DSL, and no second language
runtime to unwind. Migrating to a different toolkit later — most plausibly
iced, given the runner-up analysis in §3.2 — means rewriting view code against
the same underlying Rust library calls; the core (`src/model/**`,
`src/checks/**`, `src/result/**`, `src/semantics/**`) is completely unaffected
by this decision and untouched by any migration away from it. This is a
materially cheaper reversal than would be true of Tauri (discarding an entire
JS/webview layer) or Slint (discarding `.slint` source files and their binding
relationships).

## 8. Alternatives Rejected

- **iced** — rejected as the *first* choice, not on merit. Its Model-Update-View
  architecture is a strong, arguably superior structural match for RFC-0003's
  state-transition discipline (§9.7, §9.9); it is not adopted first because
  egui reaches a working initial surface with less new ceremony. Retained as
  the natural next candidate if egui's state-management needs outgrow its
  immediate-mode style.
- **Tauri** — rejected because its webview/IPC/serialization architecture
  contradicts PBD-0008's "running in-process" framing and reopens the exact
  `serde`-category dependency question ADR-0005 §6.4 deliberately closed for
  Slice 1, at the highest new dependency and toolchain cost of the four
  candidates, for an initial single-Trader local tool with no current
  HTML/CSS/JS requirement.
- **Slint** — rejected because its DSL split and unresolved dual-license
  (GPLv3/commercial) question add complexity and an unaddressed legal question
  without a compensating benefit over the pure-Rust options at this stage;
  see §8's unresolved-questions note below regarding the license question
  specifically.

## 9. What This Does Not Authorize

- **Toolkit adoption is not effected by this ADR alone.** No change to
  `Cargo.toml`, `Cargo.lock`, or `rust-toolchain.toml` is made or authorized by
  this document; the dependency is not added until a separately authorized
  implementation cycle does so under an accepted version of this ADR.
- **No `src/**` or `tests/**` change** is made or authorized here.
- **N-22 implementation is not authorized** by this ADR. A separate,
  specifically-scoped N-22 implementation-authorization cycle remains required,
  as stated by PBD-0008, PBA-0154, and every prior N-22-related governance
  record.
- **Packaging/distribution is not selected** (§6).
- **Q-08, N-24, N-25, and F-8 are untouched, unresolved, and unnarrowed** by
  this document.

## 10. Unresolved Questions Flagged (Not Resolved Here)

- **UI-conformance testing strategy.** No pattern exists in this repository for
  testing that a presentation surface actually satisfies RFC-0003 §9.1–§9.11
  (as distinct from unit-testing individual view functions). This is a design
  question for the eventual N-22 implementation-scoping cycle, not decided or
  assumed by this ADR.
- **Accessibility conformance level.** No accepted authority currently states a
  required assistive-technology conformance level for Ordina's desktop surface.
  egui's `AccessKit` maturity is noted as a real, current limitation (§5,
  Negative) but this ADR does not establish a minimum bar to measure it
  against — that would itself be a product/UX decision outside this ADR's
  architecture-decision authority.
- **Slint's dual license, noted for completeness only.** Not resolved here —
  Slint was rejected on architectural grounds independent of its licensing
  terms (§8), so the license question did not need to be, and was not,
  adjudicated.

---

> ADRs are immutable once Accepted. If the decision changes, write a new ADR and
> mark this one Superseded. Never edit the reasoning of a past decision — the
> record of why we were wrong is the most valuable part of the archive.
