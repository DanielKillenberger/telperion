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
