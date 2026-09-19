# Friction

## 2026-09-20 — Review fix: overly broad debug test selection

The mechanical cleanup helper selected the whole Wasm library in the default debug profile instead of only its changed diagnostics test. The diagnostics passed, but the unrelated oak/spruce mesh fixture ran for over a minute before the helper interrupted it. That broad observation is inconclusive, not green. The exact diagnostics filter or the project's optimized `ci` profile would have avoided this minute; the final optimized broad gate covers both tests. No further debug retries were made.

## 2026-09-20 — Worker: old-contract fixtures stopped successive full runs

The first full Rust run stopped at a serialized parameter pin; the next stopped at a drop fixture that authored eight laterals and depended on silently resolving to seven. Switching the next run to `--no-fail-fast` exposed the remaining old-clamp/error/schema expectations together. This added roughly five minutes of gate work before the final clean run. Using no-fail-fast for the initial broad contract migration gate, plus a bounded search for tests explicitly asserting clamped values, would avoid sequential discovery. Each failing expectation was updated to the declared new contract or its prior effective valid input; no geometry hash was repinned.

## 2026-09-20 — Worker: the cap inventory reached beyond the initial list

The source guard exposed additional form limits in foliage station placement, packed placement keys, rejection sampling, reach probing and limb clumping. Classifying these with the host and carrying their parameters through wire, blend and harness took roughly fifteen minutes beyond the four-limit starting inventory. This was necessary scope discovery, not a test retry loop. A complete classified inventory at spec capture would have made the implementation size and required compatibility work visible earlier; the new checked inventory supplies that record for future changes.

## 2026-09-20 — Worker: competing baseline workloads

Comparing the three direct-build samples per preset revealed that the baseline core suite ran concurrently with baseline timings (load average 5–11), while the after samples ran at load 1–2. The hashes are identical, but the lower after timings cannot establish a speed improvement. The comparison cost about one minute to diagnose; comparable-load baseline measurements would require another baseline build and sampling run. Serializing performance samples with test suites would avoid the ambiguity.

## 2026-09-20 — Host: shipping workflow instruction volume

While preparing the authorized PR and merge alongside implementation, the host found 3,526 lines across the make-pr workflow, cognitive-aid contract, create/finalize reference and land workflow, before their additional gated references. Reading the first 980 make-pr workflow lines alone returned about 22,900 tokens; roughly two minutes of host preparation so far. Implementation continued independently, so this has not blocked the worker. The repeated rendering and guard instructions add substantial context cost to an otherwise ordinary single-spec shipment. A shorter common path with gated exception references or deterministic PR/landing helpers would remove this overhead. This is a proposed workflow improvement only; no new spec or configuration change is authorized or made.
