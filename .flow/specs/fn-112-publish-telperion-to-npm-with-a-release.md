# Publish telperion to npm with a release pipeline in CI

## Conversation Evidence

> user: "so telperion is ready to be integrated into killenberger.com?"
> assistant: "The package is not on a registry and has no release tag. fn-101's spec leaves that step to you. Until then the site consumes it as a git dependency or a local build."
> user: "should we package an npm module?"
> assistant: "Yes, publish it. ... The package's `files` is `dist`, and `dist` is gitignored, so a git dependency would have to build on install. ... Name: `telperion`, since it is free and the repo is public. ... Version: start at `0.1.0` and tag releases `v0.1.0`. ... A release workflow beside `tests.yml`: on a `v*` tag, run the gate, `npm run build`, then `npm publish --provenance` with a trusted publisher on npm so no token lives in the repo. ... The one thing to add before the first publish is a short README section pointing at `docs/field-package.md`."
> user: "/flow-next:capture this including a release pipeline in ci"

## Goal & Context

<!-- Goal & Context: 40% [paraphrase], 60% [inferred] from the repository -->

killenberger.com integrates Telperion through the slim field package fn-101 landed on master on 2026-09-22: the `telperion/field` and `telperion/field/voxelize` exports, built by one command into a `dist` folder with the slim Wasm beside them. The package is not on any registry and the repository has no release tag, so today a consumer would install it from git and build it at install time, which needs the Rust toolchain and the wasm target on the site's deploy machine for about a minute of compile per install. [paraphrase]

This spec publishes the built package to npm and makes the publish a CI step driven by a version tag, so a consumer's install is a plain download and every published version traces to a tagged, gated commit. [paraphrase]

Facts checked on 2026-09-22: the repository is public under MIT; `package.json` names the package `telperion` at version 0.1.0, is not marked private, ships `files: ["dist"]`, and `dist` is gitignored; the `telperion` name is free on the npm registry, as is a scoped `@killenberger/telperion`; the only workflow is `tests.yml`. [inferred]

## Architecture & Data Models

The release is a second workflow beside the test workflow, triggered by a tag matching `v*`. It runs the same gate the test workflow runs, builds the package, and publishes with npm provenance under a trusted publisher configured on the npm side, so no publish token lives in the repository or its secrets. A tag whose version does not equal the version in `package.json` fails before publishing. [paraphrase]

The package publishes as `telperion`, unscoped, starting at 0.1.0; a pre-1.0 version signals that the field contract may still move. [paraphrase]

The README gains a short section that names the field package and points at the package doc, since the registry page shows the README. [paraphrase]

## Edge Cases & Constraints

- A tag pushed on a commit whose gate is red publishes nothing; the workflow stops at the gate. [inferred]
- A tag for a version already on the registry fails at publish; the workflow reports it and nothing is overwritten. [inferred]
- The published tarball holds `dist` only: the entry points, their type declarations, the slim Wasm beside `field.js`, and the full module the main entry uses. No source, no evidence, no experiments. [inferred]
- The publish step never runs on a branch push or a pull request; the test workflow is unchanged. [inferred]

## Acceptance Criteria

- **R1:** A release workflow exists beside the test workflow, triggered only by a `v*` tag, and it runs the repository's gate before any build or publish step. [paraphrase]
- **R2:** The workflow builds the package with the existing build command and publishes it to npm with provenance through a trusted publisher; no npm token is stored in the repository or its secrets. [paraphrase]
- **R3:** A tag whose version differs from `package.json` fails the workflow before the publish step, with the two versions named in the failure. [inferred]
- **R4:** A dry-run pack on a clean checkout lists only files under `dist`, and among them the `field.js`, `voxelize.js` and `telperion-field.wasm` files with their type declarations; the listing is recorded in the spec's evidence. [inferred]
- **R5:** The README carries a section naming the field package, the two exports and the doc that describes them. [paraphrase]
- **R6 (amended by the host, 2026-09-22):** npm attaches a trusted publisher only to a package that already exists, so the package is created by one publish of `v0.1.0` by the owner's hand from the tagged master commit, the trusted publisher is then attached on npmjs.com, and the workflow proves itself on the first later tag, `v0.1.1`, published with provenance and no stored token. A fresh project with no Rust toolchain installs that version from the registry and runs the Node smoke from the package doc: grow a species, query a grid, read the four answers. The tag, the install and the run are recorded in the spec's evidence with the Node version. [paraphrase, host amendment]

## Boundaries

- No change to the package's exports, the field contract or the build itself; this spec ships what master has. [inferred]
- No changelog tooling, no automatic version bumps and no pre-release channels. A version is bumped by hand in `package.json` and tagged by hand. [inferred]
- The trusted publisher on the npm side is configured by the owner, whose account owns the package; the spec's workflow assumes it exists and R6 is the proof. [paraphrase]
- The main entry `dist/telperion.js` inlines the full Wasm and the renderer glue as text, about 6.7 MB; whether to fix its loading before the first publish is the owner's open decision, recorded under Decision Context, and not built here unless the owner says so. [inferred]

## Decision Context

- Registry rather than git dependency, because `dist` is gitignored and a git install would compile Rust on the consumer's deploy machine. [paraphrase]
- Unscoped `telperion` rather than `@killenberger/telperion`: the name is free and the repository is public. The owner may still choose the scope before the first publish; only the package name changes. [paraphrase]
- Provenance with a trusted publisher rather than a stored token, so the published artifact is bound to the tagged commit and the workflow run. [paraphrase]
- Depends on fn-101, landed on master as PR #58 on 2026-09-22. [inferred]

## Decision Context, added by the host (2026-09-22)

- The first publish moves to the owner's hand because npm's trusted-publisher form lives on an existing package's settings page; the workflow's proof is the first tag after it. Recorded in RESULTS.md and FRICTION.md by the worker as an unknown, settled here.
- Open: the main entry's 6.7 MB inlined Wasm. Options: publish 0.1.0 as is, or make the main entry fetch its Wasm as a file the way the field entry does before tagging. The owner decides before the tag.

## Parked unknowns

- Whether the owner's npm account already has two-factor and a trusted publisher; R6 settles it at the first tag.
- Whether `npm run build` on a fresh CI runner completes within the runner's default time; the test workflow's node job already runs the Wasm and renderer builds, so it is expected to.
