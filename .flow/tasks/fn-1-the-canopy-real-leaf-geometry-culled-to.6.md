# fn-1-the-canopy-real-leaf-geometry-culled-to.6 Public surface and the docs that still say there is no foliage

## Description
Export the canopy stage from the barrel and retire every doc claim that the library has no foliage.

## Acceptance
- Canopy exports present; README, package.json, `src/index.ts` header and harness caption updated.

## Done summary
Exported `buildElement`/`buildCanopy`/`cullCanopy`/`shoots` with params and defaults; `silhouette.ts` deliberately not exported (a test instrument, not a stage). Fixed the README quickstart, which had never compiled.

## Evidence
- Commits: 6eab840
- Tests: npx vitest run (251 passed), npx tsc --noEmit, README quickstart typechecked standalone
- PRs:
