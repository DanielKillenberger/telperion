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
- Size trim (host continuation, 2026-09-28): CI refused a +7.9% slim field module, almost all of it the derived `Specimen::clone`. Each detail level now regrows from its rows, and nothing is cloned. `telperion-field.wasm` is 369,691 bytes (+0.5% over base 367,819). The output is identical to the cloning version. Skeleton time at the budget: plane 862 ms, beech 1,016 ms.
stage: impl-review - ran [2026-09-28] (codex fan-out over 1eb62455..e9f5b7e5: SHIP from all three draws)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 11a3682917cb9e00e445d9b5a1f19cc414eb7a7a, d42bd8eee558782b94fb581e26674e4329e8d881, b7218ba804460e9a44c63eeb8519b91717bd92f3, eaaa796311ac4ef5613e4f706937aace9dd7ba19, 1eb62455fa4aeb96042a52a238f1139691cb10df, e9f5b7e5abaeb332cc02f95caabb000999435f6f, 4102e838e289cee7b6c4b3f979a023a97457cbb6
- Tests: cargo test --profile ci --workspace --no-fail-fast, npm test
- PRs: