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

## Recount under the narrowed reading (host, 2026-10-14)

The narrowed reading: an age that carries on does so by its clock, whether or not its tip lives; a terminal age dies at its life, its laterals with it. The raw log is `raw/doomed2.log`, seeds 1 and 7. Shares are of the tree's phytomers:
- **Dies** means a terminal age past its life and its shedding delay. Where an age carries on only with chance p, (1 - p) of its share counts here.
- **Carries on** means an age past its life that stays in that age today because its tip died; under this reading it would move to its successor age instead.

| Species | Age | Dies | Carries on (to) |
|---|--:|---|---|
| silver birch | 20 | `SPRIG` 31-32% | none |
| silver birch | 40 | `SPRIG` 6-12%; `BRANCH` 4-7% | none |
| silver birch | 80 | `BRANCH` about 15% | none |
| ordinary | 40 | 12-17%: `SPRIG` (p 0.67) 6-10%, `SHOOT` 5-6%, small orders about 2% | 21-25%: `SPRIG` to `BRANCH` 18-23%, smaller orders the rest |
| ordinary | 80 | 17-21%: `SPRIG` 5-9%, `BRANCH` (p 0.67) 6-7%, `SHOOT` 4-5% | 31-35%: `SPRIG` to `BRANCH` 13-20%, `BRANCH` to `BRANCHLET` 13-15% |
| Oregon white oak | 40 | 12-15%: `TWIG` 11-14%, `SHOOT` under 1% | 5-6%: `SPRIG` to `BRANCH`, `BRANCH` to `SHOOT` |
| Oregon white oak | 80 | 10-14%: `TWIG` 10-14%, `SHOOT` under 1% | 12-15%: `BRANCH` to `SHOOT` |
| European beech | 40 | `SHOOT` 15-17% | 10-16%: `BRANCH` to `SHOOT` |
| European beech | 80 | `SHOOT` 9-12% | 10%: `BRANCH` to `SHOOT` |
| Norway spruce | 40 | 0% | 0.4%: `SPRIG`, `BRANCHLET` |
| date palm | | 0% | 0% |

Two points stand out:
- **The losses outside the birch are not small.**
  - The oak loses 10-15% at once, all twigs.
  - The beech loses 9-17%, its long shoots.
  - The ordinary tree loses 12-21%.
- **For the oak and the beech, "carries on" also ends in death.** Their `BRANCH` carries on into `SHOOT` (`continuation` 1, the ages between being unlived). `SHOOT` is terminal with a life of 5 years. So a dead-tipped branch system carried on by its clock dies five years later, with everything it bears.
  - That adds the oak's 12-15% and the beech's 10-16% to their eventual losses.
  - So the eventual losses are up to about 29% of the oak and 33% of the beech.
