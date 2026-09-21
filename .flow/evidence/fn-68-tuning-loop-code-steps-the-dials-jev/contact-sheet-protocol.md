# Contact-sheet review protocol, 2026-09-21 (`tuning-sheet-v2`)

An uncalibrated comparative question, under the same terms as the side-by-side
progress review it replaces in bundle mode: a run may ask it only under the
scoped experimental authority, every record of it carries the word
uncalibrated, and each sheet spends a visual pass.

## What the reviewer is shown

The reference photographs for one view, then two to five renders of the same
generated tree at that view and the run's seed, numbered 1 upward. The renders
are the tree the loop is standing on and the strength variants of one bundle.
The request names the species, the view, the seed, the owner's notes and the
approved priorities the router last sent to tuning.

It does not say which render is the current tree, which strength any render
is, what order they were made in, or what any measurement says. Code decides
the order from all the trial keys together, records it and the current tree's
label outside the request, and maps the answer back.

## What it answers

Per priority: the render closest to the references, a ranking of every render
best to worst, and a grade for each adjacent pair of that ranking - `clear`
when the better one is plainly better on that priority, `slight` when the
difference is real but small, `none` when they cannot be told apart.

Then one more ranking of every render, best to worst, on a different question:
which is the most believable tree of this species overall, judging the whole
tree rather than the listed priorities. Then, for every render, at most two
things that look wrong in it against the references. Then a list of anything
one render breaks that the others do not, naming the render, and one or two
sentences on what improved across the set and what is still missing in all of
them. No numbers, no scores, no overall winner beyond that ranking.

The overall ranking exists because the priorities are two of a tree's
qualities and a move can win both while losing the tree: on 2026-09-21 the
sheet graded three bundles clear on crown shape and the owner called the third
"just garbage".

## What code does with it

Binding is strict: every priority answered once, every ranking a permutation
of the labels on the sheet, one step per adjacent pair of that ranking, and no
break naming a render that is not there. An answer that does not bind is a
failed paid attempt, recoverable only by a scoped decision.

A variant is `clear` over the current tree when any adjacent step between them
on the ranking is clear, `slight` when none is clear but one is slight, and
`none` when every step between them is none; a variant ranked below the
current tree is `worse` unless every step down to it is none.

A variant is adopted when it is clear or slight on at least one tuning
priority, worse on none, breaks nothing of its own, and is not ranked below
the current tree overall. Clear beats slight,
then more priorities improved, then the smaller strength. When the best
variant is better but breaks something, the bundle is halved and the halves
are tested on one more sheet until the breaking dials are isolated or the
split budget is spent.

## Adapter

`scripts/contact-sheet-codex.py`: one isolated call, no tools, images in
metadata order with their hashes checked, the prompt and request hashed both
ways, the receipt written before the answer is read.
