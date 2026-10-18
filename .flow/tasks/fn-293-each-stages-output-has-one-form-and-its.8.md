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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
