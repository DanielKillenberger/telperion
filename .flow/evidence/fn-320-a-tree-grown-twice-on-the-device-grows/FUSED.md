# R1: why a tree grown twice on the device could grow another field

Measured on 1 Nov 2026, RTX 3080, driver 615.78.08, at `a0d446d73` plus the probes, native, `--profile ci`. Raw logs are in `raw/`. The probes are kept in `probes/`.

## The cause

One program, on one input, answered two ways. On this GPU the same compiled compute program gave two results a rounding step apart, from one run to the next. Passing each marked product and quotient through its bits XORed with a uniform zero, which stops the driver fusing it into the add that takes it, removed the second answer.

So the class is **a fused multiply-add taken on some runs and not others**. That it is fusing is inferred from the fix and from the driver's known fusing (fn-256 `UNITS.md`). Why the same program runs two ways is unknown: nothing here shows the driver's mechanism, and it is not visible from the WGSL.

The spec named two possible classes. Neither is the cause as it stood:

- **A moved box:** an effect, not a cause. A rounding step moves positions below it, and with them the box.
- **An order-dependent float sum:** not found. The read-out's leaves are fixed-point sums, and the 2.4e-4 in the spec is one `1/4096` unit. Children and members are ordered with their ties broken.

`INVESTIGATION.md` (28 Oct) read the same two fixed states, bunched in time, as "a read that sees the old or the new value by GPU timing". Step 6 below does not support that on the path traced here: one invocation, on restored words, still gave both answers. Its first divergence, `sizes_main` over `partials_main` and `combine_main`, reads ordered sums and goes through `exp2c` and marked products, so it takes the same fix. Its bunching matters for reading these results: it saw 15 of 40 in one run and 0 of 240 in two others. Before the fix, 2 of the 8 oak runs here also showed nothing (`probe4`, `probe8`), so a clean result counts only across several processes.

## How it narrowed

| Step | What ran | Result | Log |
| --- | --- | --- | --- |
| 1 | Oak 5678, 40 repeats, by stage | differs from the growth or settle on, about 1 in 10 | `probe3` |
| 2 | 120 repeats | 10 differ, at irregular intervals, in only a few exact variants | `probe5`, `probe9` |
| 3 | One grown tree settled 100 times, by stop | `Shed` 0, `Placed` 9, always the same 36,737 words | `probe12` |
| 4 | First differing axis | axis 656, depth 4, one rounding step | `probe14` |
| 5 | One depth's lay replayed from saved words, 2,000 times | 2 answers: 1,971 and 29 | `probe16` |
| 6 | Axis 656 alone, one invocation, 3,000 times | 2 answers: 2,979 and 21 | `probe17` |
| 7 | Its walk word by word | parts at one step, in `curve` only | `probe18` |
| 8 | Marks inside that step | `keep` is `3f7d1f88` or `3f7d1f89`, its inputs equal; `keep` is `exp2c`, a plain polynomial | `probe20` |
| 9 | That step alone, unkept, 100,000 times | 3,353 take the second answer | `probe22` |
| 10 | The same, kept apart, 200,000 times | one answer | `probe24` |

Step 6 rules out a race between invocations: one invocation ran, on restored words. Marks written out inside the polynomial also stopped the flip in 60,000 replays (`probe21`), which fits, since a value written out cannot be fused away.

**What was ruled out:** a missing barrier on these paths (the sweeps that lack `storageBarrier` read workgroup memory across it); child order ties; the built-ins `sin`, `cos`, `sqrt`, `atan2`, division and a product difference, each over 262,144 fixed inputs 1,500 times with no second answer (`probe19`, short and in isolation, so it is weak evidence for those built-ins inside a larger program).

**Clock speed does not explain it:** all 7 differing repeats sat at 660 to 765 MHz, with many matching repeats at the same clocks and above (`probe15`). The second answer comes in bursts, whose trigger is unknown.

## The fix and what it costs

`cycle/kept.wgsl` keeps the marked arithmetic apart, in the modules that do the contract's float work: units, layout, sums, retirement, settle and place. The modules that use the contract only for its keys stay plain.

This narrows host ruling 9.1 of 12 Oct 2026 (fn-256 `DESIGN.md`), which deleted exact mode: "It cost 34% more GPU time and 96 s of cold compile, for bit equality that is not needed." That ruling is about bit equality with the CPU, and it assumed the device repeats itself. Step 6 disproves that. This is not exact mode: division is not done in integers, and the tolerance against the CPU still stands.

| | Plain | Kept |
| --- | --- | --- |
| Four fields, warm, sum of medians (`cost-*`) | 1,048 and 1,076 ms | 1,067, 1,069 and 1,070 ms |
| First field, driver's disk cache off, three runs (`cold-compare.txt`) | 6,144, 6,280, 6,286 ms | 6,454, 6,507, 6,592 ms |

So there is no measured warm cost, and about 4% on the cold compile. The 2 s cold-compile target in ruling 9.3 was already missed on this measure before this change (6.2 s, the first field's whole time, not the compile alone).

Unmarked products remain in other shaders (`light_sweep.wgsl`'s bilinear sums, the dress, the stations). None parted in any run below, but they are not kept apart, so the guarantee is measured rather than proven.

## Results after the fix

See `RESULT.md`.
