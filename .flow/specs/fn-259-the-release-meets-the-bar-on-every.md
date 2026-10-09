# The release meets the bar on every consumer

## Goal & Context

The last fn-245 stage, and the check fn-237 rests on. The owner's bar (fn-237 R0, 2026-10-09): at least as good as 0.1.4, with slightly less performance acceptable only for much better fidelity. Astra (fn-245 `ASTRA-ARCH.md` §6) warns that the scorecard compared different default ages and counted up to 10% slower as "no worse"; neither is the owner's rule.

## Requirements

- **R1 (scorecard redone):** every shipped species, beside 0.1.4: build time, frame time at whole/mid/close, resident memory, the site's field call and its download bytes. Protocol stated before measuring: pinned artifact identities, hardware, seeds, viewport and poses; fresh-page start (shader compilation from renderer creation) apart from warm rebuilds; renderer start and rebuild times measured to a completed, fenced frame; the field call measured to the return of its requested CPU field and query answers, preparation and voxelisation included, with no renderer; a stated sample count and aggregate; failed or invalid runs recorded and never counted as passing; raw results kept in the evidence's raw directory with per-species progress; product default ages and matched-age diagnostics reported apart; no tolerance counted as parity unless the owner set it.
- **R2 (obsolete code gone):** contract A, the stepped light executor, the lattice, the fine-root cut and any grower or predictor the stages replaced are absent from the shipped artifacts, shown by a content and reachability audit of every shipped module and embedded shader; artifact sizes reported separately.
- **R3 (owner):** old-versus-new stills of every shipped species and of the site's grove, with the owner's visual verdict on each whether or not its performance passes; each remaining regression (download included) named with its measured size and the fidelity gain offered for it; the owner's verdict per item.

## Boundaries

Qualification only; fixes go to their own specs. Depends on stages 4 and 5. Release item for fn-237.
