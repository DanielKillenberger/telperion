# fn-197 step 4b: λ in range, dead-branch persistence, leader λ, upkeep 0.35, 2026-10-05

Host decisions 17 to 22 are recorded (DESIGN-OPTIONS.md, section 12). The spec's Design section and R2 are amended (R2: light must not widen the seed-to-seed spread in leaf count against neutral light).

## Done

- **λ walked within Pałubicki's 0.45 to 0.55 for PAs 0, 1 and 2,** with the reason in the test (decision 17).
- **The oak's limbs get a dead-branch persistence of 5 years** (`shedding: Some(5)` on LIMB; decision 18).
  - **Estimated, not sourced.** The searches found only an arborists' account ("oaks often retain dead branches for years") and a *Q. serrata* study that reports dead-branch zones but no persistence in years. No measured number was found (FRICTION.md).
  - Fork and leader are the trunk's continuations, never laterals, so the shedding rule never tests them. The boughs already had 4 years.
  - **The neutral oak is unchanged to the bit at 80 years, seeds 1 and 7** (`measures hash`: `eb3a775b729f8339`, `4823045dfcec130b`). No limb at neutral goes idle.
- **`MEMORY` is kept, marked estimated,** with a friction entry for the source search (decision 20).
- **Upkeep is 0.35** (decision 22).
- **A per-PA λ flag for the renders:** `--control-pas <pas>:<λ>`.

## The walks: not green

**Light walks, λ in 0.45 to 0.55:**

| Setting | Slope | Where |
|---|--:|---|
| Trunk (PA 0) λ | 22.75 | seed 2 at 0.512 (within the bound) |
| **Limbs (PA 1) λ** | **52.56** | seed 2 at 0.4655 (**fails the bound of 30**) |
| Twigs (PA 2) λ | 1.13 | |

The limbs' λ fails inside the published range. As decision 17 directs, nothing is widened, and the slope is reported here.

## The leader's λ (decision 21): height is extremely sensitive

Seed 1, every other PA at 0.45, upkeep 0.35, persistence on (`raw/step4b/explore.png`):

| Trunk, fork and leader λ | Height | Width (m) | Leaves |
|---|--:|---|--:|
| 0.45 (step 4) | 13.3 m | 19.2 × 17.2 | 0.72M |
| 0.47 | 18.0 m | 18.4 × 16.9 | 0.75M |
| 0.48 | 20.6 m | 18.2 × 16.4 | 0.77M |
| 0.5 | 26.6 m | 18.3 × 14.6 | 0.82M |
| 0.52 | 34.4 m | 19.6 × 14.8 | 0.86M |
| 0.55 | 52.3 m | 24.7 × 16.2 | 0.93M |
| 0.6 | 110.8 m | 37.7 × 22.8 | 1.02M |

- **Decision 21's 0.55 to 0.6 makes a 52 to 111 m pole.**
- **Why:** the leader's few buds get a vigour per presence far above the tree's mean, which thousands of twig buds set. Their size, (r / r̄)^ψ, has no ceiling, so every yearly leader unit is stretched. 0.48 lands at 19 to 22 m.

## Five seeds at leader λ 0.48, others 0.45, upkeep 0.35, persistence on (I viewed every still)

Sheets are `raw/step4b/seeds-bare.png` and `seeds-whole.png`, each with round 4, step 3d and step 4b.

| Seed | Height | Width (m) | Leaves | Round 4 leaves |
|--:|--:|---|--:|--:|
| 1 | 20.6 | 18.2 × 16.4 | 0.77M | 2.16M |
| 7 | 17.8 | 15.0 × 15.7 | 0.83M | 2.22M |
| 2 | 21.2 | 15.2 × 19.7 | 1.22M | 3.51M |
| 3 | 20.4 | 19.1 × 16.7 | 1.02M | 3.09M |
| 4 | 17.3 | 16.5 × 16.5 | 0.21M | 0.87M |

### Reading

- **The height is back** (17 to 21 m against round 4's 19 to 22 m), **but step 3d's domes are mostly lost.**
  - In leaf the crowns are taller, narrower ovoids on a central leader: seed 1 a column, seed 2 an upright ovoid.
  - Seeds 7 and 3 keep rounded tops but are narrower than at 3d.
  - Bare, a central stem runs up through the crown, where 3d's limbs divided low into several heavy arms.
  - **Height and dome trade against each other through one setting:** the leader's λ.
- **No limbs lie on the ground** at upkeep 0.35, before or after the persistence. Step 4's ground-lying limbs came at 0.45, and were alive but starved: they held leaves at their tips.
- **R2 (amended) is not met:**
  - Round 4 spreads 4.0× in leaves across these seeds (0.87M to 3.51M).
  - Step 4b spreads 5.8× (0.21M to 1.22M). Light widens it, most at seed 4.
- **Not closed:** the gate walk is red, and R2 is unmet. Step 5 was not started.

## For the host

1. **The limbs' λ walk fails at 52.6** inside the published range.
2. **The leader's λ sets height steeply (13 to 111 m over 0.45 to 0.6),** and the dome and the height pull against each other.
   - The cause is that sizes against the tree's mean have no ceiling for the few leader buds.
   - Whether a bound on the size, or a mean taken per order rather than over the whole tree, is the right design is the host's call.
3. **R2 (amended):** light widens the spread from 4.0× to 5.8×, driven by seed 4.
4. **The persistence (5 years) is estimated;** no source was found.
