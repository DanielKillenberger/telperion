# Where the test suite's time goes

## Conversation Evidence

> user (2026-10-02): "are we using significant amount of time running test suites?"
> user (2026-10-02, on measuring first, then a spec): "or just capture a recon spec that i could take over in another session for the test suite optimization"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 20% [user], 60% [checked], 20% [inferred] -->

A recon spec: it measures where the time of the local gate and of CI goes, and which tests earn it, and ends in a ranked list of follow-up specs. It changes no test. The owner takes it over in another session. It follows `docs/principles.md`, "Question, delete, then optimise": count who costs time, ask what each test catches and whether it can go, and only then propose making anything faster. [user, principles]

**What is known, checked 2026-10-02.** [checked]
- **fn-76 met its bound once.** On 2026-09-18 a run where every suite executes took 2 min 55 s on GitHub with warm caches (`.flow/evidence/fn76/RUNS.md`): six shards (`core 1/4` to `core 4/4`, `render`, `wasm and jev`), specimens generated once per run, determinism by committed digest, the species seeds four at a time.
- **CI now takes 7 to 8.5 minutes.** The last `Tests` runs: master 7.2 and 8.4 min; fn-180's branch 8.4, 7.0 and 1.8 min (`gh run list`). Whether this is cold caches, growth of the suite, or both is unknown.
- **The local gate takes 5 to 10 minutes.** `cargo test --profile ci --workspace --no-fail-fast` (the AGENTS.md gate; `[profile.ci]` inherits release without LTO, 16 codegen units) was reported at 307 s on fn-183 and at about 5, 6, 9 and 10 minutes in other specs' `FRICTION.md`. One run spent 9 minutes inside the core `species` suite. The compile/test split of a gate run has never been measured.
- **The core library suite alone takes about 90 s** (fn-183 task 2, `cargo test --profile ci -p telperion-core --lib`).

**Where time is lost, from the `FRICTION.md` files across `.flow/evidence/`.** [checked]
- **Repeated full gate runs:** "one extra gate run, about 10 minutes" recurs: a generated file not regenerated before the gate (`docs/parameters.md`, `src/browser/parameters.generated.ts`, `docs/generation-limits-inventory.json`), a summary filter that cut the log, the gate run before review and again after.
- **Failures found one per run:** a test stops at its first failing assertion, so the next stays hidden; fn-183 task 2 spent about five suite runs (8 minutes) surfacing three questions one run could have shown.
- **Pins in many places with no single recorder:** `suite/species/digests.json`, `identity.rs` `PINS`, `catalogue/<id>/pins.json`, the `NEUTRAL` arrays in `drop`, `sag` and `strands`, `audit.rs`. Re-pinning after an intended tree change cost about 10 minutes on fn-183, by hand and partly parsed from failure text.
- **A test keyed on rendered images:** `telperion-jev`'s beech replay looks shots up by image bytes, so any tree change breaks it locally, and re-recording needs a live Jev and network run (fn-173, fn-183).
- **A race:** `objectives::the_palm_s_materials_track…` shares a temp directory with another test (`crates/telperion-jev/tests/objectives.rs:28`); it passed 3 of 3 alone and failed in the gate on fn-183.
- **The generation-limits inventory** is keyed on source text, so a rewrite of a bounded loop fails the gate late (fn-125 era friction).
- **The background shell's 10-minute cap** sits near a gate run's length, so a green run can be lost to the harness.

## Architecture & Data Models
<!-- scope: technical -->

Recon only. The measurements below are taken at one named master commit on the owner's machine (AMD Ryzen 9 5950X) and on CI.

- **Time by test, not by suite.** One gate run with nextest's per-test timing (JUnit or `--message-format` output) gives each test's wall time; the report ranks the slowest 30 and sums time per test binary and per module.
- **Compile against test.** A gate run from a clean `target/` and one from a warm one, each split into compile and test phases; the same for one CI run, read from the job logs per shard.
- **What each slow test catches.** For each of the slowest 30, and for every pin family above: what defect it would catch, whether a neighbouring test catches the same, the last time it caught a real defect (git log of the test and its pins), and whether it asserts a requirement with a name (an owner decision, a principle, a measurement) or an unnamed one (`docs/principles.md` step 1). fn-183 deleted containment assertions that pinned a wall nobody had asked for; the recon looks for more of those.
- **The specimen cache.** Whether fn-76's once-per-run specimen cache still serves every test that builds a fixed specimen, or tests build their own again.

## Edge Cases & Constraints

- Changes no test, pin or workflow; it measures and reports. [user]
- No full-forest capture. No release suites as a second proof. [AGENTS.md]
- Nextest's output and any raw logs go to this spec's `raw/` (ignored). [docs/evidence-retention.md]

## Acceptance Criteria

- **R1:** A report ranks the slowest 30 tests and sums time per binary and module, from one local gate run and one CI run at the same commit, with each run split into compile and test time, cold and warm. [inferred]
- **R2:** For the slowest 30 tests and every pin family, the report states what each catches, what else catches it, the last real defect it caught, and whether its requirement is named, and marks each delete, merge, keep or speed up, with the reason. [principles]
- **R3:** The report explains the gap between fn-76's 2 min 55 s and today's 7 to 8.5 minutes on CI. [inferred]
- **R4:** The report ends in a ranked list of follow-up specs, each with its measured or estimated saving and the friction entries it would remove, including: one recorder that rewrites every pin from one run, a gate policy that surfaces every failing assertion in one run, the beech replay keyed by request shape rather than image bytes, the objectives race, and the generation-limits inventory's keying. It also weighs fn-183's friction that an exploratory measurement step never ran the suite on its experiment code, so four design questions surfaced only during the build: a rule that such a step runs the workspace suite with `--no-fail-fast` and reads every failing assertion. Obvious fixes with a named cause and no product change are marked as such (AGENTS.md, "Friction reports"). [inferred]

## Boundaries

- Building any follow-up is out of scope; each is its own spec. [user]
- The objectives race (`objectives.rs:28`) is fixed: each call gets its own directory (host, 2026-10-02, on the fn-183 branch). [checked]

## Decision Context

- **Why recon first (owner, 2026-10-02).** The owner asked for a spec they can take over in another session, after "Question, delete, then optimise" was adopted the same day; fn-173 had spent two sessions optimising work that should not have existed. [user]
- **fn-76** is the prior art: its design (one generation per specimen, digests, nextest, shards) and its 2026-09-18 measurements are the baseline. [checked]

## Strategy Alignment

- Serves the owner's CI bound ("i like ci being below 5min. Ideally much below", fn-76) and STRATEGY.md's "measured cost". [strategy:Our approach]
