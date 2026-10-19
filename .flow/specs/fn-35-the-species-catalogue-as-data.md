# The species catalogue as data

## Goal

The owner wanted one structured home for everything known about a species, so the pipeline runs the same way every time and records are updated in place, not lost. The first pipeline run (fn-56, 2026-09-18) showed species knowledge filed by spec number in four places and three record formats, with pointers like "fn-34 ash profile" that break when evidence is superseded; discovery searched the web for a source fn-34 already named (six dispatches on a run planned for one). This spec put the catalogue at the repository root, outside Flow, one folder per species, owned by a code check and browsable through generated pages. Flow evidence holds runs and refers to the catalogue. The registry refactor is fn-77.

## Decisions

- > user (2026-09-14): "should we actually spec a template/pipeline that would allow contributors to add trees as templates and improve the generator to make it possible to generate all trees in the world. If this is streamlined we could just recursively have grok add trees and update the generator?"
- > user (2026-09-14): "i mean the goal should be all trees some time in the future. But ideally we just have a spec per tree species and have that follow a template"
- > user (2026-09-18): "should we actually have a database for species documentation. Document store/wiki or smth that follows certain rules/structure => the species ingestion pipeline is much easier to consistently run. Also easier to update docs and not have them lost."
- > user (2026-09-18): "but i'd have that catalogue at a level outside of flow. Evidence in flow should refer to that wiki/catalogue. It should be at the root of the repo."
- > user (2026-09-18): "how can we have images stored there also to use as references? probably not fit to be in the repo directly?" / "we'd ideally have images in there"
- > user (2026-09-18): "is there also a way to browse this catalogue that fits this structure? for humans i mean" / "aye"
- > user (2026-09-18): "does fn-35 need planned? can we simplify it? 11 reqs seems alot" / "ok let's do the split as you proposed"
- Files in git with schemas and a code check over a wiki or database (history, PR review, workers can read files). Git LFS for redistributable reference images over plain commits (master already carried 236 MB of rasters) or a bucket (credentials everywhere). Generated READMEs over authored pages. Two specs over one (catalogue here, registry fn-77).
- Boundaries: no species onboarded, no harness view, no pipeline stage logic change, no history rewrite.

## Acceptance Criteria

- **R1:** Every shipped species (oak, spruce, ash, beech, birch) has a `catalogue/<id>/` folder with the required files; fn19, fn34 and fn-56 records moved there with pointers left; structure check on the test commands — met.
- **R2:** Kept reference images are LFS objects under `refs/` with records; unkept images by URL and hash, runner refuses absent or mismatched files — met (built and tested; no kept image exists yet).
- **R3:** Generated `README.md` per species and an index, `NOTES.md` quoted verbatim; stale or hand-edited pages fail the check — met.
- **R4:** Pipeline stages write into the species folder, discovery reads `catalogue/*/sources.json` first, the runner reads references from the folder, `pins.json` matches test source — met.

## Result

Five species folders with thirteen required files; moved values verbatim, only `rights` and `kept: false` authored. `node scripts/catalogue-check.mjs` on `npm test`; `scripts/catalogue-pages.mjs` renders the pages; `.gitattributes` tracks `catalogue/**/refs/**` through LFS. Every pipeline stage now takes `Paths`; `--run-dir` keeps fetch cache, ledger, logs and stills out of the catalogue. `identity.rs`, `sweep.rs` and `growth_reference.rs` assert their constants against `pins.json`. `architectural_model` stays unrecorded: no record cites a Hallé and Oldeman model.

## Open

- Whether rendered stills belong in the `.git/flow-artifacts` store rather than ignored evidence directories (out of scope).
- The fn-56 ash duplicate reappears when fn-56 merges.

## Friction

- The ash run lived on an unmerged branch and the spec named the wrong one (about 10 min of `git show` archaeology) -> a spec moving in-flight evidence should name the branch; raised to the owner.
- `serde_json` without `float_roundtrip` parsed a 17-digit bound one ULP off (about 15 min) -> enabled for `telperion-core` dev-deps; the same trap remains in `telperion-jev`.
- The workspace suite outran the 600 s tool timeout under a concurrent build (two round-trips) -> fn-76 is the standing fix.
- Every stage took a bare directory, so a second path reached forty call sites (about 25 min) -> stages now take `&Paths`.
- The local `dcg` guard blocked three ordinary shell forms (about 5 min) -> local setup, reported not specced.
