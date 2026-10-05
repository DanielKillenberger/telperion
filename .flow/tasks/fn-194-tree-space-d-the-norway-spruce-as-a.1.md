---
satisfies: [R1, R2, R4]
---
# fn-194-tree-space-d-the-norway-spruce-as-a.1 The Norway spruce as a point: values, stills, walk and the beech unchanged

## Description
The Norway spruce (*Picea abies*, Massart's model) as a point in the tree space, values only on the engine phase C left, following the beech.

- **Values:** `crates/telperion-space/src/spruce.rs` beside `beech.rs`, exported from `lib.rs`; the rules and values from `MODEL-SPRUCE.md` (fn-190 evidence) and its sources, each source listed in this spec's `SOURCES.md`. Massart: an orthotropic, monopodial, indeterminate trunk whose yearly growth unit ends in a whorl (`Zone::buds`), plagiotropic branches bearing second-order branchlets in one plane (trunk `form.plane`), branch tips turned up (`form.elevation`), branchlets shed (`shedding`).
- **Engine gaps go to the host**, never designed here (AGENTS.md, dispatch): MODEL-SPRUCE.md's F1 to F6 are checked against today's engine and reported.
- **Stills:** `crates/telperion-render/examples/space_spruce.rs`, sharing the beech's still code through `examples/space/`, dressed by the norway-spruce preset's rows with render-matched darker colours as the beech's were. A five-seed sheet at 80 years (seeds 1, 7, 2, 3, 4) and 10/20/40 years at seeds 1 and 7, beside `.flow/references/norway-spruce/`, into `.flow/evidence/fn-194-tree-space-d-the-norway-spruce-as-a/raw/`.
- **Measures:** `crates/telperion-space/examples/beech.rs` generalised or mirrored for height, width, dbh.
- **R3 walk** (beech to spruce at a fixed seed, still strips, `examples/strips.rs`) and **R4** (the beech's passed sheet re-rendered unchanged) run only after the host passes the look.
- **Gate:** the host views the sheet first, then Astra; the owner's verdict passes it (docs/tree-space.md).

## Acceptance
- R1: the spruce's values from MODEL-SPRUCE.md and its sources (SOURCES.md), at several ages and mature, seeds 1 and 7, beside the references and today's spruce.
- R2: the gate; passed on the owner's verdict (2026-10-05) with round 10's values.
- R4: the beech unchanged in look, shown on fn-196's branch (38c2de33).
- R3 is task 2, blocked by fn-206.

## Done summary
The Norway spruce (Picea abies, Massart's model) is a point in the tree space: values only, in crates/telperion-space/src/spruce.rs. It is drawn by crates/telperion-render/examples/space_spruce.rs through the shared still runner (examples/space/still.rs).

- R1: values from MODEL-SPRUCE.md and their sources (SOURCES.md; needle retention from Muukkonen and Lehtonen 2004). Rounds 1 to 11 are in RESULT.md, each sheet beside the references and today's spruce. The engine work the spruce needed went into its own specs: fn-200 (sag and ground support), fn-201 (foliage dispatch), fn-202 (dormant buds), fn-203 (the bent-lever sag), fn-205 (secondary girth).
- R2: passed on the owner's verdict (2026-10-05), with round 10's values: "spruce looks good but not as lush and big as the reference. but definitely acceptable. Still much to be improved but i think it's structurally sound."
- R4: the beech is unchanged in look, shown on fn-196's branch (38c2de33).
- R3 is task 2, blocked by fn-206.

Known gaps:
- The hanging curtains' volume, bounded by about 210M wood triangles and 16M needles per tree. Phase F's levers are cheap fine-twig drawing and shedding before growth.
- The draperies are values-limited.
- The 10-year sapling reads sparse.

Gate: cargo test --profile ci --workspace --no-fail-fast, 1044 passed, 0 failed, 21 ignored (under the GPU lock).

stage: impl-review - ran (codex, base origin/master): SHIP, with two P2s.
- Fixed in 838b6e50: the stills' walks now refuse a PA index the species lacks, by name.
- Not fixed, recorded: the spruce stills report height and width but not dbh. R1 does not ask for dbh, and the beech's measures example is left as it is.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: e6f37885, 838b6e50, 1c3f4f67, 8daeef40
- Tests: cargo test --profile ci --workspace --no-fail-fast
- PRs: #149