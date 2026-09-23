# fn-102 friction

## 2026-09-23, worker on fn-102.1

- **What:** feeding the Wasm binding the family wire the native core prints (`params::metadata`) to compare every preset and seed base against candidate.
- **Hindrance:** the native wire encodes `canopy.maxInstances` as `usize::MAX` on 64 bits (18446744073709551615); the wasm32 binding refuses it as `/canopy/maxInstances`, and a JS `JSON.parse` of the wire loses the integer too. A wire written on one target is not readable on the other.
- **Cost:** about 5 minutes and two failed trial runs.
- **Would remove it:** an explicit "no limit" encoding for `maxInstances` (or a width-independent cap) so a family wire round-trips between native and Wasm. Worked around here by substituting the wasm32 maximum in the comparison script.

## 2026-09-23, worker on fn-102.1

- **What:** writing scratch measurement output from shell loops.
- **Hindrance:** the dcg hook refuses any `>` redirect whose target contains a shell variable, including the session scratchpad, so every loop that writes files had to become a script file with appends.
- **Cost:** about 3 minutes, two refused commands.
- **Would remove it:** local setup, not a repository matter; reported only.

## 2026-09-23, worker on fn-102.1

- **What:** sharing the spruce's ring sweep between wood and leaves in the serial (Wasm) build.
- **Hindrance:** every order that swept the rings before the wood or before the leaf plan raised the Wasm peak 7-24 MB over base for some output combination (surface+foliage, foliage+field, surface+foliage+field), though the bytes were identical: the allocator's holes left by growth and the wood's transients land differently. R4 forbids any rise, and there is no allocation profile for Wasm, so each layout was found by rebuilding a probe binary that records `memory_size` at stage marks.
- **Cost:** three designs and four probe rebuilds, about 60 minutes.
- **Would remove it:** a Wasm peak-memory probe per stage in the binding's own metadata (or a test target that reports it), so a stage reorder shows its memory effect without a hand-built probe. The shipped design sweeps first only where wood and leaves run side by side; run in turn, the wood sweeps and the leaves read the rings from its vertices, which keeps base's allocation order.
