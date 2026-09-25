# One parameter table the generator flows through

## Conversation Evidence

> owner (2026-09-25): "I think we might have to also have a streamlining of the generator on how it works with the params. I can imagine that now it might be quite messy on how the params are applied. If we could write it in a way that params define which others they depend on and then make the algorithm flow cleanly through param's dependency tree like the generator pipeline on a high level does that would be great."
> owner (2026-09-25), on continuity: "I can imagine a multidimensional tree space where if you're on one part of the space some dimensions just don't change the output ... I think part of this might be a param description problem"

## Goal & Context
<!-- scope: business -->

fn-102 made the build one pipeline of named stages. This spec does the same for parameters: one table where every parameter states what it is and how it relates to the others. The generator resolves the family once, in that table's dependency order, and hands each stage only the parameters it reads. Dormancy, derivation, validation, the wire schema, the tuning dials and the harness sliders all come from the one table, so they can no longer drift apart. The date palm's tuning config went stale on a frozen copy of the dial table, and most sliders sat dead on the palm with no word of why. Output does not change: this is structure, measured as no slower. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

**What exists, checked 2026-09-25 on master (39348def).** [checked]
- `crates/telperion-core/src/params.rs` holds the wire table: a macro of 249 rows mapping a JSON path to a family field, which also emits the browser's preset metadata.
- Meanings live in doc comments across modules. Ranges live in `ranges.rs` and 14 `validate` functions.
- Derived values are computed ad hoc in `resolved()` functions (`branching.rs`, `radius.rs`, `twigs.rs`).
- The tuning dials are a separate copy with meanings and ranges transcribed from those comments: `crates/telperion-jev/data/dials.json` (227 rows) and `dials.excluded.json`. The harness has its own `harness/dials.tsx`.
- Stages read family fields directly, for example `pipeline/stage.rs` and `generation/preparation.rs`.

**The table.** One row per parameter, in the core, as data: [inferred]
- its path and type, with its unit, range and default (whole numbers marked as counts);
- its meaning;
- the stage that reads it: grow, plan, expand, cull or draw;
- its **gate**: the condition under which it is dormant, such as side-branch order at 0, stated as data and shown in its description;
- its **derivation**, where its resolved value is computed from others.

The table's edges form an acyclic dependency graph.

**Resolution.** [inferred]
- `resolve(&Family) -> Resolved` walks the graph once, in dependency order. It computes every derived value, folding today's `resolved()` functions in, and marks every dormant parameter.
- Each pipeline stage takes its own view of `Resolved` and never the raw family, so what a stage reads is in its signature.
- A parameter no stage reads is a table error.

**Generated from the table, never copied.** [inferred]
- the wire schema and browser metadata that `params.rs` emits today;
- range validation;
- the tuning dial table, which replaces `dials.json` (exclusions stay as data on the row);
- the harness sliders, with dormant ones shown with their gate;
- a generated parameter reference in `docs/`.

**Presets as data (owner, 2026-09-25).** Each shipped preset moves from its Rust function in `presets/species.rs` to a value file checked against the table's rows, so the species runner's Accept stage (fn-149) writes values, never code. Until this lands, Accept writes the Rust function in today's style. [inferred]

**Unknown.** [unknown]
- Whether every stage can move to views in one pass or needs a staged migration. The implementer maps the reads first and reports.
- Which of today's cross-parameter couplings are hidden, found only by the mapping.

## Acceptance Criteria
<!-- scope: both -->

- **R1:** Every parameter in the wire table has one row with its type, range, meaning, stage, gate and derivation. The graph is acyclic, and no row is unread. [inferred]
- **R2:** Every stage reads only its view of `Resolved`. A guard test fails on a stage that reads the raw family, and fn-151's one-path guard covers it. [inferred]
- **R3:** The wire schema, validation, tuning dial table, harness sliders and parameter reference are generated from the table. `dials.json` and the harness's hand-kept list are gone. Every shipped preset is a value file validated against the table, and fn-149's Accept writes that file. [inferred]
- **R4:** Every shipped preset is byte-identical in mesh, field and metrics. Build time and peak memory are no worse, measured on the budgets fn-151 records. [inferred]
- **R5:** The workspace gate and `npm test` are green. [inferred]

## Boundaries
<!-- scope: business -->

- Not the continuity measure or its fixes (fn-148), which then read gates from this table. Not variation ranges and locks (fn-146), which become fields on these rows. No output change.

## Strategy Alignment

- Serves "Our approach": one continuous tree space and one pipeline with explicit data flow and small interfaces. [strategy:Our approach]
