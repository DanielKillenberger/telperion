# Leaves cluster at the engine's short shoots

## Goal

The silver birch (fn-213) reached its second wrong-path stop on Astra's traits (j) and (h) on sheet 5: "repeated feather-like leafy sprays attached to long exposed axes". Leaves were seated evenly along every thin shoot, one every 12 mm, so a long hanging shoot read as a feather; a birch carries most leaves two or three together on short shoots. The owner's round-10 verdict (2026-10-08) named the same thing: leaves "small and numerous to appear like they just fill the twigs length". Host decision (2026-10-07): a capability, not more values. Leaf seating only; the birch's values are fn-213's.

## Decisions

- A short shoot is any engine axis whose physiological age states a cluster; its cluster sits at the axis's last drawn node, one per axis, never at a run end or seam; an axis with no drawn node bears none (carry plan stated in the task before code, commit 3e5d85d2).
- Cluster leaves fan level about the tip's laid heading (the old short-shoot fan's rule, spread in radians), at the tip node's centre, with no contact seating. Short-shoot wood bears no stations, so a long shoot's run continues past its short shoots.
- R3: the canopy's `shortShoot*` rows drew nothing in any shipped preset (spacing 0 everywhere), so they, `short_shoots.rs`, their tests, dial and harness entries were deleted; the beech's four dormant rows went without a byte changing.
- `cluster_leaves` mixes on a new `Whole` scale (rounded), so walks between species stay valid (codex finding; re-review SHIP).

## Acceptance Criteria

- **R1:** Short shoots selected by PA, cluster at the axis's last kept phytomer, carried on both CPU and device paths — met.
- **R2:** Rows `cluster_leaves` (0-8) and `cluster_spread` (0 to pi/2), zero byte-identical, replacing the shoot's even leaves; out-of-bounds refused — met.
- **R3:** One mechanism: the canopy's `shortShoot*` rows checked and deleted — met.
- **R4:** Cluster leaf draws keyed by tip lineage and index through the registry (`CLUSTER`) — met.
- **R5:** Positive fixtures, CPU/device equal across the seam, neutral byte-identical, invalid refused, budget still refuses, cost measured — met.
- **R6:** Gate and npm test green — met.

## Result

- Neutral rows: `artifact_digest` of five presets and five stills byte-identical to base 891c3bd0; catalogue digests re-pinned, reproducing fn-215's pins with the change stripped.
- Beech at 15 with PA 14 at 3 leaves, spread 0.6: device vs CPU 128 clusters above the cut and 195 in fine roots matched by key, tips within 9.9e-7 m, headings within 5.2e-4 rad.
- Cost (beech at 40, CPU, best of 5): build 303.0 ms -> 260.2 ms, stations 240,047 -> 190,382; clustering is cheaper.

## Open

- No row uses `Bounds::or_zero` and nothing uses the browser metadata's `zero` flag now `shortShootSpacing` is retired.
- `docs/pipeline.md` does not describe clusters yet.
- fn-230 should check how it reads the `Whole` scale when it merges.

## Friction

- The command guard blocks heredocs with `Result<()>`, `git checkout --` and `rm -rf`; rustfmt formats child modules -> proposed a repo script that formats only changed files.
- The first workspace check ran at 32 jobs (dispatch's `CARGO_BUILD_JOBS` not in the shell) -> proposed `[build] jobs` in `.cargo/config.toml` or a per-checkout env file.
