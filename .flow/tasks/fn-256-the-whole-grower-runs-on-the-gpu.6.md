# fn-256-the-whole-grower-runs-on-the-gpu.6 Fewer dispatches per cycle

## Description
COUNCIL.md round 4 step 4: re-fuse units kernels to a handful per round (cold compile under 2 s), collapse the hierarchy's narrow upper levels into one barrier-looped workgroup in the ruled order (words identical across caps), explain light's 46 dispatches a cycle; then step 5's re-measure of every preset against F4.

## Acceptance
At most 340 dispatches a cycle on Telperion; words identical across block caps; cold compile under 2 s; per-preset warm build against F4 reported.

## Done summary
Units kernels re-fused (3 dispatches a round counted, 5 emitted), the scan in at most two dispatches, the sweeps' narrow block levels folded into one workgroup, light's unneeded dispatches cut, compaction set at 0.85 from measurement, and the axis tables sized from the closed form's expected axes. Telperion goes from 954 to 417 dispatches a cycle; cold compile 2.19 s to about 2.0 s; every preset's stills are pixel-identical to task 4's.

Gates missed, for the host (NEEDS_HUMAN; PROOF.md task 6):
- 417 dispatches a cycle against 340. The rest is the five units rounds, the placing slack (measured as needed) and the units' range (held by the 2 ms bound). Three design options are listed.
- The oak at 80 still exceeds 4 GiB with its own axes hint. Its phytomer table is sized 4x what it needs, because the closed form is light-neutral.
- Cold compile measured 2.00, 2.04 and 1.99 s.

The Codex review returned NEEDS_WORK. Its only findings are these two gates.

## Evidence
- Commits: 3c240b2b07097f477a0a70f864b045df79625013, 9c7fb569f443af0ac7fb47d2fc1499f1e45b3c2b, cb655dcdb28b1ca2e8da6314e494fcdcb88572f7
- Tests: cargo test --profile ci --workspace --no-fail-fast, cargo test --profile ci -p telperion-render --lib (GPU lock), npm test
- PRs:
