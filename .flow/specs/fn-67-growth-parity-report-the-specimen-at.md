# Growth parity report: the specimen at age against the mature build

## Conversation Evidence

> user (2026-09-18): "if we have like 2000 trees (i'm exaggerating) and we're trying to then properly fix the growth algo won't that be much harder than having it now and configuring each tree to be growable properly?"
> user (2026-09-18): "ok do it"

## Goal & Context
<!-- scope: business -->

Growth over time is hidden since fn-65 and the mature tree is the product. The owner's worry is the retrofit: when growth is un-hidden with many species tuned on the mature path, making each one growable could cost a second tuning per species. It does not have to, if one rule holds from now on: growth reproduces the mature tree from the same value table, so at the preset's age the specimen lands on the direct build. Today it does not. The silver birch at seed 1 is 73,337 nodes on the direct build and 590,410 on the specimen grown to age 100. That is one generator defect, not a per-species setting, and every species tuned on the mature path is a regression target for fixing it, provided the gap is measured. [paraphrase]

This spec adds the measurement and nothing else. Every species and seed the protocol runs gets one row that says how far its specimen at the preset's age sits from its direct build. The row is reported, never gating: it shows the debt without blocking a species. The fix itself is fn-31's when it resumes, with this report as its scoreboard. [user]

## Architecture & Data Models
<!-- scope: technical -->

- **One measurement, two paths.** For a preset and a seed, the species measurement example builds the tree directly and reads the specimen at the preset's age through the existing growth machinery, and measures both with the metrics the protocol already uses: node count, primary limbs, branches by order, twigs, placed leaves, leaf area, height, crown box, occupied share. The row is the two readings and their ratio per metric. No new metric is invented. [inferred]
- **Reported, never gating.** The row goes into the protocol summary under a `growth_parity` key and into a per-species table in the evidence directory. `species:qa` exits exactly as before whatever the row says. A CLAUDE.md rule forbids turning it into a gate without the owner's word; a species gate on growth is fn-31's decision when growth returns. [user]
- **The convergence rule is written down** in fn-31's spec as its first acceptance criterion on resume: at the preset's age the specimen matches the direct build within a stated tolerance per metric, on every catalogue species, from the same value table. Growth never gets rows of its own that the mature build does not read. [paraphrase]
- **Cost is bounded.** The specimen read is the expensive half, seconds per species at the birch's size. The protocol runs it once per species and seed at the preset's age only, and fn-53's no-caps rule applies: a species whose specimen hits a resource limit records the limit in the row instead of a number. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The species measurement example takes a preset and a seed and emits one growth-parity row: direct-build and specimen-at-age readings for each protocol metric and their ratio, or the resource limit the specimen hit. [inferred] Errors: a preset with no growth-reference entry records `no growth reference` and no specimen reading.
- **R2:** `species:qa` writes the row for every species and seed it runs into the protocol summary under `growth_parity` and into `.flow/evidence/<spec>/growth-parity.tsv`, and its exit status is unchanged by the row. [user] Errors: a failed specimen read is a row with the error, never a failed run.
- **R3:** A unit test asserts the runner's exit status does not depend on the parity row, and CLAUDE.md carries the rule that the row never gates without the owner's word. [inferred]
- **R4:** fn-31's spec carries the convergence rule as its first acceptance criterion on resume, stated with a per-metric tolerance the owner sets. [user]

## Boundaries
<!-- scope: business -->

- No change to the growth algorithm, the direct build, any preset or any pin. [user]
- No gate. The row is a report. [user]
- Not before fn-58 lands, since fn-58 reshapes the protocol runner this hooks into. [paraphrase]

## Decision Context
<!-- scope: both -->

The owner chose to hide growth rather than fix it now, and asked whether that makes the fix harder later. It does only if growth becomes a second generator with its own tuning. Measuring the gap per species from now on keeps the fix one problem with many regression targets instead of a retune per tree. [paraphrase]
