# fn-201 friction

## 2026-10-04: the Codex reviewer cannot run the GPU test it reviews

Doing: the impl-review of task .1. Hindrance: the reviewer's sandbox has only llvmpipe, so `Gpu::request` returns `FallbackOnly` and every GPU test, including the new past-the-limit selection test, skips there. The review judged R2 and R3 as "partial" on reading alone, and its one finding (a 128 MiB binding limit) was a device it reasoned about, not one it ran. Cost: one extra review round (about 6 minutes) for a test-only guard. What would remove it: hand the reviewer the implementer's GPU test log (the red base run and the green run) as evidence in the review prompt, so it weighs a run instead of a skip.

## 2026-10-04: the anatomy check caught a doc comment only on the crate run

Doing: the crate tests after the fix. Hindrance: `conformance.rs` refuses anatomy words in `select.rs`, and a doc comment naming the spruce's needles failed it; the focused `select::` run did not include that test. Cost: about 4 minutes, one crate run and a rerun of the conformance test. What would remove it: nothing structural; the check is doing its job. Noted so the next agent touching `select.rs` runs `--test conformance` with its focused tests.
