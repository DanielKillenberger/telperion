---
satisfies: [R1, R2, R3, R4]
---
# fn-48-a-clumps-stems-lean-unequally.1 Implement A clump's stems lean unequally

## Description
fn-38's clump leaned every stem by one angle about one bearing, so the
birch parted in a symmetric V where the photograph has one near-vertical
stem and one leaning out. This task adds `skeleton.habit.stem_lean_spread`
(0 to 1, neutral 0): each stem's lean slides from fn-38's value toward its
place in stem order, the first at `stem_lean * (1 - spread)` and the last
at the full lean, for any number of stems. The birch states two stems,
divergence 0, lean 28 and spread 1; at seed 1 a divergence of 110 would
have pointed the lean straight away from the stills' camera.

## Acceptance
- [x] **R1** the row railed and refused by name, on the wire, blended, on
      the harness, in the regenerated metadata, in the habit audit and the
      sweep; all seven identity pins held at spread 0.
- [x] **R2** leans spread in stem order; the other reading was rejected
      because it leaves an odd clump's middle stem upright and puts two
      stems on one heading at full spread.
- [x] **R3** round 9b rendered and recorded in REPORT.md and
      `round9b-fn48/`; 48/48 protocol, heaviest birch seed 146,028 nodes.
- [x] **R4** `tests/stem_lean_spread.rs` and the stems tests.

## NEEDS_HUMAN — the owner's verdict on the combined round

Commits e5a4ca76, 283ec4e6, 50023362, 8c9af243. Gates green. The worker's
read after looking: yes, one near-vertical stem and one leaning out on
S-BARE, S-WHOLE and S-BARK. Still off and not this spec's: the curtain
hides the leaning stem sooner than the photograph (fn-51), the leaf mass is
dark (fn-52), the upright stem bends in a slow S from the birch's
crookedness row, and each seed picks its own bearing for the leaning stem.
The verdict comes on the merged round.

## Done summary
TBD, after the owner's verdict on the combined round.

## Evidence
- Commits:
- Tests:
- PRs:
