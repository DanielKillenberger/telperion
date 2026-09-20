
## 2026-09-20 — baseline tooling build

The direct mature path had no reusable native per-stage timing runner. Adding a small example and compiling its release binary has cost roughly two minutes of inspection and an ongoing build wait (over ten seconds so far). Retaining the runner removes this setup from the candidate comparison; attachment preparation remains included in placement pending host direction.

## 2026-09-20 — browser baseline build and completion boundary

Fresh core and renderer Wasm builds took 14.17 and 14.99 seconds before browser measurement. The renderer exposes frame submission but not GPU completion, so the current runner cannot measure the first completed frame without an additional benchmark-only fence. The host was informed; current evidence labels submission as a lower bound. A retained benchmark completion hook would remove that gap. No captures were taken.

## 2026-09-20 — CPU candidate verification builds

The focused foliage baseline ran 21 tests in 28.99 seconds, and a new lib-test binary requires another release compilation before the bounds checks can run. This setup costs tens of seconds per changed core build; no full suite is running in the iteration loop. The retained benchmark binary permits before/after measurements without rebuilding the old implementation. An initial benchmark fingerprint edit had an extra comma (compile failure, corrected immediately); the new transformed-box property test initially failed to compile because its helper did not yet exist, not because the old exact algorithm was incorrect.

## 2026-09-20 — bounds fixture correction

The new curved-element test initially chose width 0.00001 m, below the existing builder minimum 0.0001 m. It failed on `leaf width` before exercising bounds. Correcting the fixture cost one roughly 30-second release lib-test compile; all three bounds tests then passed. Reading the parameter validation range before authoring the fixture would have avoided that wait. A queued example build waited on Cargo's existing lock; measurements began only after both builds finished.

## 2026-09-20 — browser navigation setup failure

The first candidate browser attempt failed before any generation: its page could not resolve /src/browser/render.ts. The same script completed the baseline. Zero samples were collected; the error artifact is retained as browser-cpu-candidate-inconclusive.json. Diagnosis/retry starts here, about one minute of setup/inspection cost so far. Recording navigation origin before module import will distinguish a bad page from a generator failure. A shell guard also rejected a compound edit command because it could not parse quoted heredoc content; the edit was applied through the patch tool instead.

## 2026-09-20 — final workspace gate

The canonical workspace gate is recompiling all four Rust crates in release mode after the shared core change, with no test result yet after roughly a minute. It runs once, with a 600-second bound and output captured to a log. CI's existing faster profile is available, but the host chose the canonical local release command for this final gate; no test or profile was weakened to shorten the wait. Timed measurements finished before this build started.

## 2026-09-20 — generation-limit inventory gate

The workspace gate compiled in 2m57s, then stopped at the inventory guard after passing the preceding suites. The new segment-frame iterator needs an algorithm classification for its points-length-minus-one slice. This is a real missing declaration in the candidate, not a generator cap or a reason to relax the guard. The host was asked to review that classification; the next check will target the guard before the cached workspace run continues. Reading the inventory rule before introducing the loop would have caught this before the broad gate.

## 2026-09-20 — cached gate still serializes integration binaries

The corrected workspace rerun reused binaries in 0.12 seconds but reached three minutes of active test execution before stem_lean_spread. The preceding stem_fork_height binary alone took 26.13 seconds, and the parent's completed-frame measurements are waiting for the load to finish. This is normal progress within the 600-second bound, not a stalled compiler. Reusing the project's existing nextest parallel test runner for local gates is a possible remedy; the host has kept the canonical Cargo command for this run.

## 2026-09-20 — renderer gate crash

The corrected workspace run passed core tests but crashed in bark_plates with SIGSEGV during Vulkan extension enumeration. The saved core-dump stack also shows another test thread creating a Vulkan instance through NVIDIA GLX. There is no generator frame in the crashing stack; concurrent loader/driver initialization is an inference, not a proven cause. Memory inspection showed 21 GiB available and no OOM journal entry. The host approved one bounded exact-binary replay before any further gate work. This costs roughly a minute of diagnosis plus replay time, and blocks a green workspace claim; driver/test initialization stability must be established before GPU experimentation. No system settings or test assertions were changed.

The single exact-binary replay also crashed with SIGSEGV (exit 139), after the same first successful test. No additional replay or system change was attempted. The host is selecting the existing CI nextest isolation route for remaining renderer/Wasm validation; the original parallel Cargo workspace gate remains failed.

The selected fallback runner is absent locally (cargo nextest reports no such command). The host directed an honest checkpoint and early NEEDS_HUMAN return; no installation or further runner experimentation was attempted. This is a local setup blocker, not a new repository spec.

## 2026-09-20 — missing local CI runner

The host needed the CI runner after the canonical renderer gate and its isolated-binary replay both crashed during concurrent Vulkan instance initialization. cargo-nextest was absent locally. Installing the repository-pinned prebuilt 0.9.145 into /tmp/telperion-fn91-tools took 0.5 seconds; an isolated-process renderer/Wasm run is now bounded at 180 seconds. This is a local tooling repair, not a proposed repository spec. Checking runner availability before choosing the gate would have avoided the late setup detour; the original canonical failures remain evidence.

## 2026-09-20 — renderer test rewrites historical evidence

The isolated renderer suite rewrote the tracked fn71 smooth-bark receipt through tests/common/resolution.rs. Inspecting the one-line metric change and confirming the writer cost about one minute. A broad restore command was denied by the discard guard. The host instead saved the complete new receipt as fn-91/renderer-smooth-bark.json, then applied the reviewed one-line correction to the historical file, preserving both results. The guard also rejected a shell-based append merely describing that command; a structured patch records this entry. A caller-selected evidence directory, or an ignored test-output directory, would prevent unrelated spec changes on routine validation; no new spec is created here.

## 2026-09-20 — native RSS measurement helper

The host's first peak-RSS invocation found that GNU time is not installed, before any specimen ran. Switching to Python's os.wait4 exposes the same Linux child-process peak-RSS metric without installing another tool. This cost about one minute; checking availability first would avoid the failed invocation. This is a local setup detail, not a repository spec.

### 2026-09-20 — GPU experiment, first compile boundary

The first Rust check found that wgpu 30 error scopes return owned guards rather than using device pop calls. Reading the installed API and correcting the scope ownership cost about one minute; the next check passed in 0.50 s. The first focused release test build recompiles changed core/render code and several feature-selected dependencies. Keep the cached release profile and one bounded focused run; do not repeat the workspace gate. The browser measurement window was respected with source work performed during it, so it did not create an idle wait.

### 2026-09-20 — opening the visual comparison

Tool-emitted images did not give the owner an opened comparison. The owner's xdg-open wrapper accepts HTTP URLs and rejected a local HTML path while returning success. Hosting the four existing images and a side-by-side page in a dedicated temporary loopback directory opened successfully; all four HTTP requests returned 200. This cost about one minute. Open a browser comparison with confirmed image loads when requesting an owner visual verdict. The wrapper behavior is local setup, not a repository spec.

### 2026-09-20 — native GPU verification loop

Release LTO took 44–48 s per changed-core/render focused build while the four GPU tests themselves take about 1.5 s. The first shader test rejected WGSL's reserved `target` identifier; the corrected shader passed, and subsequent builds carried host-requested tile-phase, rejection and ownership coverage rather than repeated timing runs. The remedy is the existing CI non-LTO profile for an explicitly approved local iteration protocol, not changing the repository's required release commands in this task. A direct call to the temporary cargo-nextest executable with `list` also failed immediately because this cargo extension expects invocation through `cargo nextest`; adding its directory to PATH removes that hand step. No specimen or suite ran in that failed call.

### 2026-09-20 — nextest filtering still links unrelated core binaries

At 169 s the final nextest binary-list build was still making progress through four active rustc jobs, but package selection links all core test binaries even though the intended test filter is foliage/mesh/surface/inventory only. This is avoidable gate cost. The host directed retaining useful in-progress work within the existing 300 s build bound, without extending it. Future small core gates should select explicit Cargo test targets separately from the full renderer package; a nextest test-name filter is not a build-target selector. No unrelated core tests are being added to the execution filter.

The package-wide binary build hit its 300 s limit (exit 124) while linking renderer integration binaries; no tests ran. The parent authorized one 60 s recovery using explicit core foliage/mesh/surface/inventory targets plus renderer lib/integration targets, after recording existing binary timestamps. The recovery command and timestamp inventory are retained. This spent five minutes on an avoidably broad build; the test-execution bound is still separate and unchanged.

The explicit-target build recovery completed in 43.58 s. Isolated execution then completed in 79.701 s: 214/215 passed, with no GPU crash. The sole failure was the existing renderer family-table conformance guard: the experimental orchestrator imports the generic parameter bundle through `presets::Family`. No preset lookup or species branch exists, but the top-level import crosses the guard's stated module boundary. The host was asked to resolve the parameter-type boundary; the guard remains unchanged and the failed run is retained. All five new GPU tests and the generation-limit inventory passed in that run.

The final renderer suite again wrote the historical fn71 smooth-bark receipt (SilverBirch mean 2.5604 / p95 9.50). The fresh receipt is preserved as gpu-renderer-smooth-bark.json; only that observed legacy-row change is restored to its previous value. This repeats the already recorded test-output routing friction, not a new generator change.

## 2026-09-20 — Invocation 4 context and external contention
Initial handoff reconstruction and bounded source reads consumed about four minutes before editing. Reuse the compact source map and saved binaries for subsequent checkpoints. Another session is running a workspace CI-profile suite; timed comparisons will wait for its CPU load to end, without interrupting that session.

## 2026-09-20 — Follow-up summary schema
The local summary script assumed `count` where the existing benchmark uses `instances`; the first aggregation failed immediately (under one minute), without repeating any measurements. Corrected the field against the retained JSONL. A shared typed measurement schema would prevent this small adapter mismatch; no new framework was added.

## 2026-09-20 — Browser async target exposure
The first Wasm check found the existing allocation-accounting helpers were native-only (under one minute). Exposed those same read-only helpers on Wasm so browser evidence can retain buffer counts; no new accounting algorithm. Native check passed before this target-specific finding.

### 2026-09-20 — initial canvas resize cleared close capture

The four-image browser close capture completed, but both CPU screenshots were white while GPU screenshots contained trees. The likely cause is the initial ResizeObserver callback resetting canvas dimensions after synchronous CPU generation and drawing; GPU awaits gave that callback time to run first. The host preserved the failed images, script and metadata and will make one bounded retry after two initial animation frames settle resizing. This cost about two minutes. A capture helper should settle initial canvas sizing before its single draw. No production renderer behavior or completed-queue timing claim is changed; compositor delivery was never claimed.

## 2026-09-20 — Final checkpoint waits for capture ownership
Implementation, smoke, matrix and focused checks were complete while the host owned the separate close-view capture/retry. The worker held the commit for roughly two minutes (estimate), as requested, without repeating builds or measurements. A settled-canvas capture helper would remove the retry; the host records the concrete capture failure and remedy separately. This was a bounded handoff wait, not another implementation attempt.

Close-capture retry outcome: waiting two initial animation frames produced four nonblank images. Preserve initial resize settling in the capture harness; the views remain limited for contact inspection.

## 2026-09-20 — task .2 build queue

The candidate example build waited for the focused library-test release build's Cargo lock (under one minute; exact lock wait was not timed). The builds remained serialized and no measurements ran during compilation. Starting the example command only after the test command exits would avoid a queued command, though it would not reduce the required compilation work. The bounded task used one fresh baseline example and one candidate example build.

## 2026-09-20 wood-stage observer overhead
The scratch-only wood profile completed its requested control/instrumented matrix in one pass. Per-run timers and altered optimization increased warm medians by 6.5–21.3% (9.34–37.68 ms), limiting exact stage attribution. Cost was two release builds of about 15 seconds each and one matrix; no additional profiling ticks or tuning runs were spent. A lower-overhead profiler or coarse production-neutral sampling would remove this measurement limitation; no profiler was installed. The report retains approximate stage shares and explicitly separates observer overhead.

### 2026-09-20 — task .4 shader validation rebuild
First resident-wood test reached WGSL validation only after a 44-second release library rebuild; `meta` is a reserved WGSL name. Renamed the binding to `metadata`. Cost: about one minute and one focused failed test run. A fast standalone shader-validation command would have caught this before the LTO test rebuild; no gate was bypassed.
