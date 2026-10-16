---
satisfies: [R1, R2, R3, R4]
---
# fn-35-the-species-catalogue-as-data.1 Implement The species catalogue as data

## Description
Build the root species catalogue, its structure check, generated pages, LFS rule for kept images, and point the pipeline and runner at it (R1-R4).

## Acceptance
Every R-ID in the parent spec's Acceptance is satisfied.

## Done summary
`catalogue/<id>/` for oak, spruce, ash, beech and birch: oak and spruce projected from fn19/fn9 records, beech, birch and ash moved from fn34 and the fn-56 run, values verbatim, old locations pointing at the new. `catalogue-check.mjs` on `npm test` and the CI keys; `catalogue-pages.mjs` regenerates pages; `compare-references.py` resolves kept and unkept images and refuses by reference id. Stages take `Paths`; discovery reads `catalogue/*/sources.json` before the web; pins tests assert against `pins.json`.

## Evidence
- Commits: a6840dbe2da3c68cb9e4d39760d0d830a8046081, da6d03b78ea257b6def2de9783fbb97017907e2a, 29e31e5bf71f7d6ce263391d69474fab4422a335, e4bd81df7ff2edc6a950f51096a8b2b2123aba13, 41b56d1c14737a2fda1b2be7d26be739172654a9
- Tests: npm run rust:test, npm run typecheck, npm test (catalogue:check + vitest run), cargo fmt --all -- --check, cargo clippy --release --workspace --all-targets -- -D warnings, uv run scripts/compare-references.py --self-test
- PRs:
