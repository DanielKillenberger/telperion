# The oak at 80 builds at every seed

## Goal & Context

fn-236's round 4 (2026-10-14, SHEET-4.md): the Oregon white oak at age 80, seed 7, is refused with "invalid input: unsolved radii"; round 3's rows refuse the same way, so it predates the round. The harness's age dial reaches 80 and the site's grove draws seeded oaks, so a shipped preset refused at a reachable age is a release defect. The cause, checked in code (task 1, `CAUSE.md`): a relay join's `above` is the f32 difference of the engine's girth sum, which is zero where the stopped module's head is absorbed in the relay's share; the conversion keeps the phytomer by its relay-inclusive radius and draws the head at `min(p.radius, above)`, a node at radius zero.

## Requirements

- **R1 (repro first):** the refusal as a standalone test, red, on the CPU and the device; a sweep of the oak at 60-100 and of every shipped preset at its age and at 80, seeds 1-50, listing every "unsolved radii" refusal.
- **R2 (cause):** the solver's failure condition against this tree's structure, checked in code, written down before any fix and reported to the host.
- **R3 (fix):** every tree the growth law makes has its radii solved, by degree (no fallback to a default radius, no species branch): the conversion judges the radius it draws above an inside join, `min(p.radius, above)`, against `FINEST`, and wood thinner than that is not drawn, the module's own stated rule; `dress.wgsl` the same. The device, which runs no `validate_solved`, is checked to draw no radius-zero node on the oak at 80, seeds 3, 7, 30 and 39. The sweep has no such refusal, and lists every tree whose bytes change with the nodes removed; changes are expected only where a drawn tip was thinner than `FINEST`. A shipped preset that changes visibly at its own age is shown in stills and stops the work. The red test and the sweep example stay as the regression guard. Focused tests and review; the workspace gate is the host's.
- **R4 (count):** how many of the 486 nodes thinner than `FINEST` in the oak at 80, seed 7, the fix removes, and where the rest come from.

## Decisions

- **Fix A, in the conversion (host, 2026-10-14; `CAUSE.md`).** The code is brought to the module's stated rule that wood thinner than `FINEST` is not drawn; the engine's f32 girth sum is not changed.
- **Bytes (host, 2026-10-14).** R3's "bytes unchanged where they solved before" is replaced by the listed byte changes above (AGENTS.md, generator evolution: a correctness fix may change bytes).
- **The engine's phytomer budget (host, 2026-10-14).** The oak at 80 is also refused by the engine's phytomer budget at 6 of 50 seeds (`SWEEP.md`). That is not this fix: fn-236 is about to change the oak's values and its node count; the 80 sweep is re-run after them and reported, and the refusal by name stays meanwhile.

## Boundaries

Release item (fn-237). The oak's values are fn-236's.

## Open after fix A (host, 2026-10-15)

- At 80 the device and CPU oak node counts differ by a few nodes in 2.3 million (seed 3: +15; others +19, +35, -69), already before fix A. The age-12 agreement test is too young to see it. Untraced; a release-check item for fn-259, not a refusal.
- Ordinary at 80, seed 48 keeps 1 node thinner than FINEST. Untraced.
- Visibility judged from mesh counts, not stills: the most-changed tree at its own age loses 0.024 % of leaves and wood under 0.2 mm radius, bounds unchanged. Accepted by the host as below any supported view.
