# Native timing recipe

Baseline timing is unavailable: preflight before this worker's GPU activity reported `NVIDIA GeForce RTX 3080, 11 %, 984 MiB`. No sampling session or whole-tree build was started; this is not valid R4 evidence. Do not compute a before/after delta from absent values. Browser orbit remains separate and unmeasured.

The optional fixture timing uses the existing renderer `measure` API, with its fn-26 protocol: one initial render, eight conditioning frames, eight warmup frames, 120 measured frames. It runs on the shipped material before the depth-off diagnostic. The 400 × 400 flat patch covers 0.4 × 0.4 m; its timing is a patch cost, not a mature full-screen cylindrical trunk cost. Each full protocol JSON retains its validity verdict and unavailable/disjoint/contended status.

The current worktree contains candidate shader/material edits. These commands write candidate evidence only. A baseline must run pre-change revision `fe86a7a7` with the equivalent timing helper; never label the current candidate as baseline. R4 additionally requires a 1600 × 1000 full-screen trunk measurement matching fn-26; the 400 × 400 flat patch is diagnostic cost only.

Only after confirming an idle GPU, run from the worktree root:

```bash
BARK_CAPTURE_TIMING=1 BARK_CAPTURE_DIR="$PWD/.flow/evidence/fn-42-bark-depth-level-and-a-fair-capture/timed-candidate" CARGO_TARGET_DIR=/home/daniel/Projects/telperion/target cargo test -p telperion-render --lib capture_calibrated_bark -- --ignored --nocapture
CARGO_TARGET_DIR=/home/daniel/Projects/telperion/target cargo run -p telperion-render --example headless -- --preset oregon-white-oak --seed 7 --size 1600x1000 --view whole --out .flow/evidence/fn-42-bark-depth-level-and-a-fair-capture/timed-candidate/oak-whole.png --timing .flow/evidence/fn-42-bark-depth-level-and-a-fair-capture/timed-candidate/oak-whole-timing.json
```

Use identical camera/light/material rows, dimensions, adapter and multisampling for the candidate, into a distinct directory. Compare only valid reports. Extra cost is allowed but must be disclosed; no timing ceiling or changed tolerance is introduced here.
