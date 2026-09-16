---
satisfies: [R1, R2, R3, R4]
---
# fn-45-few-big-limbs-the-twig-budget-follows.1 Implement Few big limbs: the twig budget follows the radius

## Description
Give the twig layer its own stated depth, and set the beech's table so one
leader carries the crown to its top with few, large, steeply rising limbs
under it. See the parent spec for the botanical target and the validation
contract.

## Acceptance
- [x] R1 `skeleton.twigs.generations`, a twig row railed 1 to 6, neutral at 6, refused by name off the rail, on the wire, blended as a count, in the regenerated browser metadata and on the harness dial; every shipped preset byte-identical at neutral
- [x] R2 no twig-law lateral above the stated generation on any fixed seed, whatever the pipe model's radii; the planner's estimate counts to the cap
- [x] R3 the beech's table states the cap, the 24-seed protocol passes with no seed capped and the DBH gate held, and the matched pairs are rendered with the numbers and the branches-per-order table beside round 5c's in REPORT.md
- [x] R4 tests cover neutral byte identity per preset, the rail, the cap on a synthetic family, the estimate against the count, determinism per seed and a blend walk
- [ ] R3's last clause: the owner judges the pairs and records the verdict in fn-34

## Progress
The round is REPORT.md's "Round 6, fn-45: few big limbs (2026-09-15)".

What the round corrected on the way: round 5d had read the beech's cap as a
fifth twig-law generation grown by a higher fork exponent. It is not. The twig
law's depth falls with the exponent, and `MAX_LEVELS` (12) was never
approached. What the exponent moves is `is_twig` in `advance.rs`, which
compares an absolute radius against `twig.diameter / 2` while the pipe model
rescales every radius: at 1.8, 72,166 of the beech's first-generation laterals
on seed 1 were single twig nodes; at 2.8 only 361 were, and the other sixty
thousand became branch runs hanging a twig at every internode. Twig density is
what pays for a thick core, not depth, so the row this spec adds is real and
general without being the thing that holds the beech's core.

The row that serves both pairs is apical dominance. The scaffold ends its
leader at `crownBase + (1 - crownBase) * apicalDominance` of the height, and
the beech's 0.9 runs it to nine tenths, so the leader carries the crown to the
top rather than the limbs. Limbs leave at 48 degrees and bend up over their
run (rise 0.45, about 61 degrees by the end), side branches are held near
level (0.1), the crown starts at a twentieth of the height and is widest at
0.48 of its depth (shoulder 1.5), and the fork exponent is 2.6. At 2.8 the
wood sat on a cliff where a few degrees of limb angle collapsed the local
layer from about 150,000 nodes to 55,000. The species test's bound on the
beech's apical dominance moved from 0.6 to 0.95, which still refuses the
spruce's excurrent 1.0.

Seed 1, round 6b to final: stems at a third of the height 7 to 1; the leader
reaches 0.60 to 0.90 of the height; leaves by third of the height 1/27/72 to
6/50/44 per cent; the lowest twentieth of the leaves at 47 to 32 per cent of
the height. Seed 1, round 5c to round 6: 193,836 to 188,636 nodes, 115,255 to
87,670 branch axes, 115,080 to 86,700 twigs. All 48 protocol cases pass, the
heaviest beech seed is 197,872 nodes, none node-capped, and none of a further
120 random seeds capped either (heaviest 206,066). The DBH proxy holds at
0.889 m.

## NEEDS_HUMAN

R3 reserves the verdict to the owner and this task cannot award it. The
round-6 pairs are recorded with the numbers and the branches-per-order table
in REPORT.md, and the work is merged into `fn-34-integration`.

The host's read on the pairs, without a verdict: B-BARE shows one leader
thick at the base running to near the crown's top with limbs leaving it along
the whole height and rising steeply, which is the photograph's architecture,
and the crown is an upright oval rather than round 6c's round head. Still
wrong there: the crown is fuller and wider in its upper half than the
photograph's, which narrows toward the top; the trunk above the first limbs is
slimmer than the photograph's heavy column; and occupied reads 0.47 against
0.28, most of which is the resolution of the two measurements. B-WHOLE shows a
full rounded crown on a central trunk with the umbrella and the vase gone, and
the leaf mass sitting higher than round 6c's because steeper limbs carry their
tips up. Two faults were left to a later generator spec and not chased here:
the leaf mass stops at about a third of the height where the photograph
reaches about two metres above the ground, and the lower crown holds only the
trunk and the thick first metres of limb.

This task file was reconstructed on 2026-09-16 from REPORT.md's round 6 and
the branch commits: the original was a stub with every field TBD, although the
work had shipped and merged.

## Done summary
The twig layer states its own depth (`skeleton.twigs.generations`), and the beech's leader carries its crown. The capability is merged; the beech's own verdict is fn-62's. The owner accepted the silver birch at fn-34 round 25 (2026-09-16); the European beech's verdict moved to fn-62.
## Evidence
- Commits: 45ef8e3c
- Tests: cargo fmt --all -- --check, cargo clippy --release --workspace --all-targets -- -D warnings, cargo test --release -p telperion-core --no-fail-fast, cargo test --release -p telperion-render --no-fail-fast, npm run typecheck, npm run rust:test:wasm, npm test, uv run scripts/compare-references.py --self-test, node tests/species.mjs --measure-only (48 cases, measure/protocol-round25)
- PRs: