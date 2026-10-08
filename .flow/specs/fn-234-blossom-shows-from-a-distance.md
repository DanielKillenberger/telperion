# Blossom shows from a distance

## Goal & Context

fn-217 task 3 measured blossom at the whole-tree view (TASK3.md, M0/M1): rasterisation keeps 92-96 % of the flowers' finest coverage, so sampling is not the loss, yet a beech flowering as densely as a cherry reads as a green crown with grey speckle beside the cherry photograph's white mass, and tripling the florets with larger petals raised visible coverage only to 2.9 % of the frame. The outer leaves hide most florets and petals read grey at distance: where florets stand, and how they are lit, decides it, not how many (Astra P1-3 on fn-217's design). Telperion (fn-218) and Laurelin (fn-219) need their blossom to read from the whole-tree view; real flowering trees (cherry, apple, laburnum) do too. Host decision (2026-10-08): transferred from fn-217 R2 as its own capability spec.

## Requirements

- **R1 (measure first):** with fn-217's organ-mask instrument, where the florets of a flowering engine tree stand against the leaves that hide them (depth into the crown, share occluded from the whole-tree camera), and the petals' rendered brightness against their material colour at distance; claims checked against the code before any fix is chosen.
- **R2 (capability, by degree):** the fix the measurement names, stated before code and chosen by the host: candidates include florets seated at the crown's outer shoots (the engine's flowering positions, e.g. fn-223's short-shoot tips), flowering ahead of leaf-out as a per-age share of leaf presence, and petal lighting at distance; every new row neutral by default, so every species is byte-identical where it is.
- **R3 (proved):** a cherry-dense flowering tree at the whole-tree view reads as a white blossom mass beside the cherry photograph, and Laurelin's racemes read gold, on stills viewed; organ coverage measured before and after; cost measured.
- **R4:** gate and npm test green.

## Boundaries

Blossom placement and visibility only. Depends on fn-217. fn-218 and fn-219 depend on it.
