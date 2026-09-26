## Conversation Evidence

> owner (2026-09-24): "I think we might have to also have a streamlining of the generator on how it works with the params ... params define which others they depend on and then make the algorithm flow cleanly through param's dependency tree like the generator pipeline on a high level does"
> read map (`.flow/evidence/fn-152-one-parameter-table-the-generator-flows/READ-MAP.md`): one pass does not fit one task; 249 rows are defined across the `fields!` macro, doc comments, 15 validate functions, `blend.rs`, `dials.json` and the harness.
> Astra review (`ASTRA-REVIEW.md` beside it): keep a catalogue and typed stage inputs, and drop the graph resolver; a four-stage split.
> owner (2026-09-26): "ok that seems reasonable. /flow-next:flow this to its completion in one stack with pr's for each spec"

## Goal & Context
<!-- scope: business -->

Every generator parameter is declared once, and everything that describes it is generated from that declaration. Today its path, meaning, bounds, validation, blending, tuning dial and docs live in six hand-kept places that drift apart. The palm's tuning config went stale on one of those copies. This is the first of four stacked specs; it builds the catalogue and moves its first consumers onto it, with no change in output. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

**What exists, checked 2026-09-25 (`READ-MAP.md`).** [checked]
- `params.rs`'s `fields!` macro maps 249 wire paths to fields.
- Meanings live in doc comments. Bounds live in `ranges.rs` and 15 validate functions, with conflicts: `stations_per_internode` is 1..32 in one and 1..64 in another, and `shell_depth` is checked three times.
- `blend.rs` interpolates 248 rows by hand.
- Tuning dials are a 227-row copy in `crates/telperion-jev/data/dials.json`, which configs copy again (`tuning/live.rs:98`).
- Three rows are never read in production: `canopy.spacing`, `clump` and `clumpSpan`.

**The catalogue.** The `fields!` macro evolves into typed declarations, beside their domain docs, that emit both the family fields and one catalogue entry per row. Each entry holds: [paraphrase]
- path and type, unit, bounds and default;
- meaning;
- **consumer set:** grow, plan, expand, cull or draw, with growth-path applicability and validation scope kept separate;
- **applicability:** where the row is dormant, stated descriptively, with "depends on generated structure" allowed;
- a **deprecated** flag.

No proc macro and no runtime graph.

**Generated from the catalogue:** [paraphrase]
- scalar bounds validation, with validation scope and error order preserved;
- ordinary per-row blending;
- dial metadata;
- a parameter reference in `docs/`.

Relational validation and coupled blending (card handling, disabled supernatural terms, optional overrides) stay as named Rust functions.

**Kept on purpose:** [paraphrase]
- **Conflicting bounds** are listed in the reference and left as they are; resolving them is its own change.
- **The three unread rows** are marked deprecated, excluded from controls, and serialised unchanged.
- **The capability classifier** is corrected where it contradicts the leaf bases' shared use of the rosette's spiral rows (`capability.rs:244`).

**Tuning configs.** New configs reference dial ids plus explicit overrides and a catalogue revision. Existing snapshots stay replayable. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** Every wire row has one declaration and one catalogue entry with the fields above. The wire schema, JSON names and exported TypeScript types are unchanged. Errors: a row without a meaning, bounds or consumer set fails a build-time check. [paraphrase]
- **R2:** Scalar validation, ordinary blending, dial metadata and the parameter reference are generated. Every shipped preset, a set of intermediate blends and `None`/`Some` override fixtures are byte-identical to master. Validation keeps its accepted inputs and error order. [paraphrase]
- **R3:** New tuning configs reference dial ids with overrides and a catalogue revision, and an existing embedded-dial config still replays. [paraphrase]
- **R4:** The three unread rows are deprecated and hidden from controls, and still parse, overlay and serialise as before. [paraphrase]
- **R5:** The workspace gate and `npm test` are green. [paraphrase]

## Boundaries
<!-- scope: business -->

- Not stage signatures or visibility (fn-158), consumer migration (fn-159) or preset value files (fn-160). Not the `sheddingThreshold` split, the removal of deprecated rows, or behaviour defects. No output change.

## Decision Context
<!-- scope: both — conditionally substructured -->

The first draft proposed one resolve over a dependency graph, plus per-stage views, in one task. The read map showed that would not fit one task. Astra showed that derived values live at four lifetimes a single up-front resolve cannot reach. Dependencies become documentation on each entry, not an execution engine. [paraphrase]

## Strategy Alignment

- Serves "Our approach": one continuous tree space and lean, explicit data flow. [strategy:Our approach]
