
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
