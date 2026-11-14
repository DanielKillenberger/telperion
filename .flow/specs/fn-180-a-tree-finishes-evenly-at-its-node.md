## Goal

fn-53 kept the node budget as a named row, `maxNodes` (default 250,000), as a bound on work and memory. The trees the new rows draw reached it (fn-170's plane on every seed; three of four beech Tune candidates in fn-62's round 1), and a tree that reached it stopped wherever the count ran out, leaving branches in blunt, cut-off stubs the reviewer named on all four of the beech's failing traits. A tree must never look truncated because of a compute bound: at its budget it gives up fine detail evenly and stays whole, and a species can state the budget its look needs, at a measured cost.

## Decisions

- owner (2026-09-27), on the beech run's node-capped candidates: "why do we still have a node limit i thought we removed all arbitrary limits"
- owner, on the host's proposal that a tree finishes evenly at its budget and `maxNodes` becomes a dial: "ok"
- Measured choice of reduction: thin the finest order uniformly, in sixteenths of a twig generation (removing a whole generation left the plane over budget at 267k and dropped the beech to 68k); whole generations first, then a bisection inside the one that does not fit; the highest fitting level is kept as `diagnostics.twig_detail`.
- Host (2026-09-27): a dial declares the value its unset row stands for (`ranges::DEFAULT_MAX_NODES`); a reduced tree is whole and feasible for Tune, only an `incomplete` one (with a named reason) is refused.
- Size trim (host, 2026-09-28): each detail level regrows from its rows instead of cloning `Specimen`.
- Not render level of detail (fn-176), not removing the budget, not the beech's values (fn-62).

## Acceptance Criteria

- **R1:** On the plane and the beech's capped candidates, a build at its budget has no branch ending short of its tips, the reduction recorded — met: stubs at the default budget, plane seed 1 55,626 on master to 178; seed 2 62,066 to 169; beech candidates 34,087 to 585, 52,394 to 444, 12,367 to 271 (each fewer than the unbudgeted tree's).
- **R2:** Under budget, byte-identical to master on every preset at seeds 1 and 7 — met (skeleton digests).
- **R3:** Raising `maxNodes` walks continuously to the unbudgeted tree; on the dial table with cost per step — met: nodes and detail rise monotonically from 100k to the whole tree (plane whole at 600k, beech at 550k); a 50,000-node step costs about 0.2 to 0.9 s and 90 to 150 MB; dial window 100,000 to 1,000,000.
- **R4:** Before/after stills of the plane and a beech candidate go to the owner: do the stubs go away? — stills made (kept in the ignored `raw/`); no owner verdict recorded here.
- **R5:** Gates green; time and memory at the budget against master — met: at the budget the skeleton is 3 to 4 times slower (plane 206 to 862 ms, beech 212 to 1,016 ms); whole request plane 1,003 to 1,887 ms, beech 5,127 to 4,898 ms; peak memory the same or lower.

## Result

Diagnostics split into `reduced` (whole, with `twig_detail`, feasible for Tune) and `incomplete` (`node_capped`, named reason, refused by the tuning gates). `telperion-field.wasm` 369,691 bytes (+0.5% over base 367,819) after the clone-free trim; output identical to the cloning version. Test changes: `the_beech_stays_at_its_stated_depth_whatever_its_radii` uses a 12 m tree for its fine-twig case; dial counts 233 and 138. Codex fan-out SHIP after fixes.

## Open

- The framework alone exceeds the budget (host, 2026-09-27; not built): when the scaffold reaches `maxNodes` before any twig grows, the build stops and is flagged `incomplete` ("the framework alone exceeds the node budget"). Giving up structural detail evenly is its own spec.
- A cheaper first guess for the level search.

## Friction

- Capped Tune candidates had no family file -> proposal: a per-candidate overlay file named by case id.
- The dial half of R3 was a design fork (unset `Option`, reduced vs truncated) -> host decided the same day.
- Re-recording the beech tape (9 Jev calls) and pruning it by hand -> proposal: `tape_trim --prune <replay>`.
- A review fix to the dial's stated cost re-recorded the tape again (9 calls) -> a tape key not hashing dial wording.
- The size check caught a +7.9% clone only in CI -> host to spec running the size check locally for core changes.

## Evidence

- `.flow/evidence/fn-180-a-tree-finishes-evenly-at-its-node/plane-candidate.json` (fn-170's plane with `girthHold` 0.75, `girthFall` 4) and `beech-candidate.json` (fn-62 r3 case `candidate-a49edc4aab1c`), the overlays measured with `crates/telperion-core/examples/node_budget.rs`.
