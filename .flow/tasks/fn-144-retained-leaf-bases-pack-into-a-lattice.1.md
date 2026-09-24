# fn-144-retained-leaf-bases-pack-into-a-lattice.1 Pack retained leaf bases into a lattice of flat-faced boots

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
Retained leaf bases can now pack into a lattice. Two new canopy rows, leafBaseWidth (0 to 2) and leafBaseFlatness (0 to 1), draw each base as its spiral's cell on the bark: the reduced parastichy parallelogram, wrapped on the stem and carried out as a wedge. At 0 both rows keep today's round peg. The shape rides the sweep's own rings through Tree::sections; `serde(skip)` keeps the specimen and snapshot bytes unchanged.

- **R1:** passes at 32, 96 and 256 bases on a straightened palm trunk. Widest gap and deepest crowding are at most 3.1e-6 of a cell, and cover is 1.00000. A negative control fails the measure at width 0.9 and at width 1.1.
- **R2:** neutral rows leave every shipped preset byte-identical; the gate is green.
- **R3:** code chose width 1.0 and flatness 1.0 for the palm. The P-TRUNK and P-BASE stills are in evidence raw/. The reviewer cell is pending.
- **R4:** the rows are in the dial table, and leaf-base-lattice is in the expressed list.
## Evidence
- Commits: 965b0403
- Tests: cargo test --profile ci --workspace --no-fail-fast
- PRs: