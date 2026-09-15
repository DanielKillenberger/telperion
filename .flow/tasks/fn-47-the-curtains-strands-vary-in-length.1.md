---
satisfies: [R1, R2, R3, R4]
---
# fn-47-the-curtains-strands-vary-in-length.1 Implement The curtain's strands vary in length

## Description
Since fn-44 every strand in the birch's curtain ran the same 2.5 m to one
level hem. This task adds `skeleton.twigs.pendulousVariation` (0 to 1,
neutral 0): each hanging shoot runs its own share of the pendulous length,
drawn from the key the local law already uses (station birth identity and
bud place, xor the seed), with the sag's arc spent over each shoot's own
run. The birch states 0.95 over 3 m and, after host direction, a lumpier
shell (irregularity 0.35 at lobe scale 0.25) to roughen the hem.

## Acceptance
- [x] **R1** the row railed and refused by name, on the wire, blended, on
      the harness, in the regenerated metadata; every other preset
      byte-identical, the birch re-pinned with the reason.
- [x] **R2** runs spread between the pendulous length and a twentieth of
      it, keyed independent of growth order (direct, monthly and uneven
      builds agree), a short strand still ends vertical.
- [x] **R3** round 8b rendered and recorded in REPORT.md and
      `round8b-fn47/stills.json`; 48/48 protocol, heaviest birch seed
      107,569 nodes.
- [x] **R4** `tests/strands.rs` (six cases) and the rail in `pendulous.rs`.

## NEEDS_HUMAN — the hem is the shell

Commits 27e02b0, 49bd8e0, 0944780. Gates green: fmt, core (265), render
(99), clippy, typecheck, wasm bindings, compare self-test, harness vitest.
The worker's read after looking, honestly: strands of different lengths
show inside the curtain and the lower edge is uneven (18% of columns end
clear of the round bottom, against 7%), but the tree still reads as a round
crown with a lumpy lower edge, not a curtain falling to a ragged fringe.
While fn-44 keeps the curtain inside the shell, every column's longest
strands end on its lower surface. Letting the curtain hang past the shell's
lower surface to a ground clearance, or bunching it per limb, is a spec
decision for the owner; fn-44's worker removed the first because it broke
the repo's containment invariant, when strands ran only 2.4 m.

## Done summary
TBD, after the owner's verdict and the decision on the shell.

## Evidence
- Commits:
- Tests:
- PRs:
