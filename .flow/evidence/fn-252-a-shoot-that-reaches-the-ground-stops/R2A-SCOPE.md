# fn-252 R2a, before building: what the rule touches (worker, 2026-10-14)

The host's R2a: an axis whose age states a finite life dies at that life, its laterals with it. Before building, `doomed.rs` grew every shipped species, seeds 1 and 7, and listed the kept axes that are past their age's life today and are kept only because what they bear still lives. The raw log is `raw/doomed.log`.

How the probe counts:
- An axis's life runs from its chain's birth in its age; a relay carries its time on.
- An axis carried on (a continuation or relay child) has not ended.
- "Gone" means past its life plus its shedding delay, so the rule would drop it now, with everything it bears. The percentage is of the tree's phytomers.
- Ages with an infinite shedding delay are never shed under the existing fade, and so are unchanged. That covers every stem age, the beech's `TOP_FORK` (pa5) and its pa8.

| Species | Age | What would go (PA: share of phytomers) |
|---|--:|---|
| silver birch | 10 | nothing |
| silver birch | 20 | `SPRIG`: 31-32% |
| silver birch | 40 | `SPRIG`: 6-12%; `BRANCH` (strands with forks): 4-7% |
| silver birch | 80 | `BRANCH`: about 15% |
| ordinary | 40 | `SPRIG`: 18-31%; `SPUR`: 2-3%; `SHOOT`: 5%; `SHORT`: about 1% |
| ordinary | 80 | `SPRIG`: 16-28%; `BRANCH`: 19-21%; `SHOOT`: 4-5%; `SPUR`: 2%; `BOUGH`: 0-0.2% (2 axes at seed 7) |
| Oregon white oak | 40 | `SPRIG`: 1-3%; `BRANCH`: 3-4%; `TWIG`: 11-14% |
| Oregon white oak | 80 | `BRANCH`: 12-15%; `TWIG`: 10-14%; `SHOOT`: about 0.5%; `BOUGH`: under 0.1% |
| European beech | 40 | `BRANCH`: 9-15%; `SHOOT`: 15-17% |
| European beech | 80 | `BRANCH`: about 9%; `SHOOT`: 9-12% |
| Laurelin, Telperion | | as the beech (the same species) |
| Norway spruce | 40 | `BRANCHLET`: 0.1% or less |
| date palm | | nothing (one axis) |

What I read from it, for the host to judge:
- **No limb or scaffold system goes, apart from a sliver.** The birch's limbs and boughs are carried on by relays (`relay_ended` 1). The oak's and the ordinary tree's boughs lose under 0.2%.
- **The branch systems and twigs that clothe the limbs of the oak, beech and ordinary tree would go in large shares.**
  - Oak: up to 29% at 80.
  - Beech: up to 32% at 40.
  - Ordinary: up to 50% at 80.
  - Their crowns are owner-reviewed (fn-236's oak verdict of 2026-10-14).
- **Many of these axes belong to ages that carry on** (`continuation` 1: the ordinary `SPRIG`, the oak's and beech's `BRANCH`).
  - The axes are past their life because their own apex died early, so they were never carried on.
  - Whether "dies at its age's life" applies to an axis of a continuing age whose apex died early is a design point the ruling does not settle.
- **The rule also ends the birch's hanging strands at 12 years** with their forks: 15% of the 80-year tree.

## Where the rule lives (read, not built)

The rule must change in each of these places, together:
- **CPU, `shed.rs`:**
  - `fades`: the lateral carry into a bearer, capped at the bearer's life, and its `alive`.
  - `lasting`.
- **CPU, elsewhere:**
  - `grow/shoot.rs`: horizon and `shed_certain`.
  - `grow/cycle/retire.rs`.
  - `fine/coarse/expected.rs`.
- **Closed form:**
  - `kept.rs` and `kept/path.rs`.
  - `predictor.rs` and `predictor/axis.rs`.
  - `boundary.rs`, whose pinned rows are in `boundary/tests.rs`.
  - `kept/tests.rs` `last_living`.
- **Device:**
  - `settle_sweep.wgsl` `fade_grown`.
  - `retire_sweep.wgsl` `carried_up`.
  - `retire_axis.wgsl`.
  - `settle_assign.wgsl` `living_main`, plus the packing of each PA's life into its settle row, which is already there as `K_LIFE`.
- **Pins:** `suite/identity.rs` `PINS` and each catalogue's `pins.json`.
