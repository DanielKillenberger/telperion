# fn-352 friction

One dated entry per report, written as it happens.

## 11 Nov 2026, the probe has no structure from the drawn grow

- **Doing:** building the per-tree measures (R2).
- **What slowed it:** the pipeline's grown tree (`executor::grow`) keeps the engine `Structure` private, and the drawn `Tree` carries no birth cycle, so "new shoots in the last year" and "the lowest living limb" cannot be read off the drawn tree. The probe grows each tree twice: once through the pipeline for the drawing and the drawn measures, once through `telperion_space::grow` (from `executor::growth`, the same species and request) for the structure measures.
- **Cost:** the panel's CPU time roughly doubles, about 50 extra minutes for the twelve oaks at 120 to 300 (estimated from fn-345's grow times).
- **What would remove it:** a public read of the grown structure beside the drawn tree (`Grown::structure()` or the cycle on `Node`), a product change this spec may not make.
