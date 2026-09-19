# Generation budgets

Generation honours the caller's `skeleton.growth.maxNodes` budget. Its default remains 250,000; it is no longer a ceiling on an explicitly larger budget. The valid range is 0 through `u32::MAX`, the node-index representation. Exhausting this budget still reports `node_capped`. There is no independent headroom estimate that reduces it, 4,096-unit axis truncation, or unreachable twelve-order truncation. Foliage uses `maxInstances`, checked index arithmetic and fallible reservations instead of per-axis 512 and per-wood 65,536 limits. Zero short-shoot spacing disables short shoots; positive spacing is not raised to a hidden floor.

Former form/work limits are now ordinary authorable, blended, serialized parameters. Defaults preserve existing valid input behaviour:

| Parameter | Default | Valid range |
| --- | ---: | --- |
| `skeleton.twigs.maxInternodes` | 32 | 1–u32::MAX |
| `skeleton.twigs.maxDroop` | 0.35 | 0–10 |
| `skeleton.twigs.curtainStepClearance` | 0.8 | 0–1 |
| `skeleton.habit.reachProbeSteps` | 96 | 1–u32::MAX |
| `skeleton.samplingAttemptsPerAttractor` | 64 | 1–u32::MAX |
| `skeleton.bias.supernatural.maxWritheMagnitude` | 0.9 | 0–8 |
| `radii.maxTaperExponent` | 12 | 0–64 |
| `surface.socketContainment` | 0.9 | 0–1 |
| `growth.workBudget` | 250,000 | 1–u32::MAX |
| `canopy.clumpSystemOrder` | 2 | 0–u32::MAX |
| `canopy.clumpNeighbours` | 12 | 1–u32::MAX |

The growth budget controls the internal lifetime quantization; the mature direct-build path does not run that timeline. The Jev growth fitter currently assumes this default work budget. Reach probes use the existing height/64 spatial resolution. Clump neighbours governs an approximation's neighbour/traversal work, not a guarantee about the globally nearest limb. Socket containment scales the geometrically inscribed socket bound. Taper's upper bound leaves finite exponential headroom even with the fork exponent's existing maximum and the largest representable node count.

Touched Twig/Radius inputs that were silently clamped now fail with their field name outside the same declared intervals. Fullness endpoints 0 and 1 and positive shoulders use their authored values; endpoint arithmetic is handled explicitly. Writhe wavelength and spiral rate are no longer adapted to an eight-step sampling rail. Non-finite derived arithmetic is refused rather than silently reducing an authored value. Bounds and defaults live in `telperion_core::ranges`.

New fields default individually when reading legacy JSON. Binary specimen snapshots change from version 2 to version 3 because their serialized parameter layout changed; old binary versions are explicitly rejected, not interpreted as the new layout. The browser snapshot wrapper is version 3 as well. Regenerate old snapshots from parameters.

Wasm generation-count diagnostics now contain `twigs.generations + 1` entries instead of thirteen slots from the removed depth constant. No reachable order is discarded; consumers should use the returned array length.

## Audit boundary

`generation-limits-inventory.json` classifies the surviving limit-shaped source sites. The regression guard scans literal/named clamps, limit constants, fixed iteration budgets and literal early termination. Its mutation fixtures demonstrate detection of new literal clamps, constant ceilings and fixed-loop truncation. This focused lexical check is not proof against every possible semantic cap; a new algorithm still needs review.

The inventory distinguishes authoring domains and geometric containment from numerical root-search precision, packing/index representation, RNG constants and algorithm coefficients. Snapshot input-size security limits and occupancy-measurement work guards are not generation limits. The fn-31 shed bound is explicitly outside this task; renderer GPU limits are also outside the core/Wasm generation audit. Preset population bands were removed, while explicit resource-contract assertions and deterministic geometry hashes remain.
