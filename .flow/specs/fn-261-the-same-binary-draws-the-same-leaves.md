# The same binary draws the same leaves every run

## Goal

fn-258 (2026-10-10) found that on the fine-executor trees (spruce, Telperion) two runs of the same binary at the same seed, pose and age differ in 0.08-0.16 % of pixels, all on leaves, while the CPU-grown palm and oak are byte-identical; a cut meant to change nothing could only be held to that noise. AGENTS.md "Generator evolution" keeps repeatability within an implementation as a contract. Find the first stage whose output differs and make it order-independent on the device.

## Decisions

- Host (2026-10-11): the device-grown node table's numbering is out of scope; it moved to fn-256 R2 with `.flow/evidence/fn-261-the-same-binary-draws-the-same-leaves/grown-probe-and-queue-experiment.patch` as its check. fn-261 closes on leaves and segments.
- The fix sorts runs by an integer key (no float, no atomic), keeping a root's runs together and roots in their stubs' order (fn-212 host decision 3).

## Acceptance Criteria

- **R1:** standalone test building the spruce twice in fresh contexts, comparing leaf instances and segments byte for byte; red on F4; first differing stage named against the code — met (`the_spruce_builds_the_same_leaves_twice`; segments and leaves equal as sets, only order differs).
- **R2:** that stage made order-independent on the device; green over 10 repeats on every fine-executor preset; frame and build time before and after — met.
- **R3:** two stills of the same view byte-identical for every shipped preset; gate and npm test green — met (all 8 presets).

## Result

Cause: `stations.wgsl` `owned` ranked each bearing run within its root by `atomicAdd`, so leaf indices varied and `foliage.wgsl` drew each leaf with another leaf's `vary(id)`. Fix: `keys` writes each run's (stub, first node), a bitonic sort (`sort.wgsl`) orders them, `starts` numbers runs in sorted order; the `owned` pass is gone. Ten builds of each device preset (spruce, beech, birch, Telperion, Laurelin) byte-identical; all eight shipped presets' stills byte-identical (pre-fix spruce: 31,574 differing pixels). Cost: the stations stage about +3.5 ms at least (spruce 9.6-10.8 → 13.4-23.2 ms), hidden in 20-27 s builds; frame time unchanged. Gate green (render 239 passed), `npm test` 155.

## Open

- Device-grown node numbering is not stable run to run (beech intermittently, Laurelin always; leaves, segments, stills unaffected); a sorted overflow queue fixed the beech but not Laurelin, so growth has another unnamed order source -> fn-256 R2.
- A shared device sort for `sweep.wgsl`'s bitonic and `sort.wgsl` (host note, left).

## Friction

- First R1 run deadlocked 10 minutes on the non-reentrant test mutex holding the GPU lock -> fn-237 R12.
- dcg hook blocked restoring base files to time the pre-fix binary -> local setup; a before/after probe recipe suggested.
- `compare -metric AE` intensity-weighted again -> fn-259 R4's pixel-difference helper.
- Package-by-package gate ran jev before core -> fn-237 R10.
- "Grown tree" read as the pipeline's part; review caught it -> test-only readback of the device node table suggested (fn-256 R2).

## Evidence

- `.flow/evidence/fn-261-the-same-binary-draws-the-same-leaves/grown-probe-and-queue-experiment.patch` (node-table probe and queue experiment; fn-256 R2's check)
