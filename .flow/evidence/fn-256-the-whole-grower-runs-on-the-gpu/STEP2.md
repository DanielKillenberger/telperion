# fn-256 task 2, step 2: units and layout against the warm budget (worker, 2026-10-11)

**The stop condition does not fire.** Units and layout on the device, scaled from their busy cycles to a whole build and added to the spike's hierarchy and light, come to about 0.6-0.7 s for Telperion (F4's warm budget 1.2 s) and under 0.1 s for the beech (0.8 s). The settle is not yet measured.

## How the build was scaled

- **Device timings:** the busy cycles' times on an RTX 3080 under Vulkan, median of 9 (UNITS.md, LAYOUT.md).
- **What each phase scales by:** the run's totals from fn-255's census (`early three <preset> <age> 42 cycle`, which now prints the sums):
  - units, by phytomers grown;
  - layout, by leaves laid.
- **Dispatch overhead:** the spike's measured cost per dispatch (7.6 µs in one pass, 25.8 µs a pass each) times the cycles. That is 38 dispatches a cycle for units and layout, beside the spike's hierarchy-and-light counts.

| Telperion 82, seed 42 | Busy cycle | Over the run | Ratio | Busy ms (as is / exact) | Build ms (as is / exact) |
|---|--:|--:|--:|--:|--:|
| Units: phytomers grown | 851,381 | 18,239,310 | 21.4 | 18.6 / 24.8 | 398 / 531 |
| Layout: leaves laid | 823,805 | 17,387,929 | 21.1 | 3.6 / 5.5 | 76 / 116 |
| Hierarchy and light (SPIKE.md) | | | | | 85-115 |
| Dispatch overhead, units and layout (82 × 38) | | | | | 24-80 |
| **Total, settle excluded** | | | | | **583-669 / 756-842** |

| European beech 40, seed 42 | Busy cycle | Over the run | Ratio | Busy ms (as is / exact) | Build ms (as is / exact) |
|---|--:|--:|--:|--:|--:|
| Units: phytomers grown | 88,937 | 657,516 | 7.4 | 2.5 / 3.2 | 18 / 24 |
| Layout: leaves laid | 79,789 | 568,579 | 7.1 | 0.7 / 0.9 | 5 / 7 |
| Hierarchy, light and dispatch overhead | | | | | 30-90 (dispatch-bound at 67k axes) |
| **Total, settle excluded** | | | | | **53-113 / 61-121** |

"Exact" is with every site the driver fuses, reorders or rounds held to the CPU's order (the two patches below).

## What the host decides

1. **The f32 contract.** COUNCIL.md has floats agreeing within tolerance, with no guards on fused multiply-adds or division. On the device, as committed:
   - **Exact:** every integer, count, index, order and decision of the units, wake and layout, on 15 captures.
   - **Not exact in the units' floats:** f32 values differ in their last places, and so do two integers floored from an f32. One is `below` on Telperion 82 (2818 against 2817). The other is 7 of 145 waking ticks on the spruce, each off by one tick.
   - **Not exact in the layout's floats:** f32 geometry differs in about 2% of its words.
   - **The causes, each named file:line:** the driver fuses multiplies into adds, reorders and simplifies sums, and rounds division and square root incorrectly.
   - **What a whole build does with them:** an integer floored from a float, or a decision reading one, eventually takes another value, and a different tree follows. So "integers exact against the CPU" cannot hold over a build under the tolerance standard. Section 8 of DESIGN.md assumed only f64 reached decisions; the measurement says f32 does too.
   - **Holding every site to the CPU's order** makes all 15 captures exact in every word: `exact-units.patch` and `exact-layout.patch`. It costs about +35% on units and +50% on layout, about 170 ms on Telperion's build.
   - **The marker functions are committed as plain operations,** so either choice is a small change.
   - **The choice:** the guards (integers and trees exact; a cost of about 170 ms on Telperion), or COUNCIL.md's tolerance (the device's trees then agree with the CPU's by lineage within a measured rate of unmatched axes, as fn-255's table does for f32 order).
2. **Cold compile.** Measured with the units test's own timer around `Kernels::new`, through to the device's completion.
   - **The units module:** 37.4 to 37.7 s cold.
   - **The layout's six entries:** 3.2 s cold.
   - **Warm:** 0.02 s from the driver's cache, but only once the cache can hold the module. At the driver's default size this machine's cache is full (1.1 GB): a second run compiled again in 37.6 s, and with `__GL_SHADER_DISK_CACHE_SIZE` raised it took 0.02 s.
   - **Beside F4:** F4's shipped chain grower makes cold in 13.2 to 16.7 s (fn-235 REPRO.md). The spike's hierarchy and light kernels made in 0.35 s.
   - **Where the cost lies:** in the same place fn-235 found it, the inlined 64-bit emulation in the one body the driver compiles. fn-235's bisection method applies.
   - **Whether this is acceptable** for the site (fn-257, against 0.1.4's 273 ms) and the app is the host's call. Cutting it is engineering on the units' kernel shape: more and smaller entries, or fewer inlined 64-bit operations.
