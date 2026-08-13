# ADR-0005: Slice-1 Implementation Language and Toolchain

- **Status:** ACCEPTED
- **Date:** 2026-08-13
- **Accepted by:** Product Board, PBR-0013
- **Decision date:** 2026-08-13
- **Deciders:** Principal Engineer; accepted by Product Board
- **Related:** RFC-0002 (ACCEPTED via PBR-0012), First Runnable Slice Contract
  (ACCEPTED via PBR-0011), ADR-0002 (ACCEPTED, immutable), ADR-0003 (ACCEPTED,
  immutable), ADR-0004 (ACCEPTED, immutable), RFC-0001 (ACCEPTED), ER-0039
  (analysis), ARR-0011 (RFC-0002 review), PBA-0035 (authoring), PBA-0039
  (acceptance)
- **Review lineage:** RFC-0002 → ARR-0011 → RFC-0002 acceptance → ADR-0005 →
  ARR-0012 → ER-0043 → ARR-0013 → PBR-0013
- **Governed by:** Product Constitution v1.0 — ACTIVE — RATIFIED

> This ADR decides **how Slice 1 is built**, not what it means. The **First
> Runnable Slice Contract remains authoritative for Slice-1 behaviour**; this ADR
> realizes it and neither amends nor reinterprets it. It decides **for Slice 1
> only**. It selects **no** framework, UI, persistence, database, serialization
> format, parser, schema language, deployment target, provider, broker or
> market-data integration, evaluator implementation, hash algorithm, CI/CD
> platform, or release packaging model, and it **does not resolve Q-06**.

---

## 1. Context

The First Runnable Slice Contract (ACCEPTED) fixes Slice-1 behaviour completely:
the admitted construct inventory, the Canonical Semantic Contract for those
constructs, the direct-canonical-input boundary, the 15 applicable mandatory
checks, the per-unit-of-work prerequisite model, and the result contract —
verdict, defect identity, and a total canonical defect ordering. Its §8
obligation 8 makes **cross-implementation agreement** the acceptance test for the
slice.

What remains undecided is the implementation stack. RFC-0002 (ACCEPTED, PBR-0012)
analysed four candidates against requirements derived from the contract and
proposed Rust. ARR-0011 reviewed that proposal, returned PASS, and carried three
items forward as binding inputs to this ADR. This ADR records the decision.

Nothing here is chosen for general language merit. The contract is a specification
of **exhaustive closed categories over exactly-represented numbers with no
implicit coercion**, and the decision is driven by which stack makes violating that
hardest.

## 2. Decision Drivers

Derived from the accepted contract, cited so the decision is traceable rather than
preferential:

- **Exact decimal rational arithmetic; no binary floating point, no rounding, no
  precision loss** (Slice Contract §4.2).
- **No magnitude bound, no overflow, no underflow** — arbitrary precision (§4.2).
- **No implicit coercion, widening, or promotion; no null, absent, `NaN`, or
  infinite value** (§4.1).
- **Exhaustive closed categories** — 26 defect categories, 6 discriminator kinds,
  6 construct kinds, 3 evidence types, 6 operators, 6 expression forms (§3,
  §6.4.1, §6.4.2).
- **Contract-supplied deterministic total orders** over five opaque domains
  (§3.3.1).
- **No hash-based, locale, host-language, filesystem, source-position,
  serialization-specific, or generated ordering** (§3.3.1).
- **Total, pure, deterministic** — no I/O, clock, randomness, ambient state, side
  effects, or input mutation (§1.1).
- **Cross-implementation identical results** (§8 obligation 8).

Slice 1 has no UI, persistence, networking, broker, market-data, deployment,
serialization, parser, authoring flow, database, or external service. The decision
is therefore constrained by little except correctness.

## 3. Decision

**Slice 1 is implemented in Rust, on a repository-pinned official stable toolchain
at Edition 2024, verified by `cargo test` together with `cargo clippy`, with
`num-bigint` and `num-rational` as the only production dependencies and `proptest`
permitted as an optional dev-only dependency.**

The decision is adopted because Rust converts the largest share of the ways to fail
obligation 8 into **build failures rather than runtime divergence**: closed
categories are exhaustively matched or the build breaks; the language performs no
implicit numeric coercion; ownership makes mutation of the input a compile error;
and newtypes make the five opaque domains mutually unforgeable.

## 4. Language Scope

**This decision applies to Slice 1 only.**

It does **not** establish Rust as the product-wide implementation language, the
future evaluator language, the authoring-system language, the UI language, or the
mandatory language of any future Ordina component. Every future component decides
on its own merits through its own RFC and ADR.

## 5. Toolchain and Version Policy

| Item | Decision |
|---|---|
| Channel | Official **stable**. No nightly, no beta, no unstable feature gates. |
| Version | **Exact pin: `1.97.1`** |
| Edition | **2024** |
| Pin mechanism | Root **`rust-toolchain.toml`** naming the exact version and the **`rustfmt`** and **`clippy`** components |
| Floating channel | **Not permitted** for the authoritative Slice-1 baseline |
| `Cargo.lock` | **Committed** |
| MSRV | Equal to the pin; raised only deliberately |

**Basis for the pin.** `1.97.1` is the version RFC-0002 §4.1 recorded from the
authoritative Rust stable release channel manifest at drafting time. A floating
`stable` is excluded because it changes the compiler beneath a contract whose whole
purpose is reproducibility, which would undermine obligation 8. Committing
`Cargo.lock` departs from the usual library convention deliberately: Slice 1 is a
conformance artifact, not a published crate, and pinning the exact dependency graph
is the point — reinforced by both numeric crates being pre-1.0, where semver
guarantees are weaker.

**No file is created by this ADR.** `rust-toolchain.toml` and `Cargo.lock` are
produced by a later, separately authorized implementation bootstrap.

## 6. Numeric Representation and Dependency Policy

### 6.1 Representation

Authoritative Slice-1 numeric values are **arbitrary-precision exact rationals**,
realized as `BigRational` over `BigInt`. This satisfies the contract's exactness
and unbounded-magnitude requirements directly: no rounding, no overflow, no
underflow, no implementation-defined numeric behaviour. Because Slice 1 admits only
`+`, `−`, and `×` (Slice Contract §3.8 excludes division), arithmetic is closed
over exact rationals with no representability question.

### 6.2 Approved production dependencies

| Crate | Purpose | Licence |
|---|---|---|
| `num-bigint` | Arbitrary-precision integers | MIT **OR** Apache-2.0 |
| `num-rational` | Exact rational arithmetic over `BigInt` | MIT **OR** Apache-2.0 |

**These two are the only production dependencies.** No framework. No serialization
library. No solver, prover, or constraint library — Slice Contract §4.10 is
explicit that the required operations are ordinary comparisons between declared
literals.

**Required transitive dependencies:** `num-integer` and `num-traits`, from the same
ecosystem under the same dual licence. `num-rational` additionally depends on
`num-bigint`, which this decision approves directly.

**Concrete crate versions are not fixed by this ADR.** They are resolved at
implementation bootstrap and pinned in the committed `Cargo.lock`.

### 6.3 Dependency isolation

Authoritative numeric representation is **encapsulated behind the Slice-1 numeric
module**. Checker logic — construct modelling, the constraint-domain algebra, the
mandatory checks, defect identity and ordering — operates on an internal value type
and does not depend directly on third-party numeric APIs beyond that boundary.

This keeps the decision reversible: replacing the numeric crates, including with a
zero-dependency in-house implementation, touches one module and no check logic.

### 6.4 Prohibited dependency features

The following optional features of the approved numeric dependencies **must remain
disabled** unless a later separately-authorized architecture decision permits them:

**`serde` · `rand` · `rand_core` · `quickcheck` · `arbitrary`**

Reasons, each traceable to accepted authority:

