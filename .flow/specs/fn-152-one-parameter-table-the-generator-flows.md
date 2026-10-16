# One parameter table the generator flows through

## Goal

Every generator parameter is declared once, and everything that describes it is generated from that declaration. Before this spec a row's path, meaning, bounds, validation, blending, tuning dial and docs lived in six hand-kept places that drifted (the palm's tuning config went stale on one). This is the first of four stacked specs (fn-152 catalogue, fn-158 stage inputs, fn-159 consumers, fn-160 value files); it builds the catalogue and moves its first consumers onto it with no change in output.

Owner quotes:
> (2026-09-24) "I think we might have to also have a streamlining of the generator on how it works with the params ... params define which others they depend on and then make the algorithm flow cleanly through param's dependency tree like the generator pipeline on a high level does"
> (2026-09-26) "ok that seems reasonable. /flow-next:flow this to its completion in one stack with pr's for each spec"

## Decisions

- The read map (2026-09-25) found 249 wire rows defined across the `fields!` macro, doc comments, 15 validate functions, `blend.rs` (248 rows by hand), a 227-row `dials.json` copy and the harness, with conflicting bounds (`stations_per_internode` 1..32 vs 1..64; `shell_depth` checked three times) and three rows no production code reads (`canopy.spacing`, `clump`, `clumpSpan`). One pass would not fit one task.
- Astra's review: keep a catalogue and typed stage inputs, drop the graph resolver; derived values live at four lifetimes (family, grown tree, branch, surface sample) that one up-front resolve cannot reach. Dependencies become documentation on each entry, not an execution engine.
- The `fields!` macro becomes typed declarations emitting the family fields and one catalogue entry per row: path, type, unit, bounds, default, meaning, consumer set (grow, plan, expand, cull, draw; growth-path applicability separate), applicability, deprecated flag. No proc macro, no runtime graph.
- Generated: scalar bounds validation (scope and error order preserved), ordinary per-row blending, dial metadata, a parameter reference in `docs/`. Relational validation and coupled blending stay named Rust functions.
- Kept on purpose: conflicting bounds listed, not resolved; the three unread rows deprecated, hidden from controls, serialised unchanged; the capability classifier corrected for the leaf bases' shared use of the rosette's spiral rows.
- New tuning configs reference dial ids plus overrides and a catalogue revision; existing snapshots replay.

## Acceptance Criteria

- **R1:** One declaration and catalogue entry per wire row; wire schema, JSON names and TS types unchanged; a row without meaning, bounds or consumer set fails at build time — met (compile-time asserts in `catalogue.rs`).
- **R2:** Validation, blending, dial metadata and the reference generated; presets, intermediate blends and override fixtures byte-identical; error order kept — met (master `dials.json` at `849fa4b1`: 224 dials on 12 fields, 0 differing).
- **R3:** New configs use dial ids + overrides + catalogue revision; an embedded-dial config still replays — met.
- **R4:** The three unread rows deprecated and hidden, still parse and serialise — met.
- **R5:** Gate and `npm test` green — met.

## Result

Merged in PR #122 (`d8368b0b`).

## Open

None here; fn-158, fn-159, fn-160 continue the stack.

## Friction

- Building the read map before sizing took four agents and ~670k tokens -> answer a spec's Unknown during planning; a generated row-to-reader index.
- The size budget surfaced only on CI's package job after both gates were green (~10 min) -> run `artifact-budgets.mjs` in the local `npm test` path.

## Evidence

- `.flow/evidence/fn-152-one-parameter-table-the-generator-flows/READ-MAP.md` (row-to-reader map and the four-spec split)
- `.flow/evidence/fn-152-one-parameter-table-the-generator-flows/ASTRA-REVIEW.md` (design review; referenced by `docs/pipeline.md`)
