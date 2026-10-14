# The device builds every tree the CPU builds

## Goal & Context

fn-272's film (FILM.md, defect 1, 2026-10-14): the device build refuses trees the CPU builds. Beech to Norway spruce at age 40, walk t = 0.00037, 0.01 and 0.1 at seeds 7 and 1: "the build passed its sizes at cycle N: UnitRounds (0x1)" (t = 0.5, 0.9 and both ends build). The Laurelin alone at age 20: "Given (0x1)". The harness and the site build on the device (fn-257), so a dial the owner can reach refuses a real tree; a walk refused on the device runs only on the slow CPU path (Laurelin to Telperion took 1,203 s on the CPU in the film). Both are size-table refusals (fn-270 R4's tables sized by observed counts), unchecked: whether the tables' estimate, a cap or the walk's blended parameters cause them is unknown.

## Requirements

- **R1 (repro first, standalone):** each refusal reproduced as its own test on the device, red; the CPU build of the same params and seed succeeds in the same test.
- **R2 (cause):** the table or limit each one passes, against the count the CPU build measures; written down before any fix.
- **R3 (fix by degree):** tables sized so every tree the CPU builds within the device budget builds on the device; a tree over the budget is still refused by name before submit where its least tables can't fit (fn-270's rule). No species branch, no fallback to the CPU.
- **R4 (sweep):** every shipped preset at every whole age from 1 to its horizon and to 80, and the film path's walk frames at t = 0, 0.01, 0.1, 0.5, 0.9, 1 for every adjacent pair, at seeds 1, 7 and 42, built on the device: none refused except by the budget, named.
- **R5:** gate, npm test, review.

## Boundaries

Release item (fn-237): the harness and the site reach these trees. Speed of the CPU walk path is not in scope.

## Host rulings (host, 2026-10-14), on R2 (`.flow/evidence/fn-276-the-device-builds-every-tree-the-cpu/CAUSE.md`)

- **Fix 1, the species floor: yes.** The units rounds and the hierarchy's and the layout's generations read the parameters' own bound, a closed form over the age table, alongside the observed projection. A tree's structure then sets its limit by degree, and fn-256 task 12's saving stays. Before it is trusted, check that the closed form is at least the CPU's measured count:
  - on every preset at every whole age to its horizon and to 80;
  - on R4's walk frames;
  - with relays after failed units included.

  A single case under the CPU's count is a stop and a report, not a margin. A species whose floor passes a hard cap is refused by name before submit. Measure its cost against fn-256 task 12's saving.
- **Fix 2 (limits at their bounds every cycle): no.**
- **Fix 3, the node table: yes.** One rule with the dispatch span: the box projected side by side. Measure its memory cost per preset, charged to the ledger that the fn-270-grow-budget branch fixed. Merge that branch's `buffer.rs` fix if the ledger is touched, or tell the host.
- **The walk observation** (a lifespan of 0 mixed by value): noted, and not fn-276's; the host records it for the walk specs.
- **Then:** R4's sweep, the gate (`telperion-heavy 9 --gpu 4 --threads 4`), npm test if TypeScript changed, codex review to SHIP, `flowctl done`.
