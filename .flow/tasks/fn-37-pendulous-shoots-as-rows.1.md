---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-37-pendulous-shoots-as-rows.1 Implement Pendulous shoots as rows

## Description

The curtain the spruce and the birch already grew was a hidden mode: a
negative secondary rise flipped it on, its droop was a 0.35 cap and a 0.5
slope written into the twig advance, a pendulous shoot borrowed its length
from the twig anatomy, and neighbouring shoots stood a hard-coded four
degrees apart. This task turns all of that into four twig rows a value table
can set and the blend can walk, widens the canopy's four orientation rails to
their signed ranges so a leaf may hang under its shoot, and tunes the silver
birch's table against its S-WHOLE and S-BARE references with the new rows.

## Acceptance

- [x] **R1** `skeleton.twigs.hang`, 0 to 1, neutral 0. The pendant predicate is
      a function of it; the droop cap and slope scale with it; a family at hang
      0 is byte-identical to the tree it grew before the row existed, and the
      whole departure of a curtain lateral walks in from the one the twig law
      asked for, so no value of the row is the frame where a shoot changes
      kind. Refused by name outside its rail.
- [x] **R2** `skeleton.twigs.pendulousLength` (0.05 to 5 m),
      `pendulousRadius` (0 to 1 of the root radius) and `curtainSeparation`
      (1 to 45 degrees) are rows: railed and refused by name, on the wire, in
      the blend, in the browser metadata (regenerated, never hand-edited) and
      on the harness's own dials. The spruce's table states the values that
      reproduce the constants — hang 1, a pendulous length equal to its twig
      length, every shoot under a descending limb, four degrees apart — and its
      identity pin holds byte for byte.
- [x] **R3** `canopy.forwardLean` and `upward` accept -1 to 1, `leanRise` -2 to
      2 and `outward` -1 to 1; each is refused by its own name outside the
      widened rail. Every shipped preset at its current values is unchanged;
      the oak, the spruce, the beech and the Two Trees are byte-identical.
- [x] **R4** The birch's table uses the rows; its matched pairs are rendered
      again and the crown base and occupied numbers on S-WHOLE and S-BARE are
      recorded beside round 3's in `.flow/evidence/fn34/REPORT.md`. The pairs
      themselves stay on disk under the ignored `measure/` directory and are
      recorded by sha256 in `.flow/evidence/fn34/round4-fn37/stills.json` with
      `visual_status: unassessed`. **The owner's verdict is the open slot.**
- [x] **R5** Tests cover the predicate's continuity across zero
      (`tests/pendulous.rs`), the four rails, the floor under any hang, the
      spruce's stated constants, the oak-to-birch sweep walk (`tests/sweep.rs`)
      and the signed canopy rails on placements
      (`tests/foliage/attachments.rs`).

## Progress

The curtain moved out of the twig advance into
`crates/telperion-core/src/branching/local/pendant.rs`: a `Curtain` a shoot
carries, holding its hang, the bearing its laterals spread along and the floor
its first descending ancestor's tip set. The advance dropped from 387 to 367
lines and the new module is 112; nothing crossed the 400-line rule.

The hang row does not switch the pendant path on. A curtain lateral's whole
departure walks in from the departure the twig law asked for, so at hang 0
every expression is the one it was and at hang 1 every one is what the
constants gave. The shoot-length cap and the curtain's separation walk the
same way and are exact at both ends of the row; only the departure needs a
normalisation, which is why hang 0 and hang 1 are taken as themselves.

The birch was tuned in two passes against the S-WHOLE and S-BARE references,
reading one whole pair per pass. The first pass was too dense and too narrow
(occupied 0.61 against the photograph's 0.44); the second opened the curtain
from two and a half degrees to nine and the shell from 1.0 to 0.45, and widened
the envelope from 0.33 to 0.36.

### Decisions a reader should know about

- **The 0.8 floor clearance stayed a constant.** The spec's architecture note
  lists it among the magnitudes that "arrive as rows", but its API contract
  names exactly four twig rows and R2 names three of them. It is now
  `CLEARANCE` in the pendant module rather than two literals in the advance,
  and the spec's "never zero" holds trivially. If the owner wants it dialled,
  that is a fifth row and a new spec.
- **`pendulousRadius` is in the sweep's HELD list.** Every shipped table states
  1 — every shoot under a descending limb hangs — so no pair of presets moves
  it and the sweep cannot prove it walks. Measured: the row changes nothing
  above about 0.06 of the root radius, because a station only allocates
  laterals below `limbRadius` (0.1) already. A table that wants only its finest
  wood to weep has to go below that, and the birch does not.
- **The renderer's jitter fuzz got a larger budget, not a weaker assertion.**
  `parameter_sets_nobody_wrote_by_hand_render_the_same_way` multiplies every
  number by up to 1.25, so a row whose shipped value sits at the top of its
  rail is refused by name more often than not. With `pendulousRadius` at 1 on
  every table, 20 sets now need about 175 attempts instead of 60; the budget
  is 400 and the loop still runs in under a second.
- **The birch takes the beech's variation rule, and the rule became
  scale-free.** Its curtain reaches the shell on every seed, so neither crown
  dimension is pinned to vary; the threshold is now a thirtieth of the authored
  height, which is the flat metre it already was on the beech's 32 m envelope.
- **Commit 215352e's message names only the harness.** It also carried the
  three new test files (`tests/pendulous.rs`, the sweep walk and the signed
  canopy placement test). History is not rewritten; this is the record.

## NEEDS_HUMAN — the owner's verdict on the round-4 pairs

R4 reserves the verdict to the owner and this task cannot award it. The
round-4 pairs are on disk under `.flow/evidence/fn34/measure/pairs-fn37/` and
recorded by sha256 in `.flow/evidence/fn34/round4-fn37/stills.json` with
`visual_status: unassessed`.

Five of the eight comparable numbers moved toward the photograph, the crown
base most of all: S-BARE is within 0.004 of it and S-WHOLE halved its distance.
What the host read on the whole pair is recorded in the round-4 section of
`.flow/evidence/fn34/REPORT.md`, and it names a residual the rows cannot close:
the curtain's droop is capped at the value the constant already carried, about
nineteen degrees below horizontal, and the hang row only scales that cap down.
The birch's shoots are now long and settable but cannot point further down
than they ever could, so the crown reads as bristles standing out of a dome
rather than as strands hanging from one. Opening the cap is a row this spec
did not ask for; whether fn-34 closes on this round or waits for one is the
owner's call.

### Owner verdict, round 5 (2026-09-15)

Rejecting, on the judging page's S-WHOLE pair after round 5: "I think the
worst part is the hanging curtains are not affected by gravity or smth. It's
clearly wrong." Recorded in fn-34 under Owner verdicts. The reason is in
`pendant.rs`: a curtain lateral departs in one fixed direction, across the
crown and down by the droop, and then runs straight for its pendulous
length, so a shoot is a rod pointing sideways-and-down rather than a slender
stem bent to vertical by its own weight. The rows this task opened stand;
the missing term is a sag along the shoot's run, captured as its own spec
(pendulous shoots hang under gravity). The birch is compared again when it
lands.

## Done summary
The birch's curtain is stated as rows (hang, pendulous length, spacing) instead of constants; every other table is byte-identical at neutral. The owner accepted the silver birch at fn-34 round 25 (2026-09-16); the European beech's verdict moved to fn-62.
## Evidence
- Commits: 40e42bb5
- Tests: cargo fmt --all -- --check, cargo clippy --release --workspace --all-targets -- -D warnings, cargo test --release -p telperion-core --no-fail-fast, cargo test --release -p telperion-render --no-fail-fast, npm run typecheck, npm run rust:test:wasm, npm test, uv run scripts/compare-references.py --self-test, node tests/species.mjs --measure-only (48 cases, measure/protocol-round25)
- PRs: