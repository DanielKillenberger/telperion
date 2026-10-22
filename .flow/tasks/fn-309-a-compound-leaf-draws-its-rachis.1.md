---
satisfies: [R1, R3, R4]
---
# fn-309-a-compound-leaf-draws-its-rachis.1 CPU: a placement on wood stands its leaflets at its point, and the probe is a committed test

## Description
The rule on the CPU and the measurement that proves it. `rosette::fan` learns whether its placement is a frond (its own rachis, `rachis::girth`) or a placement on wood (stations, clusters); `leaflet::leaflets` stands a placement-on-wood's leaflets at `point`, each keeping the heading the fan gives it today. The host's diagnosis probe becomes an ignored core test reporting classes (a) to (e).

**Size:** M
**Files:** `crates/telperion-core/src/pipeline/foliage/{rosette.rs,leaflet.rs,station.rs,cluster.rs,canopy/params.rs}`, `crates/telperion-core/src/pipeline/foliage/plan/fronds.rs` (caller of `leaflets`, frond girth), `crates/telperion-core/src/suite/establish.rs:372` (caller), new `crates/telperion-core/tests/walk_flying.rs`, `docs/parameters.md` + `src/browser/parameters.generated.ts` (regenerated only if the `rachisLength` doc comment changes)
**Touches:** [crates/telperion-core/src/pipeline/foliage/**, crates/telperion-core/src/suite/**, crates/telperion-core/tests/walk_flying.rs, crates/telperion-render/examples/walk_leaf_probe.rs, docs/parameters.md, src/browser/parameters.generated.ts]

### Approach
- Change `fan`'s girth (`rosette.rs:378-423`, tuple `(p, count, split, girth)`) to `Option<f64>`: `Some(rachis::girth(..))` from the rosette (`rosette.rs:~351`), `None` from `station.rs:128-133` and `cluster.rs:131-136`. Thread the same `Option` into `leaflet::leaflets` (`leaflet.rs:186`); with `None`, each `Leaflet.at` is `point` while `axis`, `run` and `share` are computed exactly as today (`rachis(.., 0.0, t)`'s `run` and the pitch/closing/spine), so headings do not move. With `Some(g)` everything is byte-identical to today. Other callers (`plan/fronds.rs:82`, `suite/establish.rs:372`) pass `Some`.
- Update the `fan` doc (`rosette.rs:366-377`, "none for a placement's on wood, whose rachis draws no bark") and the `rachisLength` doc comment in `canopy/params.rs` only if its dormancy text must name the tuft (regenerate with `TELPERION_WRITE_REFERENCE=1` through the parameter-reference test if it changes).
- Red first: a focused unit test (in `suite/` beside `suite/rachis.rs` or `suite/organs.rs`, e.g. a family with `leafletCount` 5, `rachisLength` 2, `cauline` 1 on a small tree) asserting every station-borne leaflet of a compound placement stands at its placement point (all leaflets of one placement share one position, and that position is within a millimetre of the station point), red on the base and green after; and that a frond's leaflets keep their spacing (the palm digest pins in `suite/rachis.rs::the_axes_move_no_leaf` and `suite/species` stay unchanged).
- Commit the probe as `crates/telperion-core/tests/walk_flying.rs`, `#[ignore = "a measurement"]` tests, following `tests/walk_rachis.rs` (its `frame()`, `Wood` capsule grid, `end()`, `cauline` 0 differencing; `:1-40`, `:184`) and `tests/walk_midpoint.rs` (env-driven ignored measurements). Port from the host's uncommitted `crates/telperion-render/examples/walk_leaf_probe.rs` the class logic only: drop `film/path.rs`, `film/camera.rs` and the render branch (they exist only on the film branch); take frames as (from, to, age, seed, t) with defaults: the film's t = 0.129, 0.227, 0.34, 0.463 at seed 7, and t = 0.25, 0.5, 0.75 at seeds 1, 7, 42, all oak to palm at 40. Classes per the spec's R3: (c) told by a second build at `rachisWidth` 0 as the probe does, (d) station-borne leaves (told by a `cauline` 0 build) standing more than `seat` off drawn wood, (e) any leaf more than `seat` off every drawn element, `seat` 0.1 m. The pixel split uses a camera distance (default 28.03 m, 26.8 mm a pixel at the stem) from env; reported, never held. One test that asserts (d) = 0 and (e) = 0 on every default frame, ignored (it is a measurement of heavy walk frames, so off the gate). Then remove the untracked example `crates/telperion-render/examples/walk_leaf_probe.rs` from the working tree (the committed test replaces it; it stays in the scratchpad).
- Record the base's class counts (run the new test on the base first, through `telperion-heavy`, saving output under `.flow/evidence/fn-309-a-compound-leaf-draws-its-rachis/raw/`) and the change's, and summarise both in `.flow/evidence/fn-309-a-compound-leaf-draws-its-rachis/RESULT.md`.

### Investigation targets
**Required** (read before coding):
- `crates/telperion-core/src/pipeline/foliage/rosette.rs:366-423` — `fan`
- `crates/telperion-core/src/pipeline/foliage/leaflet.rs:160-221` — `station`, `leaflets`
- `crates/telperion-core/src/pipeline/foliage/station.rs:100-140`, `cluster.rs:110-150` — the placement-on-wood callers
- `crates/telperion-core/tests/walk_rachis.rs` — the measurement pattern to follow
- `crates/telperion-render/examples/walk_leaf_probe.rs` (untracked) — the host's classes

**Optional**:
- `crates/telperion-core/src/suite/rachis.rs` — frond pins that must not move
- `crates/telperion-core/src/suite/species.rs:27-31,247-262` — shipped digests

### Key context
- Every cargo command goes through `~/.local/bin/telperion-heavy 8 --threads 2 -- ...`, one at a time, after checking `ps -eo pid,args | grep target/ci/deps` for a running test. The task's gate is `~/.local/bin/telperion-heavy 8 --threads 4 -- python3 scripts/gate.py`, once, at the end.
- No shipped preset places compound leaves on wood (only the date palm has `leafletCount` > 1, and its `cauline` is 0), so every committed digest must stay unchanged; a moved digest is a defect, not a re-pin.
- Key on the placement kind, not on the girth's value: a frond at `rachisWidth` 0 keeps its spacing (spec Decision Context).
- Write friction in `.flow/evidence/fn-309-a-compound-leaf-draws-its-rachis/FRICTION.md` as it happens.

## Acceptance
- [ ] A placement on wood (station or cluster) with more than one leaflet stands every leaflet at its placement point with today's headings; a focused test is red on the base and green after.
- [ ] Fronds keep their spacing: `suite/rachis.rs` pins and every committed `suite/species` digest unchanged.
- [ ] `crates/telperion-core/tests/walk_flying.rs` (ignored) reports classes (a)-(e) per frame and asserts (d) = 0 and (e) = 0 at the spec's R3 frames; base and change counts recorded in the evidence RESULT.md.
- [ ] The untracked example probe is gone from the tree; docs that describe the placement-on-wood rachis updated (generated parameter docs regenerated if their source changed).
- [ ] The gate (`telperion-heavy 8 --threads 4 -- python3 scripts/gate.py`) is green, once.

## Done summary
A twig or cluster placement on wood draws no rachis, so its compound leaf now stands every leaflet at its placement point with its old heading. `fan` and `leaflets` take a frond's girth or none (`Option<f64>`). Fronds pass `Some` at any width and are byte-identical: every species digest and rachis pin is unchanged. The host's probe is now the ignored test `crates/telperion-core/tests/walk_flying.rs`. Across the 13 R3 oak-to-palm frames it counts 665 to 4,082 leaves a frame in class (e) on the base, and 0 in (d) and 0 in (e) after the change (`.flow/evidence/fn-309-a-compound-leaf-draws-its-rachis/RESULT.md`). The `rachisLength` doc now names the tuft, and the parameter reference was regenerated.

Tests: the organs suite's `a_compound_leaf_on_wood_stands_its_leaflets_at_its_point` failed on the base (a leaflet 2 m off its point) and passes after the change. The focused suites passed (63 tests), `walk_rachis` passed, and the gate `telperion-heavy 8 --threads 4 -- python3 scripts/gate.py` passed once (1,477 passed, test run 1,330 s against 1,560 s). The untracked example probe was removed; a copy stays in the scratchpad.

Follow-up, not this task: `tests/walk_midpoint.rs` (fn-308) already fails `cargo fmt --check` on the base.

stage: impl-review - ran (codex, one correctness reviewer, SHIP; two P2 findings on the measurement fixed: end spheres, and the pixel split by width; base and change re-measured)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 4ee18181423539ac47ab86d92a94ef599cc66fc5, c4336331949d0d9b70c17a30f8af25fc602a2253, a9c74b5257615af524186bb2f5f12895ef1cc1ba
- Tests: telperion-heavy 8 --threads 4 -- python3 scripts/gate.py, cargo test --profile ci -p telperion-core --lib -- leaflet suite::rachis suite::species suite::organs suite::establish suite::rosette suite::clusters, cargo test --profile ci -p telperion-core --test walk_rachis, cargo test --profile ci -p telperion-core --test walk_flying -- --ignored --nocapture --test-threads 1
- PRs: