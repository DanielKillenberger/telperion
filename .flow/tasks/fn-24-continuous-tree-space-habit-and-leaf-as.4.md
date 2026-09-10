---
satisfies: [R1, R5]
---
# fn-24-continuous-tree-space-habit-and-leaf-as.4 The flat habit on the wire, the panel and the tests

## Description
Restore the TypeScript surface to the flat trait shape after tasks 1, 2 and 3 remove the three enums from the core: regenerate the preset metadata, drop the kind label from the panel so every trait renders through the generic number-field mapping (R5), and rewrite the tests and probes that assert the old wire.

**Size:** S
**Files:** `scripts/build-wasm.mjs`, `src/browser/presets.generated.ts` (regenerated), `harness/GrowerDev.tsx`, `harness/params.ts`, `harness/params.test.ts`, `tests/browser/bindings.mjs`, `README.md`, `package.json`
**Touches:** [scripts/build-wasm.mjs, src/browser/presets.generated.ts, harness/**, tests/browser/bindings.mjs, README.md, package.json]

### Approach
- Remove the tagged-union branch (`'kind' in value`) from the type inference in `scripts/build-wasm.mjs` around line 15; flat numeric objects already infer generically. Regenerate `src/browser/presets.generated.ts` with `npm run wasm:build`.
- Delete the label line at `harness/GrowerDev.tsx:306`; the generic mapping at lines 307-315 already renders every numeric habit field, so extend the same mapping to the element and canopy trait objects instead of hand-writing inputs.
- Update `harness/params.test.ts:107` to assert the flat habit shape and the default rows; update the invalid-input probe at `tests/browser/bindings.mjs:143` so any `kind`, anatomy or attachment tag is rejected naming the field.
- Docs that go stale with the enums: the architecture paragraph at `README.md:60` and ownership table at `README.md:52`, and the description at `package.json:5`, now describe one builder from continuous traits with attractor pull as a trait.

### Investigation targets
**Required** (read before coding):
- `harness/GrowerDev.tsx:300-330` — the label line and the generic numeric mapping to reuse
- `scripts/build-wasm.mjs:1-33` — metadata to TypeScript generation
- `harness/params.test.ts:100-115` — the default-habit assertion

**Optional** (reference as needed):
- `tests/browser/bindings.mjs:130-150` — the invalid-input probe
- `README.md:45-100` — architecture and headless sections

## Acceptance
- [ ] `presets.generated.ts` carries flat numeric habit, element and canopy trait fields and no `kind`, `anatomy` or `attachment` string
- [ ] The panel shows every habit, element and attachment trait as a numeric control on every preset with no kind or anatomy label; no renderer file changed
- [ ] Harness tests and the browser binding probe pass against the flat shape; the probe rejects a stale tag naming the field
- [ ] README architecture prose and the package description describe one builder from continuous traits
- [ ] `npm run wasm:build && npm test` and `npm run typecheck` pass

## Done summary
The TypeScript surface speaks the flat trait shape again: the metadata generator
lost its tagged-variant and string-tag branches and regenerated
`src/browser/presets.generated.ts` with fifteen numeric habit rows, the element's
lobe and section traits and the canopy's lean and contact traits and no `kind`,
`anatomy` or `attachment` string in it; the panel retired its habit-and-anatomy
label for one `Traits` component that renders every numeric term of the habit,
the element and the canopy as a control; and the harness tests, the browser
binding probe and the architecture prose were rewritten against that shape.

stage: impl-review - skipped(policy: parallel wave - the conductor reviews after
it integrates this commit)

### What the regenerated metadata looks like

One preset's habit block, the Ordinary row, as `presets.generated.ts` now
carries it - flat, numeric, no tag:

    "habit": { "apicalDominance": 0.5, "attractorWeight": 1, "crookedness": 12,
      "lateralLengthRatio": 0.4, "lateralOrders": 3, "lateralPitch": 60,
      "lateralSpacing": 0.9, "lateralsPerStation": 3, "leaderInternode": 1.5,
      "pitchVariation": 15, "risePrimary": 0.05, "riseSecondary": 0,
      "sheddingThreshold": 0.45, "twigTipTaper": 1, "whorlStrength": 0.3 }

`grep -n "kind\|anatomy\|attachment" src/browser/presets.generated.ts` returns
nothing. `Family` gained `element.lobeCount/lobeDepth/sectionRoundness` and
`canopy.forwardLean/leanRise/surfaceContact` and lost the two string unions.

Both branches in `scripts/build-wasm.mjs` went, not only the tagged one: with no
enum left, a string field would type as `string` through the scalar branch, and
the union printer only ever existed for the three tags.

### The panel

`harness/GrowerDev.tsx:306` - the `{habit.kind} habit · {element.anatomy}` line -
is gone, and so are the three hand-written foliage inputs. In their place one
`Traits` component renders every numeric term of a family object as a number
control, invoked three times: the habit, the element (prefix "foliage") and the
canopy (prefix "leaf"). A trait the core adds to any of those objects now appears
under the owner's hand with no panel edit at all.

The canopy is the one that needs a filter: ten of its terms are already sliders
and `toCanopyParams` overwrites them on the next build, so a second control there
would be a dial that does nothing. `CANOPY_FROM_SLIDERS` in `harness/family.ts`
names them, `CANOPY_SKIPPED` adds `maxInstances` (a resource limit, not a trait),
and a new test in `harness/family.test.ts` drives sentinel values through
`toCanopyParams` and asserts the set of overwritten keys is exactly
`CANOPY_FROM_SLIDERS` - so the panel's filter and the translation cannot drift
apart. That test was confirmed red first (drop `scatter` from the set and it
fails naming `scatter`).

No renderer file was touched. `tests/browser/render.mjs` only ever locates
`.gd-note` by "wood tris" and "scene draws", so the deleted note breaks nothing
there.

### The tests and the probe

- `harness/params.test.ts` asserts the Ordinary habit row written out in full,
  the element's `lobeCount/lobeDepth/sectionRoundness` and the canopy's
  `forwardLean/leanRise/surfaceContact`, and that neither object carries an
  `anatomy` or `attachment` property.
- `tests/browser/bindings.mjs`: `compactSpeciesFixture` lost its `tiered` and
  `spreading` branches and shrinks nothing per species now - a 4 m envelope and
  40 attractors is the whole of it. Measured through the wasm C ABI: the oak
  fixture builds 236 nodes / 63 leaves, the spruce 1540 / 5509, both complete
  well under the 12000 cap, so no replacement knob was needed.
- The refusal list is now `[expected message, mutation]` pairs and `rejects`
  takes an optional fragment, so each probe asserts the core's own words:
  `unknown element trait` (an `anatomy` tag), `unknown canopy trait` (an
  `attachment` tag), `unknown habit trait` (a `{kind}` object), `crookedness` (a
  trait outside its range), `foliage connector length`, `leaf card carries no
  lobes and no section roundness`, and `parameter type or range`. A probe that
  expects one field's complaint can no longer pass on another field's.
- The retired `stationsPerInternode = 2` mutation is replaced by the
  out-of-range trait; task 3 removed the one-station-per-internode guard.

### One thing that had to change beyond the task's list

`attractors = 0` is no longer an empty tree - it is an invalid input, refused as
`attractor weight and attractor count` on any family whose pull is positive, and
with the pull at zero the one builder still grows a full rule-built tree (26,555
nodes on Ordinary). The binding suite used it twice as its empty case, so both
are now the node ceiling at zero: `empty.skeleton.growth.maxNodes = 0` and the
raw request `{"family":{"skeleton":{"growth":{"maxNodes":0}}},...}`. The
`zeroNodes` clone that set the same field a few lines later collapsed into it.

### Gates, in the workspace only

- `npm run wasm:build && npm test` - 65 passed, 3 files.
- `npm run typecheck` - clean.
- `npm run rust:test:wasm` - the whole browser binding suite green in headless
  Chromium; this is the real check on the probe rewrite above, and it needs no
  adapter. Not `npm run test:render`, which task 6 owns.
- Baseline before any edit: red, and red for exactly the reason this task exists
  - `harness/params.test.ts` asserted `habit == {kind:"colonizing"}` and
  typecheck failed on `habit.kind` / `element.anatomy` in the panel.
- `cargo test --release --workspace` was not run: the diff carries no Rust byte,
  and tasks 1-3 own the core suite.

Nothing under `crates/` was touched, and nothing outside the workspace was
written.

### Integration and the host review (conductor, 2026-09-10)

Cherry-picked onto the spec branch as bd0c8a2; the workspace commit 396af83 is retired with the worktree. Host review: the generator's two enum branches are gone, the panel renders every numeric trait of the habit, element and canopy through one generic component held to the canopy sliders by a test, the binding probe asserts the core's own refusal words, and no renderer file changed. Regenerating the metadata on the integrated branch reproduced the committed file byte for byte. The panel has not yet been opened in a browser; task 6 or the next dev session covers that.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: bd0c8a2
- Tests: worker, workspace: npm run wasm:build then npm test - 65 passed, 3 files, worker, workspace: npm run typecheck - clean, worker, workspace: npm run rust:test:wasm - browser binding suite green in headless Chromium, conductor, integrated target bd0c8a2: npm run wasm:build, npm test, npm run typecheck - green, rc=0, regenerated presets.generated.ts byte-identical to the commit
- PRs: