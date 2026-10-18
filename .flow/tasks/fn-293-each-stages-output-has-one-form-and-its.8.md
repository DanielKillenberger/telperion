---
satisfies: [R2]
---
# fn-293-each-stages-output-has-one-form-and-its.8 Validation drops the scaffold's preconditions; Delivery and Backend go

## Description
The two open items of tasks 3 and 5, settled by the host after task 5 (spec, "Host decisions after task 5"): `validate_skeleton` drops its checks on the scaffold-only preconditions, and the one-variant `Delivery` and `Backend` enums are deleted. Its own codex review.

**Size:** M
**Files:** `crates/telperion-core/src/pipeline/branching.rs`, `pipeline/branching/specimen.rs`, `pipeline/branching/scaffold*`, `family.rs` and its tests, `tests/catalogue_identity.rs`, `suite/troll_limbs.rs`; `crates/telperion-render/src/generation.rs`, `generation/preparation.rs`, `src/web/generation.rs`, render examples and tests naming `Delivery`/`Backend`; `src/browser/render.ts`, `scripts/benchmarks/generation-gpu-smoke.mjs`
**Touches:** [crates/telperion-core/src/pipeline/branching*, crates/telperion-core/src/pipeline/branching/**, crates/telperion-core/src/family*, crates/telperion-core/src/suite/**, crates/telperion-core/tests/**, crates/telperion-render/src/generation.rs, crates/telperion-render/src/generation/**, crates/telperion-render/src/web/**, crates/telperion-render/examples/**, crates/telperion-render/tests/**, crates/telperion-jev/**, src/browser/**, scripts/benchmarks/**, docs/generation-limits-inventory.json, .flow/evidence/fn-293-each-stages-output-has-one-form-and-its/**]

### Approach
- Validation: remove the `inner_envelope`, `resolved_growth` and `scaffold::forks_placed` calls from `validate_skeleton` (`specimen.rs:36-50`) and whatever production code they alone kept compiled (task 3 kept `scaffold/fork.rs`'s `placed`, `slot`, `siblings` for them); keep `rows`' checks on rows a production stage reads. Before dropping each, run the refusal tests (`family::tests::every_row_past_its_bound_is_refused_on_every_family`, `suite::troll_limbs`, `tests/catalogue_identity.rs::every_refusal_is_the_refusal_it_was`, `telperion-jev/tests/dial_bounds.rs`): a check a shipped preset's refusal or a test depends on stays, with its reader named in the summary. A family the engine cannot grow stays refused by name (`no engine species`).
- `Delivery` and `Backend`: delete both; `Generator::prepare` takes no delivery; the browser stage report drops `backend` and `woodBackend` (`web/generation.rs`), and `src/browser/render.ts` and `generation-gpu-smoke.mjs` drop the `Cpu`/`CpuFallback`/`Gpu` values they name.
- Digests equal the baseline (no tree changes).

### Investigation targets
**Required:**
- `crates/telperion-core/src/pipeline/branching/specimen.rs:15-75`
- `crates/telperion-render/src/generation.rs:40-70`
- task 3's and task 5's done summaries (`flowctl cat fn-293-each-stages-output-has-one-form-and-its.3`, `.5`)

### Acceptance
- [ ] `validate_skeleton` makes no scaffold-only precondition check, except any a named shipped refusal or test reads; the kept and dropped list is in the summary.
- [ ] No `Delivery` or `Backend` type, and no `backend`/`woodBackend` field in the browser stage report or its TS type.
- [ ] Digests equal the baseline; non-test core and render lines in `MEASURE.md`.
- [ ] Gate and `npm test` green; codex review SHIP.

## Acceptance
- [ ] TBD

## Done summary
`validate_skeleton` no longer checks the scaffold-only preconditions `inner_envelope` and `scaffold::forks_placed`. The `scaffold` module and `inner_envelope` now compile only into tests. `Delivery` and `Backend` are deleted along with `Prepared::backend` and `Metrics::wood_backend`. `Generator::prepare` and `prepare_async` now take only the family. The browser stage report drops `backend` and `woodBackend`, and so do its TS type (`src/browser/render.ts`), `generation_gpu`'s stages and the GPU smoke script.

Validation, kept and dropped:
- Dropped: `inner_envelope(..).validate()` and `scaffold::forks_placed`. The named refusal tests pass without them: `family::tests::every_row_past_its_bound_is_refused_on_every_family`, `suite::troll_limbs` (3), `catalogue_identity::every_refusal_is_the_refusal_it_was` and `telperion-jev/tests/dial_bounds.rs` (5). The scaffold's own `Specimen::new` still makes both checks for the test-built trees.
- Kept: `resolved_growth`, because dropping it fails two refusal tests. `every_row_past_its_bound_is_refused_on_every_family` loses the refusals of `/skeleton/growth/{influenceRadius,killDistance,maxTurnPerStep,stepDistance,trunkHeight}`, and the `every_refusal_is_the_refusal_it_was` digest moves. `blend::mixed` also reads it.
- Kept: everything in `rows` (the catalogue Sampling/Step checks, `envelope`, `bias`, `habit`, `radii.resolved`, `twigs.resolved` and the attractor-weight coupling).
- `docs/generation-limits-inventory.json` loses 3 sites that are now test-only: `inner_envelope`'s height guard and `fork.rs`'s codominance guard and clamp.

Digests equal the baseline (BASELINE.md, "After task 8"). Core: 24 lines (`raw/core-digest-t8.txt`). Render pipeline: byte-identical, `fb4ce911...` (`raw/generation-digest-t8.txt`). Resident: 24 identical lines under `telperion-heavy 10 --gpu 6` (`raw/resident-digest-t8.txt`). Non-test lines (MEASURE.md, `measure-cfg-lines.py`): telperion-core 22997 to 22825, telperion-render 23525 to 23499.

Gate: `telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast` gave 1401 passed, 0 failed and 111 ignored, the same as task 5. `npm test`: 170 passed. `tsc --noEmit` is clean. Baseline: green (`cargo check --profile ci -p telperion-core -p telperion-render -p telperion-jev --all-targets`).

Follow-ups (task 7, docs; outside Touches):
- `README.md:278` still names `Delivery::Resident`.
- The `forkDivergence` catalogue note (`traits.rs`, rendered into `docs/parameters.md`) still says "Refused where two parts of a fork leave on one heading (`forks_placed`)". That refusal is now made only by the test-only scaffold. Changing the note means regenerating `docs/parameters.md`, which is outside this task's Touches.
- `generation_gpu`'s CPU branch still labels its stages `{"backend":"Cpu"}`. That is the example's own mode label and no type, and its provenance line already carries `mode`.

Tier: not reported (model: claude-opus-5-5)

stage: impl-review - ran (codex, three draws: correctness, contracts and integration, all SHIP with no findings)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: b746dee3a19d371700ab9c7ecca5741fef983fa1, 6d4cad605a6d05f96be77ddfa8beb65d03bf38b9, 5ec7ccc873ddea5357731d59c2bc00870684f506
- Tests: telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast, npm test, artifact_digest, generation_digest, resident digest_tests::digest (seeds 1,7,42): equal to baseline
- PRs: