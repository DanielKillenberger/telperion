# fn-134 friction

## 2026-09-24: Wasm peak memory regressions had to be found by trial

- **Doing:** checking R3 after splitting the wood into a ring step and a mesh step.
- **What slowed it:** the first candidate raised peak linear memory in 44 of 240 binding builds by 0.5 to 5.4 MB, with identical bytes. The cause was allocation layout in the Wasm allocator, not extra data: a buffer that outlived its stage, a scratch that grew by doubling, and a run table that split the freed scratch below the rings. Nothing reports which allocation moved the high-water mark. The only probe was a full rebuild and the 240-build binding pass (about 4 minutes), repeated after each guess.
- **Cost:** four rebuild-and-measure rounds, about 25 minutes.
- **What would remove it:** a peak-attributing allocator for the Wasm binding in evidence tooling. It would record which allocation site set the high-water mark, with the live bytes of each site at that moment, so a regression names its cause in one run.
