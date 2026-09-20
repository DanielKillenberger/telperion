# Host friction review — 2026-09-20

Reviewed every entry in FRICTION.md before this checkpoint. The task remains open. The proposals below are for owner consideration; no new friction specs were created.

| Recorded friction | Outcome and proposed prevention |
|---|---|
| Missing reusable baseline stage runner (~2 minutes plus compile) | Runner and evidence retained; reuse them. |
| Wasm builds (about 14–15 seconds each) and missing completion fence | Actual queue fence now in benchmark; retain separate initialization and completion timings. |
| CPU focused gate compile cost, fingerprint delimiter and test-helper mistakes | Retain binaries; run cheap compile/schema checks before release builds. |
| Curved fixture below minimum width (~30 seconds rebuild) | Read parameter constraints before creating fixtures. |
| Browser origin/module failure (~1 minute), heredoc guard friction | Navigation guard added; use structured patches. |
| Canonical release workspace compile lasting minutes | Proposal: use the existing CI non-LTO profile for iteration, retain release qualification where required. |
| Missing N−1 numeric inventory entry, discovered after 2m57s | Run inventory classification checks before the expensive suite. |
| Serial integration binaries consuming minutes | Use explicit targets and isolated nextest execution. |
| Vulkan crash, replay and diagnosis | Failure retained; isolated runner passed. fn-92 already addresses native instance initialization; no duplicate proposal. |
| Missing local nextest | Temporary pinned CI runner installed in 0.5 seconds; preflight local availability. Local setup, no repository spec. |
| fn-71 receipt overwritten by tests; restrictive command guard (~1 minute) | Fresh evidence preserved and historical receipt corrected by focused edit. Proposal: caller-selected or ignored test artifact output directory. |
| Missing GNU time (~1 minute) | wait4 helper used. Local setup, no repository spec. |
| wgpu owned error-scope API mismatch (~1 minute) | Read installed API before porting calls. |
| Images not visible; local-file opener rejected (~1 minute) | Opened HTTP gallery; use verified browser links. Local setup, no repository spec. |
| GPU LTO rebuilds 44–48 seconds versus ~1.5-second tests; WGSL reserved word; runner invocation mismatch | Cheap shader checks and documented pinned runner invocation; consider the CI iteration profile above. |
| nextest execution filter still built unrelated targets (300-second timeout); recovery 43.58 seconds; Family conformance failure | Explicit build targets, and architecture/inventory checks early. Family moved to core; conformance passed. |
| Invocation-4 context reconstruction (~4 minutes), competing fn-92 workload | Compact handoff and reuse binaries; do source work while benchmark machine is busy. |
| Count/instances summary field mismatch (<1 minute) | Proposal: shared typed benchmark result schema with lightweight validation. |
| Browser allocation helpers hidden behind native cfg (<1 minute) | Pair early native and Wasm compile checks. |
| First close captures blank on CPU (~2 minutes) | Preserved failed attempt. One retry after two initial animation frames succeeded; retain initial canvas-settle step in capture harness. |

The reusable changes worth considering for follow-up are a cheap-first gate protocol with explicit build targets, isolated test artifact output, and a validated benchmark result schema. The native initialization issue already has fn-92. None of these proposals substitutes for remaining performance, memory or visual acceptance work.

## Continued work review

- The prior final-checkpoint hold added about two minutes while the host settled and opened captures. That handoff wait is separate from the capture failure itself. Keep one capture owner, use the settled-canvas helper, and commit with an explicitly pending verdict once artifacts are ready instead of waiting for optional owner input.
- Task .2 queued an example build behind its library-test build for less than one minute. Both builds were required and remained serialized; starting the second command after the first exits removes the queued wait but does not claim a reduction in total compile cost. No repository spec is proposed for this scheduling detail.
- Task .3 instrumentation added 6.5–21.3% overhead, so stage shares are approximate. Two roughly 15-second builds and one matrix were sufficient to select a boundary; no tuning loop followed. For future attribution, consider coarse sampling or a lower-overhead profiler. No profiler installation or new spec was made.
- Task .4 discovered a reserved WGSL identifier after a 44-second release rebuild, costing about one minute. The identifier was corrected and tests passed. This reinforces the existing proposal for a fast shader parse/validation check before release linking; no separate friction spec was created.

- The owner rejected the interior shaded wood comparison as unjudgeable. The replacement uses exterior framing, bare-wood view and camera-side illumination, with a full-width comparison slider. Check visibility and lighting before presenting captures; nonblank output is not a sufficient visual gate. No new spec was created.

- The host acknowledged continuation after accepted visuals but ended the turn without executing it, costing another owner prompt. This turn records acceptance, marks .4 done, and starts .5 before reporting progress. The prevention is execution discipline, not a new repository spec.

- Task .5 initially checked formatting with edition 2024 and recursive child traversal, producing inherited formatting noise in this edition-2021 crate. Cost was under one minute; the corrected scoped check passed. Read the crate edition and reuse the established scoped format command. No new spec is proposed.

- Task .6 selected debug/broad baseline tests despite an available release path, costing about two minutes plus compilation. The core run was stopped; explicit release surface/contact tests passed in 0.463 s. The renderer debug run finished separately in 44.816 s. Future dispatches should include exact release commands, and test names/counts must come from logs rather than memory. No new friction spec was created.

- Task .7 used a scratch Naga parser/validator before linking; its shader check completed in 0.02 s. This applies the earlier cheap-shader-check proposal within the task without a new dependency or friction spec. The numeric overflow failure was a useful red test, not hidden or bypassed; a conservative pre-cross range check fixed it, while the far-origin precision failure remains explicit evidence.

- Task .8's ambiguous test cast stopped compilation in about one second; explicit byte type fixed it. Its synthetic collapse fixture then cost one roughly47-second link because matching centres did not imply matching rings. The test now duplicates the full frame/radius input; verify a synthetic fixture's defining property before its rejection assertion. These reinforce cheap test compilation and fixture sanity checks, with no new spec proposed.
- Task .8 host review caught workgroup flag synchronization and fallback lifetime/accounting defects before timing. Snapshot shared flags before the next write phase, and make ownership release explicit before coherent fallback. The station capability check was moved before emission to remove the late-fallback class; focused regressions cover the seams. Cost was one extra focused link/test pass, not a discarded benchmark matrix. A reduction-helper contract and explicit fallback-ownership assertions are useful prevention within this implementation; no separate spec was created.

- Task .8's proposed full-family late-station fixture used an invalid divergence, so it never exercised the intended path. The host chose early station admission and a bounded existing-tree seam test instead of an expensive synthetic family. This is a structural prevention, not evidence that the failed test passed.
- The additional folded-normal fixture also had a false premise: neither rounded circles nor exact square rings produced the intended zero sum under the actual triangle incidence. The investigation was stopped after a CPU check disproved the second construction; no production condition or threshold changed. Two release links were recorded as cost. Check the ordered geometric sum before compiling such a fixture; retain the isolated-case coverage gap and rely only on the shared-helper/actual-output evidence actually obtained. No follow-up friction spec was created.

2026-09-20, host review of .8 browser smoke accounting: zero canonical CPU wood allocation is intentional for admitted GPU positions. The canonical branch still requires positive canonical preparation, while the candidate branch requires positive position CPU/GPU counters and no position fallback; both branches and all lifecycle checks now pass. Cost: one failed smoke invocation. Prevention: update the declared metric contract alongside its implementation. No new friction spec was created.
