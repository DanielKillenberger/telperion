# Astra's review of fn-152, 2026-09-25 (summary)

`gpt-6-astra` at high effort, read-only, against master (d6a3405c), the pre-split spec and `READ-MAP.md`. The original report file was lost when the session's scratchpad was cleared. This summary preserves its verdict, findings and recommendations with their file references.

**Verdict:** keep the single parameter catalogue and typed stage inputs, but reject a universal dependency-graph resolver. The code derives values at several lifetimes: family, generated tree, branch and surface sample. One resolve from `Family` cannot replace those.

## Findings

1. **High: `resolve(&Family)` cannot compute every derived value.**
   - `internodes()` needs each branch's radius and length (`twigs.rs:242`).
   - Curtains use position and the height of the ancestor's tip (`branching/local/pendant.rs:88`).
   - Surface sampling reads generated nodes (`surface/samples.rs:12`).
   - Requests skip outputs nobody consumes (`pipeline.rs:43`).

   *Change:* typed constructors at the lifetime where their inputs exist. Record dependency relationships for explanation only.
2. **High: `pub(crate)` does not enforce "no second chain".** Core-internal chains stay possible, public GPU exception functions stay callable, and a public test feature reopens access. GPU preparation orchestrates stages itself (`telperion-render/src/generation/preparation.rs:14`).

   *Change:* design the executor boundary first; keep stages in private pipeline modules; expose a minimal prepared-artifact interface; keep a compile-fail test with a positive control.
3. **High: removing the three unread rows is an API migration.** They are exported TypeScript properties (`src/browser/presets.generated.ts:8`), and `parse` and `overlay` reject unknown keys (`params.rs:297`, `:337`).

   *Change:* keep them as deprecated compatibility rows; remove them later through an explicit package compatibility change.
4. **High: the `sheddingThreshold` split is semantically right, but it is a compatibility change.** It is a shell depth on the direct build (`branching.rs:300`) and a vigour threshold on the growth path (`specimen/survival.rs:13`).

   *Change:* its own change, with a legacy-copy and precedence contract.
5. **High: one "gate" cannot express everything a row needs.** A row needs a set of consumers, plus growth-path applicability and validation scope. Leaf bases share the rosette's spiral rows (`leaf_bases.rs:103`), which the capability classifier contradicts (`capability.rs:244`). The dial test only checks for the word "zero" (`crates/telperion-jev/tests/dial_table.rs:240`). `clump_system_order` changes field planning on its own (`foliage/plan.rs:221`).
6. **High: validation and blending are not mechanical copies.** `Family::validate` holds contextual rules and builds an element (`family.rs:54`). Blending couples card handling, disabled supernatural terms and optional overrides (`blend.rs:193`).

   *Change:* generate scalar bounds and ordinary interpolation; keep relational validation and coupled blending as named functions; preserve validation scope and error order; add intermediate-blend and `None`/`Some` fixtures.
7. **Medium: generated dials alone leave the drift.** Tuning configs embed full dial copies (`tuning/live.rs:98`). Authored tuning windows differ from generator bounds (`dial_table.rs:19`). Harness density and torsion are transforms, not rows (`harness/family.ts:73`).

   *Change:* configs reference dial ids plus overrides and a catalogue revision, and keep historical snapshots.
8. **Medium: the verification scope misses the exposed consumers.** The GPU packs canopy values to `f32` without leaflet rows (`generation/data.rs:35`), and it overlaps GPU and CPU work (`preparation.rs:85`).

   *Change:* keep arithmetic order and RNG consumption; measure the resident GPU and the CPU separately; reproduce the leaflet defect before writing criteria for it. The `by_identity` discrepancy is confirmed (`None` becomes `Some(35)`, `presets.rs:50`), but it is not yet shown to change a tree.
9. **Low: classify the continuity follow-up.** Parameter discontinuities, count steps and executor fallback are different things (`foliage/prepared.rs:173`). The `killDistance` cap should be documented (`scaffold.rs:214`, `branching.rs:161`).

## Recommended design

- Typed domain parameters, a compile-time catalogue, and ordered typed stage preparation.
- `Family` stays a product of botanical groups.
- The `fields!` macro evolves into typed declarations that emit both the fields and the catalogue entries.
- Build-time generation turns preset value files into typed Rust.
- Dependency metadata explains defaults and dormancy; ordinary Rust functions compute.

## Recommended stages

1. The catalogue plus its first consumers.
2. Typed inputs plus the pipeline boundary, together.
3. Consumer migration.
4. Preset value files, which can precede stage 2.

Separate changes: the shedding split, removal of the deprecated rows, and the demonstrated defects.

**Cut:**
- runtime graph execution and universal dormancy marking;
- a proc macro;
- rewrapping every helper signature;
- public test access;
- automatic pilot-hash re-pinning;
- generating shader layouts;
- separate migrations for views and visibility;
- any claim that table coverage proves reads or smoothness.
