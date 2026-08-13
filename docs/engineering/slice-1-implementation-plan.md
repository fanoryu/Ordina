# Slice-1 Implementation Plan

- **Tier:** 3 (engineering)
- **Status:** ACTIVE — Phase 0 complete, **Phase 1 CLOSED**, **Phase 2A CLOSED**;
  Phase 2B and Phases 3–8 not started and **not authorized**
- **Authority:** PBA-0040 / PBA-0041; Phase-1 closure under PBA-0048; Phase-2A
  closure under PBA-0054
- **Realizes:** First Runnable Slice Contract (ACCEPTED, PBR-0011) under ADR-0005
  (ACCEPTED, PBR-0013)
- **Created:** 2026-08-13

> **This plan sequences work. It decides nothing.** The **First Runnable Slice
> Contract is authoritative for behaviour**; ADR-0005 is authoritative for the
> stack. Where this plan could ever be read as differing from either, they
> govern and this plan is defective. No phase may introduce a semantic rule that
> is not already derivable from the accepted contract — if something desirable
> is not derivable, it is an open question for the Product Board, not code.

---

## Working method

Per ADR-0005 §10, the implementer is learning Rust during Slice 1. The loop is:

> **small increment → compile → test → inspect → commit**

Prefer clear, idiomatic, simple Rust. Avoid clever type-level programming,
macros, advanced lifetime designs where owned values suffice, async and
concurrency, and unnecessary generics. Prefer small explicit functions and
compiler errors a reader can understand. **Do not introduce abstractions to
compensate for unfamiliarity** — the remedy is smaller increments and more
tests, not more machinery.

Every phase ends green on the full verification routine:

```
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets
```

run from an MSVC-initialized environment (see Phase 0). **`--all-features` is
never used** — it would activate ADR-0005 §6.4-prohibited optional features.

---

## Phase 0 — Bootstrap verification ✅ COMPLETE

| | |
|---|---|
| **Contract / ADR sections** | ADR-0005 §5 (toolchain), §6 (dependencies), §7.2 (float), §7.7 (unsafe) |
| **Modules** | `Cargo.toml`, `rust-toolchain.toml`, `Cargo.lock`, `src/lib.rs`, empty module tree, `tests/conformance/main.rs` |
| **Tests** | dependency-availability test (exact rational: ⅓+⅓+⅓ = 1); integration-target link test |
| **Prerequisites** | MSVC Build Tools + Windows SDK installed; Rust 1.97.1 MSVC |
| **Exit criteria** | ✅ all five commands exit 0; MSVC `link.exe` confirmed in effect; lockfile free of prohibited crates |
| **Explicitly not in scope** | Any contract behaviour whatsoever |

**Environment note that will recur.** Git for Windows ships `link.exe` (a Unix
coreutil) and it is on the system PATH. Cargo must be invoked from a shell
initialized by `vcvars64.bat`, where Microsoft's linker takes precedence. Left
unhandled this produces a garbled `link: missing operand` failure, not a clean
"linker not found".

## Phase 1 — Core model and opaque domains ✅ CLOSED

| | |
|---|---|
| **Contract sections** | §3 (admitted construct inventory), §3.3–§3.10, §3.3.1 (opaque domains) |
| **Modules** | `model` |
| **Tests** | construction and rejection of each admitted construct shape; opaque-domain ordering delegates to the supplied order |
| **Prerequisites** | Phase 0 |
| **Exit criteria** | Every §3 construct representable; the five opaque domains are distinct newtypes so cross-domain comparison is a **type error**; closed categories are closed Rust enums; verification green |
| **Not in scope** | Any check, any defect, any evaluation |

Two ADR-0005 rules bind here. **§7.5** — `#[derive(Ord)]` on an opaque-domain
newtype is prohibited; authoritative order comes exclusively from the
contract-supplied total order, though a hand-written `Ord` that faithfully
delegates to it is expected. **§7.4** — no catch-all `_` arm when matching a
closed authoritative category.

### Closure record — Phase 1 is CLOSED

**Closed 2026-08-13** by Product Board authorization **PBA-0048**, reported in
**ER-0051**, at baseline `946d889`.

**Basis.** Closure rests on **ARR-0016** — Phase-1 Final Closure Verification,
authorized by PBA-0047 — which returned `PASS — PHASE 1 READY FOR PRODUCT BOARD
CLOSURE`. ARR-0016 independently established from repository evidence that
ARR-0015 finding M-1 is closed, that `DeclaredCategory::declared_count()` is
removed with no semantically equivalent lossy replacement, that all four
authoritative declaration states remain distinct and that the former call sites
preserve those distinctions directly, that no Phase-1 regression exists, that
every required verification command passes, that the ADR-0005 guards remain
satisfied, that dependency and toolchain state remain within the accepted
boundary, that no Phase-2 implementation exists, that deferred decisions remain
deferred, that accepted documents remain intact, that the identifier ledger is
coherent, and that no BLOCKER, MAJOR, or MINOR finding remains.

The Phase-1 cycle in full: implementation ER-0048 (PBA-0042) → review ARR-0014
(PBA-0043) → representability remediation ER-0049 (PBA-0044) → re-review
ARR-0015 (PBA-0045) → helper remediation ER-0050 (PBA-0046) → final verification
ARR-0016 (PBA-0047) → closure ER-0051 (PBA-0048).

**What closure means.**

1. The Phase-1 representability foundation this plan requires **has been
   implemented**: every §3 construct is representable, and every malformed state
   the accepted Slice Contract assigns a Well-Formedness defect condition
   remains constructible.
2. Its final remediation has been **independently verified**.
3. **No known BLOCKER, MAJOR, or MINOR finding remains** against Phase 1.
4. The Phase-1 baseline is **accepted as the foundation** for subsequent,
   separately authorized work.

**What closure does not mean.**

5. It does **not** imply completion of Slice 1 — Phases 2 through 8 are not
   started.
6. It does **not** authorize Phase 2. Phase 2 requires its own Product Board
   authorization.
7. It resolves **no** deferred product or architecture decision. Q-06, Q-08,
   N-22, N-24, N-25 and F-8 remain **OPEN**, and no platform, framework, UI
   technology, persistence, database, serialization format, parser, schema
   language, deployment target or provider, evaluator implementation,
   evaluator-semantics version scheme, hash algorithm, CI/CD platform, release
   packaging model, broker, or market-data provider is selected. Rust remains a
   Slice-1 decision only, as established by ADR-0005.

**Carried observations — non-blocking for closure, not resolved.**

| Ref | Observation | Status |
|---|---|---|
| **O-1** | `EvidenceDeclaration.declared_values` is represented through a sequence whose derived equality is **sequence-sensitive**, although the Slice Contract §3.5 describes the domain as a **set**. | **DEFERRED — MUST BE SETTLED BEFORE CHECK 6 CONSUMES THE REPRESENTATION** (check 6 lands in Phase 5). |
| **O-2** | Check 13A's prohibited-operator defect population is structurally constrained as reviewed; its non-`NUMERIC`-leaf population remains representable, so the check is not vacuous. | **BOARD-NOTED — non-blocking.** Check 13A is not implemented. |
| **O-3** | Check 5's structural guarantee is narrower than some source prose implies; a real representable population remains through lineage resolution. | **BOARD-NOTED — non-blocking.** The accepted Slice Contract is not amended. |

## Phase 2 — Exact numeric representation (2A ✅ CLOSED · 2B not authorized)

| | |
|---|---|
| **Contract sections** | §4.2 (numeric), §4.3–§4.4 (equality, comparison), §4.6 (dimension) |
| **Modules** | `numeric` |
| **Tests** | exactness (no rounding at any magnitude); unbounded magnitude; exact equality; dimension equality only; rejection of cross-dimension comparison |
| **Prerequisites** | Phase 1 |
| **Exit criteria** | An internal exact-rational value type encapsulating `BigRational` behind the ADR-0005 §6.3 boundary, so no other module names a third-party numeric API; **no `f32`/`f64` anywhere**; verification green including `clippy` |
| **Not in scope** | Arithmetic beyond `+`, `−`, `×`; division (§3.8 excludes it) |

**§7.3** — `BigRational` is wider than the contract's admissible literal domain.
Representation capability must not become semantic admissibility: only values the
contract admits are accepted.

### Phase 2 was split into 2A and 2B

ER-0052 found that this phase's two halves have different prerequisites. The
value layer (§4.2) depends on nothing unsettled; the dimension-aware layer
(§4.3, §4.4, §4.6) is gated on two open questions — **F-1**, which module hosts
it, and **F-3**, how the distinguished `DIMENSIONLESS` symbol is bound given
that dimension symbols are opaque and caller-supplied. The phase was therefore
split so the unblocked half could proceed.

| Sub-phase | Scope | Status |
|---|---|---|
| **2A** | §4.2 — the exact numeric value layer, dimension-free | **CLOSED** |
| **2B** | §4.3, §4.4, §4.6 — dimension-aware semantics | **not started, not authorized**; gated on F-1 and F-3 |

### Closure record — Phase 2A is CLOSED

**Closed 2026-08-14** by Product Board authorization **PBA-0054**, reported in
**ER-0055**, at baseline `25d17de`.

**Basis.** Closure rests on **ARR-0018** — Phase-2A Final Closure Verification,
authorized by PBA-0053 — which returned `PASS — PHASE 2A READY FOR CLOSURE`
against all nineteen of its closure criteria.

The Phase-2A cycle in full: implementation ER-0053 (PBA-0050) → review ARR-0017
(PBA-0051), which found no semantic failure but required M-1 structural
regression evidence → remediation ER-0054 (PBA-0052), which added that evidence
with no runtime semantic change → final verification ARR-0018 (PBA-0053) →
closure ER-0055 (PBA-0054).

**A limitation the record must not obscure.** ARR-0018 disclosed that the same
agent participated in implementing, remediating and reviewing Phase 2A. The
review was independent in **method** — the diff was read unfiltered, the public
API enumerated from source, and the structural boundaries confirmed by compiling
probes outside the test harness — but **not in authorship**. This closure is
recorded on that basis and is not represented as organizationally independent
review.

**What was closed.** The bounded Phase-2A scope, and nothing beyond it:
`ExactNumber`; exact decimal-rational construction; arbitrary-precision
representation; exact mathematical equality; deterministic exact mathematical
value ordering; exact `add`, `subtract` and `multiply`; structural exclusion of
arbitrary-ratio construction; encapsulation of the internal `BigRational`;
structural absence of division; float exclusion; the `unsafe` prohibition;
purity and determinism; the dependency and feature boundaries; and the Phase-2A
regression and compile-fail evidence.

**Final finding dispositions.**

| Ref | Disposition | Basis |
|---|---|---|
| **ARR-0017 M-1** | **CLOSED** | ER-0054 added dependency-free executable compile-fail evidence that arbitrary-ratio construction, internal-representation access, and division all remain unavailable. ARR-0018 verified each boundary independently and confirmed no runtime semantic change. |
| **ER-0052 F-2** | **CLOSED** | Every publicly constructible authoritative numeric value remains within the accepted exact-decimal-rational domain: arbitrary-ratio construction unavailable, representation private, division absent, admitted arithmetic closed under `+`, `−`, `×`. |
| **ER-0052 F-4** | **CLOSED** | `ExactNumber` provides exact deterministic dimension-free value ordering, while dimension-aware semantic comparison remains absent and structurally separate — the type carries no dimension, so §4.4 comparison cannot be reached through it. |

**What closure does not mean.** Slice 1 is **not** complete. Phase 2B has not
begun and is **not** authorized by this closure — it requires its own Product
Board authorization, and the F-1 and F-3 gating questions must be resolved
first. No mandatory Well-Formedness check, defect, verdict, prerequisite gating
or canonical ordering exists. No deferred decision is resolved: Q-06, Q-08,
N-22, N-24, N-25 and F-8 all remain **OPEN**.

**Carried observations — not resolved by this closure.**

| Ref | Observation | Status |
|---|---|---|
| **F-1** | Which module hosts the dimension-aware §4.3/§4.4/§4.6 semantics — `numeric` or `semantics` — is unsettled. | **OPEN — gates Phase 2B.** |
| **F-3** | The distinguished `DIMENSIONLESS` symbol has no binding, and one must not be invented: dimension symbols are opaque with no encoding selected (§3.3.1). | **OPEN — gates Phase 2B.** |
| **B-4** | `Literal` has no `PartialEq`, `Eq`, `PartialOrd` or `Ord`. That absence is **protective**: it is what prevents `ExactNumber`'s value equality from silently becoming §4.3 equality that ignores the identical-dimension requirement. | **CARRIED.** Preserve deliberately; §4.3 equality must be built as an explicit dimension-aware operation. |
| **B-5** | `ExactNumber::from_integer(i64)` remains, retained because six `src/model/**` call sites depend on it and that tree was outside the Phase-2A authorization. It does **not** limit the authoritative domain — `from_decimal` is public and unbounded. | **CARRIED.** No migration authorized. |
| **O-1** | `EvidenceDeclaration.declared_values` has sequence-sensitive derived equality although §3.5 describes a set. Phase 2A never consumed it. | **DEFERRED — settle before check 6.** |
| **O-2** | Check 13A's prohibited-operator population is structurally empty; its non-`NUMERIC`-leaf population remains representable. | **BOARD-NOTED — non-blocking.** |
| **O-3** | Check 5's structural guarantee is narrower than some source prose implies; a real population remains through lineage resolution. | **BOARD-NOTED — non-blocking.** |

## Phase 3 — Guard and constraint-domain semantics

| | |
|---|---|
| **Contract sections** | §3.6 (guards), §4.5 (conjunction), §4.10 (constraint domains) |
| **Modules** | `semantics` |
| **Tests** | per-reference constraint domains for `NUMERIC`/`BOOLEAN`/`ENUM`; empty conjunction holds always; atom order carries no meaning; contradiction, overlap, subsumption across intervals with excluded points |
| **Prerequisites** | Phases 1–2 |
| **Exit criteria** | The three relations decided by per-reference comparison of declared literals; **no solver or constraint library**; verification green |
| **Not in scope** | Direction resolution; any check; disjunction or negation (not admitted) |

## Phase 4 — Defect / result model and canonical ordering

| | |
|---|---|
| **Contract sections** | §6.1–§6.4, including §6.4.1 (26 categories), §6.4.2 (6 discriminator kinds), §6.4.3 (literal order), §6.3 (identity + non-comparability) |
| **Modules** | `result` |
| **Tests** | total ordering — no two distinct defects compare equal; each of the 26 categories orderable; each discriminator kind; the §6.3 revision-qualification boundary |
| **Prerequisites** | Phases 1–3 |
| **Exit criteria** | Binary verdict; deterministic defect identity; a **total** canonical order; **no `HashMap`/`HashSet` on any authoritative path** (ADR-0005 §7.6); verification green |
| **Not in scope** | Producing defects — no check runs yet |

## Phase 5 — Mandatory Well-Formedness checks

| | |
|---|---|
| **Contract sections** | §5.1 (the 15 applicable checks), §5.1.1 (check-6 property mapping) |
| **Modules** | `checks` |
| **Tests** | per check: the PASS condition and **every** declared defect condition |
| **Prerequisites** | Phases 1–4 |
| **Exit criteria** | All 15 applicable checks implemented; checks 7 and 17 remain out of scope because their constructs are not admitted; verification green |
| **Not in scope** | Prerequisite gating (Phase 6); checks 7 and 17 |

Implement in dependency order: 1, 16, 15, 2 → 3, 4, 6 → 5 → 8, 10 → 9, 11, 12 →
13A/13B, 14A/14B. Take one check per increment.

## Phase 6 — Prerequisite gating and not-evaluated metadata

| | |
|---|---|
| **Contract sections** | §5.2 (governing invariant, unit of work, exhaustive edge list) |
| **Modules** | `checks`, `result` |
| **Tests** | each §5.2 worked consequence, including the empty-`ENUM` case suppressing only dependent contradiction analysis, and a missing `TAKE_PROFIT` suppressing only 14B |
| **Prerequisites** | Phase 5 |
| **Exit criteria** | Gating is **per unit of work**, never Strategy-wide; unrelated defects never suppress an independently-computable check; not-evaluated records are deterministic; verification green |
| **Not in scope** | Full conformance suite (Phase 7) |

## Phase 7 — Conformance obligation-8 suite

| | |
|---|---|
| **Contract sections** | §8 obligation 8; §5, §6 in full |
| **Modules** | `tests/conformance` |
| **Tests** | WELL-FORMED and UNUSABLE outcomes; complete defect sets; defect identities; canonical ordering; prerequisite gating; not-evaluated metadata — over **in-memory fixtures** |
| **Prerequisites** | Phases 1–6 |
| **Exit criteria** | The suite would detect any divergence a second conforming implementation could exhibit; verification green |
| **Not in scope** | A second implementation |

**Fixture rule.** Fixtures are constructed **in memory** as Rust builders. There
is **no JSON, YAML, or TOML Strategy fixture format** — a data-file format would
implicitly select the deferred serialization format and introduce a parser into
a slice defined as having none (Slice Contract §2, §7.4).

## Phase 8 — Acceptance and hardening

| | |
|---|---|
| **Contract sections** | §8 conformance obligations 1–8 |
| **Modules** | all |
| **Tests** | adversarial and malformed inputs — a malformed artifact yields UNUSABLE, never a crash or bypass |
| **Prerequisites** | Phases 1–7 |
| **Exit criteria** | All eight §8 obligations demonstrably met; totality and purity confirmed; verification green; ready for Product Board review |
| **Not in scope** | Anything beyond Slice 1 |

`proptest` may be proposed here as an optional dev-only dependency for the §4.10
algebraic properties, under ADR-0005 §8 and §8.1. Property tests may express
**only** properties already derivable from the accepted contract.

---

## Out of scope for every phase

Q-06, Q-08, N-22, N-24, N-25 and F-8 remain **OPEN** and are not resolved by any
phase. No phase selects a framework, UI technology, persistence, database,
serialization format, parser, schema language, deployment target, provider,
broker or market-data integration, evaluator implementation, hash algorithm,
CI/CD platform, or release packaging model. No phase introduces `unsafe`.
