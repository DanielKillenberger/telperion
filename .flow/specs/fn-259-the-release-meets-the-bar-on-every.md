# The release meets the bar on every consumer

## Goal & Context

The last fn-245 stage, and the check fn-237 rests on. The owner's bar (fn-237 R0, 2026-10-09): at least as good as 0.1.4, with slightly less performance acceptable only for much better fidelity. Astra (fn-245 `ASTRA-ARCH.md` §6) warns that the scorecard compared different default ages and counted up to 10% slower as "no worse"; neither is the owner's rule.

## Acceptance Criteria

R3 is met only by the owner's recorded words.


- **R1 (scorecard redone):** every shipped species, beside 0.1.4: build time, frame time at whole/mid/close, resident memory, the site's field call and its download bytes. Protocol stated before measuring: pinned artifact identities, hardware, seeds, viewport and poses; fresh-page start (shader compilation from renderer creation) apart from warm rebuilds; renderer start and rebuild times measured to a completed, fenced frame; the field call measured to the return of its requested CPU field and query answers, preparation and voxelisation included, with no renderer; a stated sample count and aggregate; failed or invalid runs recorded and never counted as passing; raw results kept in the evidence's raw directory with per-species progress; product default ages and matched-age diagnostics reported apart; no tolerance counted as parity unless the owner set it.
- **R2 (obsolete code gone):** contract A, the stepped light executor, the lattice, the fine-root cut and any grower or predictor the stages replaced are absent from the shipped artifacts, shown by a content and reachability audit of every shipped module and embedded shader; artifact sizes reported separately.
- **R3 (owner):** old-versus-new stills of every shipped species and of the site's grove, with the owner's visual verdict on each whether or not its performance passes; each remaining regression (download included) named with its measured size and the fidelity gain offered for it; the owner's verdict per item.

- **R4 (friction, fn-258 FRICTION.md):** the scorecard's tooling gets a one-build mode for rows that only need memory (fn-258's same-age sweep spent about 80 minutes on four builds a row) and a pixel-difference helper that counts differing pixels (ImageMagick 7's `compare -metric AE` is intensity-weighted and understated a difference 100×); the sweep runs with no other session holding the GPU, its free device memory recorded per row.

- **R5 (= H1, host 2026-10-19):** `tests/browser/bindings.mjs` passes in a headless Chromium with no WebGPU, run alone through a repo entry, and a CI job runs it on every pull request the tests workflow runs on. Each of today's red checks is fixed, or deleted where its subject no longer exists with the reason named in the commit and the task's done summary. Errors: a check whose subject still exists and is red is a failure, never a deletion; a check that needs WebGPU is not in this suite.
- **R6 (= H2, host 2026-10-19):** the tree is formatted once with `cargo fmt --all` in a commit containing nothing else, and CI fails a pull request whose Rust is not `cargo fmt --all --check` clean. Errors: none beyond a non-zero exit of the check.
- **R7 (= H3, host 2026-10-19):** `.flow/config.json` no longer carries `artifacts.html.enabled` or `pipeline.chainStages`, and `flowctl preflight` prints no removed-key note. Errors: no other key changes.
- **R8 (= H4, host 2026-10-19):** fn-290's spec carries the two lines (`artifact_digest` also hashes the curve; the limits inventory is keyed by function, not file). Done in `5c1d0f45a` before planning.

## Owner decisions that bound R1-R3 (host, 2026-10-19, from fn-237)

These are recorded owner and host decisions in fn-237; they narrow the requirements above without new judgment:

- **The site leaves this release** (owner, 2026-10-15, fn-237 "Scope: the site is the site's repository"). R1's "site's field call" is the engine's per-tree field call (grow plus a 36-cell query, preparation and voxelisation included, no renderer), measured as fn-257 R3' measured it against 0.1.4's per-tree time; R1's "download bytes" are the shipped package artifacts, raw, gzip and brotli. R3's grove stills and grove verdict are killenberger.com's work and leave this spec.
- **The lattice stays** (host, 2026-10-16, fn-237 "Astra's release review, incorporated": "Not touched: the cycle, the lattice and the CPU reference"). R2's audit reports the lattice as present by that decision, with what it is and where it ships, rather than as a failure.
- **The oak's device/CPU difference at 40 is accepted** (owner, 2026-10-16, fn-237 "The owner's release decisions"): one node at `FINEST` by f32/f64 rounding and 185 relabelled keys, nothing drawn differs. The drawn-geometry agreement test at each preset's shipped age is fn-289's, after the release. The "Found on the way" oak item is closed by that decision; this spec adds no agreement test.
- **Owner-set tolerances R1 may count:** the absolute build budgets of fn-237 R0a (every shipped tree ready in under 0.5 s warm in the harness on the RTX 3080; Telperion and Laurelin about 1.25 s, and Telperion's 1.33 s at seed 42 accepted, 2026-10-16); the spruce's per-tree field at about 0.35 s against 0.1.4's 0.24 s, "acceptable" (2026-10-16); the artifact ceilings raised to the sizes measured after fn-257 (2026-10-16). Every other row worse than 0.1.4, by any amount, is a regression and goes to R3's ledger for the owner.

## Boundaries

Qualification only; fixes go to their own specs. Depends on stages 4 and 5. Release item for fn-237. No agent runs WebGPU in a browser: GPU measurements run on the native probes (wgpu, no browser) under `telperion-heavy --gpu`, and a browser check that needs WebGPU runs only where CI or the owner runs it. The owner's verdicts (R3) are asked, never inferred.

## Found on the way (host, 2026-10-15, fn-286 RELAYS.md)

- The Oregon white oak's device and CPU trees disagree at age 40: F4's own oak already fails the device/CPU sweep test at 40 (every preset agrees at the gate's age 12); with fn-236's sourced sapling segment, three agreement tests fail with 219 parents differing (232 with fn-286's take-over on, one node apart). Untraced; fn-281 saw a few nodes in 2.3 million at 80. Traced and fixed, or bounded with the owner, before the release PR, and an agreement test added at each preset's shipped age. **Bounded with the owner, 2026-10-16 (above); the test is fn-289's.**
- `tests/browser/bindings.mjs` has 12 checks red on the release branch and no CI job runs it (host, 2026-10-18, fn-257 landing); fn-259 fixes it and runs it in CI (R5).

## Host additions (2026-10-19)

- **H1 (browser bindings suite):** `tests/browser/bindings.mjs` has 12 checks red on the release branch and no CI job runs it. Each red check is fixed, or deleted where its subject no longer exists with the reason named, and a CI job runs the suite. No agent runs WebGPU in a browser: a check that needs WebGPU runs in CI only, and locally the suite runs through whatever headless path it already supports, or the gap is reported.
- **H2 (formatting):** the tree is formatted once with `cargo fmt --all` in a commit of its own, and CI runs `cargo fmt --all --check`.
- **H3 (dead config keys):** the two keys fn-293's FRICTION.md names as removed, `artifacts.html.enabled` and `pipeline.chainStages`, are deleted from `.flow/config.json`.
- **H4 (fn-290 follow-ups):** fn-290's spec (post-release) gains two lines: `artifact_digest` also hashes the curve; the limits inventory is keyed by function, not file.

## Strategy Alignment

Active tracks served by this plan:
- **The tree space** — qualifies the engine's production step (F) against the generator it replaces, the track's "today's generator stays the product until each species beats it".
- **Surface and rendering at scale** — measures frame time and memory per view against the release bar.

## Decision Context

The scorecard runs on the native probes because the owner forbids agents WebGPU in a browser; the native path is the "headless native target" STRATEGY.md names as the rig that measures the frame metric. 0.1.4 is tag `v0.1.4` (`c48a4f28a`), built in its own scratch worktree with its own `target/` (AGENTS.md). `field.mjs` needs WebGPU and has no GPU runner in CI; it stays outside this spec's CI job and is reported.

## Early proof point

Task fn-259-the-release-meets-the-bar-on-every.4 validates the core approach (one preset measured on F4 and on 0.1.4 under the written protocol, every column filled). If it fails, re-evaluate whether 0.1.4 can be measured on the same native protocol before continuing with task 5.

## Quick commands

```bash
cargo fmt --all --check
node scripts/test-bindings.mjs        # task 2's entry; headless Chromium, no WebGPU
npm test
telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast
```

## Requirement coverage

| Req | Description | Task(s) | Gap justification |
| --- | --- | --- | --- |
| R1 | (scorecard redone): every shipped species, beside 0.1.4: build time, frame time at whole/mid/close, resident memory, the site's field call and its download bytes. Protocol stated before measuring: pinned artifact identities, hardware, seeds, viewport and poses; fresh-page start (shader compilation from renderer creation) apart from warm rebuilds; renderer start and rebuild times measured to a completed, fenced frame; the field call measured to the return of its requested CPU field and query answers, preparation and voxelisation included, with no renderer; a stated sample count and aggregate; failed or invalid runs recorded and never counted as passing; raw results kept in the evidence's raw directory with per-species progress; product default ages and matched-age diagnostics reported apart; no tolerance counted as parity unless the owner set it. | fn-259-the-release-meets-the-bar-on-every.4, fn-259-the-release-meets-the-bar-on-every.5 | — |
| R2 | (obsolete code gone): contract A, the stepped light executor, the lattice, the fine-root cut and any grower or predictor the stages replaced are absent from the shipped artifacts, shown by a content and reachability audit of every shipped module and embedded shader; artifact sizes reported separately. | fn-259-the-release-meets-the-bar-on-every.6 | — |
| R3 | (owner): old-versus-new stills of every shipped species and of the site's grove, with the owner's visual verdict on each whether or not its performance passes; each remaining regression (download included) named with its measured size and the fidelity gain offered for it; the owner's verdict per item. | fn-259-the-release-meets-the-bar-on-every.7 | — |
| R4 | (friction, fn-258 FRICTION.md): the scorecard's tooling gets a one-build mode for rows that only need memory (fn-258's same-age sweep spent about 80 minutes on four builds a row) and a pixel-difference helper that counts differing pixels (ImageMagick 7's `compare -metric AE` is intensity-weighted and understated a difference 100×); the sweep runs with no other session holding the GPU, its free device memory recorded per row. | fn-259-the-release-meets-the-bar-on-every.3, fn-259-the-release-meets-the-bar-on-every.5 | — |
| R5 | (= H1, host 2026-10-19): `tests/browser/bindings.mjs` passes in a headless Chromium with no WebGPU, run alone through a repo entry, and a CI job runs it on every pull request the tests workflow runs on. Each of today's red checks is fixed, or deleted where its subject no longer exists with the reason named in the commit and the task's done summary. Errors: a check whose subject still exists and is red is a failure, never a deletion; a check that needs WebGPU is not in this suite. | fn-259-the-release-meets-the-bar-on-every.2 | — |
| R6 | (= H2, host 2026-10-19): the tree is formatted once with `cargo fmt --all` in a commit containing nothing else, and CI fails a pull request whose Rust is not `cargo fmt --all --check` clean. Errors: none beyond a non-zero exit of the check. | fn-259-the-release-meets-the-bar-on-every.1 | — |
| R7 | (= H3, host 2026-10-19): `.flow/config.json` no longer carries `artifacts.html.enabled` or `pipeline.chainStages`, and `flowctl preflight` prints no removed-key note. Errors: no other key changes. | fn-259-the-release-meets-the-bar-on-every.1 | — |
| R8 | (= H4, host 2026-10-19): fn-290's spec carries the two lines (`artifact_digest` also hashes the curve; the limits inventory is keyed by function, not file). Done in `5c1d0f45a` before planning. | — | Done at plan time in `5c1d0f45a` (fn-290's spec edited); no task needed. |
