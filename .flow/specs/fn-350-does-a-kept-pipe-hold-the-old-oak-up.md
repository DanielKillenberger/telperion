# Does a kept pipe hold the old oak up

## Goal

Measure whether the existing `retained` setting (the share of a shed branch's pipe that stays in its bearer; Shinozaki's disused pipes), which every preset holds at 0, stops the old oak's wood thinning and its stem being laid down by sag. A measurement on the fix branch; no product change.

## Context

- The owner asked on 11 Nov 2026 why wood gets thinner and said "ok" to the host's proposed check: the oak at 120, 200 and 300 on three seeds with `retained` at 0, 0.5 and 1, beside today's.
- On the fix branch (`fn-349-the-corrected-balance-in-the-product`, `210f97006`) the oak's trunk at 1.3 m falls from 137 cm at 120 to 36 cm at 300 in the probe, the stem loops to the ground at 300 on seed 1001, and the oak at 200 on seed 1012 is refused ("reaches 0.005 m below the ground").
- `retained` applies to shed branches (`crates/telperion-space/src/girth/disused.rs`). What happens to the pipes of fine tips that die and of wood that is let go (`retire.rs`) is unknown.

## Acceptance Criteria

- **R1 (the value reaches the oak):** `retained` is set on every growth stage of the oak to 0, 0.5 and 1 without a product change that would land: an existing override if one exists, otherwise a probe patch saved under this spec's evidence and reverted.
- **R2 (measured):** for the oak at 120, 200 and 300 on seeds 1001, 1007 and 1012 at each value: drawn height, spread, the trunk's diameter at 1.3 m, the base diameter of the largest limb, whether the build draws or refuses it (with the error), and one-thread seconds and peak memory for seed 1001 at 300.
- **R3 (shown):** leafed and leafless whole-tree stills on JPEG sheets, one per seed: today's build, then the fix branch at 0, 0.5 and 1, rows by age.
- **R4 (what the setting does not reach):** from the code and one measurement, say whether wood lost when a tip dies or a subtree is let go keeps its pipe in the girth below it at `retained` 1, with the trunk diameter at 120, 200 and 300 as the evidence: does it still fall?
- **R5 (the younger trees):** the oak at 40 and 80 on seed 1001 and the beech at 40 at each value, to see what the setting does to trees that are right today.

## Boundaries

- No product code lands. No preset value changes. No gate, no review.
- The host views every sheet and gives the verdict; the builder reports measures and a plain read.
- GPU and heavy runs only through `telperion-heavy`, never beside another GPU job.

## Evidence

`RESULT.md` with the tables and the read, the sheets as JPEGs, the probe patch if one was needed; raw stills under ignored `raw/`.
