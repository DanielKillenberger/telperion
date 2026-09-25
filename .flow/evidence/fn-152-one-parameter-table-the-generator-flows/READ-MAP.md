# fn-152 read map

Mapped on 2026-09-25 against `1ef34d18` (origin/master `d6a3405c` plus the spec commit). Every wire row in `params.rs` `fields!` (249) was traced through telperion-core, -render, -wasm, -field, -jev and `harness/`, excluding presets, `params.rs` and tests. Paths: `core/` = `crates/telperion-core/src/`, `f/` = `core/foliage/`, `render/` = `crates/telperion-render/src/`, `gen/` = `render/generation/`.

**Verdict: one pass is not feasible within the task budget (10 commits).** The staged split is at the end. No refactor was started.

## Rows by group

| group | rows | read by (main sites) | stage |
|---|---|---|---|
| `age`, `growth.*` | 8 | growth path only (`core/growth.rs:94`, `branching/specimen/{timeline,survival,keyframes}.rs`); `Family::validate` on the direct build | growth path; validation only on the direct build |
| `skeleton.habit.*` | 21 | `branching/scaffold.rs`, `scaffold/stems.rs`, `branching.rs:300,314`, `local/advance.rs`, `local/planner.rs` | grow |
| `skeleton.{seed,attractors,samplingAttemptsPerAttractor,step}` | 4 | `branching/specimen.rs:79-130`, `branching.rs:99,162`; the seed also `pipeline/stage.rs:56`, `pipeline/drawn.rs:233`, `gen/data.rs:52` | grow; the seed every stage |
| `skeleton.envelope.*` | 7 | `envelope.rs`, `branching.rs:134-188`, `radius.rs:58`, `bias.rs`, `foliage.rs:253`, `f/reference.rs`, `gen/data.rs:79-83` | grow, plan, cull, GPU |
| `skeleton.bias.*` | 7 | `bias.rs:130-184` | grow |
| `skeleton.growth.*` (Options) | 6 | `branching.rs:98-122` `resolved_growth`, `scaffold.rs`, `local/{planner,advance}.rs` | grow |
| `skeleton.twigs.*` | 26 | `twigs.rs`, `local/advance.rs`, `local/pendant.rs` (the curtain), `local/seed.rs`; `twig.internode_length` and `stations_per_internode` through `pipeline/stage.rs:24` into `f/station.rs`, `f/plan.rs`, `gen/data.rs:53,60` | grow; those two rows plan, expand, GPU |
| `radii.*` | 4 | `radius.rs:58-78`, `radius/incremental.rs` (growth path); `trunk_radius` also `f/reference.rs:70` | grow; the leaf box |
| `surface.*` | 10 | `surface/{samples,build,angular,rings,compact}.rs`; `f/plan.rs:191`, `f/reference.rs:86`; `gen/preparation.rs:35` | expand, plan (seating), GPU |
| `canopy.*` | 44 | `f/{station,plan,placement,leaflet,rosette,short_shoots,clumping,reference,prepared}.rs`; `rosette_*` and `leaf_base_*` also `core/pipeline.rs:172,175` and `branching/leaf_bases.rs`; `gen/data.rs:61-73`, `gen/preparation.rs:38,154` | grow (fronds, bases), plan, expand, GPU |
| `element.*` | 14 | `f/element.rs`, `f/outline.rs`; `section_roundness` also `render/lib.rs:183`, `render/scene/frame.rs:102` | plan (the element); `section_roundness` draw |
| `material.*` | 95 | `render/scene/frame.rs:92-207` `Scene::set_frame` packs all 95 into one uniform; `render/wood.rs:30` `smooth_bark` picks a pipeline from five of them | draw |
| `shellDepth` | 1 | `pipeline/drawn.rs:243` into `foliage.rs:252`; `specimen/view.rs:102`; `gen/data.rs:74` | cull, GPU |

Wasm reads no row beyond `branch_diagnostics` (`telperion-wasm/src/generate.rs:32-55`: `generations`, `twig.diameter`, `length_ratio`, `ratio_power`). telperion-field reads only `skeleton.seed` (`grow.rs:13`). telperion-jev reads rows only through `params::metadata` and `params::overlay`, plus `maxNodes` (`tuning/judgments.rs:81`).

## Hidden couplings the table has to state

Derivations:
- `TwigParams::resolved()` and `RadiusParams::resolved()` derive nothing; they only range-check. The derivations live at the use sites: `resolved_growth` (`branching.rs:98`: step, kill and influence from `height·step`, trunk height from `height·crownBase`, influence from the crown volume and the scattered count), `TwigParams::internodes()` (`twigs.rs:242`), the `Curtain` methods (`local/pendant.rs`) and the surface expressions in `surface/samples.rs`.
- `presets::by_identity` fills `maxTurnPerStep` with 35 (`presets.rs:57`), so `Preset::parameters()` and the identity path give different families.
- `envelope.height` scales rows in other groups: trunk radius, length taper, twist, flare, canopy spacing and writhe.
- The effective radial side count is `max(radial_segments, 4·lobes)`, so `lobes` adds triangles at `lobe_depth = 0`.
- `fork_swell·(1+lobe_depth)·flare_radius` sets leaf seating and the reference box whenever `surface_contact > 0`.

