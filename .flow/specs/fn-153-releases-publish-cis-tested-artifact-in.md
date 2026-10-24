# Releases publish CI's tested artifact in minutes

## Goal

Publishing a release took about 17 minutes after the tag (release run 36067333639, v0.1.3: Rust suite 10 min 40 s re-run on the tagged commit, node suite and build 3 min 18 s, wasm-bindgen compiled from source 1 min 29 s), almost all re-proving what CI proved minutes earlier on the same commit. killenberger.com waits on each release to take a fix. The owner wants publishing faster without making a release less trustworthy.

Owner quotes:
> "publishing needs to be faster. what are our best options?"
> "/flow-next:capture that" (on the host's four options, all recommended: reuse CI's receipts, publish CI's artifact, prebuilt wasm-bindgen, release on merge)
> New spec, not an extension of fn-112.

## Decisions

- Proof by receipt: a release requires a green receipt for every suite on the exact commit from master's CI and re-runs nothing; a missing or red receipt refuses, never rebuilds as a fallback.
- One artifact: master's CI builds and tests the package once and uploads it; the release publishes it unchanged. wasm-bindgen-cli is the prebuilt binary at the `Cargo.lock` pin. A merge to master with a new `package.json` version releases with no hand tag.
- Host (2026-09-25): publishing does not live in `tests.yml`. `tests.yml`'s `package` job builds once, runs `npm pack` and `test:dist` on the tarball's own `dist`, and uploads `package` (tarball + `.sha256`). `release.yml` runs on `workflow_run` of a successful Tests push run on master, reads its `head_sha`, checks each suite by green job or by its `scripts/ci-key.mjs` key in master's cache, downloads and sha256-verifies the artifact. Its OIDC `workflow_ref` still names `release.yml`, so the npm trusted publisher needs no change.
- owner (2026-09-25), decision recorded as: "the owner decided that publishing does not live in tests.yml"; PR #118 was restructured to publish from `release.yml` (source: archived FRICTION.md).
- Host (2026-09-25): tag first — the annotated `v<version>` tag is pushed on `head_sha` before `npm publish`, so a rerun converges; a tag on another commit refuses; an already-published version publishes nothing.
- R6 added: trusted publishing with no stored npm token (already true since 0.1.0: `_npmUser` is the OIDC publisher).

## Acceptance Criteria

- **R1:** A release publishes in about 2 minutes after master's CI — to be measured on the first version bump after merge.
- **R2:** Publishes only with green receipts for every suite; otherwise refuses naming the suite — built; simulated on 6 suite cases.
- **R3:** The published tarball is CI's artifact, shown by matching checksums — built (`sha256sum -c` OK on PR #118's artifact).
- **R4:** wasm-bindgen-cli never compiled in CI or release — met.
- **R5:** A version bump on merge publishes; an unchanged version publishes nothing; a lower or equal version refuses — built; simulated on 7 cases plus a rerun against a bare remote.
- **R6:** OIDC trusted publishing, no stored token — met.

## Result

PR #118 (`5f21f611`, `b641a23e`, `9fb92233`, `41321c3f`); actionlint and shellcheck clean; CI run 36125486832 green with the `package` artifact holding `telperion-0.1.4.tgz` and its `.sha256`.

## Open

- R1's ~2-minute release time and R2/R3/R5 on a live release: first measured on the next version bump.

## Friction

- npm's trusted publisher binds a workflow filename (`release.yml`), unnamed in the spec; the first design (publish from `tests.yml`) was redone after CI was green (~30 min, one CI run) -> name external bindings in "What exists".
- dcg refused `mv` and variable redirects -> local setup; a `flowctl task set-satisfies` verb would help.
- npm's 2FA-bypass notice read as a token in use (~10 min) -> quote `_npmUser` beside such notices.
