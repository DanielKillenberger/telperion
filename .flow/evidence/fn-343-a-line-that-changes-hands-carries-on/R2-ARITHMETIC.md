# fn-343: the sharing rule on the logged joints, before any code

Drafted by the worker on 10 Nov 2026 at `5fc8dd5ae`, for the host, as `R1-ANSWERS.md` orders. **The rule does not halve the sharp joints at either age. It multiplies them by 2.7 to 5.0.** The answers say to stop at this point and return, so no probe of the rule was built and no still was made.

Marks: **measured** (a probe run, log named), **inferred** (arithmetic on measured geometry, not run), **source** (read from the paper's text).

## 1. Murray 1927 against its text (source)

**Read in full.** Murray, C. D. (1927), J. Gen. Physiol. 10: 725–729, "A relationship between circumference and weight in trees and its bearing on branching angles". It was read from the PDF scan (pdodds.w3.uvm.edu mirror of jgp.rupress.org), all five pages.

**What it states:**
- **A power law of branching, measured.** Equation (2), from 116 measurements on nine kinds of tree, oak among them, is `c0^2.49 = c1^2.49 + c2^2.49`, where c is the circumference.
- **The angles, imported from Murray's 1926 paper on arteries.** Equation (3), deduced from the assumption that the fork connecting three points "shall, for given circumferences of the stem and branches, require the least volume of wood", is:

      cos x = (c0^4 + c1^4 − c2^4) / (2 c0^2 c1^2),   and the like for y and for x + y

  Here x and y are the angles each branch makes with the stem's line.
- **What the paper adds about the angles.** At equal branches, x + y is 59° under the 2.5 power law and 75° under a cube law. Its Fig. 2 gives 0° at exponent 2.

**Its form beside the host's.** With sections A ∝ c², equation (3) is the triangle of the three sections. It implies two things:
- **`A1 sin x = A2 sin y`.** This is the sine relation the host recalled, and it shares the angle between the branches by section.
- **`A1 cos x + A2 cos y = A0`.** This fixes the total angle x + y from the three sections and the branching exponent. It does not keep the insertion angle.

**What was worked.** Both the host's linear form and the sine relation with the total angle held at today's angle (the angle between the lateral and the line below), as the answers keep it.
- **Murray's full form was not worked.** It would replace every insertion angle. Under the pipe model's area-preserving exponent of 2 (the branch row's), it gives 0°.
- **Whether a full form belongs here is the host's question.**

## 2. The arithmetic (inferred, on a measured walk)

**The walk.**
- **Same trees as R1.** The tagged walk was rerun with each walked node's woods logged: its own next phytomer, every lateral and relay, with radius, first segment and heading (`probes/fork.patch`). The runs are `raw/t-40-r0.01-fork.log` and `raw/t-120-r0.01-fork.log`, and the tree counts match R1's.
- **Why a rerun.** R1's log held only the taken wood, so it could not show a bearer turned by a lateral it does not take.

**The computation** (`probes/share.py` gives `raw/share.md`; `probes/worse.py` gives `raw/worse.md`).
- **Sections.** A lateral of section `A_l = r²` stands on a bearer whose own wood past the node is `A_b = r²`. A relay is left as it is.
- **Sharing.** Each lateral keeps θ, its heading against the segment below, and shares it:
  - the lateral's subtree turns back toward the line by θ − x_l;
  - the bearer's own wood turns x_b away, in their plane;
  - several laterals at one node add as rotation vectors.
- **The new turn.** It is the angle between the segment below and the taken wood, turned.
- **What counts.** Joints of 10° or more on walked wood 1 cm or more, as in R1.

| | 40, linear | 40, sine | 120, linear | 120, sine |
|---|---|---|---|---|
| walked wood ≥ 1 cm | 1,050 m | | 6,889 m | |
| today, joints ≥ 10° | 465 (0.44/m) | 465 | 4,274 (0.62/m) | 4,274 |
| under the rule | **2,304 (2.20/m, ×4.95)** | **2,105 (2.00/m, ×4.53)** | **12,675 (1.84/m, ×2.97)** | **11,359 (1.65/m, ×2.66)** |
| take-overs ≥ 10°, today → rule | 363 → 255 | 363 → 248 | 3,030 → 1,526 | 3,030 → 1,459 |
| relays ≥ 10° (unchanged) | 102 | 102 | 1,244 → 1,243 | 1,244 → 1,243 |
| made worse: bearer ran on under 10°, now 10° or more | 1,947 | 1,755 | 9,911 | 8,662 |
| limbs and boughs: today → rule | 90 → 428 (×4.76) | 90 → 406 | 372 → 1,523 (×4.09) | 372 → 1,418 |
| limbs and boughs: made worse | 349 | 328 | 1,182 | 1,078 |

**Why it goes this way:**
- **Take-overs fall less than expected.** The median take-over's own wood past the joint is 0.52 to 0.70 of the lateral's radius (R1). At 57° the linear form then leaves the taken lateral at 12° to 19°, not the 1° to 12° the answers expected from the move-on and relay ratios alone.
  - **At 40,** 70% of the take-overs stay at 10° or more.
  - **At 120,** 50% do.
- **A bearer made to turn carries one comparable lateral.** In 95 to 96% of the 1,945 (40) and 9,887 (120) bearers the linear form turns to 10° or more, a single lateral stands at the node. Its radius is a median 0.61 to 0.63 of the bearer's own (quartiles 0.55 to 0.73), at a median θ of 46°. The bearer then turns 11° to 16°.
- **This was the risk the answers named:** a bearer made to zigzag by its laterals. The oak's pipe model gives it a lateral of that size at about 1.4 to 1.9 walked joints per metre.

**The ratios behind it** (linear form, θ = 57.3°): the taken lateral turns θ·ρ²/(1 + ρ²), where ρ is the bearer's own radius over the lateral's.

| ρ | 0.13 | 0.43 | 0.52 | 0.60 | 0.70 | 0.80 |
|---|---|---|---|---|---|---|
| turn | 1.0° | 8.9° | 12.2° | 15.2° | 18.8° | 22.4° |

## Not done, by the answers' stop

- No sharing probe in the crates, and no stills.
- `probes/fork.patch` is logging only: R1's `stops.patch` plus the per-node woods. It reapplies cleanly and is reverted. `git status --short crates` is empty.