- **`serde`** is a serialization framework. **Serialization format remains
  deferred** (ADR-0002 §15; Slice Contract §7.4). It must not enter Slice 1
  through an optional feature.
- **`rand` / `rand_core`** introduce randomness, which Slice Contract §1.1 and
  §3.2 prohibit on the authoritative path.
- **`quickcheck` / `arbitrary`** are test-generation facilities whose presence
  would broaden the dependency surface without contract need.

**Current upstream default-feature behaviour is not the architectural control.**
None of these features is enabled by default today, but the prohibition rests on
this decision, not on upstream defaults remaining unchanged. The implementation
must state the feature set explicitly rather than inherit it.

The three dependency layers are kept distinct: **(1)** direct dependencies approved
above; **(2)** required transitive dependencies named above; **(3)** optional
features explicitly prohibited here.

## 7. Determinism Constraints

Binding implementation constraints. They **realize** the accepted contract and do
not alter it; where this ADR and the Slice Contract could ever be read as
differing, **the Slice Contract governs**.

### 7.1 Exact numeric semantics

Authoritative Slice-1 semantic logic must not use `f32`, `f64`, floating-point
rounding, implicit tolerance, epsilon comparison, or lossy numeric conversion.
Comparison is exact.

### 7.2 Float exclusion — structural control, with lint as defense in depth

**The primary control is structural type design:**

> **No authoritative Slice-1 semantic type or field may be `f32` or `f64`;
> floating-point values are unreachable by construction on the authoritative
> checker path.**

Because the authoritative numeric representation is fixed by §6.1 and the accepted
contract's admitted constructs contain no floating-point form, a float has no
type to inhabit on the authoritative path. Exclusion is therefore a property of the
type design, not of a check that must remember to run.

Lint enforcement is **defense in depth on top of that**, and its limits are stated
precisely because overstating them would be misleading:

- **Rust's type system does not itself forbid floating point.** No language
  mechanism removes `f32`/`f64` from availability — which is exactly why the
  structural rule above, not the language, is the primary control.
- **`clippy::float_arithmetic` provides lint enforcement**, denied at crate level.
  **`clippy::float_cmp` may additionally be denied**; if adopted it is likewise
  defense in depth and **does not replace** the structural prohibition.
- Lint coverage is **partial** — `float_arithmetic` catches arithmetic, not a
  float-typed field that is merely stored or compared, nor a cast. This is why it
  is a second layer rather than the first.
- **Lint enforcement requires `cargo clippy` to actually run.**
- The defined Slice-1 verification routine therefore **includes `cargo clippy`**.
- **`cargo test` alone does not establish compliance** with the float policy.

This introduces no new semantic rule: §7.1's prohibition and §6.1's representation
are unchanged, and the accepted Slice Contract remains authoritative for numeric
semantics.

### 7.3 `BigRational` domain boundary

`BigRational` is an **implementation representation whose mathematical domain is
wider than the accepted Slice Contract's admissible literal domain**. The contract
admits exact *decimal* rationals; `BigRational` can represent all rationals.

**Representation capability does not expand semantic admissibility.** Its use must
not make arbitrary non-decimal rational literals admissible. The implementation
accepts only values the accepted Slice Contract admits, and the Slice Contract
remains authoritative on the point. Because Slice 1 admits only `+`, `−`, and `×`,
results over admitted inputs remain within the admitted domain and no rounding ever
occurs, so the wider representation introduces no divergence between conforming
implementations.

### 7.4 Exhaustive closed types

The contract's finite categories are represented as **closed Rust enums** or
equivalent statically closed representations. Authoritative matching over such
categories is **exhaustive**.

> **Catch-all `_` arms are prohibited unconditionally when matching one of the
> contract's closed authoritative categories.**

The purpose is that **adding a new authoritative category must cause every
incomplete authoritative match to fail visibly at build time**, rather than be
silently absorbed by a wildcard. The prohibition is unconditional for those
categories precisely so that it requires no judgement call about whether a given
wildcard "might" conceal something.

This rule is **scoped to authoritative matching over the contract's closed
categories**. It does not prohibit `_` in unrelated non-authoritative Rust code,
where a wildcard carries no risk of absorbing a semantic category.

### 7.5 Opaque domains

The contract's five opaque semantic domains — lineage identities, dimension
symbols, `ENUM` categorical values, version tokens, precedence ordinals — are
represented as **distinct strongly-typed newtypes**, so values from unrelated
domains cannot be compared or substituted accidentally.

The implementation uses the **contract-supplied total orders** (§3.3.1) and must
**not** derive authoritative semantic ordering from incidental Rust representation
— not from declaration order, and not from any host-language convention.

**`#[derive(Ord)]` on an opaque-domain newtype is the paradigm case of what is
prohibited.** Deriving `Ord` yields the lexicographic or structural order of
whatever the newtype happens to wrap — a byte string, an integer encoding — which
is an **incidental representation order**, not the order the contract supplies. It
must never define the authoritative order of an opaque semantic domain.

`Ord` is **not** prohibited as an implementation trait. A hand-written `Ord`
implementation that faithfully delegates to, or represents, the contract-supplied
order is permitted and expected — that is how the supplied order is made usable by
ordinary Rust containers and sorts. **The prohibition concerns deriving semantic
authority from incidental representation**, not the use of the trait itself. The
authoritative order continues to come **exclusively** from the total order supplied
with the accepted Slice Contract input, and never from anything the implementation
chooses.

### 7.6 Collection determinism

`HashMap` and `HashSet` iteration order must not influence authoritative verdicts,
defect-set membership, defect identity, defect ordering, prerequisite-gating
results, not-evaluated metadata, or any other conformance-visible result. Use
explicitly deterministic ordered structures, or sort explicitly under the
contract's declared orders, before an authoritative result is produced.

### 7.7 Purity

The checker is a **total, pure, deterministic** computation over the accepted
in-memory input boundary. No authoritative result may depend on clock, randomness,
filesystem state, network, process-global mutable state, locale, machine-specific
ordering, or any other ambient state.

**No `unsafe` Rust is authorized or required for Slice 1.**

## 8. Test and Verification Policy

**Mandatory harness:** `cargo test`.
**Mandatory additional step:** `cargo clippy` (§7.2).

`cargo test` alone is **not** the complete Slice-1 verification routine.

**Conformance direction.** Verification must eventually establish that the Rust
implementation reproduces the authoritative **WELL-FORMED / UNUSABLE verdict**, the
**complete defect set**, **defect identities**, the **canonical defect ordering**,
**prerequisite-gating behaviour**, and **not-evaluated metadata** — preserving
Slice Contract §8 obligation 8.

**`proptest` is permitted as an optional dev-only dependency** (MIT OR Apache-2.0),
restricted to properties **already derivable from the accepted Slice Contract** —
for example overlap symmetry, subsumption reflexivity, valid transitivity,
contradictory guards overlapping nothing, and subsumption implying overlap for
non-contradictory guards. It must never become a production dependency.

**Tests are subordinate to the accepted contract.** A test, a property, an
implementation convenience, a Rust idiom, a crate behaviour, or a compiler
behaviour **must not silently become a second semantic specification**. Where
something desirable is not derivable from the contract, it is an open question for
the Product Board, not a test.

### 8.1 Randomness boundary — `proptest` versus the §6.4 prohibition

`proptest` generates test cases using randomness, and its own dependency graph
therefore includes randomness crates. That is **not** in tension with §6.4's
prohibition, and the boundary is stated here so no later reader has to infer it:

1. **Randomness is prohibited from the authoritative checker path** (§7.7; Slice
   Contract §1.1, §3.2).
2. **Production dependencies must not introduce or rely on randomness.** §6.4's
   prohibition on the `rand` and `rand_core` features of `num-bigint` and
   `num-rational` is unchanged.
