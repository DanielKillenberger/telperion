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