Rows that do more than their doc comment says:
- `killDistance` acts as `min(kill, unit)`, so above one growth unit it does nothing. Unset, it is `2·height·step`, not twice `stepDistance`.
- `sheddingThreshold` is a shell depth (a share of `height·spread`) on the direct build and an annual vigour threshold on the growth path.
- `maxWritheMagnitude` applies with `supernatural.enabled = false` and caps `bias.lean`.
- `risePrimary` bends leaning stems (order 0). `lateralLengthRatio` acts only at order 2 and deeper, `reachProbeSteps` only at order 1.
- `twig.diameter` has four jobs: the twig threshold, the twig radius, the tip cap (× `twigTipTaper`), and the leaf box when `shoot_radius = 0`.
- `twig.angle` also sets lateral separation, through `min(angle, maxTurnPerStep)`.
- `bearing_diameter` switches the lateral rule and the internode floor, and on the growth path gates leaves and caps `shoot_radius`.
- `rosette_divergence` and `rosette_depth` shape leaf bases even at `rosette_fronds = 0`. Leaf bases do not need a rosette (`pipeline.rs:175`), though `capability.rs:244` labels them as if they did.
- `clump_system_order` changes the planned field at `limb_clumping = 0`, against its doc.

Gates, the dormancy each row's entry has to carry:
- Skeleton: `lateralOrders` (every lateral row; `riseSecondary` needs 2 or more), `attractorWeight` (attractors, sampling, influence, kill), `stems` (the four stem rows), `supernatural.enabled` (amplitude, wavelength and spiral, but not the maximum magnitude), `hang` (the eight curtain rows), `irregularity` (`lobeScale`), `sheddingThreshold` (`sheddingTolerance`).
- Radii and surface: `length_taper` (`max_taper_exponent`), `flare_radius = 1` (`flare_falloff`), `lobes·lobe_depth` (`twist_rate`), `fork_socket` (`socket_containment`).
- Canopy and element: `short_shoot_spacing`, `limb_clumping`, `rosette_fronds`, `leaflet_count > 1 ∧ rachis_length > 0` (written three times), `leaf_bases ∧ leaf_base_length`, `leaf_base_width` (flatness), `acanthophyll_length`, `skirt_length ∧ rosette_fronds`, `lobe_depth` (lobe count), `card` (the outline rows).
- Material: about 25 gates. For example, `ridgeScale ≤ 0` silences all bark relief, `plateCellScale ≤ 0` the plate rows, and each `*Strength = 0` its RGB.
- The jev dial test's own list of 36 switches-at-zero does not match these.

Switches between ways of building, principle 3 breaches the table will expose but not fix:
- `rosette_fronds > 0`.
- `surface_contact > 0`.
- `short_shoot_spacing ≠ 0` and `limb_clumping > 0`: no leaf plan, the placed path, and the GPU falls back.
- `card`.
- `leaf_base_width > 0`: the lattice.
- `size = 0`.
- `divergence` past a phase-error bound (`f/prepared.rs:173`): the CPU fallback.
- `scatter > 0`: four draws a leaf instead of one, which shifts the stream.

Rows no production stage reads, which R1 would call table errors: `canopy.spacing`, `canopy.clump`, `canopy.clump_span`. Every production caller passes a twig placement, so their only reads are validation and `examples/`.

The two executors disagree:
- The GPU executor carries no leaflet row (`gen/data.rs:42-84`) and counts stations, not stations × leaflets (`f/prepared.rs:226`). A twig-borne compound-leaf family is drawn with plain blades. No test confirms this yet, but it looks like a defect.
- The growth path never calls `clear_apical_twigs` or `clothe_leaf_bases`. It places short shoots and a rosette together where the direct build takes one, and places fronds after the clumping thin.

Validation is spread over 15 functions and repeats itself:
- `shell_depth` is checked three times (`family.rs:70`, `foliage.rs:252`, `gen/preparation.rs:24`).
- `stations_per_internode` is 1..32 in twigs and 1..=64 in canopy.
- `maxNodes = Some(0)` is refused by `Family::validate` and accepted by the growth path.
- `envelope.height` is `≥ 0` in the envelope and `> 0` in the family.
- `params::parse` runs only the material, age and growth checks; the rest wait for the build.
- No check covers `seed`, `enabled`, `card` or an upper bound on `clump_system_order`.

## Hand copies of the row list

| copy | rows | what it holds | kept in step by |
|---|---|---|---|
| `crates/telperion-jev/data/dials.json` + `dials.excluded.json` | 227 + 22 | meaning, min/max, integer, step sizes, group, score visibility, bases | `tests/dial_table.rs` (coverage, steps, a SHA-256 pin on six pilot rows); tuning configs embed their own copies (`tuning/live.rs:98`) |
| `core/blend.rs:19-248` | 248 | interpolation mode per row (linear, degrees, count, density, rounding direction) | nothing; no test compares it with `fields!` (the spec does not name it) |
| `harness/params.ts` `GrowerParams`, `SLIDERS` | ~64 | labels, working windows (not core bounds), stage headings; `density` and `torsion` are harness-only | `family.test.ts` round trips |
| `harness/family.ts` | ~64 | renames both ways (`forkExponent` is `taper`) | the same round trips |
| `harness/dials.tsx` | generic | every numeric key of four groups, with skip lists | none |
| `harness/material-detail.test.ts` | 77 | material key names | itself |
| `render/scene.rs:39-96` + `shaders/common.wgsl:9` | 95 | the uniform layout, Rust and WGSL by hand | nothing asserted |
| `gen/data.rs:35` `config()` | 15 | a second GPU packing of canopy and envelope rows | none |
| `core/capability.rs:205` | ~15 | feature conditions over canopy rows | presets only |
| `docs/generation-limits.md:7-20` | 11 | defaults and ranges | none |

`src/browser/presets.generated.ts` is generated (`scripts/build-wasm.mjs`, through the wasm `catalogue()` export) and carries values and types only.

## What an R2 or R6 pass touches

- 107 core function signatures take a params struct or `&Family`, and 25 core files name `Family`.
- The GPU executor calls 16 core functions directly (`gen/preparation.rs:14-331`: `mesh::grow`, `foliage::build_element`, `Reference::of`, `surface::compact::*`, `foliage::prepared::*`, `mesh::assemble`, `surface::build`), and 22 render files import core stage modules.
- 64 core test and example files import core modules. The heaviest direct stage callers are `branching::generate` (31), `foliage::build_element` (13), `mesh::grow` (12), `surface::build` (11) and `foliage::place` (11).
- Presets: `Preset::parameters` builds oak and spruce inline and delegates beech, birch and palm to `presets/species.rs`; five material tables sit in `presets/materials.rs`.

## Why one pass does not fit

1. **R1 needs host decisions first.** The table has to say, per row, which stage reads it and what gates it, and for some rows the honest entry is a design question:
   - three unread rows: delete them from the wire, or wire them in?
   - `sheddingThreshold`'s two meanings;
   - `killDistance` being capped by the growth unit;
   - leaf bases without a rosette;
   - the growth path's disagreements with the direct build;
   - the GPU leaflet drop.

   The stage vocabulary (grow, plan, expand, cull, draw) also has no place for the 8 growth-path-only rows or for validation-only reads. Under AGENTS.md these go to the host; a guess written into the table would ship as the answer.
2. **The work is five migrations, each with its own consumer.**
   - R2 alone rewrites about 107 signatures across grow, plan and expand, plus the GPU executor's 16 entry points.
   - R6 moves 64 test and example files behind a test-only feature and cuts sanctioned items for the growth path and the GPU executor.
   - R3 replaces six hand copies with generated output, plus `blend.rs`, which the spec does not name. The jev table carries a SHA-256 pin and embedded copies in tuning configs.
   - Presets as data rewrites eight presets.

   Each migration needs its own byte-identity proof, and R2 and R6 each need before/after timings. Ten commits cannot hold that with a gate run at each seam.

## Proposed staged split

Each stage keeps output byte-identical and fits one task budget.

0. **Host decisions (no code).**
   - Settle the stage vocabulary: add `growth` and `validate`, or not.
   - Decide the three unread rows, `sheddingThreshold`, `killDistance`, leaf-base gating and the jev switch list.
   - File the GPU leaflet drop and the growth-path disagreements as their own defect specs.
1. **The table, and what reads it as data.**
   - One row per wire field in core: path, type, unit, range, count flag, default, meaning, stage, gate, derivation and interpolation mode. It replaces `fields!`.
   - Generated from it: the wire schema, `params::metadata`, range validation, `blend.rs` and `docs/parameters.md`.
   - Tests: the graph is acyclic, every row is read, and each validate function's bounds equal the row's.
   - No stage signature changes.
2. **`resolve` and stage views (R2).**
   - `resolve(&Family) -> Resolved` in dependency order, folding in `resolved_growth`, `internodes`, the curtain derivations and the `by_identity` default.
   - Order: the grow stage first, then plan, expand and cull, then the GPU executor's `data.rs` and `preparation.rs`.
   - Release-profile timings before and after.
3. **Stages private (R6).**
   - `pub(crate)` stages, with a test-only feature for the 64 test and example files.
   - Deliberately exposed items for the growth path and the GPU executor.
   - A trybuild (or a recorded one-off) compile-fail check from telperion-wasm.
4. **Generated consumers.**
   - The jev dial table from the table. This retires `dials.json` and `dials.excluded.json` and re-pins the pilot hash with a Decisions line.
   - Harness sliders, with dormant rows shown with their gate.
   - The `presets.generated.ts` metadata.
5. **Presets as data.** One value file per shipped preset, validated against the table, and the writer function fn-149's Accept calls, with a test.
