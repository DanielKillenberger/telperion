# The device builds every tree the CPU builds

## Goal

fn-272's film (defect 1, 2026-10-14): the device refused trees the CPU builds — beech to Norway spruce at age 40 at walk t = 0.00037, 0.01 and 0.1 (seeds 7 and 1) with "UnitRounds (0x1)", and the Laurelin alone at 20 with "Given (0x1)". The harness and site build on the device, so a dial the owner can reach refused a real tree. Reproduce, find the cause against the CPU's counts, size the tables so every tree the budget holds builds on the device, and sweep. Release item (fn-237).

## Decisions

- Host (2026-10-14), fix 1, the species floor: the floor drops the failure term (it was about twice the CPU's count and cost Telperion 6.4 % of its warm build); each cycle's limit is the larger of the observed projection and the floor of units a job grows where none fails; a failure relay past both is still refused by name; `FRAMES` rises to the smallest value holding every floor in the sweep (stop above 16); CPU census counters stay only where a test reads them.
- Host: fix 3, the node table grows by the box its dispatch is given, its memory charged to the ledger.
- Host: the made-phytomers dispatch takes its span from the phytomer table's sized counts (the nine Given refusals first confirmed red at 850eb039a).
- Host: the walk observation (a lifespan of 0 mixed by value) is not fn-276's; recorded for the walk specs.
- Host (task 2, from the sweep): a build's start margin leaves room for its largest first move (`Sizes::first_move`, checked with `Ledger::fits`), the same ladder, no species branch.
- Host: the gate queued once; if not admitted in 30 minutes it is skipped and the host runs it on the merged branch.

## Acceptance Criteria

- **R1:** each refusal reproduced as a standalone device test, red, with the CPU build of the same params succeeding — met (`generation::cpu_builds_tests`, seven families).
- **R2:** the table or limit each passes, against the CPU's count, written before any fix — met (UnitRounds: three stepped limits, units rounds, layout and hierarchy generations, sized from presets at default ages with slack 2, exceeded on walk frames; Given: the made-phytomers dispatch).
- **R3:** tables sized so every tree the CPU builds within budget builds on the device; over-budget trees refused by name before submit; no species branch or CPU fallback — met.
- **R4:** every preset at every whole age to its horizon and to 80, and the film's walk frames at seeds 1, 7, 42, built on the device; none refused except by the budget — met (2,034 cases).

## Result

Three causes fixed: the species floor (`telperion_space::most_units`) beside the observed projection for the units rounds and both generation limits (a job holds 11 frames); the node table grows by its dispatch's box; the made-phytomers dispatch spans the phytomer table. Sweep: 2,034 cases, budget refusals only, CPU count at or under the floor everywhere. Cost: Telperion 82 GPU time +0.8 %; peak 2,484 → 2,631 MB. Task 2: the spruce at 62 (red, 4,203 of 4,096 MiB) builds with room for its first move; the spruce at 60-80 at 6,144 MiB builds at all 63 seed/age points (card cap 5,640-5,838 MiB); at native 4 GiB it builds to 72 except 69 at seed 42, later ages refused by name at later moves; the other 1,794 cases unchanged; GPU time unchanged. Gate 1,383 passed, 1 false failure (a shader edited during the run; passes on HEAD); codex SHIP.

## Open

None (the full gate is the host's on the merged branch).

## Friction

- A refusal names only the first flag raised; three device runs to find three limits -> test switch recording every stepped limit with counts.
- The CPU census had no per-apex units count -> count it in the census.
- dcg refused variable-path redirects -> local setup.
- CPU check of every preset at every age OOM at 8 GB (two spruces near 80 at once) -> size workers by expected tree size.
- Editing a shader while the gate ran failed `every_shader_file_is_checked` -> run the gate from a frozen copy or hold edits.
