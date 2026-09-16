---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-54-the-beechs-crown-from-its-limb-systems.1 Implement the beech crown from its limb systems

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
R1 to R3 delivered and merged: the beech's architecture from B-BARE, `canopy.limbClumping` and `material.lobeShade`, all railed and neutral byte-identical. R4 is measured and unmet (B-WHOLE centre 56 against 80; depth shading costs 14 counts and buys no outline) and R5's verdict is not yet accepting; both carry into fn-62 with the owner's round-22 notes. The owner kept depth shading at 0.7. The owner accepted the silver birch at fn-34 round 25 (2026-09-16); the European beech's verdict moved to fn-62.
## Evidence
- Commits: a061b46d
- Tests: cargo fmt --all -- --check, cargo clippy --release --workspace --all-targets -- -D warnings, cargo test --release -p telperion-core --no-fail-fast, cargo test --release -p telperion-render --no-fail-fast, npm run typecheck, npm run rust:test:wasm, npm test, uv run scripts/compare-references.py --self-test, node tests/species.mjs --measure-only (48 cases, measure/protocol-round25)
- PRs:
## Where fn-54 stopped (2026-09-16, owner shutdown)

Done, on branch fn-53-beech-crown-limb-systems:
- R1 (4cba7a0d): B-BARE measured (trunk clear to about a fifth of the height,
  a core thickest to about half and running on toward the top, limbs 25 to
  40 degrees from vertical rising straight, an upright oval); the beech's
  habit, envelope, radius and twig rows set to it. The host accepted the
  core reading over the spec's two fifths and asked for steeper limbs and an
  oval; both done (limbs 28 degrees, spread 0.36, shoulder 1.8).
- R2 and R3 (3b6f4e7f): canopy.limbClumping (thinning toward the walls
  between limb systems, growth view included) and material.lobeShade (a
  leaf-mass depth grid built once at submission, read in the vertex stage),
  both railed, wired, blended, in the regenerated metadata, with tests;
  neutral skips both. Beech cuticle gloss 0.48 -> 0.18 (owner). Measured:
  beech hero frame total p50 10.23 -> 10.24 ms with the term on, oak 5.22 ms
  (round 17: 5.236); grid build 62 ms at submission for 4.73 M leaves.
- WIP (fe9ebd4e): the host's last ask, leaf mass down to about 2 m and the
  crown widest a little below the middle: crown base 0.06, fullness 0.3,
  twig length ratio 0.23, lobe shade 0.7. Not yet looked at in the full
  runner; the species.rs comments still name the old crown base.

Left:
- Look at the e4 quick look (fn54-scratch/e4 under measure/), then
  re-run the 24-seed node check (last heaviest 199,237 on two seeds).
- R4 check: the round-18 leaf faces, transmission and sheen are untouched;
  B-WHOLE's centre read 53 against round 18's 63 at lobe shade 1.0, which is
  why 0.7 is stated now; confirm on the record.
- R5: pins re-recorded once (identity, drop, sag, strands neutral pins fail
  on the beech now), the full runner capture, compare JSONs and stills.json
  under round19-fn54/, the Round 19 section in REPORT.md, the 48-case
  protocol into measure/protocol-fn54, and every gate.

## The mass grid's 62 ms, and why it is not skipped here (2026-09-16)

R3's grid is built in `select::submit` for every preset, including the oak,
the spruce and the Two Trees that state `lobe_shade: 0`. Skipping it needs the
material at submission, and `Renderer::submit_at` can read it: `set_material`
precedes `submit` in both production callers, `examples/headless.rs` and
`web.rs`, where the comment at `web.rs:219` states that the material rides
with the tree.

The host plumbed a `masses: bool` through `Renderer::submit_at`,
`foliage::submit` and `select::submit` and then reverted it. The renderer's
own test `lobe_shade.rs::at_zero_the_row_draws_the_frame_it_always_drew`
draws one submitted tree at two materials without re-submitting, so a grid
chosen at submission is empty when the material later asks for depth, and the
row silently stops working. A correct skip is a contract change, that the
material is in force before the tree goes up, or a grid built on the GPU from
the `placements` buffer that already holds the positions. Both belong to the
owner, and neither is on this task's Left list.

`mass::grid` counts 4.73 M placements into at most 64 cubed cells and then
reads each cell's column up to `REACH`. The count dominates.

## NEEDS_HUMAN: the owner's round-22 read (2026-09-16)

R5's verdict is not yet accepting. On the merged tree (round 22) the owner
wrote: "The tree has clear regular outline/border that doesn't look natural.
The taper approaching the border needs to produce thinner branches twigs.
It's also too dense at the shoulder of the crown. It's more sparse lower and
gets more dense at the top. It generally looks too dense everywhere?" The
owner kept depth shading at 0.7 (choice A). The next beech round answers
those four notes: the outline, the twig taper toward the rim, the density
profile from shoulder to top, and the overall density.
