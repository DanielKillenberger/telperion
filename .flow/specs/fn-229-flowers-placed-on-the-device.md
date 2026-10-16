# Flowers placed on the device

## Goal

fn-216 computed flowers through the CPU's `Expansion::organ` from `Tree::blooms`; on fn-212's device path the fine roots never enter the CPU's tree, so their blooms were missing (3.7-6.2 % of the florets drawn on a flowering overlay; fn-198 MERGE-F216.md). The F4 merge refused flowering families from the device path as an interim named gap (host, 2026-10-08), so a flowering share crossing 0 switched executors, which the principles do not allow to stay. Laurelin and Telperion are defined by their blossom, so it gated them. Placement only.

## Decisions

Host decisions on R1 (2026-10-08), one rule: florets follow exactly the pattern fn-212 built for leaves; "no added pass" means no added readback or round trip, extra kernels in the same submission allowed:
- **D1:** the dress writes the phytomer's 64-bit key into the drawn record and the sweep carries it; the flowering share joins the per-PA table; no binding limit raised, nothing re-keyed.
- **D2:** a second entry point in `place.wgsl`, in the same submission as the device's leaves.
- **D3:** count, then scan, deterministically, in fn-212's existing stations scan, no atomics. Read as per node in the owners scan (blooms stand on non-bearing nodes too).
- **D4:** the tree's combined leaf box padded by the largest floret's reach; the CPU block's florets repacked against it.
- **D5:** one organ buffer and mass grid through the resident handoff, CPU block first.
- Residuals allowed by named class (codex contracts finding declined on that ground).

## Acceptance Criteria

- **R1:** Florets placed on the device from the dressed fine nodes per D1-D5 — met.
- **R2:** Florets per tree equal the CPU whole tree's on beech and spruce at 40 and 80 with a flowering overlay, positions within fn-125's screen, residuals by named class — met.
- **R3:** The interim refusal in `executor::apart` deleted; flower shadows if supported, else recorded — met; flowers cast no shadow (organ draw not in the shadow pass), recorded.
- **R4:** Gate and npm test green; every shipped species unchanged — met.

## Result

- Parity (every age flowering 0.6 under the oak's catkin rows): beech and spruce at 30, 40 and 80 equal the whole tree's floret count except the "least" class (+1 to +5 device florets at the 1e-3 size threshold), e.g. spruce 80 86,520,440 florets. Positions within one 16-bit box code, frames 5.4-9.3 mrad (screen 10), scales within one half-float step. "Turned" class: a floret whose tangent sits at `Vec3::perpendicular`'s switch stands 1-4 mm off.
- Non-flowering trees: sorted leaf words, curve and crown bounds byte-identical to the base; florets byte-identical across three runs. Stills of the flowering beech at two seams viewed: both paths draw 3,369,006 florets.
- Gate: render 195 passed, rest 1,040 passed, npm test 143 passed.

## Open

- Flowers cast no shadow.
- The device leaf buffer's order varies run to run at the base (the `owned` atomicAdd rank), so device stills vary by 0.01-0.3 % of pixels.

## Friction

- R1 assumed a dressed fine node carried the bloom's key and share; it did not (25 min, ~150k tokens reading) -> the host's D1-D5; a spec should name how inputs reach the dressed nodes.
- A base build's target in the tmpfs scratchpad ran out of quota -> proposed documenting where a second checkout's `target/` goes.
- The shell guard refuses redirects to variable paths -> local setup.
