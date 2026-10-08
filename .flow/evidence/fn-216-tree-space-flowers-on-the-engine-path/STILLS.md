# fn-216 task 2: the stills, resumed after the crash (the worker's reading, not a verdict)

Seed 7, both presets at 40, the GPU executor (headless), overlays in `overlays/`. Sheets are in `raw/` (`*-r5.png`), and every one was viewed. The host's verdict and Astra's are still to come.

## What changed in tuning

The `bloom*` material rows are linear albedos on the leaves' scale: the beech's leaf face is 0.021 and the oak's is 0.021/0.048/0.021. Petals at 0.2 to 0.9 (rounds 1 to 4) were ten to forty times a leaf and clipped to flat white whatever their value. A pure red test showed the rows do reach the draw. Final: beech petals 0.1/0.097/0.09 (back x0.8), oak catkins 0.12/0.13/0.025 (back x0.85).

## Reading

- **Oak catkins, close (`sheet-oak-close-r5.png`):** strands of florets hang from the twigs, yellow-green and tapering, and read as the photograph's catkins. They are thinner and fewer per cluster than the photograph's.
- **Oak, whole and mid (`sheet-oak-whole-r5.png`):** the catkins are all but invisible (FRICTION.md: sub-pixel organs). There is no whole-tree photograph to compare against.
- **Beech blossom, close (`sheet-beech-close-r5.png`):** white five-petalled florets in clusters on the twigs, shaded. Beside the cherry corymb they are smaller and flatter, and they fuse into irregular white patches; no single flower reads as clearly as the photograph's.
- **Beech, whole and mid (`sheet-beech-whole-r5.png`):** a pale flecking of the outer crown, nowhere near the cherry crown's cover of blossom. At the hero pose they hardly show (`walk-sheet-r5.png`).
- **Walk (`walk-sheet-r5.png`, flowering 0 to 0.6 and petals 1 to 5 together, six frames):** no pop. At the hero pose the flowers are too small to judge the walk by eye, so the proof is the suite's no-pop tests (`suite/flowers.rs`, `bloom/tests.rs`).

## Host decisions (2026-10-08)

1. **Option (a):** the CPU step `Expansion::organ` serves both executors for this task. The GPU bloom pass and flowers casting shadows are named items in fn-125's spec.
2. **The stills pass as an organ test.** The oak's catkins read as catkins up close, which is the capability. A real beech's flowers are inconspicuous, so the beech is not wrong to show little. No Astra pass.

Two follow-ups, recorded in fn-217's spec for the first blossoming species (fn-217 at the latest):

- the blossom florets read as flat white scraps rather than five-petal flowers;
- blossom vanishes at whole-tree distance, where a cherry reads as a white mass.
