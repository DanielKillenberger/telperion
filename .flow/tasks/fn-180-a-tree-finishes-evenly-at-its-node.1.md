---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-180-a-tree-finishes-evenly-at-its-node.1 Implement A tree finishes evenly at its node budget

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
When a tree would grow past `maxNodes`, it now gives up twig detail evenly. Detail is thinned in sixteenths of a generation, starting with the finest order, and every branch still reaches its tips. At the default budget the plane has 178 stub axes (55,626 on master) and the beech candidate 585 (34,087 on master). Diagnostics split into `reduced` (whole, with the `twig_detail` it kept, feasible for Tune) and `incomplete` (`node_capped`, with a named reason, refused by the tuning gates). `maxNodes` is now a dial that steps from `ranges::DEFAULT_MAX_NODES` where the row is unset, and its measured cost is stated (about 0.2 to 0.9 s and 90 to 150 MB per 50,000 nodes). Trees under their budget are byte-identical to master on every preset at seeds 1 and 7.

- Unknown settled by measurement (R-MEASUREMENT.md): removing a whole generation leaves the plane over budget (267k nodes) and drops the beech to 68k, so the build thins the finest order uniformly.
- Cost for the PR: at the budget the skeleton is 3 to 4 times slower than master (5 or 6 levels tried). The whole request is plane 1,003 to 1,581 ms and beech 5,127 to 4,501 ms, and peak memory is the same or lower. A cheaper first guess is a follow-up.
- Beech tape: Tune's first round was extended twice (18 Jev calls in total, because a review fix changed the dial's wording), and the superseded answers were removed.
- Test changes: `the_beech_stays_at_its_stated_depth_whatever_its_radii` uses a 12 m tree for its fine-twig case, the bisection is classified in the generation-limits inventory, and the dial counts are 233 and 138.
- Follow-ups: the framework alone exceeding the budget (in the spec's Open); a cheaper first guess for the search; a tape key that does not hash dial wording (FRICTION.md).
- Owner question (R4): do the stubs go away? Stills: raw/stills/sheet-plane-1-bare.png, sheet-beech-a49-1-bare.png.
- Tier: none given in the dispatch; ran on the session model (actual_model: claude-opus-5-5).

stage: impl-review - ran [2026-09-27..2026-09-28] (codex fan-out NEEDS_WORK on 2 findings, fixed, re-review SHIP)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: bff4fa32c5f1933758ad4dde0b9cef843fe7873d, 6b5aacb557a0803ecee334c22b6c0e3ae52a68eb, 4d38a292e184195da0a03d9b803fb7b750219618, 8f80f1a22a76a3e1eb38b85a146ecf6243e8f17c
- Tests: cargo test --profile ci --workspace --no-fail-fast, npm test
- PRs: