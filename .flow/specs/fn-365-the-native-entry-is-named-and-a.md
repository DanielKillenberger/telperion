# The native entry is named and a headless test grows the oak through it

Child of fn-351 (P10; Phase 6). Host-written, 14 Nov 2026. Builds on `fn-364-a-tip-extends-by-what-its-balance-leaves`. Survey: `.flow/evidence/fn-351-020-the-release-candidate/PHASE6-SURVEY.md`, section 5.

## Problem

P10 (fn-351): "a native entry with no browser, harness or engine dependency takes parameters and a seed, never a species name, and returns the tree's data at any age; a headless test grows the oak through it. No plugin is built." The survey found the entry exists in substance (`telperion-core`'s `pipeline::build` and `pipeline::growth`, dependencies `telperion-space`, `slotmap`, and `serde` under `json`) and that nothing names it as the entry, no document states its contract, and no test states P10's sentence.

## The design (host)

- **The entry is what exists**: `telperion_core::pipeline::growth` (`Params::of`, `begin(&params, seed, horizon)`, `grow_to`, `settle`, `mesh`) for a tree held between years, and `pipeline::build(&Family, Request)` for one build. No new crate, no facade, no C ABI, no plugin. The seed and the age are arguments where `growth` takes them and rows of the family where `build` reads them; both stay, and the document says which is which.
- **Parameters come as data.** The test reads the oak as a parameter table through `params::parse` (the `json` feature), the way an engine would hand them over; it resolves no preset id and no species name. Where the parameter text comes from in the test is the builder's reading (a fixture written by the existing preset writer, or the preset rendered to JSON in the test's setup through a function that is not the entry); say which and why.
- **One document**, `docs/native-entry.md`, short: what an engine links (the crate and its features), the calls in order, what comes back (structure, curve, chunks, leaf instances, field, by `Request`), what is promised (a year is a state; the same tree on native and Wasm; a tree past the machine is refused by name) with the test that holds each, and what is not built (a plugin; twigs for a region asked, which fn-351's P10 does not carry). Linked from `docs/pipeline.md`.
- "Twigs for any region asked" (fn-321 R11) is out of this spec: fn-351's P10 dropped the clause and nothing builds wood on demand today.

## Acceptance Criteria

- **R1.** A reading in the evidence first: the public items of `pipeline::growth` and `pipeline::build` an outside crate can reach today, what `cargo tree -p telperion-core --no-default-features --features json` lists (run it; it is light), and anything on that surface that takes a species name or a preset id.
- **R2.** One test in `crates/telperion-core/tests/`, named for P10's sentence, with no GPU, browser or harness: the oak's parameters parsed from data, a seed given as an argument, grown through `pipeline::growth` to 20, then on to 40; at each age the settled tree and its mesh are returned and checked for what an engine reads (a non-empty structure, wood and leaf instances, bounds that contain every node, height at 40 above height at 20); the tree at 20 reached on the way to 40 equals the tree grown to 20 alone, to the bit, by the crate's existing digest. Under the gate's budgets (120 s, 4 GiB) without a budgets entry.
- **R3.** `docs/native-entry.md` as above; every promise in it names its test.
- **R4.** If R1 finds a name or id on the entry's surface inside `telperion-core`, return with it; do not remove it here. (The Wasm binding's `preset(id)` and `catalogue()` exports belong to the size spec.)
- **R5.** Narrow tests only while building; the two sub-second guards before finishing (`generation_limit_guard`, `compile_size`).

## Stops

- The entry cannot be called from an outside crate without making a private stage public (the stages are private by design, `docs/pipeline.md`): return with what is missing.
- Any question about what an engine's contract should contain beyond what exists.

## Boundaries

A test, a document, at most visibility of items already meant as the door. No change to the grower, the pipeline's stages, the binding or any preset.

## Rigor

Opus builds. Heavy runs through `telperion-heavy` with `TELPERION_HEAVY_RESERVE=8` in the background, checked by the clock; the yield watcher may freeze them. Friction in this spec's FRICTION.md as it happens.
