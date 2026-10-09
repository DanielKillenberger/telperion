# Trunks without weird dents

## Goal & Context

The owner, looking at the release branch's trees in the harness (2026-10-09): "Most trunks have some weird dents in them, that look weird." Which part of the wood surface makes them is unknown: candidates are the lobed cross-section winding along the stem (the curve's "one swept surface, lobed section winding along its own length"), the root flare and buttress, bark relief or plates (fn-221), the curve's per-frame surfacing at the screen error, or a seam between rings. Host (2026-10-09): release blocker.

## Requirements

- **R1 (find it first):** trunk close-ups and mid views of every shipped species at 40 and 80 (seeds 1, 7, 42), viewed; the dents located and the stage and rows that make them named by switching each candidate off in turn (lobes, flare, bark relief, surfacing error), checked against the code; reference photographs of real trunks of the same species beside them. If the cause is a row value the species set, the fix is values; if it is the surface's construction, the design of the fix goes to the host before code.
- **R2 (fixed):** the trunks read as smooth or as their species' real bark form at the harness's views; every parameter still changes the tree by degree; the CPU and device paths agree within fn-125's screen.
- **R3 (proved):** the same stills before and after, viewed; the owner's verdict; gate and npm test green.

## Boundaries

The trunk and limb surface only. Release blocker for fn-237.
