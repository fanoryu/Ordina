# Ordina — Operating Instructions

> **Tier 5 — non-normative. Pointers only.**
> This file contains **no product rules**. Copying rules here causes them to
> drift silently out of date. Read the source documents.

## Read before acting, in precedence order

1. `docs/product/constitution.md` — product identity, principles, constraints,
   boundary test. **Currently v1.0 ACTIVE — RATIFIED (POR-0001).**
2. `docs/product/trading-decision-model.md` — the decision lifecycle.
   `docs/product/validation-model.md` — validation dimensions, statuses, status
   resolution.
   `docs/product/ux-constitution.md` — interface constraints.
   `docs/product/user-zero.md` — the initial user.
   `docs/product/system-context.md` — the external product boundary.
3. `docs/engineering/constitution.md` — engineering invariants.
4. `docs/adr/`, `docs/rfc/` — decisions and the debates behind them.

Also: `docs/product/open-questions.md`, `docs/product/risk-register.md`,
`docs/engineering/workflow.md`, `docs/glossary.md`.

## Hard stops

1. **No production code** without explicit approval. Currently foundation phase.
2. **Do not begin RFC-0001** or domain-model work without approval.
3. **Do not initialize git** without approval.
4. **Do not choose** a database, persistence model, language, framework,
   deployment model, broker, or market-data provider. Each requires an RFC
   and an ADR.
5. **Never skip to implementation.** Discussion → RFC → Decision → ADR → Plan →
   Implementation → Review → Docs.
6. **Do not independently ratify or accept any document.**
   Ratification and acceptance decisions are reserved to the human Product Owner / Product Board.

   Claude MAY mechanically record and synchronize a ratification or acceptance decision only when the human Product Owner has explicitly made that decision in the current conversation.

   In that case, Claude may update document status, governance ledger, roadmap/status records, allocators, and related provenance, and may commit/push those synchronization changes when explicitly authorized.

   Claude MUST NOT:
   - infer acceptance from review readiness or a PASS review;
   - self-authorize ratification or acceptance;
   - change PROPOSED to ACCEPTED without an explicit human Product Owner decision;
   - resolve outstanding findings or Product Board dispositions on behalf of the Product Owner;
   - create an ADR merely to ratify or amend the Product Constitution.

   Ratification remains product governance, not an architecture decision (Constitution §9).
7. **The Constitution is v1.0 ACTIVE — RATIFIED (POR-0001).** It is now binding.
   Changes to product identity, boundaries, principles, or C1–C10 require the
   formal amendment process with explicit Product Owner sign-off; ADRs cannot
   amend product identity. Lower-tier documents were **not** auto-promoted.

## Behaviour

- Apply the Constitution §2.1 boundary test to every proposed feature. **Report
  conflicts; never implement around them.**
- Do not add requirements that were not decided. If something is unstated, raise
  it as an open question rather than inventing an answer.
- Use `docs/glossary.md` vocabulary exactly. Never introduce forbidden terms into
  code, UI copy, or internal naming.
- Challenge weak ideas. Agreement without reasoning is a failure.
- Prefer boring designs.
- Docs change with code, in the same commit.
- ADRs are immutable once accepted — supersede, never edit.
