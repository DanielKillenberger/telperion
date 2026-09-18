---
satisfies: [R1, R2, R3, R4]
---
# fn-35-the-species-catalogue-as-data.1 Implement The species catalogue as data

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
The species catalogue now sits at the repository root: `catalogue/<id>/` holds one species' whole record for oak, spruce, ash, beech and birch, a code check owns its structure, a generated page per species lets a person browse it, the pipeline writes its artifacts there while a run keeps its scratch in the evidence tree, and the species runner reads a preset's references from the folder.

**R1** — five species folders carry the thirteen required files. Oak and spruce are projected from `.flow/evidence/fn19/references.json` and `fn9/profiles.json`, beech, birch and ash move out of `.flow/evidence/fn34/<id>/`, and the fn-56 ash run's manifest, provenance, decisions and resolutions come in from its branch. Every moved value is verbatim; the only authored fields are `rights` on a source (the old schema had none) and `kept: false` on every reference record (no image is redistributable today). Old locations carry a one-line pointer. `node scripts/catalogue-check.mjs` is on `npm test`, the CI receipt keys and the README command list.

**R2** — `.gitattributes` tracks `catalogue/**/refs/**` through Git LFS, the check fails a kept image whose file is missing or whose bytes differ from `asset_sha256`, and `compare-references.py` resolves a kept image under the species' `refs/` and an unkept one in the ignored cache, refusing either by the reference id. Its self-test covers both branches. No species carries a kept image yet, so the path is built and tested rather than exercised on real bytes.

**R3** — `scripts/catalogue-pages.mjs` renders every species page and the index from the records, with `NOTES.md` quoted verbatim, kept images inline and the stills table with verdicts; the check regenerates in memory and fails a page whose committed bytes differ.

**R4** — every pipeline stage takes `Paths` rather than a bare directory, `--run-dir` takes the fetch cache, the ledger, the command log and rendered stills out of the catalogue, discovery lists every source in `catalogue/*/sources.json` before it searches the web, the species runner reads `catalogue/<id>/packet/references.json`, and `identity.rs`, `sweep.rs` and `growth_reference.rs` each assert their own constants against `pins.json` and name the species that disagrees.

Three things the owner should see. The catalogue's `architectural_model` is unrecorded everywhere: no record in the repository names a Hallé and Oldeman model, and asserting one without a cited source would be an invention, so the page reads "not recorded" until a species spec fills it. The ash folder's `packet/profile.json` is the complete fn-34 research profile rather than the fn-56 run's two-metric draft, and a later run of the select stage would replace it, because the pipeline replaces a packet record rather than merging into it — a gap worth its own spec. And `serde_json`'s default parse is a bit off on a seventeen-digit decimal, which the identity pins caught; `float_roundtrip` is now on for the core's dev-dependency and for `telperion-jev`, and it exposed one latent test expectation (90 ft is 27.432000000000002 m, not 27.432) that only passed under the lossy parse.

Friction is reported in `.flow/evidence/fn-35-the-species-catalogue-as-data/FRICTION.md`: five entries, of which the local command guard is a setup matter and the rest name a fix.

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: a6840dbe2da3c68cb9e4d39760d0d830a8046081, da6d03b78ea257b6def2de9783fbb97017907e2a, 29e31e5bf71f7d6ce263391d69474fab4422a335, e4bd81df7ff2edc6a950f51096a8b2b2123aba13, 41b56d1c14737a2fda1b2be7d26be739172654a9
- Tests: npm run rust:test, npm run typecheck, npm test (catalogue:check + vitest run), cargo fmt --all -- --check, cargo clippy --release --workspace --all-targets -- -D warnings, uv run scripts/compare-references.py --self-test
- PRs: