---
satisfies: [R1, R2, R3, R6]
---
# fn-283-a-frond-draws-its-rachis.1 The CPU hangs each frond's axis as wood, with the rachisWidth row

## Description
The proof point: the CPU reference hangs one woody run per frond in the expansion's first step, from the same fronds and stations the leaflets are placed at, behind one new row. Device work follows in .2 and .3; this task leaves the device unchanged (the device build will disagree with the CPU on the palm until .3; record that in the done summary, and keep every CPU=device test that does not build the palm green).

**Size:** M
**Files:** `crates/telperion-core/src/pipeline/foliage/canopy/params.rs`, `crates/telperion-core/src/pipeline/foliage/rosette.rs`, `crates/telperion-core/src/pipeline/foliage/leaflet.rs`, a new `crates/telperion-core/src/pipeline/foliage/rachis.rs` (or `branching/`, whichever keeps `fronds` private to the pipeline), `crates/telperion-core/src/pipeline/branching/leaf_bases.rs`, `crates/telperion-core/src/pipeline.rs`, `crates/telperion-core/presets/date-palm.values`, generated `src/browser/parameters.generated.ts`, `src/browser/presets.generated.ts`, `docs/parameters.md`, tests under `crates/telperion-core/src/suite/`
**Touches:** [crates/telperion-core/**, src/browser/*.generated.ts, docs/parameters.md, catalogue/date-palm/**]

### Approach
- Row: follow `leaf_base_length` in `canopy/params.rs:322-331` (metres, `Bounds::closed(0.0, 10.0)`, `check: input(Site::Rosette, <next>, "rachis width")`, next unused `wire`; read the file for the highest wire in use). `applies`: `rachisLength` zero or `leafletCount` one. Regenerate the TS and docs with the repo's generator (find it from `parameters.generated.ts`'s header). Date palm `/canopy/rachisWidth = 0.25` with the FAO 156 ch. I citation in the comment ("The greatest width of the frond midrib attains 0.5 m, but elsewhere it is only half this size and rapidly narrows from the base upwards").
- Stations: factor the station parameter `t_k` out of `leaflet::leaflets` (`leaflet.rs:50-55`) so the axis and the leaflets read one function; the axis takes `t_k` for k in 0..count, preceded by `t = 0` at `frond.at`.
- Fronds: iterate rosettes exactly as `rosette::clothe` does (`rosette.rs:273-318`): `apex(p, node)`, `Crown::of`, `fronds(&rosette, p, crown, blade)`, `frond_leaflets(p, frond.canopy.leaflet_count, frond_key(node, serial))`, `placement(&frond.canopy, key)` for the count. Factor that walk into one iterator both `rosette::clothe` and the axes read (each frond with its count and drawn table), so the orchestration is not written twice. The blade is `crown::blade(element)`; the element is built from its rows (`executor::element` / `foliage::build_element`), so `clothe` can get it before the plan stage; pass it through `Inputs` rather than building it twice if the plan already holds it.
- Nodes: follow `leaf_bases::hung` (`leaf_bases.rs:233-288`): own `branch` id per run, `kind: NodeKind::Branch`, `stem: false`, `radius` = distal edge, `start_radius` = proximal edge (the previous node's radius), `base_radius` = the run's first. First node parented to the stem node at the frond's depth (reuse `leaf_bases::along`, made shareable, on the stem polyline). Radius per the spec's Architecture section.
- Idempotency: make `clothed` recognise bases plus axes as one tail; `pipeline::clothe` hangs both (`pipeline.rs:206-209`). Callers: `pipeline.rs:221`, `executor.rs:232`, `space/growth.rs:140`, `mesh.rs:79`, `pipeline/tests.rs:272`.
- Rail: check the node total as `clothe_leaf_bases` does (`leaf_bases.rs:199-205`) with its own `ResourceLimit("rachis nodes")`.

### Investigation targets
**Required** (read before coding):
- `crates/telperion-core/src/pipeline/branching/leaf_bases.rs` - the wood-hanging pattern and `clothed`
- `crates/telperion-core/src/pipeline/foliage/rosette.rs:202-318` - `fronds`, `clothe`, `fan`
- `crates/telperion-core/src/pipeline/foliage/leaflet.rs:24-80` - `rachis`, `leaflets`
- `crates/telperion-core/src/pipeline/foliage/crown.rs:186-250` - `segments`, `Split::segment`
- `crates/telperion-core/src/pipeline/surface/curve.rs` - what the drawn curve holds per node (the measure reads it)
**Optional:**
- `crates/telperion-core/src/suite/rosette.rs:195` - per-row refusal test
- `crates/telperion-core/src/suite/leaf_bases.rs` - leaf base tests to mirror
- `.flow/evidence/fn-283-a-frond-draws-its-rachis/raw/walk-probe2.tsv` - base measurement

### Key context
- The leaf bases' `clothed` check requires the tree to END in its bases; appending axes after them without updating it hangs the bases twice on a handed-in tree.
- `validate_solved` refuses radius <= 0; a frond at size 0 has rachis length 0 and is dormant (no run), never a run of coincident nodes.
- The surface builder smooths; the test measures leaflet bases against `tree.curve`, never against node positions. If a node's ring does not land on its station, say so in FRICTION.md and stop with the measurement before changing the sampling.
- No leaflet, stream or draw moves. The leaf box (`foliage/reference.rs:107` `wood_box`, every node padded by the reach) takes the axes as it takes all wood, so the palm's leaf instances shift by the wider box's float32 quantisation only (spec, Architecture). Do not special-case the box.

## Acceptance
- [ ] `rachisWidth` row: bounds 0 to 10 m, refused by its own name outside them; generated TS and docs updated; date palm 0.25 m with its citation.
- [ ] Every shipped preset other than the date palm builds byte-identical meshes to the base (a test or a recorded `space_hash` comparison in the evidence); the palm's leaf instances keep their count, order and draws, and each matrix matches the base's within the float32 quantisation of the wider box (a test against the base's instances, recorded before the change).
- [ ] Test: on the date palm at 40 and at a young age where fronds are split (the age `a_seedlings_fronds_are_entire_and_become_pinnate` uses), every frond leaflet and spine base lies within 1 mm of the drawn curve (R6), and every axis ends at its frond's last station.
- [ ] Test: zero width hangs no axis; a one-leaflet frond and a frond of no rachis length hang none; the axis radius never falls below FINEST and tapers by `t`.
- [ ] Test: clothing a clothed palm leaves it unchanged (bases and axes hung once).
- [ ] The axes' node rail is refused by name past its limit.

## Done summary
The CPU hangs each frond's axis as wood behind the new `rachisWidth` row: one run per frond of two or more leaflets and some rachis length, from its insertion through every leaflet station (`leaflet::station`, one frond walk `rosette::placed` shared with placement), tapering from half the width times the frond's size share, never below FINEST. Bases and axes are one clothing tail, recognised once and bearing no leaf. On a table at 0.25 every frond leaflet and spine base lies within 1 mm of the drawn curve at ages 40 and 12, and the palm keeps its 6,554 leaves to the bit before quantisation. Every shipped preset, the palm included, builds the base's CPU mesh (HASHES.md).

Decision (conductor): clothing tail unbearing. `Tree::clothing` marks where the bases and axes begin; `plan::bearing_of` and placement's slender shoots bear nothing at or past it (cauline 1, shootRadius 0.1: 77,518 leaves at width 0.25 before, 6,554 after).
Decision (conductor): palm value moves to .3. The shipped palm keeps rachisWidth 0; .3 sets 0.25 (FAO 156 ch. I) with the device.

Items for .3: the device mirror of the unbearing tail, and the axis records (the device draws no axis yet; with the palm at 0.25 the device disagrees with the CPU, as boots_tests and agree_tests showed at 1676ca65b).

Gate: `telperion-heavy 8 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast` at 9839dc3d9 green, 1,408 passed, 0 failed, 111 ignored (raw/gate.log). The gate at 1676ca65b was red with the palm at 0.25 (raw/gate-1676ca65b.log): palm tests counted the axes as bases or as stem, and the new row's pins; fixed for each cause, holding at 0 and 0.25.

Follow-ups: leaf_bases.rs and rosette.rs were over the ~400-line rule before this change and grew.

stage: impl-review - ran (codex: round 1 NEEDS_WORK 2 findings fixed; round 2 SHIP; round 3 on the gate fixes SHIP)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 167a1d8b4009617e2f6959ef79aecaae20b1fe8a, 25184cd0eeef1639d1276cff4a2225fe36a9849b, 320469a659befbbad1f20a4de3998aa616bf6edd, de9a4818c1693264ca4d40d51fab39283bc59048, dbc233dc6a19b40b94e65d001f929078a4527780, 1676ca65b4119d63f95737842b699aaa92f79965, d5c940ddcce93588bc42df36c568ae5dbe9b59e8, 9839dc3d912f7b4cbfe1d01a1f36adb4ebea627b
- Tests: telperion-heavy 8 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast, cargo test --profile ci -p telperion-core --lib rachis
- PRs:
## Decision (conductor, 2026-10-20, after the red gate at 1676ca65b)

The shipped date palm keeps `rachisWidth` 0 in this task; the 0.25 m value and its FAO citation land in .3, with the device that draws it, so no CPU=device test is red between tasks. The row, the CPU axes and the clothing rule ship here, proven by tests that set the width on their own table. Every core and jev failure the gate named is fixed for its cause (checked against the base) so that it holds at width 0 and at 0.25.
