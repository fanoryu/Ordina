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
| PBA-0064 and above | AVAILABLE-FUTURE | — |

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

**Next allocatable PBA: `PBA-0064`.**

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
| PBR-0015 and above | AVAILABLE-FUTURE | — |

**Collision resolved (PBD-0005 §4).** `PBR-0010` was used twice: for the Slice
Contract Final Review Authorization that produced ARR-0007, and later for the Slice
Contract acceptance. The **Final Review Authorization has priority**; the
acceptance is reassigned **PBR-0011**. **The acceptance decision itself is
unchanged** — decision date 2026-08-13, contract status ACCEPTED, architecture and
engineering substance untouched. Only the governance identifier is corrected.

**Next allocatable PBR: `PBR-0015`.**

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
| ARR-0021 and above | AVAILABLE-FUTURE | — |

Two review reports were issued under **temporary, deliberately unnumbered**
identifiers by Board instruction: the *Pre-Git Governance & Repository Integrity
Audit* and the *Governance Identifier Collision Reconciliation Report*. Neither
consumes an ARR number.

**Next allocatable ARR: `ARR-0021`.**

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
| ER-0063 and above | AVAILABLE-FUTURE | — |

**Next allocatable ER: `ER-0063`.**

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
| PBA | PBA-0063 | **PBA-0064** |
| PBR | PBR-0014 | **PBR-0015** |
| ARR | ARR-0020 | **ARR-0021** |
| ER | ER-0062 | **ER-0063** |
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
