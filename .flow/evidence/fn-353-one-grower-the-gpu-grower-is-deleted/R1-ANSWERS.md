# fn-353: the host's answers to R1-EQUALITY.md, 11 Nov 2026

The host's design decisions; none is the owner's ruling.

## 1. The equality: option 1 here, and the gap goes to the goal

The finding is right and it is not this spec's to fix. The grower takes the age asked for as its horizon (`grow.rs:201`, `grow/cycle/paused.rs:203`), so the tree at year y on the way to 12 is not the tree asked for at age y. That is the goal's criterion P4 unmet, on both growers, since before this spec. Making the grower ignore the horizon changes how trees grow and gets its own child of fn-351, surveyed first.

For this task, R1's test holds what is true today and no more:

- a year drawn twice from the same growth is the same tree (repeat);
- a growth paused and resumed reaches the same tree as one grown straight through;
- at the horizon the grown-on tree equals the tree asked for at that age (the existing test's claim, through the new binding).

The test's doc comment says, in one line, that the years before the horizon are the tree on its way to that age and names P4 of fn-351. Option 2 (a new growth per year) is not taken: it would make grow-in cost the sum of every year's tree to hide a property the grower does not have.

Keep `R1-EQUALITY.md` and the probe patch; the goal's P4 child starts from them.

## 2. The browser handle owns its growth without `unsafe`

No self-referential owner and no `unsafe`. The growth state owns what it reads: where `GrowthState` and the grower borrow their `Params` and species, they hold them by shared ownership (`Arc`, or `Rc` if nothing crosses a thread) so a handle can keep a growth between calls. If that turns out to be more than a mechanical change of a borrow to a shared pointer (a lifetime replaced along one chain of types, no behaviour change, the pinned digests unchanged), stop and return with what it touches.

## 3. The harness's draw order

As you planned: draw, wait out the 150 ms floor, then grow the next year, so the canvas repaints between years.

## 4. For task 2

Noted: the still runner grows through `executor::grow(&family)` and `.expansion()`; task 2 keeps those or moves the still runner first.

Go on with R1 (as restated), R2 and R3.

## 5. Ownership: option A

The host's decision on `R1-OWNERSHIP.md`, in its words:

The grower owns its species: `Paused` and `Grower` hold an `Arc<Species>`, `grow_cycle` clones the species into it once per tree, `Rules` drops its species and `Shoot` reads it from the grower. Reason: one way of holding the species for every caller (no type parameter standing in for a lifetime, no second flavour of grower), and a species is a small value table, so one clone per tree is nothing beside growing it. Hold that claim to a measurement: the oak at 40, seed 1001, one thread, grow time before and after, three runs each through telperion-heavy; if the median moves by more than 2%, stop and return with the numbers. Hold behaviour to the pinned digests: the existing digest and repeat tests in telperion-space and telperion-core pass unchanged, with no baseline edited. At the borrow sites that copy the species out and then change the grower, clone the Arc; do not restructure the passes. If anything beyond ownership has to change (a pass's order, a public signature outside telperion-space and the callers you listed, a test's expectation), stop and return with it.

Do this as its own commit(s) first, then R1 as restated, then R2 and R3.

## 6. The measure, the field's source, and landing R1 to R3

The host's answers to `R1-OWNERSHIP-MEASURED.md`, in its words:

(1) The ownership change is accepted. The 2% rule was the host's mistake: it was two-sided and tighter than the runs' own spread (8 to 13%). The claim it was meant to test, that one clone per tree costs nothing that shows, holds: the median did not rise. No more timing runs for it.
(2) `compileField` and the field's `source` option stay, with `TreeEngine.create` accepting a compiled `WebAssembly.Module`: sharing one compiled module across workers has nothing to do with the device, and the field's contract is unchanged.
(3) Now verify the parked R1 to R3 and land them on the task branch: build the Wasm modules, typecheck, run the node tests that cover what you touched (field, test-dist, render unit tests, the harness's own), and the narrow Rust tests for the new growth test and the binding crate. Fix what is red. What only a browser with WebGPU can show (the binding drawing a year, the harness's controls) stays unverified by you: list exactly those items in the done summary so the host can have them checked. Then finish the task (flowctl done) if R1 to R3 hold; otherwise return with what is red.
Keep heavy runs inside telperion-heavy at a size that fits beside the scoreboard job running from the fn-319 checkout; never raise a cap to get past a kill, lower the test threads instead.

## 7. The wood's programs compile inside `createRenderer`

The host's decision on task 2's kept `preparePrograms`, in its words:

`preparePrograms` and the browser renderer's public `programs` member go. The wood's surfacing passes are compiled inside `createRenderer` and awaited there, so a renderer that has been created can always draw a CPU tree, and a caller has nothing to prepare. If `createRenderer` cannot await that compile without changing its public signature (it is already asynchronous if it returns a promise), stop and return with what it would change.
