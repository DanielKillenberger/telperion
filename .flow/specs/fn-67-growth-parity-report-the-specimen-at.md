# Growth parity report: the specimen at age against the mature build

## Goal

With growth hidden since fn-65, the owner worried that un-hiding it later would cost a second tuning per species. It does not if one rule holds: growth reproduces the mature tree from the same value table, so at the preset's age the specimen lands on the direct build (the birch at seed 1 was 73,337 nodes direct against 590,410 grown to 100). This spec was to add a reported, never-gating growth-parity row per species and seed, as the scoreboard for fn-31's fix.

## Decisions

- user (2026-09-18): "if we have like 2000 trees (i'm exaggerating) and we're trying to then properly fix the growth algo won't that be much harder than having it now and configuring each tree to be growable properly?"
- user (2026-09-18): "ok do it"
- The row is a report, never a gate without the owner's word; no change to the growth algorithm, direct build, presets or pins; after fn-58.
- Closed 2026-10-02 without being built: fn-181 removes the growth path (owner, 2026-09-27: "The implementation was never good enough anyway. We'll need some different approach"). Its premise, a grown specimen that must land on the direct build at the preset's age, is the first lesson in `docs/growth-path.md` and the rule a rewrite starts from.

## Acceptance Criteria

- **R1:** species measurement example emits one growth-parity row per preset and seed (or the resource limit hit) — dropped (growth path removed).
- **R2:** `species:qa` writes the row into the protocol summary and a per-spec TSV, exit status unchanged — dropped.
- **R3:** unit test that exit status ignores the row; CLAUDE.md rule against gating on it — dropped.
- **R4:** fn-31 carries the convergence rule as its first acceptance criterion — dropped; the rule lives in `docs/growth-path.md`.

## Result

Not built; closed by fn-181.

## Open

None.
