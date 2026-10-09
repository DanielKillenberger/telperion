# Blossom shows from a distance

## Goal & Context

fn-217 task 3 measured blossom at the whole-tree view (TASK3.md, M0/M1): rasterisation keeps 92-96 % of the flowers' finest coverage, so sampling is not the loss, yet a beech flowering as densely as a cherry reads as a green crown with grey speckle beside the cherry photograph's white mass, and tripling the florets with larger petals raised visible coverage only to 2.9 % of the frame. The outer leaves hide most florets and petals read grey at distance: where florets stand, and how they are lit, decides it, not how many (Astra P1-3 on fn-217's design). Laurelin (fn-218) and Telperion (fn-219) need their blossom to read from the whole-tree view; real flowering trees (cherry, apple, laburnum) do too. Host decision (2026-10-08): transferred from fn-217 R2 as its own capability spec.

## Requirements

- **R1 (measure first):** with fn-217's organ-mask instrument, where the florets of a flowering engine tree stand against the leaves that hide them (depth into the crown, share occluded from the whole-tree camera), and the petals' rendered brightness against their material colour at distance; claims checked against the code before any fix is chosen.
- **R2 (capability, by degree):** the fix the measurement names, stated before code and chosen by the host: candidates include florets seated at the crown's outer shoots (the engine's flowering positions, e.g. fn-223's short-shoot tips), flowering ahead of leaf-out as a per-age share of leaf presence, and petal lighting at distance; every new row neutral by default, so every species is byte-identical where it is. The capability is built on both executors wherever it touches them: the CPU path (`pipeline/space.rs::convert`, `mesh::build`) and the resident device path the browser uses (`prepare_async(..., Delivery::Resident)` through `prepare_fine`, with flowering sites in `generation/fine/blooms.wgsl` and floret placement mirrored there).
- **R3 (proved):** a cherry-dense flowering tree at the whole-tree view shows its blossom as a distinct white across the crown, and Laurelin's racemes as a distinct warm colour, on stills viewed beside the cherry photograph (amended 2026-10-09, host: the photograph's leafless mass is phenology, a post-release spec; Laurelin's gold hue is fn-218's values); organ coverage measured before and after on both executors; positive fixtures with the capability on compare the CPU and the resident device in flower counts and positions across the partition seams (fn-125's screen), in leaf presence where it changes, and in whole-tree organ coverage; neutral rows byte-identical on both paths; cost measured.
- **R4:** gate and npm test green.

## Boundaries

Blossom placement and visibility only. Depends on fn-217. fn-218 and fn-219 depend on it.
