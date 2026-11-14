# Bark as plates and scales

## Goal

The owner accepted fn-29 but judged that both trees "lack significant details to make realistic bark textures" and asked for a path to procedural realistic bark. The missing primitive was structure: a network of plates and furrows that branch and merge on the oak, small round scales lifting at the edge on the spruce, each plate with its own tilt, lift and hue, weathered on its face and fresh in its furrow. This spec added that plate network, depth beyond a tilted normal, directional occlusion and colour by structure, all as rows, and made measurement against the reference photographs the first judge so the owner's eye is spent on the last round.

## Decisions

- > user (turn 1, on fn-29's round-four oak trunk): "why is the oak blue? lol"
- > user (turn 2, after the round-five stills): "i mean i'm colorblind but still like it's blue to me. Maybe we do a QA pass and compare against references"
- > user (turn 3): "oak looks much better, spruce looks better but not much. Both lack significant details to make realistic bark textures. Textured leaf is nice."
- > user (turn 4): "i think we can accept this spec. But do we have a path to procedural realistic looking bark?"
- > user (turn 5, on the six-point path the host proposed): "aye /flow-next:capture it"
- A cellular plate network over extending the ridge field (ridges cannot branch, merge or carry identity). Directional occlusion from the height field toward the sun over screen-space AO. Measurement-first judging: fn-29 spent two still rounds on a hue a crop measurement caught.
- Depth: parallax on the height field chosen over near-level vertex displacement by measurement — 3.7x the picture change for +0.049 ms; displacement is bounded by tessellation and would desync shadow and selection. Displacement deleted. (The eight-tree forest had been retired; the single-oak protocol was used for both.)
- After round one's rejection the owner directed acceptance within this spec by a measured hill climb: "wouldn't we want to achieve acceptance within this spec?"

## Acceptance Criteria

- **R1:** Anisotropic, girth-scaled cellular plate network with per-cell identity; oak plates, spruce scales, by rows, no species branch, no image — met.
- **R2:** Depth beyond the shaded normal via one mechanism chosen by a recorded measurement; mesh bytes and fn-24 pins identical with the row off — met (parallax).
- **R3:** Directional occlusion darkens a furrow floor on its shade side only, no new light — met.
- **R4:** Colour by structure: per-plate jitter, weathered faces vs fresh furrows, orientation term — met.
- **R5:** Every still measured against its reference (crop mean, channel order, structure statistics) before anyone views it — met.
- **R6:** Owner judges the eight stills beside the references and fn-29's — met (accepted 2026-09-15 for plates and scales).
- **R7:** Oak native frame recorded beside fn-29's 3.9823 ms, within the owner's bound; orbit 60 fps — met by owner acceptance at 5.2142 ms.

## Result

- Fifteen rows (then `plateFurrowWidth`), default zero: plate cell scale, elongation, dome, edge lift, identity, weathering strength and RGB, orientation strength and RGB, directional occlusion, depth strength. Telperion and Laurelin keep defaults.
- Round one (column-and-cut network): native 4.5192 ms p50. Owner, round one (2026-09-14, at `bc38f63`): "i see some issues in how organic it looks. Looks like armor plating in some cases. Spruce is much better. But also doesn't look too organic."
- Rounds two and three: a six-component structure score (orientation entropy, area variation, furrow curvature, junction arms, dark fraction, furrow period) against the centroid of each species' references, guarded by `bark_score.rs`; it reproduced the owner's ranking unprompted. Scripted hill climb, no model in the loop, 1,510 trials. The row plateau left the oak at 64% of its start, so the primitive changed once to a weighted cellular partition with warps. Round two shipped oak 1.2284, spruce 0.1657; round three added `plateFurrowWidth`, colour-band and dark-fraction constraints and a level pass (oak trunk 1.3293, spruce trunk 0.1870 under re-measured pins).
- Owner verdict (R6 and R7, 2026-09-15), on the round-three stills at `07665bc`: on round two, "it's much much better. but still has much room for improvement."; on accepting this spec for plates and scales and carrying the remaining gaps to a new spec: "ok do it". R7 accepted at native total p50 5.2142 ms (p95 5.6993); the bound moves with the cost work in the follow-on.
- The leaf and needle stills are byte-identical to fn-29's.

## Open

Deferred to the follow-on (fn-42): the bark base colour and level (fn-29's row), fissure depth against the anti-aliasing contract with the near-only path as the cost lever, and a bark-only capture at a known scale so the reference comparison is fair.

## Evidence

- `.flow/evidence/fn32/stills.json` (loaded by `crates/telperion-render/tests/bark_evidence.rs`; its `png` names refer to the original PNGs, archived)
- `.flow/evidence/fn32/stills/{oak-trunk,oak-trunk-wide-furrow,oak-branch,spruce-trunk,spruce-branch}.jpg` (round-three verdict stills; leaf stills are fn-29's)
- `.flow/evidence/fn32/depth-comparison.json` (R2 measurement), `.flow/evidence/fn32/references.json`
