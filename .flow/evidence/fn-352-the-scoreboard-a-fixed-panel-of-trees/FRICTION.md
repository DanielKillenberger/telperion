# fn-352 friction

One dated entry per report, written as it happens.

## 11 Nov 2026, the probe has no structure from the drawn grow

- **Doing:** building the per-tree measures (R2).
- **What slowed it:** the pipeline's grown tree (`executor::grow`) keeps the engine `Structure` private, and the drawn `Tree` carries no birth cycle, so "new shoots in the last year" and "the lowest living limb" cannot be read off the drawn tree. The probe grows each tree twice: once through the pipeline for the drawing and the drawn measures, once through `telperion_space::grow` (from `executor::growth`, the same species and request) for the structure measures.
- **Cost:** the panel's CPU time roughly doubles, about 50 extra minutes for the twelve oaks at 120 to 300 (estimated from fn-345's grow times).
- **What would remove it:** a public read of the grown structure beside the drawn tree (`Grown::structure()` or the cycle on `Node`), a product change this spec may not make.

## 11 Nov 2026 13:46, the old oaks dominate the panel's wall time

- **Doing:** the first full panel, `base-2026-11-11`, one heavy job at 24 threads.
- **What slowed it:** the seven other presets' 84 trees and the oak at 40 took 9 minutes together; the oak at 80 took 8.5 minutes for 13 trees and the oak at 120 about 1.5 to 2 minutes a tree. At that rate the oak at 200 and 300 will take about another 1.5 to 2 hours, roughly three quarters of the panel. Each tree grows twice (the entry above), and the panel draws 13 oaks at each age where the sheets need 4 and the measures 12.
- **Cost:** about two hours of wall time per run while no other heavy job may start; worker time spent waiting.
- **What would remove it:** the structure read above (halves it); or fewer measure seeds at 200 and 300 (a panel design call for the host).
