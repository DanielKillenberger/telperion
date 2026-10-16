# Tree space F2: the exact speed levers

## Goal

fn-198's cost table found 75 to 79% of grown phytomers are shed, 92 to 97% of those in subtrees whose shedding is certain at the bud's birth, growth single-threaded, and the oak's last-cycle light work wasted. The host (2026-10-05) asked for levers that make full fidelity cheap without changing any tree. Depends on fn-206.

## Decisions

- Host design (2026-10-05): skip subtrees certain to be shed where nothing reads them while they live (hash-identical); grow independent axes on all cores (lineage keys make order irrelevant); skip the last cycle's light work, gate the rough layout on species that read light; a compact phytomer.
- "Certain" (`shed.rs::lasting`, `grow/shoot.rs::shed_certain`): a lateral is skipped when its delay and the delays of every lateral or relay it stands on fit before the tree's age; unbounded where an axis bears its own PA, holds waking sleeping buds, or relays. Applies only where no bud reads light and no PA has `retained` or `leaf_girth`: beech, spruce, palm. The oak is excluded.
- Threads via `std::thread::scope` (no rayon), helpers drawn from the process's spare cores so concurrent trees do not each take every core.
- Not built: dormant cull (e1, not free), exact compact phytomer (e3, 4% of peak), one-record leaf shoot (e4, a design question).
- Found and built: the closed form reads each sleeping bud's year once (`Windows::new` was the oak's largest cost).

## Acceptance Criteria

- **R1:** Each lever with its hash proof — met for beech, spruce, palm and the parallel/light levers (identical hashes at 80 years, seeds 1 and 7, neutral and lit); the oak's shed subtrees under light not built, raised as a host design question.
- **R2:** Per-stage cost at 80 years, every species, before and after — met (table below).
- **R3:** Workspace gate; Codex review — met (gate 1,100/0; Codex SHIP rounds 1 and 2).

## Result

Totals at 80 years, before / after (seconds): beech 1 3.56 / 0.85 (lit 7.10 / 0.84), beech 7 5.82 / 1.36, spruce 1 16.95 / 3.40, spruce 7 16.70 / 3.32, oak 1 25.13 / 3.20, oak 7 31.45 / 7.17, oak 1 lit (production) 28.50 / 5.83, oak 7 lit 38.85 / 12.55. `Windows::new` fell from 20.5 s to 0.5 s of the oak's growth. Peak memory: spruce 6.45 to 2.26 GB, beech 1.95 to 0.56 GB.

The oak's design question: 56 to 62% of its growth is certain to be shed, but four couplings read that wood while it lives (lattice shade, vigour share, carbon balance, retained pipe girth). Skipping it with no stand-in changes kept wood by 11% and height by 1 to 1.5 m; a stand-in needs F1's closed form for each coupling.

Codex round 2's P2 (a caller always works its own spread, so 32 concurrent trees may run 63 threads) was not changed: bounding callers belongs to F4's forest.

## Open

- The oak's certain-shed subtrees under light: stand-in form per coupling, or wait for F1's R0.
- Remaining cost: the oak's rough layout, re-lays and settle under light; spruce and oak settle (sequential); sag levers.
- Caller-side thread bound (F4).

## Friction

- The spec was captured on the fn-198 branch, not the worker's base, and the dcg hook refused `git checkout/show` copies (~5 min) -> capture a phase's specs on the branch its worker starts from.
- Baseline timings ran under other sessions' load (1-minute load 61) -> before/after run back to back and repeated (~10 min); a machine-wide timing lock would remove it.
