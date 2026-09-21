# Progress review protocol, 2026-09-21

An uncalibrated comparative question. No replay qualifies it, so a run may ask
it only under the scoped experimental authority, every record of it carries the
word uncalibrated, and each review spends a visual pass.

## What the reviewer is shown

The reference photographs for one view, then two renders of the same generated
tree at that view and the run's seed, labelled A and B in metadata order. The
request names the species, the view, the seed, the owner's notes and the
approved priorities the router last sent to tuning, each as an id and the
observation the owner approved.

It does not say which render is the candidate, which is newer, which round
either came from, what any measurement says, or what the loop hopes to see.
Code decides the sides from the two trial keys together, records the assignment
on the trial as `candidate_is`, and maps the answer back.

## What it answers

Per priority: `a_better`, `b_better`, `same`, or `unknown` when the view cannot
show it. Then one or two sentences on what differs for the better, what is
still missing in both against the references, and a list of anything one render
breaks that the other does not, naming A or B. No numbers, no scores, no
overall winner.

## What code does with it

Every requested priority must be answered exactly once and nothing else may
appear; an answer that does not bind is a failed attempt, recoverable only by a
scoped decision, never a dropped row. A candidate is adopted when the reviewer
judged it better on at least one priority, worse on none, and named nothing it
breaks. Among eligible candidates the most `better` wins, ties keeping the
order the proposals arrived in, which is their direction-mass order. A round
with nothing eligible is a visual stall, and the reviewer's words go to the
next proposal and the next routing call.

The five still-side numbers stay in the trial table as telemetry. They no
longer decide, and the reviewer never supplies a number.

## Adapter

`scripts/progress-review-codex.py`: one isolated call, no tools, images in
metadata order with their hashes checked, the prompt and request hashed both
ways, the receipt written before the answer is read.
