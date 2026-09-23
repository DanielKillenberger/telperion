---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-118-a-species-literature-covers-what-the.1 Implement fn-118-a-species-literature-covers-what-the

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
R1, R2, R3 and R5 are delivered to the host decisions of 2026-09-23. R4 moved to the host in fn-80, so this task has no evidence for it.

- R1: `crates/telperion-jev/data/species-requirements.json` has rows for broadleaf, conifer and palm. Each row lists the size fields at the table's bar and the six appearance traits. Every appearance level maps each `MaterialParams` field it feeds to a range. Tested in `pipeline::requirements::tests`: `the_table_covers_the_three_growth_forms_and_every_name_it_uses` and `every_appearance_level_maps_each_fed_field_to_a_range_the_material_accepts`.
- R2: `manifest::validate` runs the table's check on a manifest at `schema_version` 2. The check refuses a manifest with a missing field, a field below the table's bar, a missing appearance trait, or an unknown growth form, and names every shortfall. Version 1 manifests load unchanged, and `discover` writes its draft at version 2. Tested in `admission_refuses_a_short_manifest_naming_every_missing_field`.
- R3: `quality` files `requirements-unmet` for a required field below the table's bar. It is owner-only, offers only `add-sources`, and refuses `lower-bar` by name. `select` files the same kind for a required appearance trait no source describes. An `add-sources` resolution that adds no source is held open on reconcile. The conductor routes the decision to `AwaitOwner`, and the pipeline CLI prints `NEEDS_HUMAN: <ids>`. Tested in `tests/requirements.rs`, `consume::tests::a_required_field_cannot_be_lowered_and_an_add_that_adds_nothing_stays_open`, the conductor ownership test and `policy::tests::routine_decisions_never_lower_a_bar`.
- Appearance is copied, never rendered: `select` scores the level and code copies its ranges into `profiles[0].appearance`, and `generate` records a skip note. The article gains a generated `appearance` block, checked in `scripts/catalogue-docs.test.mjs`.
- Docs: `docs/species-pipeline.md` has a new "The requirements table" section and the new decision kind; `docs/species-onboarding.md` explains how the requirement is enforced.
- R5: `cargo test --profile ci --workspace --no-fail-fast` exited 0 (944 passed). Vitest passed and `catalogue:check` passed.

Open, for the host:
1. The bars are my first cut, held as data: height and trunk diameter at `partial`, crown and organ sizes at `proxy_only`. The sufficiency question set scores age-indexed curves, and its lowest level describes "only mature ranges". A frond or leaflet size quoted at maturity may therefore score `none` and stop the run. That is NEEDS_HUMAN, so it fails safe, but the question set may need a reading for mature sizes.
2. `manifest.rs` is 454 lines and `generate/mod.rs` is 410, a little over the 400-line guide.

R4, for the host in fn-80. Rewrite `.flow/evidence/date-palm/pipeline/manifest.json` at `schema_version` 2 to the palm row. The fields are `height_m` and `dbh_m` at `partial`, and `crown_width_m`, `frond_length_m`, `leaflet_length_m` and `leaflet_width_m` at `proxy_only`. The appearance traits are `bark_colour`, `bark_roughness`, `leaf_front_colour`, `leaf_back_colour`, `leaf_hue_range` and `leaf_brightness_range`, each with its sources. Then rerun from the root with `P=target/release/species-pipeline; D="--dir catalogue/date-palm --run-dir .flow/evidence/<spec>/pipeline"`, run inside `bash -ic` so the Jev key is set: `$P fetch $D; $P extract $D; $P screen $D; $P quality $D; $P select $D`, then `verify`, `fit`, `gate --example`, `generate --example --profile-id date-palm`, `document` and `report`. Any `NEEDS_HUMAN:` line names the fields that missed their bar.

stage: impl-review - skipped(config: REVIEW_MODE=none)
Tier: session (host-settled design)
## Evidence
- Commits: 0118ca37c5185af4f8061d580133ead1ce014f76, 5c4f0a1f0b34edd06474d6f1911c7704a60eedcc, 281b80bafc0993fc855806e02d20cec8567da398
- Tests: baseline: not run pre-edit (project rule: the gate runs once, at the end of a task), cargo test --profile ci --workspace --no-fail-fast (exit 0, 944 passed, 0 failed), npx vitest run scripts/catalogue-docs.test.mjs (14 passed), npm run catalogue:check (5 species pass)
- PRs: