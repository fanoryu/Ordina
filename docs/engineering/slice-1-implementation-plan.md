# Slice-1 Implementation Plan

- **Tier:** 3 (engineering)
- **Status:** ACTIVE — Phase 0 complete, **Phase 1 CLOSED**, **Phase 2A CLOSED**,
  **Phase 2B CLOSED**, **Phase 3 CLOSED**, **Phase 4 CLOSED**; **Phase 5 Slice E
  CLOSED**, **Phase 5 Slice F CLOSED**, **Phase 5 Slice G CLOSED (independently
  verified under ARR-0041)**, **Phase 5 Check 13A CLOSED (independently verified
  under ARR-0042)**, **Phase 5 Check 13B CLOSED (independently verified under
  ARR-0043)**, **Phase 5 Check 14A CLOSED (independently verified under
  ARR-0044)**, Phases 6–8 not started and **not authorized**
- **Authority:** PBA-0040 / PBA-0041; Phase-1 closure under PBA-0048; Phase-2A
  closure under PBA-0054; Phase-2B closure under PBA-0066; Phase-3 closure under
  PBA-0072; Phase-4 closure under PBA-0078
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
| **O-1** | `EvidenceDeclaration.declared_values` is represented through a `Vec<EnumValue>` whose derived equality is sequence-sensitive. This is **deliberately permissive**, not a contract violation: the accepted contract (§3.5 "a finite, non-empty set of **distinct** opaque categorical values"; §5.1.1 the "value set is empty" and "value set contains duplicate values" violations) treats the value set as a **set** — order is meaningless and duplicates are a *fault to detect*, not a multiplicity to preserve. The model comment in `src/model/evidence.rs` confirms the sequence is intentional so check 6 can observe empty/duplicate states. | **RESOLVED-BY-DETERMINATION (PBA-0118 / ER-0099) — NOT a model defect; the O-1 gate is discharged.** Revision 4 already uniquely fixes the semantics as a set. Check 6 must apply set semantics (membership / empty / duplicate detection) and must **not** use order or duplicate multiplicity. **The earlier "describes a set … MUST BE SETTLED BEFORE CHECK 6" framing misstated the contract** and has been corrected; no `src/model` change, no shared helper, no Revision 5. Check 6 is eligible for separate authorization under DETERMINATION B. |
| **O-2** | Check 13A's prohibited-operator defect population is structurally constrained as reviewed; its non-`NUMERIC`-leaf population remains representable, so the check is not vacuous. | **CORRECTED-BY-DETERMINATION (PBA-0126 / ER-0104).** The prohibited-operator/form structural-emptiness observation **stands**: the closed six-variant `DerivationExpression` (`src/model/derivation.rs`) makes every §3.8-prohibited form unrepresentable (proven in part by the module's `compile_fail` doctest). The claim that **non-`NUMERIC` leaves keep 13A non-vacuous is withdrawn as incorrect** — unresolved and non-`NUMERIC` derivation leaves are owned by **Check 2** (`UNRESOLVED_REFERENCE` / `TYPE_MISMATCH`, emitted by `check_derivation_expression` in `src/checks/reference_integrity.rs`), and §5.2 makes Check 2 on that derivation a **13A prerequisite**, so such a leaf makes 13A *non-computable* rather than producing `INVALID_DERIVATION_FORM`. **Check 13A is therefore structurally zero-population** under the accepted closed-type model — an accepted consequence of ADR-0005 §7.4, **not** a contract defect and **not** a Revision-5 trigger. Per the determination, Check 13A is **retained and shall be explicitly realized** (Check-16 `UNKNOWN_CONSTRUCT` precedent), preserving its boundary (prohibited forms only; leaves remain Check 2; dimensional mismatch remains Check 14A); the model must not be broadened and Check-2 leaf faults must not be re-reported. Next permitted source-writing action: **Phase-5 Check 13A implementation only.** |
| **O-3** | Check 5's structural guarantee is narrower than some source prose implies; a real representable population remains through lineage resolution. | **BOARD-NOTED — non-blocking.** The accepted Slice Contract is not amended. |
|| **O-4** | `Guard.atoms` is represented through a sequence, although Slice Contract §3.6/§4.5 give a guard **set** semantics ("two guards with the same atom set are the same guard"), so a content-identical duplicate atom is representable. Same class as O-1. Reachable through check 5 and, prospectively, accepted check 2. | **OPEN (prospective).** ER-0097 RULING 2 settles the semantics: one defect per **distinct** atom. Check 5 was conformed locally at its evaluation layer by PBA-0116 / ER-0098; **the `Guard` representation is unchanged and O-4 is not resolved.** Check 2's parallel divergence is frozen until separately authorized. Slice-G (Checks 9, 11, 12) conforms Guards 9/11/12 locally without touching the representation; O-4 remains OPEN. |

### Post-closure remediation — canonical occurrence identity (Revision 4)

**Remediated 2026-08-16** under Product Board authorization **PBA-0096**, reported
in **ER-0086**, at baseline `7c11d69`, on the authority of accepted **Slice
Contract Revision 4** (PBR-0016).

Revision 4 §C.1 adds a sixth opaque domain — a mandatory, non-optional,
caller-supplied, opaque **canonical construct-occurrence identity** with a supplied
total order — and makes it the offending-construct anchor for defect identity and
canonical ordering (§6.3/§6.4). This bounded post-closure cycle implements **only**
the Phase-1 consequence: an [`OccurrenceId`] opaque newtype (`src/model/opaque.rs`,
alongside the five semantic domains, hand-written supplied order, no `derive(Ord)`)
and a mandatory `occurrence: OccurrenceId` field on every construct occurrence — the
Strategy envelope and every evidence declaration, configuration parameter,
condition, Direction rule, and candidate derivation. The field is non-optional, so
a construct cannot be built without an occurrence identity; it is present even when
`lineage` is absent, and it is a distinct domain from lineage.

**Scope boundary.** No defect model, discriminator, or check was changed: Phase-4's
`DefectCore.offending` still uses `LineageId`, and `src/checks/**` remains
non-substantive. The **Phase-4 result-model remediation** (`offending →
OccurrenceId`, the `LINEAGE` discriminator, the COUNTERPART occurrence payload,
check-9/check-11 anchors) and Phase-5A remain **pending** and separately
authorized. O-1/O-2/O-3 remain OPEN; O-1 still gates check 6. Phase 1 remains
**CLOSED**; this remediation awaits an **independent Phase-1 reconciliation review**
before the Phase-4 remediation may be authorized.

## Phase 2 — Exact numeric representation (2A ✅ CLOSED · 2B ✅ CLOSED)

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

### Closure record — Phase 2B is CLOSED

**Closed 2026-08-15** by Product Board authorization **PBA-0066**, reported in
**ER-0064**, at baseline `44d9181`.

**Basis.** Closure rests on **ARR-0021** — Phase-2B Independent Implementation
Review, authorized by PBA-0065 — which returned `PASS — READY FOR PHASE-2B FORMAL
CLOSURE` with zero BLOCKER, MAJOR or MINOR findings and three non-blocking
observations.

The Phase-2B cycle in full: implementation ER-0063 (PBA-0064) → independent
review ARR-0021 (PBA-0065) → closure ER-0064 (PBA-0066).

> **A limitation the record does not obscure.** ARR-0021 disclosed that the same
> agent implemented and reviewed Phase 2B. The review was independent in method
> — the diff was read directly, the B-4 compile-fail evidence was re-verified by
> compiling probes outside the harness, and each contract branch was re-derived
> — but not in authorship. This closure is recorded on that basis.

**Closed scope — the §4.3 / §4.4 / §4.6 semantic primitives in `src/semantics/`,
and nothing beyond.**

- **§4.3 equality** — the named `equal` operation: numeric same-dimension exact
  equality; numeric cross-dimension `DimensionMismatch` (a type defect, never
  `false`); boolean equality; enum categorical equality; cross-kind
  `TypeMismatch`; no coercion.
- **§4.4 comparison** — the named `compare` operation: numeric same-dimension
  `Less`/`Equal`/`Greater`; numeric cross-dimension `DimensionMismatch`;
  `OrderingUndefined` on `BOOLEAN`/`ENUM`; cross-kind `TypeMismatch`; no public
  `Ordering` result; `Literal` carries no ordering traits.
- **§4.6 dimensional rules** — `additive_dimension` and `scaling_dimension`
  returning a closed `DimensionOutcome`, with `DimensionBinding` carrying the
  canonical-input-supplied `DIMENSIONLESS` identity, recognised by exact
  dimension-symbol equality; no inferred dimensionless identity, no unit
  conversion, no dimension algebra beyond Slice 1.

**Carried decisions confirmed.** F-1 (semantics in `src/semantics/`,
`semantics → model → numeric`) and F-3 (CLOSED; binding consumed, never
manufactured) both satisfied. B-1 satisfied — `DimensionBinding` passed per
invocation, no ambient state. B-4 satisfied — `Literal` implements none of
`PartialEq`/`Eq`/`PartialOrd`/`Ord`, compiler-verified. B-5 retained —
`ExactNumber::from_integer(i64)` unchanged. Phase 2A remains CLOSED: `ExactNumber`
unchanged, dimension-free, no `BigRational` leakage, no division, no floats.

**What closure does not mean.** Slice 1 is **not** complete. These are semantic
**primitives**; no mandatory check (2, 6, 14A, 16), defect, verdict, or canonical
ordering is implemented. **As of that Phase-2B closure, Phase 3 (§4.10
constraint domains) was not started or authorized** (it was subsequently
authorized under PBA-0067 and is now CLOSED — see the Phase 3 closure record
below). No deferred decision is resolved: Q-06, Q-08, N-22, N-24, N-25 and F-8
remain **OPEN**; **N-24** in particular is untouched.

**Carried observations — non-blocking, not resolved.**

| Ref | Observation | Status |
|---|---|---|
| **B-1 caller authority** | The future checker-harness authorization must state that the `DimensionBinding::new(...)` argument originates from canonical input. The binding is correctly supplied per invocation today; the enforcement point is one layer up, in the harness. | **CARRIED to Phase 3 / harness.** |
| **§4.6 mismatch granularity** | `scaling_dimension` returns one `Mismatch` for both a dimensioned numeric scalar and a non-numeric scalar. Contract-faithful at the primitive layer; the check layer must not assume the primitive distinguishes which mandatory check owns the higher-level failure. | **CARRIED to the check layer.** |
| **Total primitive API** | `equal`/`compare` are total over all `Literal` pairs; cross-kind `TypeMismatch` is part of the primitive surface. ARR-0021 concluded `TOTAL PRIMITIVE APPROPRIATE`. | **CARRIED — no action.** |

**Carried gating observations from Phase 2A** (unchanged): **B-4** and **B-5**
above; **O-1** still gates check 6; **O-2**, **O-3** Board-noted.

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
| **2B** | §4.3, §4.4, §4.6 — dimension-aware semantics | **CLOSED** (F-1 and F-3 both resolved) |

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

### Phase-2B gating decisions

Recorded under **PBA-0056** (ER-0057), continuing the analysis of ER-0056.
Three of the four gating items are settled; the fourth is not, and Phase 2B
stays blocked on it.

| Item | Status |
|---|---|
| **F-1** — module placement | **RESOLVED** |
| **F-3** — distinguished-member binding | **CLOSED** — resolved normatively by Slice Contract Revision 2, accepted under PBR-0014 (2026-08-14) |
| **B-4** — `Literal` equality | **DISPOSITION RECORDED** |
| **B-5** — `from_integer(i64)` | **DISPOSITION RECORDED** |

> **Phase 2B is UNBLOCKED** — all four gating items are settled. F-1, B-4 and
> B-5 constrain *how* it will be built; F-3, which decided *whether* it could be
> built conformantly, is now closed. Unblocking is **not** an implementation
> authorization: Phase 2B still requires a separate Product Board authorization
> before any code is written.

#### F-1 — dimension-aware semantics belong in `semantics`

The §4.3 / §4.4 / §4.6 operations go in `src/semantics/`, not `numeric` and not
`model`.

The reason is dependency direction, read from the live tree rather than
preference. `numeric` imports nothing from the crate; five `model` files import
`numeric`. The established direction is **`model → numeric`**, with `numeric` a
leaf. Dimension-aware comparison needs `DimensionSymbol` and `Literal`, both in
`model`, so placing it in `numeric` would close a module cycle — and two things
would break: ADR-0005 §6.3's property that replacing the numeric representation
"touches one module and no check logic" would become false, and `numeric` would
stop being the narrow third-party isolation boundary §6.3 defines. `model` is
equally wrong: it declares constructs, and these are semantics *over* declared
constructs.

`semantics` already owns semantic rules — Phase 3 is assigned to it — and exists
empty, so no new module name is required. The resulting order is
`semantics → model → numeric`, acyclic.

Read this plan's Phase-2 **Modules** cell as naming the phase's *primary*
module; §4.3/§4.4/§4.6 live in `semantics`.

#### B-4 — `Literal` must not gain Rust's equality or ordering traits

`Literal` must continue to derive and implement **no** `PartialEq`, `Eq`,
`PartialOrd` or `Ord`. §4.3 equality arrives instead as a **named semantic
operation** in `semantics`, returning a closed multi-state outcome matched
exhaustively.

`PartialEq` can never faithfully mean §4.3 equality, and the reason is decisive
rather than stylistic: §4.3 says cross-dimension equality "is **not** `false` —
it is a **type defect**", while `PartialEq::eq` returns `bool`. A faithful result
needs at least *equal*, *not equal*, *dimension mismatch* and *type mismatch*, so
implementing the trait would necessarily collapse a type defect into `false` —
silently turning a check-2/check-14 defect into a passing comparison. That is the
failure mode already remediated twice, at ARR-0014 MAJOR-1 and ARR-0015 M-1.

Within the operation: numeric same-dimension equality delegates to Phase-2A's
`ExactNumber` value equality; dimension mismatch and type mismatch each keep
their own outcome. Because no `PartialEq` exists, `l1 == l2` does not compile —
there is no syntax to reach for, so the guarantee is structural.

**A consequence for Phase 4.** §6.4.2's ATOM discriminator orders atoms partly by
literal under §6.4.3, which is a total order *across* literal types. `Literal`
will therefore need an **ordering key** — and, exactly as F-4 established for
`ExactNumber`, it must be a named defect-ordering function and never an `Ord`
impl, so it cannot be mistaken for §4.4 semantic comparison.

#### B-5 — `from_integer(i64)` is retained unchanged

No migration is scheduled. The constructor is convenience-only; it does **not**
define the authoritative magnitude boundary, because `from_decimal` remains the
public arbitrary-precision construction path and admits values of any size. No
change to `ExactNumber` semantics is implied.

Replacing it was considered and rejected: an arbitrary-precision variant would
have to accept `BigInt`, breaking the ADR-0005 §6.3 isolation boundary, or a
digit sequence — which `from_decimal` already is. If a migration is ever wanted,
the occasion is a cycle already authorized to touch `src/model/**` for an
independent reason, realistically Phase 5. It should not motivate a cycle of its
own.

#### F-3 — why it was blocked, and how it was closed

> **CLOSED 2026-08-14.** F-3 is resolved by **Slice Contract Revision 2 §2.1**,
> accepted by the Product Board under **PBR-0014** (authoring PBA-0059; review
> ARR-0019 → remediation ER-0061 → re-review ARR-0020; acceptance PBA-0063 /
> ER-0062). §2.1 makes the distinguished-member bindings for `DIMENSIONLESS` and
> `SM-1` **constitutive canonical input**, so a conforming checker receives their
> identities and manufactures neither. The account below is the authoring-time
> analysis, preserved for provenance. `WF-1` is unchanged; N-24 stays OPEN.

`DIMENSIONLESS` appears **exactly once** in the **accepted Revision-1** corpus — Slice
Contract §4.6, "a **dimensionless literal** carries the distinguished dimension
symbol `DIMENSIONLESS`" — and no accepted document says how a conforming
implementation identifies it. Every route is closed: picking a handle selects an
encoding for an opaque domain (§3.3.1, ADR-0005 §7.5); reading it out of the
supplied order contradicts §3.3.1's statement that the dimension order "carries
no semantics"; and inferring it from an absent dimension contradicts §4.6 and
§4.1. §2's m-2 paragraph makes exactly two things constitutive of the canonical
input — the domain's values **and** its total order — and a distinguished-member
binding is a third.

This is not optional. §5.1 **check 14A** requires that every `*` node have a
dimensionless literal operand, so a conforming checker cannot evaluate a
mandatory check without deciding dimensionlessness.

**The Product Board has ruled** (PBA-0056 §4) that a contract-declared
distinguished member of an opaque domain is part of the canonical semantic input
for that domain, supplied as a binding, never manufactured by the
implementation. **That ruling is not yet normative against the accepted
contract**, and this plan cannot make it so: Slice Contract §11.3 requires "an
explicitly authorized successor or revision — never a silent rewrite", and this
plan decides nothing. The clarification belongs in §2, and the authorized act
that makes it effective is a separate Product Board matter.

Recorded here as current-state status only. Phase-2B implementation must not
begin until the ruling is normatively effective.

**The same gap exists for `SM-1`.** §3.4.1 declares the semantic-model token as
a contract-declared constant and check 16 compares against it, but `VersionToken`
is opaque with no encoding selected — the identical identification problem, one
check later. A ruling framed only around `DIMENSIONLESS` would leave it to be
rediscovered in Phase 5.

## Phase 3 — Guard and constraint-domain semantics

| | |
|---|---|
| **Contract sections** | §3.6 (guards), §4.5 (conjunction), §4.10 (constraint domains) |
| **Modules** | `semantics` |
| **Tests** | per-reference constraint domains for `NUMERIC`/`BOOLEAN`/`ENUM`; empty conjunction holds always; atom order carries no meaning; contradiction, overlap, subsumption across intervals with excluded points |
| **Prerequisites** | Phases 1–2 |
| **Exit criteria** | The three relations decided by per-reference comparison of declared literals; **no solver or constraint library**; verification green |
| **Not in scope** | Direction resolution; any check; disjunction or negation (not admitted) |

### Closure record — Phase 3 is CLOSED

**Closed 2026-08-15** by Product Board authorization **PBA-0072**, reported in
**ER-0068**, at the post-remediation baseline `3956089`.

**Lineage.** Implementation `PBA-0067 / ER-0065` → independent review
`PBA-0068 / ARR-0022` (**FAIL — REMEDIATION REQUIRED**, one MAJOR finding M-1)
→ bounded remediation `PBA-0069 / ER-0066` → independent re-review
`PBA-0070 / ARR-0023` (**PASS — M-1 CLOSED**) → attribution-history remediation
`PBA-0071 / ER-0067` (Git metadata only; Phase-3 tree preserved) → formal
closure `PBA-0072 / ER-0068`.

**What was closed — the §4.10 per-reference constraint-domain algebra in
`src/semantics/constraint/`, and nothing beyond it.** Three per-reference domain
types, each with admission, emptiness (contradiction), intersection, overlap
(non-empty intersection) and containment (subsumption, `D2 ⊆ D1` iff
`D1.contains(&D2)`):

- **`NumericDomain`** — an exact-rational interval (inclusive/exclusive/unbounded
  bounds) minus a finite excluded set, folding all six comparison operators; no
  float, no solver, exact `ExactNumber` comparison throughout.
- **`BooleanDomain`** — subsets of `{true, false}`.
- **`EnumDomain`** — an admitted subset of the declared value set by exact opaque
  identity; order-independent, relabelling-invariant.

**M-1 — CLOSED.** The reviewed and independently re-verified rule is
`D1.contains(&D2)` iff `D2 ⊆ D1` for **every** representable `NumericDomain`.
Endpoint canonicalization (`[a, b] \ {a}` ≡ `(a, b]`) preserves admitted-set
identity for the lower endpoint, the upper endpoint, both endpoints, the
singleton/empty edge, unbounded sides, and intersection-produced domains
(ARR-0023 §5, §13).

**Reviewed boundaries preserved.** `PER-REFERENCE PRIMITIVE BOUNDARY SUFFICIENT`
(whole-guard contradiction/overlap/subsumption belong to checks 9/11/12, not
here) and `CORRECT LAYERING` (builders consume already-typed constraints; check-2
typing is an upstream prerequisite).

**What closure does not mean.** Slice 1 is **not** complete. No mandatory check,
defect, verdict, or canonical ordering is implemented. **Phase 4 is NOT STARTED
and NOT AUTHORIZED.** O-1, O-2, O-3 remain unresolved; Q-06, Q-08, N-22, N-24,
N-25, F-8 remain OPEN. B-1 remains a future checker/harness caller obligation.

## Phase 4 — Defect / result model and canonical ordering

| | |
|---|---|
| **Contract sections** | §6.1–§6.4, including §6.4.1 (26 categories), §6.4.2 (6 discriminator kinds), §6.4.3 (literal order), §6.3 (identity + non-comparability) |
| **Modules** | `result` |
| **Tests** | total ordering — no two distinct defects compare equal; each of the 26 categories orderable; each discriminator kind; the §6.3 revision-qualification boundary |
| **Prerequisites** | Phases 1–3 |
|| **Exit criteria** | Binary verdict; deterministic defect identity; a **total** canonical order; **`HashMap`/`HashSet` iteration order must not influence authoritative or conformance-visible results; use deterministic ordered structures or explicitly sort under the contract's declared orders before an authoritative result is produced** (ADR-0005 §7.6); verification green |
| **Not in scope** | Producing defects — no check runs yet |

### Closure record — Phase 4 is CLOSED

**Closed 2026-08-15** by Product Board authorization **PBA-0078**, reported in
**ER-0072**, at the post-remediation baseline `3310743`.

**Lineage.** Implementation `PBA-0074 / ER-0070` → independent review
`PBA-0075 / ARR-0024` (**FAIL — REMEDIATION REQUIRED**, one MINOR finding M-1:
the §6.2 defect-set/verdict biconditional was not enforced on the `unusable`
path) → bounded remediation `PBA-0076 / ER-0071` → independent re-review
`PBA-0077 / ARR-0025` (**PASS — M-1 CLOSED**) → formal closure
`PBA-0078 / ER-0072`.

**What was closed — the §6.1–§6.4 result and defect model in `src/result/`, and
nothing beyond it.** Data model and canonical ordering only; **no mandatory
check runs and no defect is produced.**

- **Verdict / result (`verdict.rs`, `mod.rs`)** — binary `WellFormed` /
  `Unusable`; `WellFormednessResult` carries verdict, defect set, not-evaluated
  set, and the bound checker-semantics token `WF-1`. The §6.2 invariant "defect
  set empty **iff** `WellFormed`" is structural: `well_formed` yields an empty
  set, `unusable` requires a first defect, so neither forbidden state is
  constructible (M-1 fix).
- **Defect model (`category.rs`, `discriminator.rs`, `identity.rs`)** — the 26
  §6.4.1 categories with explicit ordinals and owning check numbers; the 6
  §6.4.2 discriminator kinds; `DefectCore` with the total §6.4 order (check →
  lineage → category → discriminator); optional caller-supplied
  `RevisionIdentity`.
- **§6.3 non-comparability** — `DefectIdentity` implements no `Eq`/`Ord`/`Hash`;
  comparison is only `try_cmp`/`try_eq` (`None` across the qualification
  boundary or across differing revisions) and `canonical_sort` (rejects a mixed
  set). Verified independently by ARR-0024 §7 and ARR-0025 §5.
- **§6.4.3 literal order** — numeric by exact value then supplied dimension
  order; `false < true`; enum by supplied order — all delegating to supplied
  opaque orders, never handle magnitude (ADR-0005 §7.5). `HashMap`/`HashSet`
  iteration order must not influence authoritative or conformance-visible
  results; use deterministic ordered structures or explicitly sort under the
  contract's declared orders before an authoritative result is produced
  (ADR-0005 §7.6).

**M-1 — CLOSED.** No safe public path constructs `UNUSABLE` + empty or
`WELL-FORMED` + non-empty; a `compile_fail` doctest and runtime tests guard the
boundary (ARR-0025 §5–§6).

**What closure does not mean.** Slice 1 is **not** complete. No mandatory check,
defect production, prerequisite gating, Direction logic, or verdict orchestration
is implemented; `src/checks/**` remains empty. **Phase 5 is NOT STARTED and NOT
AUTHORIZED.** O-1, O-2, O-3 remain unresolved (O-1 still gates check 6); Q-06,
Q-08, N-22, N-24, N-25, F-8 remain OPEN. Carried observations OBS-1 (identity
component accessors) and OBS-3 (`PropertyPosition` supplied ordinal) require no
remediation; OBS-2 (crate-doc currency) was resolved in this closure cycle.

### Post-closure remediation — occurrence-anchored defect identity (Revision 4)

**Remediated 2026-08-16** under Product Board authorization **PBA-0098**, reported
in **ER-0087**, at baseline `c902eec`, on the authority of accepted **Slice
Contract Revision 4** (PBR-0016), following the Phase-1 occurrence-identity
remediation (ER-0086) and its reconciliation (ARR-0030 PASS).

This bounded post-closure cycle migrates the §6.3/§6.4 result model to the accepted
occurrence-anchor design:

- **`DefectCore.offending: LineageId → OccurrenceId`** — the universal defect
  anchor is now the occurrence identity (Revision 4 §C.4), so defects stay
  distinct and orderable even when a construct's lineage is absent or duplicated.
  §6.4 key 2 orders by the supplied occurrence order; the totality rationale no
  longer relies on lineage uniqueness (§C.9).
- **`Discriminator::Counterpart` payload `LineageId → OccurrenceId`** (kind 6,
  ordinal unchanged) — a pairwise defect's counterpart is representable when a
  rule's lineage is absent or duplicated (§C.7/§C.8).
- **New `Discriminator::Lineage(LineageId)` kind 7** — carries the duplicated
  lineage identity of an envelope-anchored `DUPLICATE_LINEAGE_IDENTITY` defect,
  ordered by the supplied lineage order (§C.5/§C.8). Kinds 1–6 are not renumbered.

The result model now represents the accepted anchor rules: ordinary → offending
occurrence; Strategy/class-level → envelope occurrence + (ROLE | LINEAGE)
discriminator; check-9 symmetric → lower-occurrence anchor + COUNTERPART occurrence;
check-11 asymmetric → subsumed-rule occurrence anchor (regardless of order) +
subsuming-rule COUNTERPART occurrence. Focused adversarial tests exercise each.

**Scope boundary.** The `ATOM` discriminator's referenced payload stays lineage
identity (a semantic reference, not an anchor); the §6.2 **not-evaluated
representation is unchanged** (Revision 4 §C.11 preserves it — its `construct`
field remains `LineageId`); no §5.1 population, §5.2 prerequisite, or mandatory
check was implemented; `src/checks/**` remains non-substantive. Phase 4 remains
**CLOSED**; this remediation awaits an **independent Phase-4 reconciliation
review**. O-1/O-2/O-3 remain OPEN. This completes the accepted Revision-4
implementation delta; only Phase-5A (mandatory checks) remains, separately
authorized and still blocked.

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

### Phase 5A — check 1 IMPLEMENTED (independent review pending)

**Implemented 2026-08-16** under Product Board authorization **PBA-0100**, reported
in **ER-0088**, at baseline `3081abd`, on the reconciled Revision-4 model
(ER-0086/ARR-0030 Phase 1; ER-0087/ARR-0031 Phase 4).

`checks::required_property` implements **check 1** (`MISSING_REQUIRED_PROPERTY`) as
pure per-construct defect production returning `Vec<DefectCore>` (empty = PASS). It
follows the accepted Revision-3 §C.3 / Revision-4 ownership matrix exactly: it emits
for the envelope's five required collections (positions 3–7), evidence-declaration
lineage (1) and declared-type absence (2), configuration-parameter lineage/dimension/
bounds/adjustable (1, 3, 4, 5, 6), Direction-rule precedence/guard (2, 3),
candidate-derivation role/expression (2, 3), and condition guard (3). Each defect
anchors on the offending construct's **occurrence identity** (Revision 4 §C.4), so a
missing-lineage defect on an evidence declaration or configuration parameter is
reportable even when that lineage is itself absent.

**Exclusions honoured:** operative-node lineage absence (envelope, condition,
Direction rule, candidate derivation) → check 15 only; envelope version absence →
check 16 only; present-but-invalid states → checks 6/3/10/4/16; `declared_values`
never inspected (O-1 stays OPEN). No prerequisite gating, not-evaluated record,
verdict, revision qualification, or global canonical sort is implemented; no other
mandatory check exists. **Independent review of check 1 is required** before the
next check increment. This is a per-check increment of Phase 5, not Phase-5
completion.

### Phase 5 Slice B — checks 15 and 16 IMPLEMENTED (independent review pending)

**Implemented 2026-08-16** under **PBA-0103**, reported in **ER-0090**, at baseline
`b494a27`, per the ER-0089 roadmap.

- `checks::operative_provenance` — **check 15** (`MISSING_OPERATIVE_PROVENANCE`):
  one defect per operative node (envelope, condition, Direction rule, candidate
  derivation) whose lineage is absent, anchored on the node's occurrence identity,
  NONE discriminator. EvidenceDeclaration/ConfigurationParameter lineage absence is
  **not** reported (check 1's, Revision 3 §C.3); duplicate present lineage is not
  reported (check 2's). No dual reporting.
- `checks::semantic_model_version` — **check 16** version half
  (`UNSUPPORTED_SEMANTIC_MODEL_VERSION`): absent or non-`SM-1` envelope version →
  one envelope-anchored defect, NONE discriminator. The contract-declared supported
  token is a **caller-supplied parameter** (§3.4.1 forbids the implementation
  selecting it; the Phase-1 model does not encode it), compared by exact equality
  (§3.3.1). `UNKNOWN_CONSTRUCT` is **structurally unrepresentable** in the typed
  model (§3.4) and is not produced; no unknown-construct state is invented, and the
  category remains defined for a future external-shape-admission boundary.

Both are defect production only (`Vec<DefectCore>`); no gating, verdict, or sort.
Check 1 (ER-0088) is unchanged. O-1/O-2/O-3 remain OPEN. Remaining slices per
ER-0089: C (check 2), D (checks 8+10), E (checks 3+4+5), O-1 gate, F (check 6),
G (checks 9+11+12), H (checks 13A/B+14A/B).

### Phase 5 Slice C — check 2 IMPLEMENTED (independent review pending)

**Implemented 2026-08-16** under **PBA-0105**, reported in **ER-0091**, at baseline
`d32f402`, per the ER-0089 roadmap.

`checks::reference_integrity` implements **check 2** — all four categories:

- `DUPLICATE_LINEAGE_IDENTITY` — one defect per duplicated lineage **value** (not
  per occurrence) over every lineage-bearing construct Strategy-wide; **envelope
  occurrence anchor + `LINEAGE` discriminator** (Revision 4 §C.5); absent lineage
  never participates.
- `UNRESOLVED_REFERENCE` — a reference (guard-atom evidence, derivation evidence /
  configuration leaf) that does not resolve to **exactly one** declared item;
  duplicate targets are ambiguous → unresolved (§4.7, the checker never picks one).
- `TYPE_MISMATCH` — an ill-typed guard atom (type tag, operator admissibility, or
  NUMERIC dimension identity, §4.1/§4.4/§4.6) or a non-`NUMERIC` derivation leaf.
- `ENUM_VALUE_NOT_DECLARED` — an `ENUM` atom whose value is not a **member** of the
  referent's declared value set — membership only, so **O-1 stays OPEN**.

Discriminators: guard-atom defects use `ATOM`, derivation-node defects use
`NODE-PATH`, duplicate-lineage uses `LINEAGE` (§6.4.2). Anchors are occurrence
identities. Typing is computed only against a determinate declared type / present
dimension / present value set (intrinsic defect definition, **not** §5.2 gating).
Defect production only; deterministic; no prerequisite gating, verdict, or sort.
Checks 1/15/16 unchanged; `src/model`/`src/result` unchanged. O-1/O-2/O-3 OPEN.
Remaining: D (checks 8+10), E (3+4+5), O-1 gate, F (6), G (9+11+12), H (13A/B+14A/B).

**M-1 remediation (PBA-0108 / ER-0093, 2026-08-17).** ARR-0034 FAIL / ER-0092
**RULING A**: an ENUM atom that both uses an operator inadmissible for ENUM **and**
carries a value outside the declared set emits **both** `TYPE_MISMATCH` and
`ENUM_VALUE_NOT_DECLARED` (distinct at the §6.4 category ordinal; "one per atom"
bounds `TYPE_MISMATCH` only). The ENUM arm of `check_typed_atom` now evaluates the
operator and membership predicates independently; TYPE_MISMATCH stays one-per-atom;
O-1 isolation and all other check-2 surfaces are preserved. Independent re-review of
M-1 required before Slice D.

**Slice D — checks 8 and 10 (PBA-0110 / ER-0094, 2026-08-17).** Implements Direction
coverage and Direction determinism on the reconciled Revision-4 model, defect
production only. **Check 8** (`src/checks/direction_coverage.rs`, `check_8`): three
independent §5.1 populations — `DIRECTION_COVERAGE_INCOMPLETE` (no always-applicable
rule, incl. zero rules; Strategy-level → **envelope** occurrence, `NONE`, one per
Strategy), `MULTIPLE_TOTAL_DIRECTION_RULES` (>1 always-applicable; envelope, `NONE`,
one per Strategy), `TOTAL_RULE_NOT_LOWEST_PRECEDENCE` (an always-applicable rule with
a strictly-lower present precedence; anchored on **that rule's** occurrence, `NONE`,
one per offending rule). Multiple-total and not-lowest are evaluated **independently**
(ER-0092 RULING-A independence). "Always-applicable" = present-and-syntactically-empty
guard; "not lowest" uses the **supplied** §3.3.1 precedence order, never container /
traversal / occurrence order. Reads guards only for emptiness — **no** check-2/check-6
dependency. **Check 10** (`src/checks/direction_determinism.rs`, `check_10`):
`INVALID_DIRECTION_OUTCOME`, one per rule whose outcome is zero/multiple/out-of-domain,
anchored on the rule occurrence, `NONE`. Outcome validity is check 10's sole ownership
(R3 §3.7 table — outcome absence owned by check 10, not check 1), so no check-1
duplication. Collection *presence* stays check 1's: both checks evaluate only over a
**present** `direction_rules` collection (predicate computability, not §5.2 gating).
No new discriminator; `src/model`/`src/result` unchanged; checks 1/2/15/16 unchanged;
no Phase-6 orchestration. O-1/O-2/O-3 OPEN. Independent read-only review required.
Remaining: E (3+4+5), O-1 gate, F (6), G (9+11+12), H (13A/B+14A/B).

**Slice E — checks 3, 4, 5 (PBA-0113 / ER-0096, 2026-08-17).** Defect production
only, on the reconciled Revision-4 model. **Check 3** (`condition_classification.rs`,
`check_3`): `INVALID_CONDITION_CLASSIFICATION`, one per condition whose classification
is zero/multiple/out-of-domain, anchored on the condition occurrence, `NONE`. Sole
owner of classification validity (guard absence stays check 1's, lineage absence check
15's) — no duplication. **Check 4** (`configuration_bounds.rs`, `check_4`): the two
§4.11 conjuncts as independent categories — `INVERTED_CONFIGURATION_BOUNDS`
(`lower > upper` by exact `ExactNumber` value; one per parameter; envelope-free —
parameter-occurrence anchor, `NONE`) and `CONFIGURATION_BOUND_DIMENSION_MISMATCH` (a
NUMERIC bound whose `DimensionSymbol` ≠ the parameter's; one per offending bound;
parameter-occurrence anchor, **`PROPERTY`** position 4=lower/5=upper — the §6.4.2
discriminator that keeps two same-parameter dimension defects distinct). Bound
absence stays check 1's; a non-numeric/absent operand leaves the predicate
intrinsically undefined (no manufactured defect, no §5.2 gating). No numeric/dimension
algebra reimplemented — reuses `ExactNumber: Ord` and `DimensionSymbol: Eq`. **Check 5**
(`configuration_position.rs`, `check_5`): `CONFIGURATION_IN_STRUCTURAL_POSITION`. Under
the accepted typed model the only reachable structural position is a **guard atom**
(condition or Direction-rule guard) whose `evidence` lineage resolves — by the same
exactly-one §4.7 resolution as check 2 — to a declared configuration parameter; that is
the **O-3** population that "remains through lineage resolution", implemented without
amending the contract or resolving O-3. Anchored on the enclosing construct occurrence
with the existing **`ATOM`** discriminator (required for §6.4 totality; mirrors check 2's
guard-atom faults); one per offending reference. Never emits a check-2 category; a
guard atom resolving to a config parameter is independently check 2's
`UNRESOLVED_REFERENCE`, produced by check 2 alone. No new discriminator/category;
`src/model`/`src/result`/`src/numeric`/`src/semantics` unchanged; checks 1/2/8/10/15/16
unchanged; no Phase-6 orchestration. O-1/O-2/O-3 remain OPEN (O-3 BOARD-NOTED). Independent
read-only review required. Remaining: O-1 gate, F (6), G (9+11+12), H (13A/B+14A/B).

**Slice-E review and B-1/B-2 determination (PBA-0114 / ARR-0037; PBA-0115 / ER-0097,
2026-08-17).** ARR-0037 returned **FAIL — BOUNDED REMEDIATION OR BOARD DETERMINATION
REQUIRED**. Checks 3 and 4 were independently verified with **zero findings** and are
not reopened. Two BLOCKERs were raised against check 5: **B-1**, that `ATOM` is used
where §6.4.2's `Used by` cell names checks 2 and 12; and **B-2**, that two
content-identical offending guard atoms produced defects equal on all four §6.4 keys —
the same ATOM behaviour being reachable through accepted check 2. Two MINORs: **m-1**
(no content-identical-atom test) and **m-2** (check-2/check-5 co-occurrence unasserted).

**ER-0097**, adopted by the Product Owner, disposes of both under the ER-0092 precedent
and **requires no successor revision**. **RULING 1** — §6.4.2's `Used by` entries are
**descriptive applicability guidance, not exhaustive assignments**: §6.3 component 3
introduces the same check lists with "**for example**", states the entitlement
structurally ("where one construct can produce multiple defects of the same category"),
and delegates only *kinds and their values* to §6.4.2. **Check 5's use of `ATOM` is
therefore authorized, and B-1 is CLOSED — NOT A DEFECT.** **RULING 2** — under §3.6 and
§4.5 a guard has **set** semantics, so content-identical atoms in one guard are **one
semantic atom**; "one per atom" / "one per offending reference" means **one per distinct
atom**. Exactly one defect is required, `ATOM(lineage, operator, literal)` **is total**
over the semantic atom set, the Revision-4 §C.9 totality proof **stands as written**,
and **no positional, ordinal, traversal, or occurrence-based component may enter defect
identity** (§6.4, §C.1, §4.5). Duplicate-identical atoms collapse as canonical-input set
semantics **before** predicate evaluation — never by deduplicating emitted defect
identities, which §6.3 forbids. **B-2 is therefore reclassified BLOCKER → MAJOR**: an
incorrect-multiplicity divergence confined to `src/checks/configuration_position.rs`,
with m-1 and m-2 carried as its test obligations. `src/result/**` is **not** touched and
**no Revision 5 is required**. Accepted **check 2** shows the same divergence
prospectively — its acceptance and ARR-0035 **stand**, and it is **out of scope and
frozen** until separately authorized. New observation **O-4** records the underlying
`Guard.atoms` sequence-vs-set representation issue. Check-5 bounded remediation and its
independent re-review remain **pending authorization**; O-1 gate, F closed, G implemented, H unchanged.

**B-2 bounded remediation (PBA-0116 / ER-0098, 2026-08-17).** Implements ER-0097
RULING 2 in `src/checks/configuration_position.rs` — the only file changed. A
`distinct_atoms` helper applies §3.6/§4.5 guard-**set** semantics at the check-5
evaluation layer: the predicate is evaluated once per **distinct** atom, keyed on the
same `(referenced lineage, operator, literal)` content the `ATOM` discriminator uses, so
two content-identical offending atoms in one guard now emit **exactly one**
`CONFIGURATION_IN_STRUCTURAL_POSITION` defect. The collapse is input-level and happens
**before** predicate evaluation; no emitted `DefectCore` is deduplicated (§6.3), and no
collection index, traversal order, or source position participates (§6.4, §C.1, §4.5).
Unchanged: the O-3 guard-atom population, exactly-one §4.7 resolution, condition and
Direction-rule guard coverage, the enclosing-construct occurrence anchor, the existing
`ATOM` discriminator (no new kind), and legitimate check-2/check-5 co-occurrence.
Distinct atoms still yield distinct defects. Tests added: **m-1** in both enclosing
forms, a content-scope guard (atoms differing only by operator stay two defects), a
duplicated non-offending atom case, and **m-2** (check-2 `UNRESOLVED_REFERENCE` and
check-5 co-occurring, neither suppressing the other). Baseline 300 unit + 1 integration
+ 5 doctests + 14 compile-fail, 0 failures; fmt/check/clippy clean. **B-1 remains CLOSED
— NOT A DEFECT.** `src/model`/`src/result`/`src/numeric`/`src/semantics`, every other
check, `Cargo`/toolchain, and Revision 4 unchanged; **check 2 not modified**; no Phase-6
machinery. **O-4 remains OPEN and prospective** — `Guard` representation untouched.
Independent read-only re-review of the remediation required. Remaining: O-1 gate,
F (6), G (9+11+12), H (13A/B+14A/B).

**O-1 representation-settlement determination (PBA-0118 / ER-0099, 2026-08-17).**
Product Board determination under the ER-0092 precedent, adopted by the Product
Owner. Reconstructing from first principles, Revision 4 **already uniquely fixes**
`declared_values` semantics as a **set**: §3.5 specifies a "finite, non-empty set of
**distinct** opaque categorical values" (distinct is normative), §5.1.1 lists the
"value set is empty" and "value set contains duplicate values" violations (so
duplicates are a *fault*, not a multiplicity), and `src/model/evidence.rs` states
the `Vec<EnumValue>` is "deliberately so: §5.1 check 6 must be able to detect an
**empty** set and **duplicate** values." Therefore member order is meaningless and
duplicate-identical members have no multiplicity: `[A,B] ≡ [B,A]` and `[A,A] ≡ [A]`.
**DETERMINATION B — existing contract sufficient; Check 6 implementation must apply
set semantics.** Check 6 is structurally per-member / set-level: a present-and-non-
empty test (absent/empty → one `INVALID_EVIDENCE_DECLARATION`, PROPERTY 4), a
duplicate detection over members (duplicate → one defect, PROPERTY 4), and a
per-member membership test for each guard-atom value (`ENUM_VALUE_NOT_DECLARED`,
ATOM discriminator). It requires neither collection-identity comparison nor sequence
semantics; order and duplicate multiplicity must be **ignored**, never deduplicated
post-hoc. **No `src/model` change, no shared helper, no Revision 5.** The prior
plan framing ("sequence-sensitive derived equality … §3.5 describes a set … MUST BE
SETTLED BEFORE CHECK 6") **misstated the contract** and is corrected above; O-1 is
reclassified from DEFERRED-gate to **RESOLVED-BY-DETERMINATION**. The O-1 gate is
**discharged**; **Check 6 is eligible for separate authorization.** O-4 remains
OPEN/prospective and independent (analogous pattern, different domain — not settled
by this determination); check 2 frozen. ARR-0038's single non-blocking MINOR carried
forward. Remaining: **F (6), G (9+11+12), H (13A/B+14A/B)**.

**Check 6 implementation (PBA-0119 / ER-0100, 2026-08-17).** Implements check 6
`INVALID_EVIDENCE_DECLARATION` in `src/checks/evidence_integrity.rs` (module
registered in `src/checks/mod.rs`). Check 6 is the **exclusive owner of
type-dependent evidence-declaration validity** (§3.5 M-1): it assumes check 1
passed for the declaration's unconditional properties (lineage identity, declared
type presence) and judges only dimension/value-set validity. Each violation → one
`INVALID_EVIDENCE_DECLARATION`, anchored on the declaration's `OccurrenceId`,
discriminated by the **PROPERTY** kind (§6.4.2 kind 2) carrying the §3.5 property
position — declared type = 2, declared dimension = 3, declared value set = 4 — per
§5.1.1 m-3. **O-1 set semantics (ER-0099 DETERMINATION B)** are applied: duplicate
detection returns exactly one defect per declaration regardless of repetition
(`[A,A]`, `[A,A,A]`, `[A,B,A]`, `[A,A,B,B]` each → one value-set defect); order is
irrelevant (`[A,B] ≡ [B,A]` both clean); an empty set and a duplicate set are
distinct single defects; duplicate representation does **not** inflate an unrelated
dimension defect; **no** emitted-`DefectCore` deduplication and **no** collection
index / traversal / serialization / host-language order enters identity. A
non-semantic `PropertyPosition::position()` read accessor was added to
`src/result/discriminator.rs` (to assert exact discriminator values in tests); no
other `src/model`/`src/result` logic changed. Baseline 318 unit (was 300) + 1
integration + 5 doctests + 14 compile-fail, 0 failures; fmt/check/clippy clean.
Prior checks byte-identical; **check 2 frozen and unchanged**; **O-4 remains OPEN
and prospective** (`Guard.atoms` untouched); no Phase-6 machinery. **Governing
determination ER-0099 (DETERMINATION B) consumed.** **Reviewed by ARR-0039
(FAIL):** G-1 — the `PropertyPosition::position()` accessor lay outside PBA-0119's
authorized surface (`src/result/**` forbidden) and was not required; S-1 — the
`declared type = 2` arm over-reported `Absent`, double-reporting check 1's
PROPERTY(2). Both findings remediated by **PBA-0121** / ER-0101 (below; the remediation was mis-recorded as PBA-0120 in commit `1a1a978` and corrected under PBD-0005 §2 — PBA-0120 belongs to the ARR-0039 review). Remaining:
**G (9+11+12), H (13A/B+14A/B)**.

**Check 6 bounded remediation (PBA-0121 / ER-0101, 2026-08-17).** Remediates both
ARR-0039 findings. **G-1:** the unauthorized `PropertyPosition::position()` accessor
was removed from `src/result/discriminator.rs`; the file is restored **byte-identical**
to its pre-ER-0100 state. The one `.position()` call site in the check-6 test helper
was rewritten to compare whole `PropertyPosition` values (`new(3)` / `new(4)`).
**S-1:** `check_declaration`'s declared-type handling corrected so `Absent` → **no**
check-6 defect (it is check 1's `MISSING_REQUIRED_PROPERTY`/PROPERTY(2)), and
`Multiple | OutOfDomain` → exactly one `INVALID_EVIDENCE_DECLARATION`/PROPERTY(2); the
`match` remains exhaustive (`Absent` is an explicit empty arm). All other Check-6
semantics preserved (NUMERIC/BOOLEAN/ENUM rules, O-1 set semantics, duplicate
detection/multiplicity, order independence, occurrence anchors, PROPERTY(3)/(4),
population traversal, check-2 / checks 15-16 ownership). **T-1 discharged:** four new
tests prove the ownership split (`Absent` → no check-6 PROPERTY(2); check 1 owns
`Absent`; `Multiple`/`OutOfDomain` → one check-6 PROPERTY(2), check 1 not duplicating).
Baseline 322 unit (was 318) + 1 integration + 5 doctests + 14 compile-fail, 0 failures;
fmt/check/clippy clean. Prior checks byte-identical; **check 2 frozen and unchanged**;
**O-4 remains OPEN and prospective**; no Phase-6 machinery. **Check 6 remains pending
**Check 13B implemented (PBA-0128 / ER-0106, 2026-08-17).** New `src/checks/candidate_derivation_presence.rs`
+ `src/checks/mod.rs` registration. **Check 13B IMPLEMENTED (pending independent review under ARR-0043).**
**Check 13A CLOSED** (independently verified under ARR-0042). **Check 13B CLOSED** (independently verified under ARR-0043).

**Check 14A implemented (PBA-0130 / ER-0107, 2026-08-18).** New `src/checks/derivation_dimension.rs` + `src/checks/mod.rs` registration. Verification: 414 unit + 1 integration + 5 doctest + 14 compile-fail, 0 failures; fmt/check/clippy clean. **Check 14A CLOSED** (independently verified under ARR-0044 — verdict PASS; one non-blocking MINOR-001, two non-blocking NOTEs; O-4 remains OPEN/prospective; no Phase-6 leakage; frozen surfaces clean).

**Check-14B discovery-contradiction reconciliation (read-only, same cycle as PBA-0132/ER-0108).** Adjudicated a contradiction in the prior Check-14B discovery between §5.2's `check 14B → 13B + 14A on all three derivations` (a genuine semantic prerequisite) and a discovery sentence claiming a Strategy could carry both a 14A defect and an independently-emitted 14B defect. From accepted contract text alone (§5.2's dependency graph plus the worked missing-`TAKE_PROFIT`-suppresses-14B consequence), confirmed: a 13B or 14A prerequisite failure on any of the three derivations suppresses 14B entirely; the two never co-emit in one evaluation. The discovery sentence was corrected. No new authority; no contract revision; no repository mutation in the reconciliation itself.

**Check 14B implemented (PBA-0132 / ER-0108, 2026-08-18).** New `src/checks/candidate_dimension_divergence.rs` + `src/checks/mod.rs` registration. `check_14b` reconstructs the 13B prerequisite (exact-one-per-role) and, per role, a local `result_dimension` mirroring 14A's per-node computability discipline; emits at most one `CandidateDimensionDivergence` (anchor = Strategy envelope occurrence, `Discriminator::None`) only when all three root dimensions are computable and not pairwise equal; any prerequisite failure suppresses 14B with no defect emitted. No production call to `check_13b`/`check_14a`/Check 2/any runner; no `not-evaluated` metadata. Verification: 450 unit (was 414) + 1 integration + 5 doctest + 14 compile-fail, 0 failures; fmt/check/clippy clean. **Check 14B CLOSED** (independently verified under ARR-0045 — verdict PASS; two non-blocking NOTEs (NOTE-001: `result_dimension`/`evaluate` intentional duplication is a maintainability risk, not a defect; NOTE-002: no dedicated duplicate-role-plus-differing-dimension test, subsumed by existing duplicate-role suppression logic); corrected suppression invariant independently confirmed across all 14A/Check-2 non-computability shapes; O-4 remains OPEN/prospective; no Phase-6 leakage; frozen surfaces clean).

### Phase 5 status — current/canonical

| Check | Status | Review |
|---|---|---|
| 13A | **CLOSED** | ARR-0042 |
| 13B | **CLOSED** | ARR-0043 |
| 14A | **CLOSED** | ARR-0044 |
| 14B | **CLOSED** | ARR-0045 |

**Phase 5 — COMPLETE / CLOSED.** All fifteen applicable mandatory Phase-5 checks (1, 2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 13A, 13B, 14A, 14B, 15, 16 — see `src/checks/mod.rs`) are implemented and independently verified. No Phase-5 check remains pending. The historical slice-by-slice implementation narrative above (Slices A–H) is preserved as-is; this status block is the canonical current-state summary and supersedes any "remaining: H (14B only)" language earlier in this document.

Do not begin Phase 6 without explicit authorization.

## Phase 6 — Prerequisite gating and not-evaluated metadata

**Status: NOT STARTED / NOT AUTHORIZED (source implementation).** Read-only discovery completed (same cycle as the Check-14B closure baseline `d2133f4`); verdict **V2 — PHASE 6 PARTIALLY UNBLOCKED**, identifying two narrow D-class governance questions (`NotEvaluatedRecord`'s single-blocker capacity under a conjunctive prerequisite; whether the not-evaluated set requires a canonical total order) as the only items accepted authority left unsettled.

**Phase-6 NotEvaluated Blocker-Selection and Ordering-Scope Determination (PBA-0134 / ER-0109, 2026-08-18).** Product Board determination, read-only, no source-writing, adopted by the Product Owner. **RULING A:** one `NotEvaluatedRecord` per suppressed `(check, construct)` unit, `blocking: DefectIdentity` unchanged (singular); the root blocker propagates transitively through prerequisite chains (an immediate prerequisite's own not-evaluated blocker is inherited as a candidate), and where multiple independent root-blocker candidates exist for one unit, the existing §6.4 canonical `DefectIdentity`/`DefectCore` order selects the canonical-minimum one — no duplicate records, no `Vec<DefectIdentity>`. **RULING B:** the not-evaluated collection is set-membership semantics only; §6.4's canonical total order is textually scoped to defects, never extended to not-evaluated records; no new `Ord`/canonical-sort is introduced for `NotEvaluatedRecord`; emission order is an implementation convenience, never a conformance requirement. Both rulings verified realizable using only already-public `src/result/**` APIs (`DefectIdentity::try_cmp`, `DefectCore::Ord`) — **no `src/result/**` change required**, ARR-0039 G-1 intact, **no contract revision required** (both fill textual silences, not conflicts). **Classification: Phase 6 FULLY UNBLOCKED for bounded implementation slicing** — the discovery's contingent 6E (not-evaluated ordering) slice is no longer necessary.

**Revised implementation slicing (supersedes the discovery's provisional 6A–6E list):**

| Slice | Scope |
|---|---|
| 6A | reference/dependency enumeration (walk `Guard`/`DerivationExpression`, collect referenced lineages — reads `model/**` only) |
| 6B | prerequisite evaluation-state + root-blocker propagation (realizes Ruling A) |
| 6C | fixed-sequence orchestration / check execution (calls all 17 existing check functions once each) |
| 6D | defect aggregation via the existing `canonical_sort`, `NotEvaluatedRecord` collection via Rulings A/B, verdict assembly via the existing `WellFormednessResult` constructors |

Each slice is independently authorizable and independently reviewable, one check/slice per increment per established Phase-5 convention.

**Slice 6A implemented (PBA-0135 / ER-0110, 2026-08-18).** New `src/checks/dependency_enumeration.rs` + `src/checks/mod.rs` registration. `guard_dependencies(&Guard) -> Vec<LineageId>` and `derivation_dependencies(&DerivationExpression) -> Vec<LineageId>` perform pure structural enumeration only — no reference resolution, no type/dimension validation, no guard-analysis, no derivation-form/role-completeness validation, no call to any prior check; both sort (§3.3.1) then dedup, mirroring the existing `guard_analysis::every_reference` determinism pattern. No `NotEvaluatedRecord`, `WellFormednessResult`, `Verdict`, blocker-selection, or `canonical_sort` logic — Phase-6-leakage grep-audited clean. O-4 remains OPEN/prospective, not closed; `Guard.atoms` read structurally only, permutation-invariance proven by test. Verification: 468 unit (was 450) + 1 integration + 5 doctest + 14 compile-fail, 0 failures; fmt/check/clippy clean. **Slice 6A IMPLEMENTED, awaiting independent review under ARR-0046 (reserved, not allocated).** Do not mark Slice 6A CLOSED before ARR-0046.

| Slice | Status |
|---|---|
| 6A | **IMPLEMENTED — awaiting independent review (ARR-0046)** |
| 6B | NOT STARTED |
| 6C | NOT STARTED |
| 6D | NOT STARTED |

**Phase 6 is not complete.** Next Phase-6 implementation authorization (Slice 6B) requires its own PBA/ER pair (PBA-0136 / ER-0111, reserved, not allocated). Independent review of Slice 6A requires its own ARR (ARR-0046, reserved, not allocated).

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
