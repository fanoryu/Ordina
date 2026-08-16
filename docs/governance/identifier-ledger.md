# Governance Identifier Ledger

- **Tier:** governance metadata (non-normative as to product or architecture)
- **Status:** ACTIVE
- **Authority:** Product Board, **PBD-0005** — Governance Identifier Ledger
  Establishment and Collision Resolution
- **Established:** 2026-08-13

> **This ledger allocates identifiers. It decides nothing else.**
> It **must not** reinterpret any accepted RFC, ADR, contract, open question, or
> risk, and nothing recorded here changes the meaning, status, or authority of any
> artifact it names. Where this ledger and an accepted document disagree about what
> an artifact *says*, the accepted document governs; this ledger is authoritative
> only for **which number belongs to which artifact**.

---

## 1. Purpose and Authority

No authoritative allocation record was maintained before PBD-0005. Repository
evidence plus the preserved workstream chronology therefore constitute the
historical evidence set, and this ledger is its canonical consolidation.

From this determination forward, the ledger is the **authoritative allocator** for
`PBA`, `PBR`, `ARR`, `ER`, and `PBD` identifiers, and may record other governance
families where useful. No identifier in these namespaces may be issued without
being recorded here first.

## 2. Namespace Rules

| Namespace | Meaning | Issued by |
|---|---|---|
| `PBA` | Product Board Authorization — authorizes a piece of work | Product Board |
| `PBR` | Product Board Review / acceptance record | Product Board |
| `PBD` | Product Board Determination — a governance decision | Product Board |
| `ARR` | Architecture / contract review report | Architecture Reviewer |
| `ER` | Execution report — the record of authorized work performed | Principal Engineer |

Other families appear in the repository as historical artifacts and are recorded in
§9 for reference: `POR` (Product Owner Ratification), `ARW` (architecture working
note), `WN` (workshop note), `WR` (working record), `SCW` (System Context
workshop).

## 3. Immutable Historical-Assignment Rule

A historical assignment is **preserved** and is **never** renumbered merely because
its numerical sequence is non-monotonic.

An assignment may be corrected **only** where:

- **(a)** two distinct artifacts are **affirmatively known** to share one
  identifier; or
- **(b)** an existing repository reference is **affirmatively known** to identify
  the wrong artifact.

**Repository silence is not evidence of a collision.** An identifier absent from
every repository file is not thereby proven free, proven occupied, or proven to be
in conflict.

## 4. No-Gap-Filling Rule

Numbers below a namespace's high-water mark that are absent from the evidence set
**must not** be reused. Every identifier carries exactly one allocation state:

| State | Meaning | May be newly assigned? |
|---|---|---|
| **ALLOCATED** | Bound to a known artifact | No |
| **RESERVED** | Deliberately withheld (e.g. known historical collision) | No |
| **SUPERSEDED-ID** | The artifact's *identifier* was corrected; the artifact itself stands | No |
| **UNKNOWN-HISTORICAL** | Below the high-water mark, disposition unrecoverable | No |
| **AVAILABLE-FUTURE** | Above the high-water mark and unissued | **Yes** |

**Only AVAILABLE-FUTURE identifiers may be newly assigned.**

## 5. PBA — Product Board Authorizations

| Identifier | State | Assigned to |
|---|---|---|
| PBA-0001 | ALLOCATED | Product Model reconciliation (executes WR-0003) — Constitution v0.3 |
| PBA-0002 | ALLOCATED | Constitution revision cycle |
| PBA-0003 | UNKNOWN-HISTORICAL | — |
| PBA-0004 | ALLOCATED | Constitution revision cycle |
| PBA-0005 | ALLOCATED | Constitution revision cycle |
| PBA-0006 | ALLOCATED | Constitution revision cycle |
| PBA-0007 | UNKNOWN-HISTORICAL | — |
| PBA-0008 | UNKNOWN-HISTORICAL | — |
| PBA-0009 | ALLOCATED | RFC-0001 authorization (dispositions of N-19, N-20, N-21; Q-16, Q-11b in scope) |
| PBA-0010 | UNKNOWN-HISTORICAL | — |
| PBA-0011 | UNKNOWN-HISTORICAL | — |
| PBA-0012 | ALLOCATED | ADR-0002 authorization |
| PBA-0013 | UNKNOWN-HISTORICAL | — |
| PBA-0014 | ALLOCATED | ADR-0004 canonical register alignment authorization → ER-0027 |
| PBA-0015 | ALLOCATED | First Runnable Slice Contract authoring authorization → ER-0028 |
| **PBA-0016** | **ALLOCATED** | **ADR-0003 authorization** (priority assignment; PBD-0005 §2) |
| PBA-0017 | ALLOCATED | Slice Contract independent re-review authorization → ARR-0008 |
| PBA-0018 | ALLOCATED | Slice Contract second bounded remediation authorization → ER-0030 |
| PBA-0019 | ALLOCATED | Slice Contract final closure review authorization → ARR-0009 |
| PBA-0020 | ALLOCATED | Slice Contract final two-item remediation authorization → ER-0031 |
| PBA-0021 | UNKNOWN-HISTORICAL | Must not be reused (PBD-0005 §3) |
| PBA-0022 | UNKNOWN-HISTORICAL | Must not be reused (PBD-0005 §3) |
| PBA-0023 | ALLOCATED | ADR-0004 authorization (W-1…W-5 rulings) |
| **PBA-0024** | **ALLOCATED** | **Bounded First Runnable Slice Contract remediation authorization → ER-0029** (corrected identifier; PBD-0005 §2) |
| **PBA-0025** | **ALLOCATED** | **Pre-Git Governance Integrity Final Independent Review authorization** → ARR-0010 (2026-08-13) |
| **PBA-0026** | **ALLOCATED** | **Identifier Ledger Post-Review Synchronization authorization** → ER-0034 (2026-08-13) |
| **PBA-0027** | **ALLOCATED** | **Identifier Ledger Baseline Freeze Synchronization authorization** → ER-0035 (2026-08-13). Baseline-freeze administrative allocation — see §11.1. |
| **PBA-0028** | **ALLOCATED** | **Git Baseline & Private GitHub Repository Authorization Readiness** → ER-0036 (2026-08-13) |
| **PBA-0029** | **ALLOCATED** | **Git Baseline & Private GitHub Repository Execution Authorization** → ER-0037 (2026-08-13) |
| **PBA-0030** | **ALLOCATED** | **Post-Baseline Repository Hygiene authorization** → ER-0038 (2026-08-13) |
| **PBA-0031** | **ALLOCATED** | **Slice-1 Implementation Stack Decision authorization** → ER-0039 (2026-08-13) |
| **PBA-0032** | **ALLOCATED** | **RFC-0002 Slice-1 Implementation Stack Authoring authorization** → ER-0040 (2026-08-13) |
| **PBA-0033** | **ALLOCATED** | **RFC-0002 Product Board Review authorization** → ARR-0011 (2026-08-13) |
| **PBA-0034** | **ALLOCATED** | **RFC-0002 Acceptance authorization** → ER-0041 (2026-08-13) |
| **PBA-0035** | **ALLOCATED** | **ADR-0005 Authoring authorization** → ER-0042 (2026-08-13) |
| **PBA-0036** | **ALLOCATED** | **ADR-0005 Independent Architecture Review authorization** → ARR-0012 (2026-08-13) |
| **PBA-0037** | **ALLOCATED** | **ADR-0005 Final Bounded Remediation authorization** → ER-0043 (2026-08-13) |
| **PBA-0038** | **ALLOCATED** | **ADR-0005 Final Remediation Verification authorization** → ARR-0013 (2026-08-13) |
| **PBA-0039** | **ALLOCATED** | **ADR-0005 Acceptance authorization** → ER-0044 (2026-08-13) |
| **PBA-0040** | **ALLOCATED** | **Slice-1 Rust Bootstrap and Implementation-Plan authorization** → ER-0045 (2026-08-13) |
| **PBA-0041** | **ALLOCATED** | **Windows MSVC Prerequisite Remediation and Slice-1 Bootstrap Retry authorization** → ER-0046 (2026-08-13) |
| **PBA-0042** | **ALLOCATED** | **Slice-1 Implementation Phase 1 authorization** (core model and opaque domains) → ER-0047 (2026-08-13) |
| **PBA-0043** | **ALLOCATED** | **Slice-1 Phase 1 Independent Architecture Review authorization** → ARR-0014 (2026-08-13) |
| **PBA-0044** | **ALLOCATED** | **Phase-1 Representability Remediation authorization** → ER-0049 (2026-08-13) |
| **PBA-0045** | **ALLOCATED** | **Phase-1 Representability Remediation Independent Re-Review authorization** → ARR-0015 (2026-08-13) |
| **PBA-0046** | **ALLOCATED** | **Phase-1 Final Bounded Helper Remediation authorization** → ER-0050 (2026-08-13) |
| **PBA-0047** | **ALLOCATED** | **Phase-1 Final Closure Verification authorization** → ARR-0016 (2026-08-13) |
| **PBA-0048** | **ALLOCATED** | **Phase-1 Formal Closure authorization** → ER-0051 (2026-08-13) |
| **PBA-0049** | **ALLOCATED** | **Phase-2 Scope and Implementation-Plan Review authorization** → ER-0052 (2026-08-13) |
| **PBA-0050** | **ALLOCATED** | **Slice-1 Phase-2A Exact Numeric Value Layer Implementation authorization** → ER-0053 (2026-08-13) |
| **PBA-0051** | **ALLOCATED** | **Slice-1 Phase-2A Independent Implementation Review authorization** → ARR-0017 (2026-08-14) |
| **PBA-0052** | **ALLOCATED** | **Slice-1 Phase-2A M-1 Structural Regression-Evidence Remediation authorization** → ER-0054 (2026-08-14) |
| **PBA-0053** | **ALLOCATED** | **Slice-1 Phase-2A Final Closure Verification authorization** → ARR-0018 (2026-08-14) |
| **PBA-0054** | **ALLOCATED** | **Slice-1 Phase-2A Formal Closure Recording authorization** → ER-0055 (2026-08-14) |
| **PBA-0055** | **ALLOCATED** | **Slice-1 Phase-2B Gating Decisions authorization** → ER-0056 (2026-08-14) |
| **PBA-0056** | **ALLOCATED** | **Slice-1 Distinguished Opaque-Domain Member Binding Decision authorization** → ER-0057 (2026-08-14) |
| **PBA-0057** | **ALLOCATED** | **Canonical Distinguished-Member Binding Normative Revision authorization** → ER-0058 (2026-08-14) |
| **PBA-0058** | **ALLOCATED** | **Normative Engineering Contract Revision Convention Determination authorization** → PBD-0006 / ER-0059 (2026-08-14) |
| **PBA-0059** | **ALLOCATED** | **First Runnable Slice Contract Revision 2 Authoring authorization** → ER-0060 (2026-08-14) |
| **PBA-0060** | **ALLOCATED** | **First Runnable Slice Contract Revision 2 Independent Review authorization** → ARR-0019 (2026-08-14) |
| **PBA-0061** | **ALLOCATED** | **First Runnable Slice Contract Revision 2 Final Bounded Remediation authorization** → ER-0061 (2026-08-14) |
| **PBA-0062** | **ALLOCATED** | **First Runnable Slice Contract Revision 2 Final Re-Review authorization** → ARR-0020 (2026-08-14) |
| **PBA-0063** | **ALLOCATED** | **First Runnable Slice Contract Revision 2 Product Board Acceptance authorization** → PBR-0014 / ER-0062 (2026-08-14) |
| **PBA-0064** | **ALLOCATED** | **Slice-1 Phase-2B Dimension-Aware Semantics Implementation authorization** → ER-0063 (2026-08-15) |
| **PBA-0065** | **ALLOCATED** | **Slice-1 Phase-2B Independent Implementation Review authorization** → ARR-0021 (2026-08-15) |
| **PBA-0066** | **ALLOCATED** | **Slice-1 Phase-2B Formal Closure authorization** → ER-0064 (2026-08-15) |
| **PBA-0067** | **ALLOCATED** | **Slice-1 Phase-3 Constraint-Domain Semantics Implementation authorization** → ER-0065 (2026-08-15) |
| **PBA-0068** | **ALLOCATED** | **Slice-1 Phase-3 Independent Implementation Review authorization** → ARR-0022 (2026-08-15) |
| **PBA-0069** | **ALLOCATED** | **Slice-1 Phase-3 Numeric Containment Bounded Remediation authorization** → ER-0066 (2026-08-15) |
| **PBA-0070** | **ALLOCATED** | **Slice-1 Phase-3 M-1 Independent Remediation Re-Review authorization** → ARR-0023 (2026-08-15) |
| **PBA-0071** | **ALLOCATED** | **Repository Attribution Trailer Removal and Bounded History Remediation authorization** → ER-0067 (2026-08-15) |
| **PBA-0072** | **ALLOCATED** | **Slice-1 Phase-3 Formal Closure authorization** → ER-0068 (2026-08-15) |
| **PBA-0073** | **ALLOCATED** | **Slice-1 Post-Phase-3 Implementation Plan Review authorization** → ER-0069 (2026-08-15) |
| **PBA-0074** | **ALLOCATED** | **Slice-1 Phase-4 Defect / Result Model and Canonical Ordering Implementation authorization** → ER-0070 (2026-08-15) |
| **PBA-0075** | **ALLOCATED** | **Slice-1 Phase-4 Independent Architecture and Implementation Review authorization** → ARR-0024 (2026-08-15) |
| **PBA-0076** | **ALLOCATED** | **Slice-1 Phase-4 MINOR-1 Bounded Remediation authorization** → ER-0071 (2026-08-15) |
| **PBA-0077** | **ALLOCATED** | **Slice-1 Phase-4 MINOR-1 Independent Re-Review authorization** → ARR-0025 (2026-08-15) |
| **PBA-0078** | **ALLOCATED** | **Slice-1 Phase-4 Formal Closure authorization** → ER-0072 (2026-08-15) |
| **PBA-0079** | **ALLOCATED** | **Slice-1 Phase-5 Implementation Planning Review authorization** → ER-0073 (2026-08-15) |
| **PBA-0080** | **ALLOCATED** | **Slice-1 Phase-5A Check-1 Implementation authorization** → ER-0074 (2026-08-15; STOPPED before write — clarification required) |
| **PBA-0081** | **ALLOCATED** | **Slice-1 Check-1 Defect-Population Clarification Review authorization** → ER-0075 (2026-08-15) |
| **PBA-0082** | **ALLOCATED** | **Slice Contract Revision 3 Check-1 Ownership Clarification Authoring authorization** → ER-0076 (2026-08-15) |
| **PBA-0083** | **ALLOCATED** | **Slice Contract Revision 3 Independent Review authorization** → ARR-0026 (2026-08-15) |
| **PBA-0084** | **ALLOCATED** | **Slice Contract Revision 3 M-1 Bounded Remediation authorization** → ER-0077 (2026-08-15) |
| **PBA-0085** | **ALLOCATED** | **Slice Contract Revision 3 M-1 Independent Re-Review authorization** → ARR-0027 (2026-08-15) |
| **PBA-0086** | **ALLOCATED** | **Slice Contract Revision 3 Acceptance authorization** → PBR-0015 / ER-0078 (2026-08-15) |
| **PBA-0087** | **ALLOCATED** | **Revision 3 Post-Acceptance Model-Comment Currency Correction authorization** → ER-0079 (2026-08-15) |
| **PBA-0088** | **ALLOCATED** | **Slice-1 Governance Recovery and Canonical Construct-Occurrence Identity Determination authorization** → ER-0080 (2026-08-15; read-only; restarted the determination from the live allocator after the assumed PBA-0088–0092 / ER-0080–0084 chain was found unrecorded) |
| **PBA-0089** | **ALLOCATED** | **Slice Contract Revision 4 Canonical Construct-Occurrence Identity Authoring authorization** → ER-0081 (2026-08-15; **STOPPED before write** — class-level `DUPLICATE_LINEAGE_IDENTITY` and COUNTERPART absent-lineage anchoring blockers; no contract authored) |
| **PBA-0090** | **ALLOCATED** | **Slice-1 Class-Level and Counterpart Defect Identity Determination authorization** → ER-0082 (2026-08-15; read-only; integrated design determined — REVISION 4 SUFFICIENT) |
| **PBA-0091** | **ALLOCATED** | **Slice Contract Revision 4 Canonical Occurrence and Integrated Defect Identity Authoring authorization** → ER-0083 (2026-08-15; authors `first-runnable-slice-contract-r04.md` as PROPOSED and synchronizes PBA-0088–0091 / ER-0080–0083) |
| **PBA-0092** | **ALLOCATED** | **Slice Contract Revision 4 Independent Architecture and Contract Review authorization** → ARR-0028 (2026-08-16; read-only; verdict FAIL — BOUNDED REMEDIATION REQUIRED, one MAJOR M-1 (check-11 pairwise anchor)) |
| **PBA-0093** | **ALLOCATED** | **Slice Contract Revision 4 Check-11 Pairwise Anchor Bounded Remediation authorization** → ER-0084 (2026-08-16; corrects ARR-0028 M-1 in `first-runnable-slice-contract-r04.md`; Revision 4 remains PROPOSED; independent re-review required) |
| **PBA-0094** | **ALLOCATED** | **Slice Contract Revision 4 Independent Post-Remediation Re-Review authorization** → ARR-0029 (2026-08-16; read-only; verdict PASS — ARR-0028 M-1 CLOSED; ACCEPTANCE-READY) |
| **PBA-0095** | **ALLOCATED** | **Slice Contract Revision 4 Product Board Acceptance authorization** → PBR-0016 / ER-0085 (2026-08-16; accepts Revision 4, supersedes Revision 3 in governance provenance) |
| **PBA-0096** | **ALLOCATED** | **Phase-1 Canonical Occurrence Identity Post-Closure Remediation authorization** → ER-0086 (2026-08-16; implements the Revision-4 §C.1 Phase-1 consequence — `OccurrenceId` opaque domain and a mandatory occurrence field on every construct; no Phase-4/check change; Phase 1 remains CLOSED; independent reconciliation required) |
| **PBA-0097** | **ALLOCATED** | **Phase-1 Canonical Occurrence Identity Independent Reconciliation Review authorization** → ARR-0030 (2026-08-16; read-only; verdict PASS — PHASE-1 REMEDIATION RECONCILED; zero BLOCKER/MAJOR/MINOR) |
| **PBA-0098** | **ALLOCATED** | **Bounded Phase-4 Result-Model Remediation authorization** → ER-0087 (2026-08-16; migrates `DefectCore.offending` to `OccurrenceId`, COUNTERPART payload to `OccurrenceId`, adds the `LINEAGE` discriminator kind 7, per accepted Revision 4; no `src/checks/**` change; Phase 4 remains CLOSED; independent reconciliation required) |
| **PBA-0099** | **ALLOCATED** | **Phase-4 Result-Model Independent Reconciliation Review authorization** → ARR-0031 (2026-08-16; read-only; verdict PASS — PHASE-4 REMEDIATION RECONCILED; zero findings) |
| **PBA-0100** | **ALLOCATED** | **Phase-5A Check-1 Required-Property Completeness Implementation authorization** → ER-0088 (2026-08-16; implements check 1 `MISSING_REQUIRED_PROPERTY` in `src/checks/required_property.rs` on the reconciled Revision-4 model; defect production only, no other check/gating/verdict; independent review required) |
| **PBA-0101** | **ALLOCATED** | **Phase-5A Check-1 Independent Implementation Review authorization** → ARR-0032 (2026-08-16; read-only; verdict PASS — CHECK 1 IMPLEMENTATION VERIFIED; zero findings) |
| **PBA-0102** | **ALLOCATED** | **Slice-1 Phase-5 Remaining-Roadmap Reconciliation and Implementation Planning Review authorization** → ER-0089 (2026-08-16; read-only; PHASE-5 REMAINING ROADMAP RECONCILED — Slices B–H, check-8 scheduled in Slice D, O-1 gate before Slice F) |
| **PBA-0103** | **ALLOCATED** | **Phase-5 Slice B (Checks 15 and 16) Implementation authorization** → ER-0090 (2026-08-16; implements check 15 and check 16 version half in `src/checks/**`; defect production only; independent review required) |
| **PBA-0104** | **ALLOCATED** | **Phase-5 Slice-B Independent Implementation Review authorization** → ARR-0033 (2026-08-16; read-only; verdict PASS — SLICE B CHECKS 15/16 VERIFIED; SM-1 binding contract-faithful; UNKNOWN_CONSTRUCT unrepresentability confirmed; zero findings) |
| **PBA-0105** | **ALLOCATED** | **Phase-5 Slice-C (Check 2) Implementation authorization** → ER-0091 (2026-08-16; implements check 2 — duplicate lineage, reference resolution, typing, ENUM membership — in `src/checks/reference_integrity.rs`; defect production only; O-1 isolated; independent review required) |
| **PBA-0106** | **ALLOCATED** | **Phase-5 Slice-C Independent Implementation Review authorization** → ARR-0034 (2026-08-16; read-only; verdict FAIL — BOUNDED REMEDIATION / BOARD DETERMINATION REQUIRED, one MAJOR M-1 (ENUM atom overlap defect-population precedence)) |
| **PBA-0107** | **ALLOCATED** | **Check-2 ENUM Atom Overlap Defect-Population Determination authorization** → ER-0092 (2026-08-16; read-only Product Board determination; RULING A — co-occurrence; determination sufficient, no successor revision) |
| **PBA-0108** | **ALLOCATED** | **Phase-5 Slice-C M-1 Bounded Remediation authorization** → ER-0093 (2026-08-17; implements ER-0092 RULING A co-occurrence in `src/checks/reference_integrity.rs`; independent re-review required) |
| **PBA-0109** | **ALLOCATED** | **Slice-1 Phase-5 Slice-C M-1 Independent Re-Review authorization** → ARR-0035 (2026-08-17; read-only; verdict PASS — SLICE-C M-1 CLOSED; CHECK 2 VERIFIED; RULING-A co-occurrence faithfully implemented, TYPE_MISMATCH one-per-atom preserved, O-1 isolation and all ARR-0034 PASS surfaces intact; zero findings) |
| **PBA-0110** | **ALLOCATED** | **Phase-5 Slice-D (Checks 8 and 10) Implementation authorization** → ER-0094 (2026-08-17; implements check 8 `DIRECTION_COVERAGE_INCOMPLETE`/`MULTIPLE_TOTAL_DIRECTION_RULES`/`TOTAL_RULE_NOT_LOWEST_PRECEDENCE` and check 10 `INVALID_DIRECTION_OUTCOME` in `src/checks/{direction_coverage,direction_determinism}.rs`; defect production only; no other check/gating/verdict; `src/model`/`src/result` unchanged; independent review required) |
| **PBA-0111** | **ALLOCATED** | **Slice-D Commit Attribution Trailer Removal and Bounded History Remediation authorization** → ER-0095 (2026-08-17; removes the prohibited `Co-Authored-By` trailer from the Slice-D implementation commit via metadata-only `--amend`; tree identity preserved; independent review of Slice D against the clean commit required) |
| **PBA-0112** | **ALLOCATED** | **Slice-1 Phase-5 Slice-D Independent Implementation Review authorization** → ARR-0036 (2026-08-17; read-only; verdict PASS — SLICE D CHECKS 8/10 VERIFIED; populations/multiplicities/anchors/discriminators contract-faithful, check-8 co-occurrence independently justified, check-10 ownership exact, no prior-check regression, no Phase-6 leakage; two non-blocking OBSERVATIONs) |
| **PBA-0113** | **ALLOCATED** | **Phase-5 Slice-E (Checks 3, 4, 5) Implementation authorization** → ER-0096 (2026-08-17; implements check 3 `INVALID_CONDITION_CLASSIFICATION`, check 4 `INVERTED_CONFIGURATION_BOUNDS`/`CONFIGURATION_BOUND_DIMENSION_MISMATCH`, check 5 `CONFIGURATION_IN_STRUCTURAL_POSITION` in `src/checks/{condition_classification,configuration_bounds,configuration_position}.rs`; defect production only; O-3 population implemented without resolving it; no other check/gating/verdict; `src/model`/`src/result` unchanged; independent review required) |
| PBA-0114 and above | AVAILABLE-FUTURE | — |

**Collision resolved (PBD-0005 §2).** `PBA-0016` was used twice: for the ADR-0003
authorization, and later for the bounded Slice Contract remediation that produced
ER-0029. The **ADR-0003 assignment has priority and is preserved**; ADR-0003 is not
modified. The remediation authorization is reassigned **PBA-0024**. The
reassignment supersedes **only the erroneous identifier**, never the authorization
itself, its rulings, or the work performed under it.

**Non-monotonic sequence accepted.** PBA-0023 (ADR-0004) was issued before
PBA-0014…PBA-0020. PBD-0005 §3 accepts this as historical numbering irregularity
and forbids inventing collisions for those identifiers without affirmative
evidence.

**Next allocatable PBA: `PBA-0114`.**

## 6. PBR — Product Board Reviews / Acceptances

| Identifier | State | Assigned to |
|---|---|---|
| PBR-0001 | UNKNOWN-HISTORICAL | — |
| PBR-0002 | UNKNOWN-HISTORICAL | — |
| PBR-0003 | ALLOCATED | RFC-0001 acceptance (2026-08-12) |
| PBR-0004 | UNKNOWN-HISTORICAL | — |
| PBR-0005 | UNKNOWN-HISTORICAL | — |
| PBR-0006 | ALLOCATED | ADR-0002 acceptance (2026-08-12) |
| PBR-0007 | ALLOCATED | ADR-0003 acceptance (2026-08-12) |
| **PBR-0008** | **RESERVED** | **Historical collision — DO NOT REUSE** (PBD-0005 §5) |
| PBR-0009 | ALLOCATED | ADR-0004 acceptance (2026-08-13) |
| **PBR-0010** | **ALLOCATED** | **First Runnable Slice Contract Final Review Authorization → ARR-0007** (priority assignment; PBD-0005 §4) |
| **PBR-0011** | **ALLOCATED** | **First Runnable Slice Contract acceptance** (2026-08-13; corrected identifier; PBD-0005 §4) |
| **PBR-0012** | **ALLOCATED** | **RFC-0002 acceptance** (2026-08-13; authorization PBA-0034) |
| **PBR-0013** | **ALLOCATED** | **ADR-0005 acceptance** (2026-08-13; authorization PBA-0039) |
| **PBR-0014** | **ALLOCATED** | **First Runnable Slice Contract Revision 2 acceptance** (2026-08-14; authorization PBA-0063). Revision 2 becomes ACCEPTED and normative; Revision 1 SUPERSEDED in governance provenance, its file preserved immutable; closes F-3. |
| **PBR-0015** | **ALLOCATED** | **First Runnable Slice Contract Revision 3 acceptance** (2026-08-15; authorization PBA-0086). Revision 3 becomes ACCEPTED and normative; Revision 2 SUPERSEDED in governance provenance, its file preserved immutable. Records the check-1/check-15/check-16 required-property ownership clarification. |
| **PBR-0016** | **ALLOCATED** | **First Runnable Slice Contract Revision 4 acceptance** (2026-08-16; authorization PBA-0095). Revision 4 becomes ACCEPTED and normative; Revision 3 SUPERSEDED in governance provenance, its file preserved immutable. Records the canonical construct-occurrence identity and integrated defect-identity design (OccurrenceId anchor, LINEAGE discriminator kind 7, COUNTERPART occurrence payload, symmetric check-9 / asymmetric check-11 anchors). Contract/design side of O-A resolved; Phase-1/Phase-4 implementation remediation still pending. |
| PBR-0017 and above | AVAILABLE-FUTURE | — |

**Collision resolved (PBD-0005 §4).** `PBR-0010` was used twice: for the Slice
Contract Final Review Authorization that produced ARR-0007, and later for the Slice
Contract acceptance. The **Final Review Authorization has priority**; the
acceptance is reassigned **PBR-0011**. **The acceptance decision itself is
unchanged** — decision date 2026-08-13, contract status ACCEPTED, architecture and
engineering substance untouched. Only the governance identifier is corrected.

**Next allocatable PBR: `PBR-0017`.**

## 7. ARR — Architecture / Contract Review Reports

| Identifier | State | Assigned to |
|---|---|---|
| ARR-0001 | ALLOCATED | ADR-0002 initial architecture review |
| ARR-0002 | ALLOCATED | ADR-0002 final review (PASS) |
| ARR-0003 | ALLOCATED | ADR-0003 initial architecture review |
| ARR-0004 | ALLOCATED | ADR-0003 final review (PASS with minor) |
| ARR-0005 | ALLOCATED | ADR-0004 initial architecture review |
| ARR-0006 | ALLOCATED | ADR-0004 final review (PASS — READY FOR ACCEPTANCE) |
| ARR-0007 | ALLOCATED | Slice Contract final review (REMEDIATION REQUIRED — B-1/B-2/B-3, D-3) |
| ARR-0008 | ALLOCATED | Slice Contract final re-review (REMEDIATION REQUIRED — M-1…M-4, m-1…m-5) |
| ARR-0009 | ALLOCATED | Slice Contract final closure review (FAIL — F-1 + §6.3 clerical) |
| **ARR-0010** | **ALLOCATED** | **Pre-Git Governance Integrity Final Review** (authority PBA-0025; 2026-08-13; verdict PASS — PRE-GIT INTEGRITY CLOSED) |
| **ARR-0011** | **ALLOCATED** | **RFC-0002 Product Board Review** (authority PBA-0033; 2026-08-13; verdict PASS — READY FOR PRODUCT BOARD ACCEPTANCE) |
| **ARR-0012** | **ALLOCATED** | **ADR-0005 Independent Architecture Review** (authority PBA-0036; 2026-08-13; verdict PASS — READY FOR PRODUCT BOARD ACCEPTANCE) |
| **ARR-0013** | **ALLOCATED** | **ADR-0005 Final Remediation Verification** (authority PBA-0038; 2026-08-13; verdict PASS — READY FOR PRODUCT BOARD ACCEPTANCE) |
| **ARR-0014** | **ALLOCATED** | **Slice-1 Phase 1 Independent Architecture Review** (authority PBA-0043; 2026-08-13; findings MAJOR-1, MINOR-1, MINOR-2, MINOR-3, OBSERVATION-1 — remediated under PBA-0044 / ER-0049) |
| **ARR-0015** | **ALLOCATED** | **Phase-1 Representability Remediation Independent Re-Review** (authority PBA-0045; 2026-08-13; verdict PASS WITH MINOR REMEDIATION REQUIRED — finding M-1, observations O-1/O-2/O-3) |
| **ARR-0016** | **ALLOCATED** | **Phase-1 Final Closure Verification** (authority PBA-0047; 2026-08-13; verdict PASS — PHASE 1 READY FOR PRODUCT BOARD CLOSURE; observations O-1/O-2/O-3 carried) |
| **ARR-0017** | **ALLOCATED** | **Slice-1 Phase-2A Independent Implementation Review** (authority PBA-0051; 2026-08-14; verdict PASS WITH MINOR REMEDIATION REQUIRED — finding M-1, observations B-4/B-5/B-6) |
| **ARR-0018** | **ALLOCATED** | **Slice-1 Phase-2A Final Closure Verification** (authority PBA-0053; 2026-08-14; verdict PASS — PHASE 2A READY FOR CLOSURE; M-1/F-2/F-4 closed; B-4/B-5/B-6 carried; authorship overlap disclosed) |
| **ARR-0019** | **ALLOCATED** | **First Runnable Slice Contract Revision 2 Independent Review** (authority PBA-0060; 2026-08-14; verdict PASS WITH MINOR REMEDIATION REQUIRED — findings M-1, M-2; observations B-1, B-2; authorship overlap disclosed) |
| **ARR-0020** | **ALLOCATED** | **First Runnable Slice Contract Revision 2 Final Re-Review** (authority PBA-0062; 2026-08-14; verdict PASS — READY FOR PRODUCT BOARD ACCEPTANCE; M-1/M-2 closed; observations B-1/B-2 carried; authorship overlap disclosed) |
| **ARR-0021** | **ALLOCATED** | **Slice-1 Phase-2B Independent Implementation Review** (authority PBA-0065; 2026-08-15; verdict PASS — READY FOR PHASE-2B FORMAL CLOSURE; zero BLOCKER/MAJOR/MINOR; three carried observations; authorship overlap disclosed) |
| **ARR-0022** | **ALLOCATED** | **Slice-1 Phase-3 Independent Implementation Review** (authority PBA-0068; 2026-08-15; verdict FAIL — REMEDIATION REQUIRED — one MAJOR finding M-1 (numeric containment not equivalent to §4.10 subset containment); observations O-A/O-B/O-C carried; remediated under PBA-0069 / ER-0066) |
| **ARR-0023** | **ALLOCATED** | **Slice-1 Phase-3 M-1 Remediation Re-Review** (authority PBA-0070; 2026-08-15; verdict PASS — M-1 CLOSED; READY FOR PHASE-3 FORMAL CLOSURE; observations O-A/O-B carried unchanged) |
| **ARR-0024** | **ALLOCATED** | **Slice-1 Phase-4 Independent Architecture and Implementation Review** (authority PBA-0075; 2026-08-15; verdict FAIL — REMEDIATION REQUIRED — one MINOR finding M-1 (§6.2 defect-set/verdict biconditional not enforced on the `unusable` path); observations OBS-1/OBS-2/OBS-3 carried; remediated under PBA-0076 / ER-0071) |
| **ARR-0025** | **ALLOCATED** | **Slice-1 Phase-4 MINOR-1 Remediation Re-Review** (authority PBA-0077; 2026-08-15; verdict PASS — M-1 CLOSED; READY FOR PHASE-4 FORMAL CLOSURE; observations OBS-1/OBS-2/OBS-3 carried) |
| **ARR-0026** | **ALLOCATED** | **Slice Contract Revision 3 Independent Review** (authority PBA-0083; 2026-08-15; verdict FAIL — BOUNDED REMEDIATION REQUIRED — one MINOR M-1 (Revision-3 §E model-comment currency list omitted `src/model/derivation.rs`); substantive §C ruling sound; remediated under PBA-0084 / ER-0077) |
| **ARR-0027** | **ALLOCATED** | **Slice Contract Revision 3 M-1 Remediation Re-Review** (authority PBA-0085; 2026-08-15; verdict PASS — M-1 CLOSED; READY FOR ACCEPTANCE; observation O-B carried, resolved under ER-0079). *Row added under PBA-0093 (2026-08-16) to complete a pre-existing clerical omission: this identifier was already recorded as allocated in the Revision-3 header, the §11.1 administrative table, and ER-0079, and in the §11 high-water / next-allocatable values; no assignment is changed, displaced, or reused.* |
| **ARR-0028** | **ALLOCATED** | **Slice Contract Revision 4 Independent Architecture and Contract Review** (authority PBA-0092; 2026-08-16; read-only; verdict FAIL — BOUNDED REMEDIATION REQUIRED — one MAJOR M-1 (§C.7 applied symmetric occurrence-order offender selection to the asymmetric check-11 `UNREACHABLE_SUBSUMED_RULE`); occurrence-identity architecture otherwise independently verified; remediated under PBA-0093 / ER-0084) |
| **ARR-0029** | **ALLOCATED** | **Slice Contract Revision 4 Independent Post-Remediation Re-Review** (authority PBA-0094; 2026-08-16; read-only; verdict PASS — ACCEPTANCE-READY; ARR-0028 M-1 CLOSED; zero BLOCKER/MAJOR/MINOR; check-9 symmetric and check-11 asymmetric anchors, OccurrenceId/LINEAGE/COUNTERPART, §6.4 totality, and Revision-3 preservation all independently re-verified) |
| **ARR-0030** | **ALLOCATED** | **Phase-1 Canonical Occurrence Identity Independent Reconciliation Review** (authority PBA-0097; 2026-08-16; read-only; verdict PASS — PHASE-1 REMEDIATION RECONCILED; universal coverage, envelope identity, absent/duplicate-lineage independence, supplied-order neutrality, occurrence/lineage separation, and Phase-4/Phase-5 boundary all independently verified; zero findings) |
| **ARR-0031** | **ALLOCATED** | **Phase-4 Result-Model Independent Reconciliation Review** (authority PBA-0099; 2026-08-16; read-only; verdict PASS — PHASE-4 REMEDIATION RECONCILED; `DefectCore.offending` occurrence migration, LINEAGE kind 7, COUNTERPART occurrence payload, §6.4 key-2 occurrence order, symmetric/asymmetric pairwise anchors, ATOM and not-evaluated preservation all independently verified; zero findings) |
| **ARR-0032** | **ALLOCATED** | **Slice-1 Phase-5A Check-1 Independent Implementation Review** (authority PBA-0101; 2026-08-16; read-only; verdict PASS — CHECK 1 IMPLEMENTATION VERIFIED; ownership matrix, PROPERTY positions, occurrence anchoring, O-1 isolation, and Phase-4/Phase-6 boundaries all independently verified; zero findings) |
| **ARR-0033** | **ALLOCATED** | **Phase-5 Slice-B Independent Implementation Review** (authority PBA-0104; 2026-08-16; read-only; verdict PASS — SLICE B CHECKS 15/16 VERIFIED; population/exclusions/anchors, NONE discriminator, contract-faithful SM-1 binding, and UNKNOWN_CONSTRUCT unrepresentability all independently verified; zero findings) |
| **ARR-0034** | **ALLOCATED** | **Phase-5 Slice-C Independent Implementation Review** (authority PBA-0106; 2026-08-16; read-only; verdict FAIL — one MAJOR M-1: check-2 ENUM atom overlap (inadmissible operator + undeclared value) resolved by implementation-selected precedence; all other check-2 surfaces independently verified PASS; resolved by ER-0092 RULING A / ER-0093) |
| **ARR-0035** | **ALLOCATED** | **Slice-1 Phase-5 Slice-C M-1 Remediation Re-Review** (authority PBA-0109; 2026-08-17; read-only; verdict PASS — SLICE-C M-1 CLOSED; CHECK 2 VERIFIED; ER-0092 RULING-A co-occurrence faithfully implemented, TYPE_MISMATCH one-per-atom preserved, O-1 isolation and all ARR-0034 PASS surfaces independently re-verified; zero findings) |
| **ARR-0036** | **ALLOCATED** | **Slice-1 Phase-5 Slice-D Independent Implementation Review** (authority PBA-0112; 2026-08-17; read-only; verdict PASS — SLICE D CHECKS 8/10 VERIFIED; check-8 populations/multiplicities/anchors/discriminators and precedence-order use, check-8 co-occurrence (independently justified from §5.1 + §3.7 Prohibited list, not resting on ER-0092), check-10 sole outcome-validity ownership, occurrence anchors, NONE discriminators, check-2/prior-check freeze, and Phase-6 boundary all independently verified; two non-blocking OBSERVATIONs (O-1: §3.7 prose vs §5.1 population; O-2: absent-lineage anchoring test coverage)) |
| ARR-0037 and above | AVAILABLE-FUTURE | — |

Two review reports were issued under **temporary, deliberately unnumbered**
identifiers by Board instruction: the *Pre-Git Governance & Repository Integrity
Audit* and the *Governance Identifier Collision Reconciliation Report*. Neither
consumes an ARR number.

**Next allocatable ARR: `ARR-0037`.**

## 8. ER — Execution Reports

| Identifier | State | Assigned to |
|---|---|---|
| ER-0001…ER-0013 | UNKNOWN-HISTORICAL | — |
| ER-0014 | ALLOCATED | ADR-0002 bounded remediation |
| ER-0015 | ALLOCATED | ADR-0002 final bounded remediation |
| ER-0016, ER-0017 | UNKNOWN-HISTORICAL | — |
| ER-0018 | ALLOCATED | ADR-0003 bounded remediation (closes F-1…F-7) |
| ER-0019 | ALLOCATED | ADR-0003 editorial remediation (closes MINOR-1/MINOR-2) |
| ER-0020…ER-0022 | UNKNOWN-HISTORICAL | — |
| ER-0023 | ALLOCATED | ADR-0004 bounded remediation (closes M-1/M-2/M-3, m-1…m-4) |
| ER-0024 | ALLOCATED | ADR-0004 editorial remediation (closes f-1/f-2/f-3) |
| ER-0025, ER-0026 | UNKNOWN-HISTORICAL | — |
| ER-0027 | ALLOCATED | ADR-0004 canonical register alignment report |
| ER-0028 | ALLOCATED | Slice Contract authoring report |
| ER-0029 | ALLOCATED | Slice Contract bounded remediation report (authorized by PBA-0024) |
| ER-0030 | ALLOCATED | Slice Contract final bounded remediation report |
| ER-0031 | ALLOCATED | Slice Contract two-item remediation and closure report |
| ER-0032 | ALLOCATED | Slice Contract acceptance report |
| ER-0033 | ALLOCATED | Pre-Git governance integrity remediation report (this remediation) |
| **ER-0034** | **ALLOCATED** | **Identifier Ledger Post-Review Synchronization Report** (authority PBA-0026; 2026-08-13; result: LEDGER SYNCHRONIZED — PRE-GIT BASELINE READY) |
| **ER-0035** | **ALLOCATED** | **Identifier Ledger Baseline Freeze Synchronization Report** (authority PBA-0027; 2026-08-13). Baseline-freeze administrative allocation — see §11.1. |
| **ER-0036** | **ALLOCATED** | **Git Baseline & Private GitHub Repository Readiness Report** (authority PBA-0028; 2026-08-13; verdict PASS — PRIVATE GITHUB BASELINE CREATION MAY BE AUTHORIZED) |
| **ER-0037** | **ALLOCATED** | **Ordina Git & Private GitHub Baseline Establishment Report** (authority PBA-0029; 2026-08-13) |
| **ER-0038** | **ALLOCATED** | **Post-Baseline Repository Hygiene Report** (authority PBA-0030; 2026-08-13) |
| **ER-0039** | **ALLOCATED** | **Slice-1 Implementation Stack Recommendation** (authority PBA-0031; 2026-08-13; read-only analysis, no repository change) |
| **ER-0040** | **ALLOCATED** | **RFC-0002 Slice-1 Implementation Stack Authoring Report** (authority PBA-0032; 2026-08-13) |
| **ER-0041** | **ALLOCATED** | **RFC-0002 Acceptance Report** (authority PBA-0034; 2026-08-13) |
| **ER-0042** | **ALLOCATED** | **ADR-0005 Authoring Report** (authority PBA-0035; 2026-08-13) |
| **ER-0043** | **ALLOCATED** | **ADR-0005 Final Bounded Remediation Report** (authority PBA-0037; 2026-08-13) |
| **ER-0044** | **ALLOCATED** | **ADR-0005 Acceptance Report** (authority PBA-0039; 2026-08-13) |
| **ER-0045** | **ALLOCATED** | **Slice-1 Rust Bootstrap and Implementation Plan Report** (authority PBA-0040; 2026-08-13; halted at preflight — missing MSVC linker; no repository change) |
| **ER-0046** | **ALLOCATED** | **Windows MSVC Prerequisite Remediation and Slice-1 Bootstrap Report** (authority PBA-0041; 2026-08-13) |
| **ER-0047** | **ALLOCATED** | **Slice-1 Phase 1 Authorization Preparation Report** (authority PBA-0042; 2026-08-13) |
| **ER-0048** | **ALLOCATED** | **Slice-1 Phase 1 Execution Report** — core model and opaque domains (authority PBA-0042; 2026-08-13) |
| **ER-0049** | **ALLOCATED** | **Phase-1 Representability Remediation Execution Report** (authority PBA-0044; 2026-08-13; closes ARR-0014 MAJOR-1, MINOR-1, MINOR-3 and OBSERVATION-1; MINOR-2 dispositioned to the external boundary) |
| **ER-0050** | **ALLOCATED** | **Phase-1 Final Bounded Helper Remediation Execution Report** (authority PBA-0046; 2026-08-13; closes ARR-0015 M-1) |
| **ER-0051** | **ALLOCATED** | **Phase-1 Formal Closure Report** (authority PBA-0048; 2026-08-13; records PHASE 1 CLOSED on the basis of ARR-0016) |
| **ER-0052** | **ALLOCATED** | **Phase-2 Scope and Implementation-Plan Review** (authority PBA-0049; 2026-08-13; verdict PASS WITH MINOR CONDITIONS — findings F-1…F-4, observations B-1…B-3) |
| **ER-0053** | **ALLOCATED** | **Slice-1 Phase-2A Exact Numeric Value Layer Implementation Report** (authority PBA-0050; 2026-08-13; closes ER-0052 F-2 and F-4; F-1 and F-3 remain open for Phase 2B) |
| **ER-0054** | **ALLOCATED** | **Slice-1 Phase-2A M-1 Structural Regression-Evidence Remediation Report** (authority PBA-0052; 2026-08-14; closes ARR-0017 M-1) |
| **ER-0055** | **ALLOCATED** | **Slice-1 Phase-2A Formal Closure Report** (authority PBA-0054; 2026-08-14; records PHASE 2A CLOSED on the basis of ARR-0018) |
| **ER-0056** | **ALLOCATED** | **Slice-1 Phase-2B Gating Decisions (F-3 STOP)** (authority PBA-0055; 2026-08-14; read-only — F-1/B-4/B-5 determined, F-3 blocked; no repository write) |
| **ER-0057** | **ALLOCATED** | **Slice-1 Distinguished Opaque-Domain Member Binding Decision Report** (authority PBA-0056; 2026-08-14; records F-1/B-4/B-5; F-3 ruling PROPOSED, not normative — Phase 2B remains blocked) |
| **ER-0058** | **ALLOCATED** | **Distinguished-Member Binding Normative Revision (Governance STOP)** (authority PBA-0057; 2026-08-14; read-only — no revision convention existed, so no artifact could be authored; no repository write) |
| **ER-0059** | **ALLOCATED** | **Normative Engineering Contract Revision Convention Determination Report** (authority PBA-0058; 2026-08-14; establishes PBD-0006) |
| **ER-0060** | **ALLOCATED** | **First Runnable Slice Contract Revision 2 Authoring Report** (authority PBA-0059; 2026-08-14; authors `first-runnable-slice-contract-r02.md` as PROPOSED — Revision 1 remains authoritative; F-3 and Phase 2B remain blocked) |
| **ER-0061** | **ALLOCATED** | **First Runnable Slice Contract Revision 2 Bounded Remediation Report** (authority PBA-0061; 2026-08-14; closes ARR-0019 M-1 and M-2 inside §2.1; Revision 2 remains PROPOSED; F-3 and Phase 2B remain blocked) |
| **ER-0062** | **ALLOCATED** | **First Runnable Slice Contract Revision 2 Acceptance Report** (authority PBA-0063; 2026-08-14; records PBR-0014 acceptance — Revision 2 ACCEPTED, Revision 1 superseded, F-3 CLOSED, Phase 2B UNBLOCKED) |
| **ER-0063** | **ALLOCATED** | **Slice-1 Phase-2B Dimension-Aware Semantics Implementation Report** (authority PBA-0064; 2026-08-15; implements §4.3 / §4.4 / §4.6 semantic primitives in `src/semantics/`; independent review required before closure) |
| **ER-0064** | **ALLOCATED** | **Slice-1 Phase-2B Formal Closure Report** (authority PBA-0066; 2026-08-15; records PHASE 2B CLOSED on the basis of ARR-0021) |
| **ER-0065** | **ALLOCATED** | **Slice-1 Phase-3 Constraint-Domain Semantics Implementation Report** (authority PBA-0067; 2026-08-15; implements §4.10 per-reference constraint-domain algebra in `src/semantics/constraint/`; independent review required before closure) |
| **ER-0066** | **ALLOCATED** | **Slice-1 Phase-3 Numeric Containment Remediation Report** (authority PBA-0069; 2026-08-15; closes ARR-0022 M-1 — restores exact §4.10 numeric subset containment in `src/semantics/constraint/numeric.rs`; independent re-review required before closure) |
| **ER-0067** | **ALLOCATED** | **Repository Attribution Trailer Removal and History Remediation Report** (authority PBA-0071; 2026-08-15; removes a prohibited AI attribution trailer from the Phase-3 M-1 remediation commit via metadata-only history rewrite — tree identity preserved; `6e68dc8` → `4fb85da`; no source, test, dependency, toolchain, or accepted-document change) |
| **ER-0068** | **ALLOCATED** | **Slice-1 Phase-3 Formal Closure Report** (authority PBA-0072; 2026-08-15; records **Phase 3 CLOSED** in the Slice-1 implementation plan on the basis of ARR-0023; resolves ARR-0022/ARR-0023 observations O-A (crate-doc currency) and O-B (plan currency); documentation-only — no source, test, dependency, toolchain, or accepted-document change) |
| **ER-0069** | **ALLOCATED** | **Slice-1 Post-Phase-3 Implementation Plan Review** (authority PBA-0073; 2026-08-15; read-only; disposition NEXT PHASE IDENTIFIED — READY FOR IMPLEMENTATION AUTHORIZATION (Phase 4, §6.1–§6.4)) |
| **ER-0070** | **ALLOCATED** | **Slice-1 Phase-4 Defect / Result Model and Canonical Ordering Implementation Report** (authority PBA-0074; 2026-08-15; implements §6.1–§6.4 result/defect model and canonical ordering in `src/result/`; no defect production, no check; independent review required before closure) |
| **ER-0071** | **ALLOCATED** | **Slice-1 Phase-4 MINOR-1 Remediation Report** (authority PBA-0076; 2026-08-15; closes ARR-0024 MINOR-1 — makes the §6.2 `UNUSABLE`+empty state unrepresentable via a required first-defect argument on `WellFormednessResult::unusable`; independent re-review required before closure) |
| **ER-0072** | **ALLOCATED** | **Slice-1 Phase-4 Formal Closure Report** (authority PBA-0078; 2026-08-15; records **Phase 4 CLOSED** in the Slice-1 implementation plan on the basis of ARR-0025; resolves observation OBS-2 (crate-doc currency in `src/lib.rs`); documentation-only — no source, test, dependency, toolchain, or accepted-document change) |
| **ER-0073** | **ALLOCATED** | **Slice-1 Phase-5 Implementation Planning Review** (authority PBA-0079; 2026-08-15; read-only; disposition NEXT PHASE IDENTIFIED — READY FOR IMPLEMENTATION AUTHORIZATION (Phase 5, check 1 first)) |
| **ER-0074** | **ALLOCATED** | **Slice-1 Phase-5A Check-1 Implementation Report (STOPPED)** (authority PBA-0080; 2026-08-15; halted before any repository write on a check-1/check-15 lineage-ownership ambiguity; no source/ledger change) |
| **ER-0075** | **ALLOCATED** | **Slice-1 Check-1 Defect-Population Clarification Review** (authority PBA-0081; 2026-08-15; read-only; classification SUCCESSOR CONTRACT CLARIFICATION REQUIRED; recommends operative-node lineage exclusively check 15) |
| **ER-0076** | **ALLOCATED** | **Slice Contract Revision 3 Check-1 Ownership Clarification Authoring Report** (authority PBA-0082; 2026-08-15; authors `first-runnable-slice-contract-r03.md` as PROPOSED — Revision 2 remains authoritative; independent review and Product Board acceptance required before Phase-5A) |
| **ER-0077** | **ALLOCATED** | **Slice Contract Revision 3 M-1 Remediation Report** (authority PBA-0084; 2026-08-15; completes the Revision-3 §E model-comment currency inventory by adding `src/model/derivation.rs` — ARR-0026 M-1; no normative §C change; Revision 3 remains PROPOSED; independent re-review required) |
| **ER-0078** | **ALLOCATED** | **Slice Contract Revision 3 Acceptance Report** (authority PBA-0086; 2026-08-15; records **PBR-0015** — Revision 3 ACCEPTED and authoritative, Revision 2 SUPERSEDED; clerical status transition only, substantive §C unchanged; Phase 5A remains unimplemented) |
| **ER-0079** | **ALLOCATED** | **Revision 3 Post-Acceptance Model-Comment Currency Report** (authority PBA-0087; 2026-08-15; comment-only alignment of `src/model/strategy.rs`, `src/model/condition.rs`, `src/model/derivation.rs` to accepted Revision 3 §C; resolves ARR-0027 O-B; no executable model change) |
| **ER-0080** | **ALLOCATED** | **Slice-1 Governance Recovery and Canonical Construct-Occurrence Identity Determination** (authority PBA-0088; 2026-08-15; read-only; concludes lineage identity is an insufficient universal defect anchor for both absent and duplicate lineage, and that a canonical opaque `OccurrenceId` + supplied total order is required — SUCCESSOR REVISION REQUIRED; no source/ledger change) |
| **ER-0081** | **ALLOCATED** | **Slice Contract Revision 4 Authoring Report (STOPPED)** (authority PBA-0089; 2026-08-15; halted before any repository write on two anchoring blockers — `DUPLICATE_LINEAGE_IDENTITY` class-level anchor and COUNTERPART absent-lineage representability; no contract authored, no source/ledger change) |
| **ER-0082** | **ALLOCATED** | **Slice-1 Class-Level and Counterpart Defect Identity Determination** (authority PBA-0090; 2026-08-15; read-only; integrated design — envelope anchor + new `LINEAGE` discriminator for `DUPLICATE_LINEAGE_IDENTITY`, COUNTERPART payload → `OccurrenceId`, first-occurrence pairwise selection — REVISION 4 SUFFICIENT; no source/ledger change) |
| **ER-0083** | **ALLOCATED** | **Slice Contract Revision 4 Canonical Occurrence and Integrated Defect Identity Authoring Report** (authority PBA-0091; 2026-08-15; authors `first-runnable-slice-contract-r04.md` as **PROPOSED** and synchronizes PBA-0088–0091 / ER-0080–0083; Revision 3 remains authoritative; independent review and Product Board acceptance required before Phase-1/Phase-4 remediation) |
| **ER-0084** | **ALLOCATED** | **Slice Contract Revision 4 Check-11 Anchor Remediation Report** (authority PBA-0093; 2026-08-16; splits §C.7 into symmetric check-9 occurrence-order selection and asymmetric check-11 subsumed-rule anchor per ARR-0028 M-1; closes ARR-0028 O-1 editorially (NONE for Strategy-level defects); COUNTERPART/OccurrenceId/LINEAGE architecture unchanged; Revision 4 remains PROPOSED; independent re-review required) |
| **ER-0085** | **ALLOCATED** | **Slice Contract Revision 4 Acceptance Report** (authority PBA-0095; 2026-08-16; records **PBR-0016** — Revision 4 ACCEPTED and authoritative, Revision 3 SUPERSEDED; clerical status/metadata transition only, substantive §C byte-frozen from the ARR-0029-reviewed version; contract/design O-A resolved; Phase-1/Phase-4 implementation remediation pending; Phase 5A remains blocked) |
| **ER-0086** | **ALLOCATED** | **Phase-1 Canonical Occurrence Identity Remediation Report** (authority PBA-0096; 2026-08-16; adds the `OccurrenceId` opaque domain in `src/model/opaque.rs` and a mandatory `occurrence` field on the Strategy envelope and all five child constructs, with focused occurrence-identity tests; no Phase-4 result-model or `src/checks/**` change; Phase 1 remains CLOSED; independent Phase-1 reconciliation review required before Phase-4 remediation) |
| **ER-0087** | **ALLOCATED** | **Bounded Phase-4 Result-Model Remediation Report** (authority PBA-0098; 2026-08-16; migrates `src/result/{identity,discriminator,mod}.rs` to the Revision-4 occurrence anchor — `DefectCore.offending: OccurrenceId`, COUNTERPART payload `OccurrenceId`, new `LINEAGE` discriminator kind 7, §6.4 key-2 occurrence order — with focused anchor-semantics tests; ATOM payload and §6.2 not-evaluated representation unchanged; no `src/checks/**` change; Phase 4 remains CLOSED; independent Phase-4 reconciliation required) |
| **ER-0088** | **ALLOCATED** | **Slice-1 Phase-5A Check-1 Implementation Report** (authority PBA-0100; 2026-08-16; implements check 1 `MISSING_REQUIRED_PROPERTY` in `src/checks/required_property.rs` — per-construct `Vec<DefectCore>` production, occurrence-anchored, ownership matrix exact, O-1 isolated; `src/model/**` and `src/result/**` unchanged; lib.rs crate-doc currency only; no other mandatory check, gating, or verdict; independent review required) |
| **ER-0089** | **ALLOCATED** | **Slice-1 Phase-5 Remaining-Roadmap Reconciliation Report** (authority PBA-0102; 2026-08-16; read-only; reconciles the remaining Phase-5 checks into Slices B–H, corrects the check-8 scheduling omission (Slice D), schedules the O-1 determination before Slice F, carries O-2/O-3 non-blocking; next slice = B (checks 15+16); no source change) |
| **ER-0090** | **ALLOCATED** | **Slice-1 Phase-5 Slice-B Checks 15/16 Implementation Report** (authority PBA-0103; 2026-08-16; implements check 15 `MISSING_OPERATIVE_PROVENANCE` (`src/checks/operative_provenance.rs`) and check 16 version half `UNSUPPORTED_SEMANTIC_MODEL_VERSION` (`src/checks/semantic_model_version.rs`); occurrence-anchored, NONE discriminator, `UNKNOWN_CONSTRUCT` unrepresentable and not invented; no other check/gating/verdict; `src/model`/`src/result` unchanged; independent review required) |
| **ER-0091** | **ALLOCATED** | **Slice-1 Phase-5 Slice-C Check-2 Implementation Report** (authority PBA-0105; 2026-08-16; implements check 2 in `src/checks/reference_integrity.rs` — `DUPLICATE_LINEAGE_IDENTITY` (envelope anchor + LINEAGE, one per duplicated value), `UNRESOLVED_REFERENCE` (exactly-one resolution; ambiguous→unresolved), `TYPE_MISMATCH` (atoms + derivation leaves), `ENUM_VALUE_NOT_DECLARED` (membership only); ATOM/NODE-PATH/LINEAGE discriminators; O-1 isolated and OPEN; no other check/gating/verdict; `src/model`/`src/result` unchanged; independent review required) |
| **ER-0092** | **ALLOCATED** | **Check-2 ENUM Atom Overlap Defect-Population Determination** (authority PBA-0107; 2026-08-16; read-only Product Board determination; **RULING A — co-occurrence**: an ENUM atom failing both operator admissibility and value membership emits both `TYPE_MISMATCH` and `ENUM_VALUE_NOT_DECLARED`, distinct at §6.4 category ordinal; "one per atom" limits TYPE_MISMATCH only; confirms accepted text, no successor revision) |
| **ER-0093** | **ALLOCATED** | **Slice-1 Phase-5 Slice-C M-1 Remediation Report** (authority PBA-0108; 2026-08-17; implements ER-0092 RULING A in `src/checks/reference_integrity.rs` — the ENUM arm now evaluates operator admissibility and value membership independently and emits both defects when both hold; TYPE_MISMATCH stays one-per-atom; O-1 isolation and all ARR-0034 PASS surfaces preserved; `src/model`/`src/result` unchanged; independent re-review required) |
| **ER-0094** | **ALLOCATED** | **Slice-1 Phase-5 Slice-D Checks 8/10 Implementation Report** (authority PBA-0110; 2026-08-17; implements check 8 in `src/checks/direction_coverage.rs` (`DIRECTION_COVERAGE_INCOMPLETE`/`MULTIPLE_TOTAL_DIRECTION_RULES`/`TOTAL_RULE_NOT_LOWEST_PRECEDENCE`, envelope/member occurrence anchors, `NONE` discriminator, independent populations, supplied precedence order, no check-2/6 dependency) and check 10 in `src/checks/direction_determinism.rs` (`INVALID_DIRECTION_OUTCOME`, one per rule, rule-occurrence anchor, `NONE`, sole outcome-validity ownership); defect production only; no new discriminator; `src/model`/`src/result` unchanged; checks 1/2/15/16 unchanged; no Phase-6 orchestration; O-1/O-2/O-3 OPEN; independent review required) |
| **ER-0095** | **ALLOCATED** | **Slice-D Commit Attribution Trailer Removal Report** (authority PBA-0111; 2026-08-17; removes a prohibited `Co-Authored-By: Claude Opus 4.8` trailer from the Slice-D implementation commit via metadata-only history rewrite — tree identity preserved; `f592bb9` → `b704949`; no source, test, dependency, toolchain, or accepted-document change; Slice-D semantics under ER-0094 unchanged) |
| **ER-0096** | **ALLOCATED** | **Slice-1 Phase-5 Slice-E Checks 3/4/5 Implementation Report** (authority PBA-0113; 2026-08-17; implements check 3 in `src/checks/condition_classification.rs` (`INVALID_CONDITION_CLASSIFICATION`, condition-occurrence anchor, NONE, sole classification-validity owner), check 4 in `src/checks/configuration_bounds.rs` (`INVERTED_CONFIGURATION_BOUNDS` NONE + `CONFIGURATION_BOUND_DIMENSION_MISMATCH` PROPERTY 4/5, parameter-occurrence anchor, exact numeric/dimension primitives, intrinsic computability), check 5 in `src/checks/configuration_position.rs` (`CONFIGURATION_IN_STRUCTURAL_POSITION`, guard-atom-resolves-to-config-parameter O-3 population, enclosing-construct anchor, ATOM discriminator, exactly-one §4.7 resolution); defect production only; no new discriminator/category; `src/model`/`src/result`/`src/numeric`/`src/semantics` unchanged; O-1/O-2/O-3 remain OPEN (O-3 preserved, not resolved); independent review required) |
| ER-0097 and above | AVAILABLE-FUTURE | — |

**Next allocatable ER: `ER-0097`.**

## 9. PBD — Product Board Determinations, and other families

| Identifier | State | Assigned to |
|---|---|---|
| PBD-0001 | ALLOCATED | Product Model decisions (referenced across Tier-1/Tier-2 documents) |
| PBD-0002 | ALLOCATED | Constitution v0.3 dispositions (D1, D2, D6, D9 …) |
| PBD-0003 | ALLOCATED | Trader Readiness sourcing (adopts WN-0009) |
| PBD-0004 | ALLOCATED | System Context dispositions (adopts SCW-01 Findings 1–7) |
| **PBD-0005** | **ALLOCATED** | **Governance Identifier Ledger Establishment and Collision Resolution** |
| **PBD-0006** | **ALLOCATED** | **Normative Engineering Contract Revision Convention** (authority PBA-0058; 2026-08-14). Establishes the successor convention for an accepted normative engineering contract that is neither an ADR nor an RFC and carries no version lifecycle. Recorded in `docs/engineering/workflow.md`. |
| PBD-0007 and above | AVAILABLE-FUTURE | — |

Other historical families, recorded for reference only and not allocated by this
ledger: **POR-0001** (Product Owner Ratification, Constitution v1.0);
**ARW-0001/0002/0003** (discovery bases for ADR-0002/0003/0004);
**WN-0001/0002/0003/0006/0009** (workshop notes); **WR-0001/0003/0004** (working
records); **SCW-01** (System Context workshop).

**Next allocatable PBD: `PBD-0007`.**

## 10. Reserved and Unknown-Historical Identifiers

**RESERVED — must never be reused:** `PBR-0008`.

**UNKNOWN-HISTORICAL — below the high-water mark, disposition unrecoverable, must
not be reused:** `PBA-0003`, `PBA-0007`, `PBA-0008`, `PBA-0010`, `PBA-0011`,
`PBA-0013`, `PBA-0021`, `PBA-0022`; `PBR-0001`, `PBR-0002`, `PBR-0004`,
`PBR-0005`; `ER-0001`…`ER-0013`, `ER-0016`, `ER-0017`, `ER-0020`…`ER-0022`,
`ER-0025`, `ER-0026`.

**SUPERSEDED-ID:** none. Both corrected assignments (PBA-0024, PBR-0011) are
recorded as ALLOCATED against their artifacts; the identifiers they replaced
(PBA-0016, PBR-0010) remain ALLOCATED to their **priority** artifacts and were
never vacated.

## 11. High-Water Marks and Next Allocatable Identifiers

| Namespace | High-water mark | Next allocatable |
|---|---|---|
| PBA | PBA-0113 | **PBA-0114** |
| PBR | PBR-0016 | **PBR-0017** |
| ARR | ARR-0036 | **ARR-0037** |
| ER | ER-0096 | **ER-0097** |
| PBD | PBD-0006 | **PBD-0007** |

**No pending synchronization remains.** Every issued identifier in every namespace
is recorded as an ordinary allocation in §5–§9. No identifier that has been issued
is represented as AVAILABLE-FUTURE, and no entry above requires a further
ledger-only task.

### 11.1 Baseline Freeze Record

The pre-baseline synchronization terminates here. Four cycles ran immediately
before and around the first Git baseline. Two were clerical cycles whose sole
purpose was bringing this ledger current; the other two were the pre-Git readiness
review and the baseline establishment itself, recorded here because their
identifiers were allocated in the same period:

| Cycle | Authorization | Report | Purpose |
|---|---|---|---|
| Post-review synchronization | PBA-0026 | ER-0034 | Record PBA-0025 / ARR-0010; close ARR-0010 MINOR-1 and MINOR-3 |
| **Baseline freeze** | **PBA-0027** | **ER-0035** | Record PBA-0026 / ER-0034, record this cycle's own identifiers, and terminate the synchronization procedure |
| Git baseline readiness | PBA-0028 | ER-0036 | Read-only pre-Git readiness review (no ledger write; recorded by the cycle below, per §12(6)) |
| **Git baseline establishment** | **PBA-0029** | **ER-0037** | Record PBA-0028 / ER-0036 and this cycle's own identifiers; establish the Git and private GitHub baseline |
| Post-baseline hygiene | PBA-0030 | ER-0038 | Correct §11.1 narrative drift; establish the repository line-ending policy |
| Slice-1 stack analysis | PBA-0031 | ER-0039 | Read-only stack recommendation (no ledger write; recorded by the cycle below, per §12(6)) |
| RFC-0002 authoring | PBA-0032 | ER-0040 | Record PBA-0031 / ER-0039 and this cycle's own identifiers; author RFC-0002 |
| RFC-0002 review | PBA-0033 | ARR-0011 | Read-only Board review (no ledger write; recorded by the cycle below, per §12(6)) |
| RFC-0002 acceptance | PBA-0034 | ER-0041 | Record PBA-0033 / ARR-0011, PBR-0012, and this cycle's own identifiers; accept RFC-0002 |
| ADR-0005 authoring | PBA-0035 | ER-0042 | Author ADR-0005 (PROPOSED); record this cycle's own identifiers |
| ADR-0005 review | PBA-0036 | ARR-0012 | Read-only architecture review (no ledger write; recorded by the cycle below, per §12(6)) |
| ADR-0005 remediation | PBA-0037 | ER-0043 | Record PBA-0036 / ARR-0012 and this cycle's own identifiers; close ARR-0012 findings |
| ADR-0005 verification | PBA-0038 | ARR-0013 | Read-only final verification (no ledger write; recorded by the cycle below, per §12(6)) |
| ADR-0005 acceptance | PBA-0039 | ER-0044 | Record PBA-0038 / ARR-0013, PBR-0013, and this cycle's own identifiers; accept ADR-0005 |
| Slice-1 bootstrap (halted) | PBA-0040 | ER-0045 | Halted at preflight — no MSVC linker; no ledger write, recorded by the cycle below |
| Slice-1 bootstrap (retry) | PBA-0041 | ER-0046 | Record PBA-0040 / ER-0045 and this cycle's own identifiers; MSVC prerequisite + Slice-1 bootstrap |
| Phase 1 authorization | PBA-0042 | ER-0047 | Prepare the Slice-1 Phase 1 implementation authorization; record this cycle's own identifiers |
| Phase 1 execution | PBA-0042 | ER-0048 | Execute Phase 1 — core model and opaque domains |
| Phase 1 review | PBA-0043 | ARR-0014 | Read-only independent architecture review of Phase 1 (no ledger write; recorded by the cycle below, per §12(6)) |
| Phase 1 representability remediation | PBA-0044 | ER-0049 | Record PBA-0043 / ARR-0014 and this cycle's own identifiers; make contract-defined malformed instances representable |
| Phase 1 remediation re-review | PBA-0045 | ARR-0015 | Read-only independent re-review of ER-0049 (no ledger write; recorded by the cycle below, per §12(6)) |
| Phase 1 helper remediation | PBA-0046 | ER-0050 | Record PBA-0045 / ARR-0015 and this cycle's own identifiers; close ARR-0015 M-1 |
| Phase 1 closure verification | PBA-0047 | ARR-0016 | Read-only final closure verification (no ledger write; recorded by the cycle below, per §12(6)) |
| **Phase 1 formal closure** | **PBA-0048** | **ER-0051** | Record PBA-0047 / ARR-0016 and this cycle's own identifiers; record **Phase 1 CLOSED** in the Slice-1 implementation plan. Governance-only write — no source, test, dependency or toolchain change. Terminates under §12(7). |
| Phase 2 scope review | PBA-0049 | ER-0052 | Read-only Phase-2 scope and implementation-plan review (no ledger write; recorded by the cycle below, per §12(6)) |
| Phase 2A implementation | PBA-0050 | ER-0053 | Record PBA-0049 / ER-0052 and this cycle's own identifiers; implement the Phase-2A exact numeric value layer |
| Phase 2A review | PBA-0051 | ARR-0017 | Read-only independent implementation review (no ledger write; recorded by the cycle below, per §12(6)) |
| Phase 2A M-1 remediation | PBA-0052 | ER-0054 | Record PBA-0051 / ARR-0017 and this cycle's own identifiers; add compile-fail regression evidence for the Phase-2A structural boundaries |
| Phase 2A closure verification | PBA-0053 | ARR-0018 | Read-only final closure verification (no ledger write; recorded by the cycle below, per §12(6)) |
| **Phase 2A formal closure** | **PBA-0054** | **ER-0055** | Record PBA-0053 / ARR-0018 and this cycle's own identifiers; record **Phase 2A CLOSED** in the Slice-1 implementation plan. Governance-only write — no source, test, dependency or toolchain change. Terminates under §12(7). |
| Phase 2B gating analysis | PBA-0055 | ER-0056 | Read-only; STOPPED on F-3 before any write (no ledger write; recorded by the cycle below, per §12(6)) |
| Phase 2B gating decisions | PBA-0056 | ER-0057 | Record PBA-0055 / ER-0056 and this cycle's own identifiers; record F-1 / B-4 / B-5 and the blocked status of F-3 in the Slice-1 implementation plan. Governance-only write. Terminates under §12(7). |
| Contract revision authoring | PBA-0057 | ER-0058 | Read-only; STOPPED before authoring — no revision convention existed (no ledger write; recorded by the cycle below, per §12(6)) |
| Revision convention determination | PBA-0058 | ER-0059 | Record PBA-0057 / ER-0058 and this cycle's own identifiers; issue **PBD-0006** establishing the normative engineering contract successor convention in `docs/engineering/workflow.md`. Governance-only write. Terminates under §12(7). |
| Slice Contract Revision 2 authoring | PBA-0059 | ER-0060 | Author `first-runnable-slice-contract-r02.md` as **PROPOSED** under PBD-0006; record this cycle's own identifiers. No ARR or PBR allocated — no review or acceptance occurred. Terminates under §12(7). |
| Slice Contract Revision 2 review | PBA-0060 | ARR-0019 | Read-only independent review (no ledger write; recorded by the cycle below, per §12(6)) |
| Slice Contract Revision 2 remediation | PBA-0061 | ER-0061 | Record PBA-0060 / ARR-0019 and this cycle's own identifiers; close ARR-0019 M-1/M-2 inside §2.1 of the PROPOSED Revision 2. No PBR allocated — no acceptance occurred. Terminates under §12(7). |
| Slice Contract Revision 2 re-review | PBA-0062 | ARR-0020 | Read-only final re-review (no ledger write; recorded by the cycle below, per §12(6)) |
| **Slice Contract Revision 2 acceptance** | **PBA-0063** | **PBR-0014 / ER-0062** | Record PBA-0062 / ARR-0020 and this cycle's own identifiers; **accept Revision 2 under PBR-0014**, superseding Revision 1 in governance provenance (predecessor file preserved immutable), closing F-3 and unblocking Phase 2B in the implementation plan. Governance-only write — no source, test, dependency or toolchain change. Terminates under §12(7). |
| Phase 2B implementation | PBA-0064 | ER-0063 | Record this cycle's own identifiers; implement the Phase-2B §4.3 / §4.4 / §4.6 dimension-aware semantic primitives in `src/semantics/`. No ARR or PBR allocated — implementation is not review or closure. Terminates under §12(7). |
| Phase 2B review | PBA-0065 | ARR-0021 | Read-only independent implementation review (no ledger write; recorded by the cycle below, per §12(6)) |
| **Phase 2B formal closure** | **PBA-0066** | **ER-0064** | Record PBA-0065 / ARR-0021 and this cycle's own identifiers; record **Phase 2B CLOSED** in the Slice-1 implementation plan. Governance-only write — no source, test, dependency or toolchain change. Terminates under §12(7). |
| Phase 3 implementation | PBA-0067 | ER-0065 | Record this cycle's own identifiers; implement the Phase-3 §4.10 per-reference constraint-domain algebra in `src/semantics/constraint/`. No ARR or PBR allocated — implementation is not review or closure. Terminates under §12(7). |
| Phase 3 review | PBA-0068 | ARR-0022 | Read-only independent implementation review (no ledger write; recorded by the cycle below, per §12(6)). Verdict FAIL — REMEDIATION REQUIRED (MAJOR M-1). |
| Phase 3 M-1 remediation | PBA-0069 | ER-0066 | Record PBA-0068 / ARR-0022 (pending from the read-only review, §12(6)) and this cycle's own identifiers; correct the §4.10 numeric containment defect (ARR-0022 M-1) in `src/semantics/constraint/numeric.rs`. Source-writing cycle — not ledger-only; independent re-review required before closure. |
| Phase 3 M-1 re-review | PBA-0070 | ARR-0023 | Read-only independent re-review of the M-1 remediation (no ledger write; recorded by the cycle below, per §12(6)). Verdict PASS — M-1 CLOSED. |
| Attribution trailer removal | PBA-0071 | ER-0067 | Record PBA-0070 / ARR-0023 (pending from the read-only re-review, §12(6)) and this cycle's own identifiers; remove a prohibited AI attribution trailer from the Phase-3 M-1 remediation commit via metadata-only history rewrite (`6e68dc8` → `4fb85da`, tree identity preserved). No source, test, dependency, toolchain, or accepted-document change; Phase 3 remains IMPLEMENTED, not CLOSED. |
| **Phase 3 formal closure** | **PBA-0072** | **ER-0068** | Record this cycle's own identifiers; record **Phase 3 CLOSED** in the Slice-1 implementation plan on the basis of ARR-0023, and resolve observations O-A (crate-doc currency in `src/lib.rs`) and O-B (plan currency). Documentation-only write — no source, test, dependency, toolchain or accepted-document change. Terminates under §12(7). |
| Post-Phase-3 planning review | PBA-0073 | ER-0069 | Read-only review of the implementation plan after Phase-3 closure (no ledger write; recorded by the cycle below, per §12(6)). Disposition: Phase 4 READY FOR IMPLEMENTATION AUTHORIZATION. |
| Phase 4 implementation | PBA-0074 | ER-0070 | Record PBA-0073 / ER-0069 (pending from the read-only planning review, §12(6)) and this cycle's own identifiers; implement the §6.1–§6.4 defect/result model and canonical ordering in `src/result/`. No ARR or PBR allocated — implementation is not review or closure. Terminates under §12(7). |
| Phase 4 review | PBA-0075 | ARR-0024 | Read-only independent architecture/implementation review (no ledger write; recorded by the cycle below, per §12(6)). Verdict FAIL — REMEDIATION REQUIRED (MINOR-1). |
| Phase 4 MINOR-1 remediation | PBA-0076 | ER-0071 | Record PBA-0075 / ARR-0024 (pending from the read-only review, §12(6)) and this cycle's own identifiers; enforce the §6.2 defect-set/verdict biconditional on `WellFormednessResult::unusable` (ARR-0024 MINOR-1) in `src/result/mod.rs`. Source-writing cycle — not ledger-only; independent re-review required before closure. |
| Phase 4 MINOR-1 re-review | PBA-0077 | ARR-0025 | Read-only independent re-review of the MINOR-1 remediation (no ledger write; recorded by the cycle below, per §12(6)). Verdict PASS — M-1 CLOSED. |
| **Phase 4 formal closure** | **PBA-0078** | **ER-0072** | Record PBA-0077 / ARR-0025 (pending from the read-only re-review, §12(6)) and this cycle's own identifiers; record **Phase 4 CLOSED** in the Slice-1 implementation plan on the basis of ARR-0025, and resolve observation OBS-2 (crate-doc currency in `src/lib.rs`). Documentation-only write — no source, test, dependency, toolchain or accepted-document change. Terminates under §12(7). |
| Phase 5 planning review | PBA-0079 | ER-0073 | Read-only review of the implementation plan after Phase-4 closure (no ledger write; recorded by the cycle below, per §12(6)). Disposition: Phase 5 check 1 READY FOR IMPLEMENTATION AUTHORIZATION. |
| Phase 5A check-1 (stopped) | PBA-0080 | ER-0074 | Read-only STOP before any write on a check-1/check-15 lineage-ownership ambiguity (no ledger write; recorded by the cycle below, per §12(6)). |
| Check-1 ownership clarification review | PBA-0081 | ER-0075 | Read-only normative clarification review (no ledger write; recorded by the cycle below, per §12(6)). Classification: SUCCESSOR CONTRACT CLARIFICATION REQUIRED. |
| Slice Contract Revision 3 authoring | PBA-0082 | ER-0076 | Record PBA-0079/ER-0073, PBA-0080/ER-0074, PBA-0081/ER-0075 (pending from the read-only/stopped cycles, §12(6)) and this cycle's own identifiers; author `first-runnable-slice-contract-r03.md` as **PROPOSED** under PBD-0006 (check-1 ownership clarification). Revision 2 remains authoritative; no ARR or PBR allocated — no review or acceptance occurred. Terminates under §12(7). |
| Slice Contract Revision 3 review | PBA-0083 | ARR-0026 | Read-only independent review of proposed Revision 3 (no ledger write; recorded by the cycle below, per §12(6)). Verdict FAIL — BOUNDED REMEDIATION REQUIRED (MINOR M-1). |
| Slice Contract Revision 3 M-1 remediation | PBA-0084 | ER-0077 | Record PBA-0083 / ARR-0026 (pending from the read-only review, §12(6)) and this cycle's own identifiers; complete the Revision-3 §E model-comment currency inventory by adding `src/model/derivation.rs` (ARR-0026 M-1). No normative §C change; Revision 3 remains PROPOSED; independent re-review required before acceptance. Terminates under §12(7). |
| Slice Contract Revision 3 re-review | PBA-0085 | ARR-0027 | Read-only independent re-review of the M-1 remediation (no ledger write; recorded by the cycle below, per §12(6)). Verdict PASS — M-1 CLOSED; READY FOR ACCEPTANCE. |
| **Slice Contract Revision 3 acceptance** | **PBA-0086** | **PBR-0015 / ER-0078** | Record PBA-0085 / ARR-0027 (pending from the read-only re-review, §12(6)) and this cycle's own identifiers; **accept Revision 3 under PBR-0015**, superseding Revision 2 in governance provenance (predecessor file preserved immutable). Clerical status transition of `first-runnable-slice-contract-r03.md` only — no substantive §C change, no source/test/dependency/toolchain change. Terminates under §12(7). |
| Revision 3 model-comment currency | PBA-0087 | ER-0079 | Record this cycle's own identifiers; align the closed Phase-1 model doc/test comments in `src/model/{strategy,condition,derivation}.rs` to accepted Revision 3 §C (operative-node lineage → check 15; version → check 16; candidate-derivation correction). **Comment-only** — no executable model semantic change; resolves ARR-0027 O-B. Terminates under §12(7). |
| Canonical occurrence-identity determination | PBA-0088 | ER-0080 | Read-only determination (no ledger write; recorded by the cycle below, per §12(6)). Concludes lineage identity is an insufficient universal defect anchor; SUCCESSOR REVISION REQUIRED. |
| Revision 4 authoring (stopped) | PBA-0089 | ER-0081 | Read-only STOP before any write on two anchoring blockers (`DUPLICATE_LINEAGE_IDENTITY` class-level anchor; COUNTERPART absent-lineage representability). No contract authored (no ledger write; recorded by the cycle below, per §12(6)). |
| Class-level / counterpart identity determination | PBA-0090 | ER-0082 | Read-only integrated determination (no ledger write; recorded by the cycle below, per §12(6)). Disposition REVISION 4 SUFFICIENT. |
| **Slice Contract Revision 4 authoring** | **PBA-0091** | **ER-0083** | Record PBA-0088/ER-0080, PBA-0089/ER-0081, PBA-0090/ER-0082 (pending from the read-only/stopped cycles, §12(6)) and this cycle's own identifiers; author `first-runnable-slice-contract-r04.md` as **PROPOSED** under PBD-0006 (canonical occurrence and integrated defect identity). Revision 3 remains authoritative; no ARR or PBR allocated — no review or acceptance occurred. Terminates under §12(7). |
| Slice Contract Revision 4 review | PBA-0092 | ARR-0028 | Read-only independent review of proposed Revision 4 (no ledger write; recorded by the cycle below, per §12(6)). Verdict FAIL — BOUNDED REMEDIATION REQUIRED (MAJOR M-1, check-11 pairwise anchor). |
| **Slice Contract Revision 4 M-1 remediation** | **PBA-0093** | **ER-0084** | Record PBA-0092 / ARR-0028 (pending from the read-only review, §12(6)) and this cycle's own identifiers; correct the ARR-0028 M-1 check-11 pairwise anchor in `first-runnable-slice-contract-r04.md`, and complete the pre-existing ARR-0027 §7 clerical omission (§3 currency, no assignment changed). Revision 4 remains PROPOSED; independent re-review required before acceptance. Terminates under §12(7). |
| Slice Contract Revision 4 re-review | PBA-0094 | ARR-0029 | Read-only independent post-remediation re-review (no ledger write; recorded by the cycle below, per §12(6)). Verdict PASS — ARR-0028 M-1 CLOSED; ACCEPTANCE-READY. |
| **Slice Contract Revision 4 acceptance** | **PBA-0095** | **PBR-0016 / ER-0085** | Record PBA-0094 / ARR-0029 (pending from the read-only re-review, §12(6)) and this cycle's own identifiers; **accept Revision 4 under PBR-0016**, superseding Revision 3 in governance provenance (predecessor file preserved immutable). Clerical status/metadata transition of `first-runnable-slice-contract-r04.md` only — no substantive §C change, no source/test/dependency/toolchain change. Terminates under §12(7). |
| Phase-1 occurrence-identity remediation | PBA-0096 | ER-0086 | Record this cycle's own identifiers; implement the Revision-4 §C.1 Phase-1 consequence — the `OccurrenceId` opaque domain and a mandatory `occurrence` field on every construct — in `src/model/**`, with focused tests. Source-writing cycle; no Phase-4/`src/checks/**` change; Phase 1 remains CLOSED. Independent reconciliation review required before Phase-4 remediation. Terminates under §12(7). |
| Phase-1 occurrence-identity reconciliation review | PBA-0097 | ARR-0030 | Read-only independent reconciliation review of ER-0086 (no ledger write; recorded by the cycle below, per §12(6)). Verdict PASS — PHASE-1 REMEDIATION RECONCILED. |
| **Phase-4 result-model remediation** | **PBA-0098** | **ER-0087** | Record PBA-0097 / ARR-0030 (pending from the read-only review, §12(6)) and this cycle's own identifiers; migrate the §6.3/§6.4 result model to the Revision-4 occurrence anchor (`DefectCore.offending → OccurrenceId`, COUNTERPART → occurrence, new `LINEAGE` kind 7) in `src/result/**`, with focused tests. Source-writing cycle; no `src/checks/**` change; Phase 4 remains CLOSED. Independent reconciliation review required. Terminates under §12(7). |
| Phase-4 result-model reconciliation review | PBA-0099 | ARR-0031 | Read-only independent reconciliation review of ER-0087 (no ledger write; recorded by the cycle below, per §12(6)). Verdict PASS — PHASE-4 REMEDIATION RECONCILED. |
| **Phase-5A check-1 implementation** | **PBA-0100** | **ER-0088** | Record PBA-0099 / ARR-0031 (pending from the read-only review, §12(6)) and this cycle's own identifiers; implement mandatory check 1 (`MISSING_REQUIRED_PROPERTY`) in `src/checks/required_property.rs` on the reconciled Revision-4 model, with focused tests. Source-writing cycle; defect production only — no other check, gating, verdict, or `src/model`/`src/result` change (lib.rs crate-doc currency only). Independent review required. Terminates under §12(7). |
| Phase-5A check-1 review | PBA-0101 | ARR-0032 | Read-only independent review of check 1 (no ledger write; recorded by the cycle below, per §12(6)). Verdict PASS — CHECK 1 IMPLEMENTATION VERIFIED. |
| Phase-5 roadmap reconciliation | PBA-0102 | ER-0089 | Read-only planning review reconciling the remaining Phase-5 checks into Slices B–H (no ledger write; recorded by the cycle below, per §12(6)). Corrects the check-8 scheduling omission; schedules the O-1 gate before Slice F. |
| **Phase-5 Slice B implementation (checks 15, 16)** | **PBA-0103** | **ER-0090** | Record PBA-0101 / ARR-0032 and PBA-0102 / ER-0089 (pending from the read-only cycles, §12(6)) and this cycle's own identifiers; implement checks 15 and 16 (version half) in `src/checks/{operative_provenance,semantic_model_version}.rs`, with focused tests. Source-writing cycle; defect production only — no other check, gating, verdict, or `src/model`/`src/result` change. Independent review required. Terminates under §12(7). |
| Phase-5 Slice-B review | PBA-0104 | ARR-0033 | Read-only independent review of checks 15/16 (no ledger write; recorded by the cycle below, per §12(6)). Verdict PASS — SLICE B CHECKS 15/16 VERIFIED. |
| **Phase-5 Slice C implementation (check 2)** | **PBA-0105** | **ER-0091** | Record PBA-0104 / ARR-0033 (pending from the read-only review, §12(6)) and this cycle's own identifiers; implement check 2 in `src/checks/reference_integrity.rs`, with focused tests. Source-writing cycle; defect production only — no other check, gating, verdict, or `src/model`/`src/result` change. Independent review required. Terminates under §12(7). |
| Phase-5 Slice-C review | PBA-0106 | ARR-0034 | Read-only independent review of check 2 (no ledger write; recorded by the cycle below, per §12(6)). Verdict FAIL — one MAJOR M-1 (ENUM atom overlap population). |
| Check-2 ENUM overlap determination | PBA-0107 | ER-0092 | Read-only Product Board determination of the M-1 overlap population (no ledger write; recorded by the cycle below, per §12(6)). RULING A — co-occurrence; determination sufficient. |
| **Phase-5 Slice-C M-1 remediation** | **PBA-0108** | **ER-0093** | Record PBA-0106 / ARR-0034 and PBA-0107 / ER-0092 (pending from the read-only cycles, §12(6)) and this cycle's own identifiers; implement ER-0092 RULING A (ENUM atom co-occurrence) in `src/checks/reference_integrity.rs`, with regression tests. Source-writing cycle; local to M-1 — no other check-2 semantic, no `src/model`/`src/result` change. Independent re-review required. Terminates under §12(7). |
| Phase-5 Slice-C M-1 re-review | PBA-0109 | ARR-0035 | Read-only independent re-review of the ER-0093 M-1 remediation (no ledger write; recorded by the cycle below, per §12(6)). Verdict PASS — SLICE-C M-1 CLOSED; CHECK 2 VERIFIED. |
| **Phase-5 Slice D implementation (checks 8, 10)** | **PBA-0110** | **ER-0094** | Record PBA-0109 / ARR-0035 (pending from the read-only re-review, §12(6)) and this cycle's own identifiers; implement checks 8 and 10 in `src/checks/{direction_coverage,direction_determinism}.rs`, with focused tests. Source-writing cycle; defect production only — no other check, gating, verdict, or `src/model`/`src/result` change. Independent review required. Terminates under §12(7). |
| **Slice-D attribution-trailer removal** | **PBA-0111** | **ER-0095** | Record this cycle's own identifiers; remove the prohibited `Co-Authored-By` trailer from the Slice-D implementation commit via metadata-only `git commit --amend` (`f592bb9` → `b704949`), tree and parent identity preserved, then `git push --force-with-lease`. Metadata/history remediation only — no source, test, or accepted-document change; ER-0094 Slice-D semantics unchanged. This governance-synchronization commit is itself trailer-free. Terminates under §12(7). |
| Phase-5 Slice-D review | PBA-0112 | ARR-0036 | Read-only independent review of checks 8/10 (no ledger write; recorded by the cycle below, per §12(6)). Verdict PASS — SLICE D CHECKS 8/10 VERIFIED. |
| **Phase-5 Slice E implementation (checks 3, 4, 5)** | **PBA-0113** | **ER-0096** | Record PBA-0112 / ARR-0036 (pending from the read-only review, §12(6)) and this cycle's own identifiers; implement checks 3, 4, 5 in `src/checks/{condition_classification,configuration_bounds,configuration_position}.rs`, with focused tests. Source-writing cycle; defect production only — no other check, gating, verdict, or `src/model`/`src/result` change. Independent review required. Terminates under §12(7). |

**Every identifier in the table above is recorded as an ordinary ALLOCATED entry in
§5 and §8**, including each cycle's own. They are additionally listed here so their
**administrative character** is explicit and so a later reader does not mistake
them for product, architecture, or engineering authorizations.

**Termination.** Recording a synchronization cycle's own identifiers **does not
create an obligation to run another synchronization cycle.** That is the whole
point of this section: the loop stops here, by rule, not by omission. See §12(7).

## 12. Maintenance — the Record-at-Issue Procedure

Every future identifier in an allocated namespace is recorded here **at issue**, not
retrospectively. The procedure below makes that rule operational; it is a
clarification of the existing rule, **not** a new artifact-numbering policy.

1. **Responsibility.** The **Product Board**, or the **Board-authorized executor
   acting for the Board**, is responsible for ensuring that a newly issued
   governance identifier is recorded in this canonical ledger. The obligation
   travels with the authorization; it is never nobody's.
2. **Consult before issuing.** Allocation **MUST** consult the current ledger
   before an identifier is issued. An identifier is never chosen from memory,
   from `max + 1`, or from the apparent shape of a sequence.
3. **Eligibility.** Only an identifier currently eligible under the
   **AVAILABLE-FUTURE** rule (§4) may be allocated. ALLOCATED, RESERVED,
   SUPERSEDED-ID, and UNKNOWN-HISTORICAL identifiers are never reused, and §11.1
   "Pending synchronization" entries are never treated as available.
4. **When to record.** The allocation is recorded **in the same authorized
   governance cycle in which the identifier is issued** — or, where that cycle's
   issuing action is itself read-only, **at the first repository-writing step
   authorized for that cycle**.
5. **Verify before the cycle completes.** The executor confirms that: the
   identifier appears **exactly once** as an active allocation; the namespace
   **high-water mark** is correct; the **next-allocatable** value is correct; and
   **no existing historical assignment was displaced or reused**.
6. **Read-only reviewers do not mutate this ledger.** A reviewer is neither
   required nor permitted to edit the ledger merely because its own authorization
   or report consumes identifiers. Those allocations are carried in §11.1 and
   **must be synchronized by the next authorized repository-writing action, before
   any baseline or publication action.**

7. **Termination rule for ledger-only cycles (baseline freeze).** A governance
   cycle whose **sole purpose** is synchronizing this ledger records **its own**
   authorization and report identifiers **within its own edit**, as ordinary
   ALLOCATED entries, and advances the high-water and next-allocatable values to
   match. Having done so, it is **complete**: recording a synchronization cycle's
   own identifiers **never** obliges a further synchronization cycle. This
   distinguishes (A) identifiers of governance artifacts whose existence must be
   recorded here, from (B) the authorization/report pair of a purely clerical
   ledger-synchronization cycle — both are recorded, but only (A) can generate
   further governance work. This is a termination rule for the pre-baseline
   procedure; it changes no historical numbering policy and creates no exception to
   §12(2) or §12(3).

**Corrections to history.** Any correction to a historical assignment requires an
explicit Product Board determination citing §3(a) or §3(b). Both the superseded and
the corrected identifier are retained in the tables above so the history stays
legible.
