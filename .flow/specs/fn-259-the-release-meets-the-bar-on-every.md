# The release meets the bar on every consumer

## Goal & Context

The last fn-245 stage, and the check fn-237 rests on. The owner's bar (fn-237 R0, 2026-10-09): at least as good as 0.1.4, with slightly less performance acceptable only for much better fidelity. Astra (fn-245 `ASTRA-ARCH.md` §6) warns that the scorecard compared different default ages and counted up to 10% slower as "no worse"; neither is the owner's rule.

## Requirements

- **R1 (scorecard redone):** every shipped species, beside 0.1.4: build time, frame time at whole/mid/close, resident memory, the site's field call and its download bytes. Protocol stated before measuring: pinned artifact identities, hardware, seeds, viewport and poses; fresh-page start (shader compilation from renderer creation) apart from warm rebuilds; renderer start and rebuild times measured to a completed, fenced frame; the field call measured to the return of its requested CPU field and query answers, preparation and voxelisation included, with no renderer; a stated sample count and aggregate; failed or invalid runs recorded and never counted as passing; raw results kept in the evidence's raw directory with per-species progress; product default ages and matched-age diagnostics reported apart; no tolerance counted as parity unless the owner set it.
- **R2 (obsolete code gone):** contract A, the stepped light executor, the lattice, the fine-root cut and any grower or predictor the stages replaced are absent from the shipped artifacts, shown by a content and reachability audit of every shipped module and embedded shader; artifact sizes reported separately.
- **R3 (owner):** old-versus-new stills of every shipped species and of the site's grove, with the owner's visual verdict on each whether or not its performance passes; each remaining regression (download included) named with its measured size and the fidelity gain offered for it; the owner's verdict per item.

- **R4 (friction, fn-258 FRICTION.md):** the scorecard's tooling gets a one-build mode for rows that only need memory (fn-258's same-age sweep spent about 80 minutes on four builds a row) and a pixel-difference helper that counts differing pixels (ImageMagick 7's `compare -metric AE` is intensity-weighted and understated a difference 100×); the sweep runs with no other session holding the GPU, its free device memory recorded per row.

## Boundaries

Qualification only; fixes go to their own specs. Depends on stages 4 and 5. Release item for fn-237.

## Found on the way (host, 2026-10-15, fn-286 RELAYS.md)

- The Oregon white oak's device and CPU trees disagree at age 40: F4's own oak already fails the device/CPU sweep test at 40 (every preset agrees at the gate's age 12); with fn-236's sourced sapling segment, three agreement tests fail with 219 parents differing (232 with fn-286's take-over on, one node apart). Untraced; fn-281 saw a few nodes in 2.3 million at 80. Traced and fixed, or bounded with the owner, before the release PR, and an agreement test added at each preset's shipped age.
- `tests/browser/bindings.mjs` has 12 checks red on the release branch and no CI job runs it (host, 2026-10-18, fn-257 landing); fn-259 fixes it and runs it in CI.