3. **`proptest` may use randomness solely inside dev/test execution** — to choose
   which inputs to try.
4. **Such dev/test randomness must not participate in, seed, influence, or become
   an input to an authoritative checker result.** The checker is invoked on
   concrete constructed inputs and behaves deterministically for each; the
   generator selects *which* inputs are exercised, never *what the checker
   computes*.
5. **`proptest` remains dev-only and must never become a production dependency.**
6. **Adopting `proptest` authorizes randomness nowhere else** — not in the
   authoritative path, not in production dependencies, not in any feature enabled
   on them.
7. **Optional `proptest` features that introduce filesystem or process facilities
   — including `tempfile` and `rusty-fork` — are not enabled for Slice 1** unless
   separately authorized, consistent with §7.7's exclusion of filesystem and
   process-global state.

The §6.4 prohibitions on enabling `serde`, `rand`, `rand_core`, `quickcheck`, and
`arbitrary` through the Slice-1 numeric production dependencies remain in force
exactly as written. This clause scopes an existing prohibition; it establishes no
general dependency or supply-chain policy.

## 9. Q-06 Severability

**Q-06 remains OPEN.**

Slice 1 has **no platform or form-factor surface** — no UI, no networking, no
deployment target, no process model. The language decision is therefore **severable
from the future platform/form-factor decision**, and this ADR neither resolves nor
prejudges it.

This ADR selects and implies **no** web, mobile, desktop, or API-first target, and
**no** native, WebAssembly, FFI, or server deployment. Such execution targets may be
noted **only** as evidence that the decision preserves optionality; **none is a
selected target**, and nothing here may be cited as having chosen one.

## 10. Team-Fluency and Implementation Consequences

The Product Board has recorded that the primary implementer is willing to learn
Rust during Slice-1 implementation, and accepts the learning curve as an
implementation cost.

This acceptance:

- **does not weaken correctness requirements**;
- **does not waive conformance** with the accepted Slice Contract;
- **does not authorize semantic shortcuts**;
- **does not authorize `unsafe`**;
- **does not change the accepted Slice Contract**.

Implementation proceeds **incrementally and test-led**. **No additional framework
or abstraction may be introduced solely to compensate for initial Rust
unfamiliarity** — the remedy for unfamiliarity is smaller increments and more
tests, not more machinery.

The workload is favourable to a ramp: Slice 1 is a pure tree-walking computation
over owned immutable data with no concurrency, no lifetimes complexity, no I/O, and
no `unsafe` — the parts of Rust with the steepest curve are absent from it.

## 11. Alternatives Considered

| Alternative | Genuinely stronger at | Why not selected |
|---|---|---|
| **TypeScript / Node.js** | Highest velocity; zero required dependencies (`BigInt` in-language); built-in test runner; idiomatic discriminated unions | Its **default numeric type is IEEE-754 double**, under a contract forbidding binary floating-point semantics; the type system that would catch it is **erased at runtime**. Its broader coercion surface runs against §4.1, which forbids null, absent, and `NaN` entirely. Mitigable only by discipline — the weakest control for the property that matters most. |
| **Python** | **Best numeric story of the four** — `fractions.Fraction` in the standard library satisfies exactness and unbounded magnitude with **zero dependencies**; high velocity | Type discipline is **opt-in**, so exhaustiveness over 26 defect categories and 6 discriminator kinds depends on an external checker being run and kept strict; `set` iteration order **varies between runs** under hash randomization, a live hazard for a component whose acceptance test is reproducible ordering. The closest call on numerics. |
| **Go** | Exact rationals in the standard library (`math/big.Rat`); excellent reproducibility; built-in test harness | The contract is **ADT-shaped** and Go has **no sum types and no exhaustiveness checking**; modelling 26 defect categories and 6 discriminator kinds through interfaces and type switches surrenders the compile-time guarantee that motivates this decision. Map iteration is deliberately randomized. |
| **Zero-production-dependency Rust numerics** | Removes both production dependencies; smallest possible supply-chain surface | Requires hand-writing arbitrary-precision arithmetic in the one module where the contract is least forgiving. Reinvents mature, well-tested code and concentrates correctness risk exactly where a defect would corrupt every downstream verdict. Remains available later at low cost because of the §6.3 isolation boundary. |

Rust's advantage here is **structural enforcement of the accepted determinism and
type invariants — not general language superiority**. On numeric ergonomics Python
and Go are better; on velocity TypeScript and Python are better.

## 12. Positive Consequences

Exhaustiveness failures become build failures. No implicit coercion exists to guard
against. Purity is enforced by ownership rather than by convention. Newtypes make
the opaque domains unforgeable and cross-domain comparison a type error. The
toolchain pins exactly and reproducibly. The dependency surface is two small,
permissively dual-licensed crates behind a single module boundary. Byte-level
determinism — which the deferred canonicalization and revision-identity work will
need — is native rather than retrofitted.

## 13. Negative Consequences and Risks

1. **Lowest developer velocity of the candidates**, and a real learning curve
   (§10).
2. **Two production dependencies**, where each rejected alternative needs zero.
   Mitigated by isolation (§6.3) and licence clarity, not eliminated.
3. **The float prohibition is lint-enforced, not type-enforced** (§7.2). It holds
   only while `cargo clippy` is actually run, which is why the verification routine
   names it. This is the weakest control in the decision and is recorded as such.
4. **`HashMap` remains available in the language** and would silently reintroduce
   nondeterminism if used on an authoritative path. Addressed by explicit
   prohibition (§7.6), not by hope.
5. **Both numeric crates are pre-1.0**, so upstream semver guarantees are weaker.
   Addressed by the committed `Cargo.lock`.
6. **Optional upstream features could expand scope silently** if defaults change.
   Addressed by the explicit prohibition in §6.4 rather than by inheriting
   defaults.

## 14. Reversibility

**Relatively low cost.** Slice 1 is pure, I/O-free, platform-neutral, and governed
by a **language-independent accepted behavioural contract**. A later port must
reproduce the same authoritative verdicts, defect sets, defect identities, canonical
ordering, and not-evaluated metadata, because the **First Runnable Slice Contract —
not the implementation language — is authoritative for behaviour**.

Within the decision, the numeric-dependency choice is separately reversible behind
the §6.3 module boundary.

This ADR binds nothing beyond Slice 1 (§4).

## 15. Explicit Non-Decisions and Deferrals

**Not selected by this ADR:** application framework; web framework; UI framework;
UI technology; persistence; database; serialization format; parser; schema
language; deployment target; hosting or cloud provider; broker integration;
market-data provider; evaluator implementation; **evaluator-semantics version
scheme (N-24)**; hash or digest algorithm; CI/CD platform; release packaging model.

**Remaining unresolved / deferred, and untouched by this ADR:** **N-22**
(Well-Formedness-failure UX) · **N-24** (evaluator-semantics version) · **N-25**
(authoring-to-canonical translation) · **Q-06** (platform / form factor) · **Q-08**
(jurisdictions) · **F-8** (privacy / preimage on erased low-entropy content).

None is silently resolved here.

## 16. Accepted-Document Compatibility

This ADR **amends, reinterprets, and supersedes nothing**. Verified against each
governing document:

| Source | Compatibility |
|---|---|
| **Product Constitution v1.0** | C3/E1 preserved — the checker is deterministic and model-free, with no AI, score, or probability on the path. C4 preserved — defects anchor to lineage identity. C5 preserved — results reproducible, input unmutated. |
| **RFC-0001** | §12 detect-report-never-repair preserved; no implementation constraint here rewrites, defaults, coerces, or repairs input. |
| **ADR-0002** | §10 determinism prohibitions honoured; §10c decidability admission untouched; §15 deferrals (serialization, evaluator, persistence) preserved. |
| **ADR-0003** | Canonicalization and revision identity untouched and not computed by Slice 1; hash algorithm remains unselected. |
| **ADR-0004** | The Well-Formedness architecture is realized, not modified; the checker-semantics version (`WF-1`) and semantic-model version (`SM-1`) remain as the Slice Contract fixes them. |
| **First Runnable Slice Contract** | **Authoritative for behaviour.** This ADR realizes it. Where the two could be read as differing, the contract governs. |
| **RFC-0002** | This ADR records the decision RFC-0002 proposed, with the three ARR-0011 inputs incorporated in §6.4, §7.2, and §7.3. |
| **Canonical registers** | Unchanged. No open question resolved; no risk retired. |

**No conflict with any higher-authority accepted document was found.** Had one been
found, this ADR would not have been written — the conflict would have been reported
instead.

## 17. Implementation Handoff

**This ADR authorizes no implementation.** Acceptance of ADR-0005 is necessary but
not sufficient; project bootstrap requires a **separate implementation
authorization**.

When that authorization is issued, the bootstrap should establish, in this order:
`rust-toolchain.toml` at the pinned version with `rustfmt` and `clippy`; a
`Cargo.toml` declaring the two production dependencies with the §6.4 features
disabled explicitly; the committed `Cargo.lock`; the module layout of RFC-0002 §8;
and the conformance suite — with **fixtures constructed in memory**, never as
serialized data files, since a fixture format would implicitly select the deferred
serialization format and introduce a parser into a slice defined as having none.

## 18. Review / Acceptance Record

**ADR-0005 is ACCEPTED by the Product Board via PBR-0013**, decision date
**2026-08-13**.

**Review lineage:** RFC-0002 → ARR-0011 → RFC-0002 acceptance (PBR-0012) →
ADR-0005 → ARR-0012 → ER-0043 → ARR-0013 → PBR-0013.

**ARR-0012** (independent architecture review) returned PASS with two MINOR
findings and two observations. **ER-0043** closed all four under PBA-0037: the
`proptest`/randomness boundary (§8.1), structural float exclusion (§7.2), the
unconditional catch-all prohibition (§7.4), and the `#[derive(Ord)]` clarification
(§7.5). **ARR-0013** (final remediation verification) returned
**`PASS — READY FOR PRODUCT BOARD ACCEPTANCE`** with no BLOCKER, MAJOR, or MINOR
finding.

Acceptance records the decision only and changed no substantive reviewed text.

1. **The accepted substantive decision is now an immutable historical
   architecture record.** §§1–17 as reviewed are settled.
2. **A future material architecture change requires a separately authorized
   successor or superseding ADR — never a silent rewrite.** ADRs are immutable
   once accepted; supersede, never edit.
3. **Acceptance does not amend the accepted First Runnable Slice Contract**, nor
   any other accepted document. This ADR supersedes nothing.
4. **The First Runnable Slice Contract remains authoritative for Slice-1
   behaviour.** Where this ADR and the contract could ever be read as differing,
   the contract governs (§7, §16).
5. **Acceptance authorizes no implementation by itself.** Project bootstrap —
   `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `src/`, `tests/`, dependency
   installation, or any Rust source — requires a **separate implementation
   authorization** (§17).

**Preserved boundaries.** Rust is selected **for Slice 1 only** (§4) and is not the
implementation language of the Ordina product. **Q-06 remains OPEN**, and **N-22,
N-24, N-25, Q-08, and F-8 remain unresolved/deferred** (§9, §15). No framework, UI
technology, persistence, database, serialization format, parser, schema language,
deployment target, provider, broker or market-data integration, evaluator
implementation, evaluator-semantics version scheme, hash algorithm, CI/CD platform,
or release packaging model is selected by this acceptance.

---

> This ADR is now **ACCEPTED**. Its decision text is an immutable historical
> architecture record. ADRs are immutable once accepted — **supersede, never
> edit**. Any future material architecture change must be recorded through a new
> ADR that explicitly supersedes this one.
