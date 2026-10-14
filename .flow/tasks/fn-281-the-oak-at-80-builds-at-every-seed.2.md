---
satisfies: [R3, R4]
---
# fn-281-the-oak-at-80-builds-at-every-seed.2 Fix A: wood drawn above a join thinner than FINEST is not drawn

## Description
The host's rulings of 2026-10-14 (spec Decisions): the conversion and dress.wgsl judge min(p.radius, above) against FINEST; a device check on the oak at 80, seeds 3, 7, 30, 39; the sweep re-run with byte digests against the base; the 486 nodes of seed 7 counted.

## Acceptance
R3 and R4: the red test green, the device check green, the sweep with no unsolved radii and every byte change listed with nodes removed; codex review SHIP.

## Done summary
Fix A: above a relay's inside join, the conversion and `dress.wgsl` compare the radius actually drawn against `FINEST`, and wood thinner than that ends the axis at the join, the module's own rule. Results (`FIX.md`):

- **Oak at 80:** green on the CPU at seed 7. The device draws no radius-zero node at seeds 3, 7, 30 and 39.
- **Sweep:** no unsolved radii remain. The engine-budget refusals are unchanged; the host left them to fn-236.
- **Bytes:** 463 of 950 trees change (`BYTES.txt`), each only by its wood thinner than `FINEST`. At their own age, the most-changed trees lose at most 0.024% of their leaves, with bounds unchanged; on those numbers no stills were rendered.
- **R4:** all 486 thin nodes of the oak at 80, seed 7 are removed.
- **Open:** device and CPU node counts at 80 already differ by a few nodes in 2.3 million on the base. Not traced.

Tier: none given by the conductor.

stage: impl-review - ran (codex, 3 reviewers: correctness, contracts, integration; SHIP)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 91be2ee4f, 2372fff4f, a4fb43f48
- Tests: telperion_core radii_tests --ignored (green), telperion_core pipeline::space (24 passed), telperion_render thin_tests --ignored (device, 4 seeds), telperion_render every_preset_is_drawn_as_the_cpu_converts_it, .flow/evidence/fn-281-the-oak-at-80-builds-at-every-seed/sweep.sh (OUT=base, OUT=fix) + compare.py, GATE_SKIPPED:workspace:host owns the workspace gate (coordinator, 2026-10-14)
- PRs: