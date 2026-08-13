# RFC-0002: Slice-1 Implementation Language, Toolchain, and Test Tooling

- **Status:** PROPOSED — PRODUCT BOARD REVIEW REQUIRED
- **Created:** 2026-08-13
- **Authorizing artifact:** PBA-0032
- **Analysis basis:** ER-0039 (Slice-1 Implementation Stack Recommendation)
- **Governed by:** Product Constitution v1.0 — ACTIVE — RATIFIED
- **Conforms to:** RFC-0001 (ACCEPTED), ADR-0002 (ACCEPTED), ADR-0003 (ACCEPTED),
  ADR-0004 (ACCEPTED), First Runnable Slice Contract (ACCEPTED)
- **Decision vehicle:** ADR-0005 (not yet created)

> This RFC **proposes** the minimum implementation stack for **Slice 1 only**. It
> is not an architecture decision and confers no authority to write code. It
> selects **no** framework, UI, persistence, database, serialization format,
> parser, schema language, deployment target, hosting provider, broker or
> market-data integration, evaluator implementation, hash algorithm, CI/CD
> platform, or release packaging model — and it **does not resolve Q-06**.

---

## 1. Summary

Slice 1 is the deterministic Strategy Well-Formedness checker fixed by the
**ACCEPTED First Runnable Slice Contract**. It is a pure, total function from an
already-canonical in-memory Strategy structure to a verdict — Well-Formed, or
UNUSABLE with a deterministic defect set, defect identities, canonical defect
ordering, and not-evaluated metadata.

This RFC proposes:

| Item | Proposal |
|---|---|
| Language | **Rust** |
| Toolchain family | Official **stable** channel via `rustup` |
| Edition | **2024** |
| Toolchain policy | **Pin one exact stable version** (see §4) |
| Test harness | **`cargo test`** (built-in) |
| Production dependencies | **`num-bigint`**, **`num-rational`** — and nothing else |
| Dev-only dependency | **`proptest`** (optional, recommended) |
| `Cargo.lock` | **Committed** |
| Nature of Slice 1 | Pure library, **no I/O** |

The case for Rust rests on one property and not on general language merit: the
Slice-1 contract is a specification of **exhaustive discriminated unions over
exactly-represented numbers with no implicit coercion**, whose acceptance test
(Slice Contract §8, obligation 8) is that two independently written
implementations produce **identical** verdicts, defect sets, defect identities,
and defect ordering. Rust converts the largest share of the ways to fail that test
into **compile-time errors** rather than runtime surprises.

Its principal cost — a learning curve and lower initial velocity — is recorded
plainly in §11 and is not concealed.

## 2. Requirements Derived from the Accepted Contract

These, not preference, drive the proposal. Each cites the accepted contract.

| # | Requirement | Source |
|---|---|---|
| R1 | **Exact decimal rational** values; no binary floating-point semantics, no rounding, no precision loss, no implementation-defined numeric behaviour | §4.2 |
| R2 | **No minimum or maximum magnitude, no overflow, no underflow** — arbitrary precision | §4.2 |
| R3 | **No implicit coercion, widening, or promotion**; no null, absent, missing, `NaN`, or infinite value anywhere | §4.1 |
| R4 | **Exhaustive closed categories** — 6 construct kinds, 3 evidence types, 6 operators, 6 expression forms, **26 defect categories**, **6 discriminator kinds** | §3, §6.4.1, §6.4.2 |
| R5 | **Deterministic total orders** over five opaque domains, supplied with the input | §3.3.1 |
| R6 | **No hash-based, locale, host-language, filesystem, source-position, serialization-specific, or generated ordering** anywhere | §3.3.1 |
| R7 | **Total, pure, deterministic** — no I/O, clock, randomness, ambient state, side effects, or mutation of the input | §1.1 |
| R8 | Constraint-domain algebra — intervals with finitely many excluded points, boolean subsets, enum subsets; intersection, emptiness, containment | §4.10 |
| R9 | Per-unit-of-work prerequisite gating with deterministic not-evaluated metadata | §5.2 |
| R10 | **Cross-implementation identical results** | §8 obligation 8 |

Slice 1 has **no** UI, persistence, networking, broker, market-data, deployment,
serialization, parser, authoring flow, database, or external service dependency.
The stack decision is therefore constrained by little except correctness.

## 3. Proposed Language — Rust

**Proposed, not decided.** ADR-0005 is the decision vehicle (§12).

Rust is proposed because it enforces, structurally, the contract properties that
are otherwise matters of discipline:

- **R4 — exhaustiveness.** Closed semantic categories map onto Rust `enum`s and are
  consumed by exhaustive `match`. Adding a 27th defect category breaks the build at
  every site that must handle it. Under a checker whose acceptance test is
  cross-implementation agreement, a missed case becoming a compile error rather
  than a silent divergence is worth a great deal.
- **R3 — no implicit coercion.** Rust performs **no** implicit numeric conversion.
  There is no widening, no promotion, and no cross-type equality to guard against.
  The contract's hardest ambient rule is the language's default.
- **R7 — purity.** Ownership makes accidental mutation of the input a compile
  error rather than a code-review question.
- **R5 — opaque domains.** Newtypes make the five opaque domains mutually
  unforgeable, so a dimension symbol cannot be compared against a lineage identity
  by accident.
- **Forward fit.** Canonicalization (ADR-0003) and revision identity are the next
  work; byte-level determinism and exact numerics are native rather than
  retrofitted. This is stated as evidence of fit, **not** as a decision about those
  future components.

Rust is **not** proposed as generally superior to the alternatives. §10 records
where each alternative is genuinely stronger.

## 4. Toolchain and Version Policy

### 4.1 Proposed exact pin

**`1.97.1`**

**Basis, established at drafting time and not from memory.** The authoritative
Rust stable release channel manifest
(`https://static.rust-lang.org/dist/channel-rust-stable.toml`) — the same manifest
`rustup` itself consumes — was retrieved on **2026-08-13**. It reports:

- manifest `date = "2026-07-16"`;
- `[pkg.rust] version = "1.97.1 (8bab26f4f 2026-07-14)"`;
- `[pkg.rustc] version = "1.97.1 (8bab26f4f 2026-07-14)"`;
- artifact names cross-check as `rustc-1.97.1-<target>`.

Edition 2024 requires Rust ≥ 1.85; **1.97.1 satisfies this**.

No Rust toolchain is installed in the drafting environment, so the pin rests on the
authoritative published manifest rather than on a local `rustc --version`. The
Board should treat the pin as *the current stable at drafting time*; if
implementation begins materially later, the bootstrap task should re-read the same
manifest and the ADR should record whatever exact version is pinned.

### 4.2 Policy

- **Channel:** official **stable**. No nightly. No beta. No unstable feature gates.
- **Pinning:** an **exact version**, never a floating channel. A floating `stable`
  silently changes the compiler beneath a contract whose entire purpose is
  reproducibility, and would defeat R10.
- **Representation in the repository:** a root **`rust-toolchain.toml`** naming the
  exact version and the `rustfmt` and `clippy` components, so every contributor and
  every machine resolves the identical toolchain automatically.
- **Edition:** **2024**.
- **MSRV:** equal to the pinned version; raised only deliberately.
- **`Cargo.lock`:** **committed**. This departs from the usual library convention
  deliberately — Slice 1 is a conformance artifact, not a published crate, and
  pinning the exact dependency graph is the point.

**No file is created by this RFC.** These are the policies ADR-0005 would record
and an implementation-bootstrap authorization would realize.

## 5. Determinism Policies (proposed implementation constraints)

These realize the accepted contract. **They do not alter it.** Where this RFC and
the Slice Contract could ever be read as differing, the accepted contract governs.

### 5.1 Numeric semantics

- Authoritative Slice-1 numeric values are **arbitrary-precision exact rationals** —
  conceptually `BigRational` over `BigInt`, via `num-rational` + `num-bigint`. This
  satisfies **R1** and **R2** directly: no magnitude bound, no overflow, no
  underflow, no rounding.
- **No authoritative semantic path may use binary floating point.**
- **`f32` and `f64` are prohibited** from authoritative Slice-1 semantic logic.
  Proposed enforcement: deny `clippy::float_arithmetic` at crate level, so a float
  cannot enter without a deliberate, visible, reviewable override.
- Slice 1 admits only `+`, `−`, and `×` (Slice Contract §3.8 excludes division), so
  arithmetic is closed over exact rationals with no representability question.
  `1.0`, `1.00`, and `1e0` denote one value, as §4.2 requires.
- Comparison is **exact**. No epsilon, no tolerance, no quantization.

### 5.2 Type discipline

- **Rust `enum`s with exhaustive `match`** for every closed semantic category —
  construct kinds, evidence types, operators, expression forms, the 26 defect
  categories, the 6 discriminator kinds. No catch-all `_` arm on a closed category,
  so that extending a category is a build failure rather than a silent omission.
- **Strongly-typed newtypes** for the five opaque domains — `LineageId`,
  `DimensionSymbol`, `EnumValue`, `VersionToken`, `PrecedenceOrdinal` — each
  carrying its contract-supplied total order. Cross-domain comparison becomes a
  type error, which is precisely the accidental violation §3.3.1 warns against.

### 5.3 Deterministic ordering

**Hash-container iteration must not determine any authoritative result** —
verdict, defect identity, defect set, defect ordering, not-evaluated ordering, or
any semantic outcome (**R6**).

- `HashMap` and `HashSet` are **prohibited** on any path influencing an
  authoritative result. `BTreeMap`, `BTreeSet`, or explicitly sorted vectors are
  used instead.
- All ordering derives from the contract's declared orders — §6.4 keys, §6.4.1
  category ordinals, §6.4.2 discriminator rules, §6.4.3 literal order, and the
  §3.3.1 input-supplied domain orders.

### 5.4 Purity

The checker is a deterministic pure transformation over already-canonical
in-memory Strategy data (**R7**). Prohibited on the Slice-1 path: clock, randomness,
filesystem, network, environment-variable semantics, external process, and ambient
mutable state. Realized as a free function over owned, immutable input returning a
result value, with no interior mutability and no `&mut` on the input.

## 6. Dependency Proposal

Smallest viable surface. **No framework dependency is permitted.**

| Dependency | Purpose | Prod / dev | License | Transitive implications | Reversibility / isolation |
|---|---|---|---|---|---|
| **`num-bigint`** | Arbitrary-precision integers — satisfies R2 | **Production** | MIT **OR** Apache-2.0 | Pulls `num-traits`, `num-integer` — same ecosystem, same dual licence, no further fan-out | Confined to the `numeric/` module behind an internal value type; replaceable without touching check logic |
| **`num-rational`** | Exact rationals over `BigInt` — satisfies R1 | **Production** | MIT **OR** Apache-2.0 | Depends on `num-bigint`, `num-traits`, `num-integer` | Same isolation |
| **`proptest`** | Property tests for the §4.10 constraint-domain algebra | **Dev-only, optional** | MIT **OR** Apache-2.0 | Dev-tree only; never shipped | None — removable at any time |

**Versions are deliberately not pinned in this RFC.** Exact dependency versions are
resolved and locked in **`Cargo.lock`** during implementation bootstrap. Selecting
crate versions here, without current registry evidence, would risk stating a
version that is stale or inappropriate by the time the ADR is accepted.

**In-house alternative, recorded honestly.** Because Slice 1 needs only `+`, `−`,
`×`, a scaled-integer decimal implemented over `BigInt` alone — or over a
hand-written bignum — would remove one or both production dependencies. The RFC
**does not recommend** hand-rolling bignum arithmetic: it reinvents mature,
well-tested code in the one place the contract is least forgiving. If the Board
prefers a zero-production-dependency posture, the honest trade is stated rather
than hidden.

**No serialization, parser, solver, prover, or constraint library** is proposed.
Slice Contract §4.10 is explicit that the required operations are ordinary
comparisons between declared literals, which is exactly why the fragment is
bounded.

## 7. Test Strategy

### 7.1 Required — `cargo test`

The built-in harness, **zero additional dependency**. It supports unit tests
colocated with modules and integration tests under `tests/`, which is all Slice 1
requires.

### 7.2 Table-driven conformance tests

The conformance obligation is behavioural, so the primary suite is table-driven
over in-memory fixtures, covering:

- **WELL-FORMED** outcomes;
- **UNUSABLE** outcomes;
- **complete authoritative defect sets** — aggregate reporting, never fail-fast;
- **defect identities** — construct lineage identity, category, discriminator, and
  the optional revision qualification, including the §6.3 non-comparability rule
  across the revision-identity boundary;
- **canonical defect ordering** — §6.4 keys 1–4 including the §6.4.1 ordinals,
  §6.4.2 discriminator kinds, and §6.4.3 literal order;
- **not-evaluated metadata** — per-unit-of-work prerequisite gating (§5.2),
  including the worked cases the contract states.

Every applicable mandatory check (the 15 of Slice Contract §5.1) receives coverage
for both its PASS condition and each of its declared defect conditions.

### 7.3 Property-based testing — optional, dev-only

If `proptest` is adopted, it is **strictly limited to properties already implied by
the accepted semantics** of §4.10:

- overlap is **symmetric**;
- subsumption is **reflexive**;
- subsumption is **transitive** in the valid cases;
- a **contradictory guard overlaps nothing**;
- for non-contradictory guards, **subsumption implies overlap**.

**Property tests must not invent new product semantics.** A property not derivable
from the accepted contract does not belong in the suite; if such a property seems
desirable, that is an open question for the Board, not a test.

## 8. Minimal Implementation Layout

Conceptual only. **No path is created by this RFC.**

```
src/
  lib.rs          — public entry: check(strategy) -> WfResult
  model/          — §3 construct types + newtype opaque domains
  numeric/        — exact-rational value type; the only module aware of BigRational
  semantics/      — §4.10 constraint domains: intersection, emptiness, containment
  checks/         — the 15 applicable mandatory checks
  result/         — verdict, defect, defect identity, §6.4 ordering, not-evaluated
tests/
  conformance/    — one suite per check
  fixtures/       — in-memory Strategy builders
```

**Fixture rule, stated because the instinctive approach would violate the
contract.** Conformance fixtures are **constructed directly in memory** as Rust
builder functions. There is **no JSON, YAML, or TOML Strategy fixture format**.
Writing fixtures as data files would implicitly select a serialization format that
ADR-0002 §15 and Slice Contract §7.4 explicitly defer, and would introduce a parser
into a slice defined as having none. This is a direct consequence of the Slice
Contract §2 direct-canonical-input ruling.

No parser, no serialization decision, no persistence. No broader application
architecture is proposed.

## 9. Q-06 Severability

**Slice 1 has no platform or form-factor surface.** It has no UI, no networking, no
deployment target, and no process model. It is a pure function over in-memory data.

**Selecting Rust for Slice 1 does not resolve Q-06.** **Q-06 remains OPEN.** No
web, mobile, desktop, or API-first decision is made, implied, or prejudged by this
RFC.

Rust is **not** justified here by reference to any future platform. Possible future
execution targets — native library, WebAssembly, FFI from another runtime, or
server-side execution — are noted **only** as evidence that the choice preserves
optionality and reversibility. They are **not selected targets**, and nothing in
this RFC may be cited as having chosen one.

## 10. Alternatives Considered

Reasoning preserved from ER-0039 without exaggeration. Each alternative is
genuinely stronger than Rust somewhere, and that is recorded.

### 10.1 TypeScript / Node.js

**Stronger than Rust at:** developer velocity (highest of the four); zero required
production dependencies — `BigInt` is in-language, and a decimal modelled as
`{mantissa, exponent}` handles `+`, `−`, `×` exactly; a built-in test runner
(`node:test`); discriminated unions are idiomatic and exhaustiveness is checkable
via the `never` idiom.

**Why not proposed:** its **default numeric type is IEEE-754 double**, under a
contract whose §4.2 forbids binary floating-point semantics outright. Every numeric
literal written without care violates R1, and the type system that would catch it
is **erased at runtime**. The broader coercion surface — `==` across types, `+`
across types, `undefined` propagation — runs against R3, which forbids null,
absent, and `NaN` entirely. All of this is mitigable by lint rules and discipline;
discipline is simply the weakest available control for the property that matters
most here.

### 10.2 Python

**Stronger than Rust at:** exact rationals — **`fractions.Fraction` and
`decimal.Decimal` are in the standard library**, satisfying R1 and R2 with **zero
dependencies**, which is the single best numeric story of the four; high developer
velocity; a mature test ecosystem.

**Why not proposed:** type discipline is **opt-in** — exhaustiveness over 26 defect
categories and 6 discriminator kinds depends on an external checker being run and
kept strict, rather than on the language. And `set` iteration order **varies
between runs** under hash randomization, a live hazard for R6 in a component whose
acceptance test is reproducible ordering. Both are avoidable by convention; neither
is prevented by the language.

This was the closest call on numerics, and the Board should read it as such.

### 10.3 Go

**Stronger than Rust at:** exact rationals in the standard library (`math/big.Rat`,
`math/big.Int`) with zero dependencies; excellent cross-platform reproducibility
and build simplicity; a built-in test harness; high velocity.

**Why not proposed:** the contract is **ADT-shaped** — its core is discriminated
unions requiring exhaustive handling — and Go has **no sum types and no
exhaustiveness checking**. Modelling the 26 defect categories and 6 discriminator
kinds through interfaces and type switches surrenders exactly the compile-time
guarantee that motivates this proposal. Go's map iteration is additionally
*deliberately randomized*, a further R6 hazard.

### 10.4 Summary of the trade

**Rust's principal advantage for this slice is structural enforcement of the
accepted determinism and type invariants — not general language superiority.** On
raw numeric ergonomics Python and Go are better; on velocity TypeScript and Python
are better. Rust is proposed because Slice 1's whole value is being provably
deterministic.

## 11. Consequences and Trade-offs

**Positive.** Exhaustiveness failures become build failures. No implicit coercion
exists to defend against. Purity is enforced by ownership rather than by
convention. The toolchain pins cleanly and reproducibly. Newtypes make the opaque
domains unforgeable. Byte-level determinism is native for the canonicalization work
that follows.

**Negative — recorded plainly, not concealed:**

1. **Lowest developer velocity of the four candidates.** Rust's learning curve is
   real. The mitigating factor specific to this slice is that Slice 1 is a pure
   tree-walking function over owned immutable data — no concurrency, no lifetime
   complexity, no I/O — which is close to the easiest possible Rust.
2. **Two required production dependencies**, where each alternative needs zero.
   Small, permissively dual-licensed, and isolated behind one module, but a non-zero
   surface the others do not have.
3. **Team-fluency dependency.** A language the implementers do not know produces
   its own defects, and Slice 1's value is defect-freedom. **If the implementing
   engineers have no Rust experience and no appetite to ramp, this proposal should
   be reconsidered** — Python would then be the stronger candidate, on the strength
   of stdlib `Fraction`, with mandatory strict type checking and a prohibition on
   unsorted `set` iteration. This is a Board input, recorded in §14.
4. **`HashMap` remains available in the language** and would silently reintroduce
   nondeterminism if used on an ordering path. Addressed by explicit prohibition
   plus lint, not by hope.

## 12. Reversibility

This decision is **relatively reversible**, because Slice 1 is:

- **pure** — a function with no hidden state;
- **I/O-free** — no persistence, network, or filesystem coupling to unwind;
- **platform-neutral** — no deployment or form-factor commitment;
- governed by an **accepted, language-independent behavioural contract**.

A later port must reproduce the **same accepted authoritative results** — the same
verdicts, defect sets, defect identities, canonical ordering, and not-evaluated
metadata — because the First Runnable Slice Contract, not the implementation
language, is authoritative for behaviour.

**This RFC implies nothing about future Ordina components.** It does not establish
Rust as the language of the product, of the evaluator, of any authoring surface, or
of anything beyond Slice 1. Each future component decides on its own merits through
its own RFC and ADR.

## 13. Explicit Non-Decisions

This RFC selects **none** of the following:

application framework; web framework; UI framework; UI technology; persistence;
database; serialization format; parser; schema language; deployment target;
hosting or cloud provider; broker integration; market-data provider; evaluator
implementation; hash or digest algorithm; CI/CD platform; release packaging model.

The following remain **OPEN / deferred** and are untouched:

**Q-06** (platform / form factor) · **Q-08** (jurisdictions) · **N-22**
(Well-Formedness-failure UX) · **N-24** (evaluator-semantics version) · **N-25**
(authoring-to-canonical translation) · **F-8** (privacy / preimage on erased
low-entropy content).

## 14. Open Questions for the Product Board

1. **Team Rust fluency** (§11.3) — the one input that would legitimately change the
   recommendation. Does the implementing team have working Rust experience, or does
   the Board accept the ramp?
2. **Dependency posture** (§6) — accept `num-bigint` + `num-rational`, or require a
   zero-production-dependency implementation with the correctness trade stated?
3. **`proptest`** (§7.3) — adopt as an optional dev-only dependency, or defer?
4. **Pin currency** (§4.1) — accept `1.97.1` as pinned, or require the bootstrap
   task to re-read the authoritative manifest if implementation begins materially
   later?

## 15. Decision Vehicle and Next Step

This RFC **proposes**; it does not decide. The final architecture decision is
recorded by:

> **ADR-0005 — Slice-1 Implementation Language and Toolchain**

ADR-0005 does not exist and is not created by this RFC. **No code may begin solely
because this RFC has been drafted**, and none may begin before ADR-0005 is
accepted and an implementation-bootstrap authorization is issued.

ADR-0005 must not amend the First Runnable Slice Contract, which is **ACCEPTED and
immutable** — the ADR realizes that contract, it does not modify it.

---

> **Status:** PROPOSED — PRODUCT BOARD REVIEW REQUIRED. This document is not
> normative until the Product Board accepts it, and even then the architecture
> decision is recorded by ADR-0005, not here.
