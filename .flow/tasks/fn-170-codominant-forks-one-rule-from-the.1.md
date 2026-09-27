---
satisfies: [R1, R2, R3, R4, R5, R6, R7, R8]
---
# fn-170-codominant-forks-one-rule-from-the.1 Implement Codominant forks: one rule from the ground to the crown

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
Codominant forks are one rule from the ground to the crown. Every structural axis decides one fork from its own stream: a height from a bell over the tree's height, crossed ascending at least one station above its birth. The root alone forks at its base, which is how a clump is made. Forks grow in over the rate and `forkWays`' fraction. Parts fill four fixed slots, and each part decides its next fork from a key of this fork's. The sibling's weight is recorded on the node (`Node::codominant`). One allocation function divides wood by role in both radius solves: `lateralShare`, `forkBalance` times the sibling's weight, and the pipe model at neutral. The five stems rows are retired and refused by name, with their replacement, on the wire, in overlays and in value files. The birch is restated through the fork rows.

Which host findings changed code already written:
- Astra findings 2 and 3 replaced code already built:
  - the count-dependent fan became fixed slots
  - the Bernoulli fork became grow-in with a weight
  - the lateral cap was dropped from the radius law
  - the bool mark became a carried weight
- Findings 4 and 5 needed no change to the scaffold: the remaining length, the bound and the mark carried through reads were already done. Finding 1 needed only the least-rise guard. Finding 8 added role-based dbh, branch order and clumping.

Codex review, two fan-out dispatches plus one re-review. SHIP on the re-review.
- The first dispatch's findings were fixed but its round was refunded, because the fixes were committed before it was finalized. They were:
  - the primary's fork key never advanced
  - the primary heading jumped at negligible weight
  - the row descriptions still described the old fan
- The second round found three more, all fixed:
  - the rise frame flipped at plumb
  - overlays did not name the retired rows
  - the stem-radius reference jumped when a crown fork appeared
- Red-first tests exist for the key, the frame and the grow-in regressions.

Neutrality: every preset other than the birch keeps its skeleton positions, mesh, leaves, field and plan digests (generation digest before and after, 7 presets x seeds 1 and 7). Only the Tree bytes and GPU byte metrics moved: Node gained an Option<f64>. The specimen snapshot is now schema 6.

R7 (owner):
- Birch at seed 1, before and after: raw/stills/sheet-birch-1-{bare,whole}.png. The fork is at 0.96 m, the leaning stem is at 28 degrees, and the bearing is kept from the seed.
- Plane candidate: plane-candidate.json (oak base, 26 m, codominance 0.85, forkHeight 0.2 +/- 0.15, ways 3.2, lengthTaper 0.2, lateralShare 0.45). Stills are raw/stills/sheet-plane-{bare,whole}.png, measurements are measure-summary.json.
- Over 8 seeds, the plane has 11,231 to 45,531 structural nodes against 10,141 to 11,635 for the no-fork baseline, and 204,469 to 238,769 twig nodes against 238,365 to 239,859. Both hit the 250,000 node ceiling, so the forked tree gives up at most 15% of its twig budget. Median build time is 769 ms against 700 ms, and peak memory 619 MB against 536 MB. The birch as shipped: 2,098 to 2,530 structural nodes, 2,851 ms, 176 MB.
- Owner question: do the plane's limbs hold their girth to the crown edge, or is a taper-profile spec needed? The worker's read of the bare sheet: the low forks show, and the limbs dissolve into fine wood by mid-crown.

The beech tape was extended live twice (8 Jev calls each). The second extension replaced the first's answers after a dial wording fix. See FRICTION.md.

stage: impl-review - ran [2026-09-27] SHIP (codex, rounds: fan-out x2, re-review x1)
## Evidence
- Commits: 2379b8c21394c694797aba1686a060aeed5764d1, e9460e5394d2aabb32d75cf3c2d4211de45161d8, 7c402402316437651b1b384297e515c844d4e95d, 87af2d73c6c39be95d4c0e1c044296172b1d4668, 90e1bda28fd51e3db6cd136223f1c5818ff57233, 7d7207ef10184d9248a9cf5b7c0934cb057b7984, d9b5f97370bd091a5f0851b68770eb11cb506da6, 2b301f5f88a7ff3d2557907cd2a5a3d611a1f510
- Tests: baseline: none (spec defines no Quick commands; AGENTS.md runs the workspace gate once at task end), cargo test --profile ci --workspace --no-fail-fast, npm test, python3 scripts/test-reviewers.py
- PRs: