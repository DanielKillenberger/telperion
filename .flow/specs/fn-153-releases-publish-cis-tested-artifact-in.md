## Conversation Evidence

> user: "publishing needs to be faster. what are our best options?"
> assistant (measured on release run 36067333639, v0.1.3, about 17 minutes): Rust suite 10 min 40 s re-running the whole workspace on the tagged commit; node suite and build 3 min 18 s rebuilding the Wasm and dist; wasm-bindgen install 1 min 29 s compiling the CLI from source; the rest about 1 min.
> assistant: options, best first: (1) reuse CI's receipts instead of re-running, since the tag sits on a master commit whose CI just passed; (2) publish the artifact CI built rather than rebuilding it, so what was tested is byte for byte what ships; (3) fetch wasm-bindgen prebuilt; (4) release on merge when a merge to master bumps the version. "My recommendation: all four. Together a release is about 1 to 2 minutes after the merge's CI."
> user: "/flow-next:capture that"
> user (duplicate check): new spec, not an extension of fn-112.

## Goal & Context
<!-- scope: business -->

Publishing a Telperion release to npm takes about 17 minutes after the tag, and almost all of it re-proves what CI proved minutes earlier on the same commit. killenberger.com waits on each release to take a fix, so the wait sits between a merged fix and the site. The owner wants publishing faster without making a release less trustworthy. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, checked 2026-09-25 on master.** `.github/workflows/release.yml` (fn-112, #63) runs on a `v*` tag: a version job checks the tag against `package.json`; a `rust` job re-runs the full workspace suite under nextest "without its receipts" by design; a `core-plan` job builds the core without geometry; a `publish` job installs wasm-bindgen-cli with `cargo install`, runs `npm ci`, typecheck, build, the node suite and `test:dist`, then `npm publish --provenance`. `tests.yml` saves a receipt per suite keyed on that suite's inputs and skips a suite whose receipt matches; master writes its own receipts. [checked]
- **Proof by receipt.** A release requires a green receipt for every suite on the exact tagged commit, written by master's CI, and publishes without re-running the suites; a missing or red receipt refuses the release. [paraphrase]
- **One artifact.** Master's CI uploads the built package (`dist` and its Wasm) for the commit; the release publishes that artifact unchanged and never rebuilds. [paraphrase]
- **Prebuilt tooling.** wasm-bindgen-cli comes as the prebuilt binary at the crate's pinned version, or from a cache, never compiled per run. [paraphrase]
- **Release on merge.** A merge to master whose `package.json` version is new tags that commit and publishes once its CI is green, with no hand tag step. [paraphrase]
- **Host decision (2026-09-25): publishing does not live in tests.yml.** `tests.yml` tests and builds only. Its `package` job builds once, runs `npm pack` and `test:dist` on the tarball's own `dist`, and uploads the artifact `package`, which holds the tarball and its `.sha256`. `release.yml` publishes. It runs on `workflow_run` of Tests (`completed`, `branches: [master]`) only when the run concluded success and was a push to this repository. It reads the run's `head_sha`, never master's tip. A suite is green when its jobs in that run succeeded (jobs API, `actions: read`), or when they were skipped and the suite's `scripts/ci-key.mjs` key for `head_sha` is in master's cache (caches API). Otherwise the release refuses and names the suite. It downloads `package` from that run (`run-id`, `github-token`) and refuses on a missing file or a sha256 mismatch. Permissions: `contents: write`, `id-token: write`, `actions: read`. The run's OIDC `workflow_ref` names `release.yml`, because `workflow_run` executes the default branch's `release.yml`, so the trusted publisher npm already has for `release.yml` still matches. [user]
- **Host decision (2026-09-25): tag first.** The annotated `v<version>` tag is pushed on `head_sha` before `npm publish`. A rerun after a failed publish finds the tag on the same commit and publishes. A tag on another commit refuses. A version already on npm publishes nothing. [user]

## Edge Cases & Constraints
<!-- scope: technical -->

- A tag or version whose commit never ran master's CI (a tag pushed on a branch commit, or CI still running) cannot find its receipts or artifact and is refused, never rebuilt as a fallback. [inferred]
- npm provenance stays on: the published tarball is attested to the workflow run. [inferred]
- A version already published is refused before any upload. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A release publishes in about 2 minutes or less after master's CI finishes on the commit, measured on one real release and reported. Errors: a release that re-runs a suite or rebuilds the package fails the criterion. [paraphrase]
- **R2:** A release publishes only when every suite has a green receipt for the tagged commit. Errors: a missing, stale or red receipt refuses the release with the suite named; nothing is published. [paraphrase]
- **R3:** The published tarball is the artifact master's CI built and tested for that commit, shown by matching checksums in the release log. Errors: a missing artifact refuses the release. [paraphrase]
- **R4:** wasm-bindgen-cli is not compiled during a CI or release run. Errors: no error surface beyond the pinned version failing to download, which fails the run. [paraphrase]
- **R5:** A merge to master that bumps the version publishes that version with no manual tag; a merge that leaves the version unchanged publishes nothing. Errors: a version lower than or equal to the latest published one refuses. [paraphrase]
- **R6:** The publish step authenticates to npm by trusted publishing, with no stored npm token, once the owner has configured the package on npmjs.com. [user]

## Boundaries
<!-- scope: business -->

- Not a change to what the test suites check or how CI runs on pull requests.

## Decision Context
<!-- scope: both — conditionally substructured -->

Re-running every suite on the tag was fn-112's deliberate choice to prove the tagged commit itself. Receipts keyed on the commit's inputs prove the same commit without repeating the run, and publishing CI's own artifact closes the gap that a rebuild leaves open, so a release becomes faster and stricter at once. [paraphrase]

## Strategy Alignment

- Serves "The core and integration": consumers such as killenberger.com take a fix as soon as it is merged. [strategy:The core and integration]
