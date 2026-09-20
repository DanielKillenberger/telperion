# Final validation

The optimization search has ended. Browser completed-frame medians are 158.1/180.8 ms for oak seeds 1/7 (9.93×/10.30× the original baseline) and 178.9/164.2 ms for spruce (42.64×/44.97×). Oak seed 1 misses strict 10× by 1.1 ms; it was not retimed to claim a crossing. These measurements use an initialized desktop renderer at 1280×720, not cold page startup.

## Checks

- Full Rust gate: `/tmp/telperion-fn91-tools/cargo-nextest nextest run --cargo-profile ci --workspace --test-threads 8 --no-fail-fast` passed 742 tests, with 20 explicitly skipped, in 102.375 seconds of test execution. The existing CI profile preserves optimized tests without expensive release LTO. See `final-rust.log`.
- JavaScript gate: `npm --ignore-scripts test` passed the catalogue structure check and 108 tests in eight files. Both current Wasm modules had already been built by the worker, so the duplicate pretest build hook was omitted. See `final-js.log` and `stations/wasm-final.log`.
- Both release Wasm builds, TypeScript checking, scoped Rust formatting, browser lifecycle smoke and actual post-submission fallback/error regressions passed.
- All four mature full station-record comparisons and all eight CPU/GPU-assisted foliage-output pairs match exactly. Earlier GPU wood arithmetic differences remain within the recorded accepted numeric/visual evidence; the entire original CPU/GPU tree is not claimed byte-identical.
- Two-axis final review found no correctness issues and two API documentation/type issues, both resolved and type-checked. The out-of-axis test typo was caught by the full build and corrected; its failed first build is preserved. See `FINAL-QUALITY.md`.
- The smooth-bark test's fresh visual receipt is preserved as `final-smooth-bark.json`; the unrelated historical fn-71 receipt was restored.

## Qualification and tracking

Tasks .2–.14 are verified done. The original all-requirements task .1 is blocked on explicit remaining qualification, rather than left running: strict oak seed-1 and CPU-owned targets, observed memory increases, cold startup, phone and full-memory evidence. No sixth original invocation or automatic follow-up optimization is authorized. Spec validation passes with 14 tasks and no errors or warnings. The spec remains open; no PR, push or merge was performed.

Live CPU/GPU capacity envelopes remain unchanged in .14, but oak seed 7 Wasm linear-memory high-water rises 26,279,936 bytes and native GPU-assisted CPU-owned oak seed 1 RSS rises 40,084 KiB. These separate observations prevent a universal memory-nonincrease claim. The full requirement assessment is `COMPLETION-ASSESSMENT.md`.

Every friction entry has been reviewed in `FRICTION-REVIEW.md`, including the final verification-flag omission, test rename typo and historical test-output rewrite. Proposed prevention remains a cheap-first gate protocol, validated benchmark schemas and isolated artifact output; no unrequested friction specs were created.

## Flow report

Flow stopped at: the practical optimization milestone, with original qualification gaps explicit.

Route taken: work → focused qualification → final quality and aggregate validation.

stage: work — ran, .13/.14 retained; rejected candidates remain archived.

stage: implementation review — skipped(config: review backend none); host direct review and two-axis quality audit ran.

stage: completion review — skipped(config: review backend none; original all-requirements task remains blocked).

stage: QA — browser lifecycle and existing accepted visual/numeric qualification ran; no new image approval was requested for exact outputs.

stage: make-pr — skipped(policy: original qualification task remains blocked).

Gates: full Rust and JavaScript gates ran green; no cached receipt was used.

Tracker sync: n/a (bridge inactive).

Next: preserve this measured implementation as the finished optimization milestone; remaining qualification requires a separate scope decision.
