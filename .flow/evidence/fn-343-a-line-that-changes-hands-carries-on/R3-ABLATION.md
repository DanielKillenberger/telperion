# fn-343: the ablation sheet, read by eye

Drafted by the worker on 10 Nov 2026 at `417c5e3dc`, for the host, as `R2-ANSWERS.md` orders. The tree is today's oak with one cause switched off at a time. These are not candidates for the product. The verdicts below are the worker's eye on the stills, not the owner's.

Marks: **eye** (read off a still), **measured** (printed by the run), **read** (from the code, not run).

## How it was made

- **The probe.** `probes/ablate.patch` touches the still example only (`crates/telperion-render/examples/space/`), not the generator.
  - **`ABLATE`.** This environment variable is read in `tuning.rs` and switches values in the preset's species before it grows:
    - `B`: `abortion` 0 on limb (PA 6) and bough (PA 7);
    - `C`: `form.wander` 0 on every PA;
    - `D`: `form.sag` 0 on every PA;
    - `E`: `insertion` halved on branch (PA 9, 1.0 to 0.5 rad) and twig (PA 13, 0.8 to 0.4 rad);
    - `F`: the branch's `lifespan` from 12 to 1000 years.
  - **Trunk measure.** `still.rs` also prints the trunk's diameter at 1.3 m, along the thickest wood.
  - **Reverted.** The patch is reverted; `git status --short crates` is empty.
- **The runs.**
  - `probes/stills.sh` ran inside one `telperion-heavy 12 --gpu 2 --threads 4` job, started at 17:15:21 with the card free. Each run logged the values it set (`ABLATE ...`); the logs are in `raw/stills/oak-<age>-<variant>.log`.
  - **A matches fn-342.** Its node, leaf and size figures equal fn-342's stills logs (272,095 / 372,691 / 1,876,936 nodes).
- **The sheets.** `probes/sheets.sh` lays out two sheets per variant, B to G, beside A.
  - Each sheet has three rows of trees: 40/1001, 40/1007 and 120/1001.
  - Each row has four tiles: A whole, the variant whole, A mid, the variant mid.
  - `sheet-<V>-bare.jpg` is leafless (`bare` and `limb` shots), and `sheet-<V>-whole.jpg` is leafed (`whole` and `spray` shots).
  - Full-size four-ups used for the read are `raw/pair-40-1001-bare.jpg` (A, E, C, B) and `raw/four-120-1001-bare.jpg` (A, E, D, C).
- **D did not stop the landing.** It is sag 0 only; nothing else was switched to stop the low limbs reaching the ground.
  - At 40 they still lie along the ground, with the same structure as A (identical node counts).
  - So at 40 the landing comes from where the limbs grow, not from sag.

## The measures beside A's (measured)

Height, then spread (two axes), trunk at 1.3 m, then current-year leaves:

| variant | 40 / 1001 | 40 / 1007 | 120 / 1001 |
|---|---|---|---|
| A today | 10.6 m, 7.6 × 8.0 m, 55.7 cm, 47,279 | 10.4 m, 7.8 × 7.7 m, 74.9 cm, 76,368 | 25.1 m, 26.6 × 25.5 m, 142.6 cm, 304,559 |
| B no limb/bough abortion | 10.3, 7.1 × 7.9, 55.3, 47,068 | 10.3, 8.0 × 7.6, 75.4, 74,917 | 24.6, 31.3 × 24.5, 141.1, 293,963 |
| C no wander | 10.6, 7.6 × 8.1, 55.4, 46,374 | 11.1, 7.9 × 7.9, 72.8, 78,043 | 27.7, 24.3 × 18.6, 139.9, 337,322 |
| D no sag | 10.6, 7.5 × 8.0, 55.7, 47,279 | 10.4, 7.8 × 7.6, 74.9, 76,368 | 26.4, 24.6 × 22.8, 142.6, 304,559 |
| E branch and twig insertion halved | 10.6, 8.1 × 9.2, 57.3, 47,349 | 10.4, 8.3 × 8.7, 74.6, 77,964 | 25.0, 31.3 × 27.3, 149.3, 311,050 |
| F branch never moves on | 10.6, 8.6 × 9.2, 65.6, 82,918 | 10.3, 9.8 × 10.4, 87.3, 113,464 | 28.7, **45.6 × 48.2**, **413.5**, **1,089,991** |
| G = B + E + F | 10.3, 9.3 × 9.5, 65.6, 84,246 | 10.3, 10.2 × 10.9, 87.5, 110,639 | **not drawn** |

- **F is a different tree at both ages.** At 120 it is a dome nearly twice A's spread, with a 4.1 m trunk, 62 km of wood against 17 and 3.6 times the leaves.
- **G at 120 could not be drawn.** It grew 185.9 million wood vertices. The renderer refused it by name: "the wood points buffer needs 295399072 bytes, above the device's limit of 268435456". That is the machine's limit, which is not overridden. The sheets show grey tiles saying so.
- **B and C change the tree at 120.** B is 31.3 × 24.5 m against 26.6 × 25.5 m and loses the two-limb V. C is 24.3 × 18.6 m, taller and one-sided.
- **D and E keep the tree.** D at 120 is 24.6 × 22.8 m, narrower as the limbs stand more erect. E is wider: 8.1 × 9.2 m at 40/1001, 31.3 × 27.3 m at 120.

## The read (eye)

**B, no limb and bough abortion.**
- **At 40, about the same as A.** The upper crown is a little rounder and its stem a little straighter where limb relays were. The low limbs zigzag along the ground as in A.
- **At 120, a different tree.** A central stem stands where A's two-limb V is, with a bushier, more cluttered crown. The mid shot frames empty sky because the camera fits the bounds, so it shows no wood to compare.
- **Lost:** the open V of the oak's main limbs at 120.
- **Verdict: not less jagged at 40; not comparable at 120.**

**C, no wander.**
- **At 40, the same as A on both seeds.** Every elbow A shows is still there, including the zigzag low limb on 1001 and the elbowed bough in the 1007 close-up.
- **At 120, a different tree.** It is taller and one-sided, its stem leaning. Its limbs run a little straighter between joints, but the joints in the branch wood are as angular.
- **Lost:** the V and the crown's breadth at 120.
- **Verdict: not less jagged.**

**D, no sag.**
- **At 40, the same as A.** The structure is identical, and so are the low limbs on the ground.
- **At 120, the same tree with its limbs more erect.** The Z-shaped branch in the 120 mid shot is still there, with the same elbows.
- **Lost:** the outward settle of the limbs, the crown 2 m narrower at 120.
- **Verdict: the same jaggedness.**

**E, branch and twig insertion halved.**
- **At 40, clearly less jagged.** Laterals leave in long, swept, straight lines instead of short spikes. The low limbs of 1001 stretch out smoothly where A's zigzag. The stem's side wood no longer reads as a bristle.
- **At 120, also smoother.** The branch wood runs in long sweeps up to the crown edge, and the two-limb V is kept.
- **Lost: the oak's character in the fine and middle wood.**
  - The crown reads upswept and feathery, like a vase-shaped elm or a broom, not the oak's open, wide-angled, twiggy texture.
  - In leaf, the crown at 40 has a thinner, more see-through look on 1001.
  - It is wider: about 1 m at 40, about 5 m at 120.
- **Verdict: less jagged, at the cost of looking less like an oak.**

**F, branch never moves on.**
- **At 40, the look changes.** It reads as a broad, bushy cone with long branches down to the ground. Those branches run fairly straight, so the elbow count looks lower, but the tree no longer looks like A.
- **At 120, a flat dome about 46 m across.** It has a 4 m trunk and long horizontal branches, and its mid shot shows heavy straight branches.
- **Lost:** the oak's form entirely.
- **Verdict: smoother in places but a different tree, so it does not answer the question.**

**G, B + E + F.**
- **At 40, the smoothest lines on the sheet.** The tree is a dense, spruce-like cone with long straight branches to the ground. It carries F's loss and E's.
- **At 120, not drawn,** for the reason given above.
- **Verdict: less jagged, and not an oak.**

**Summary.**
- **Only E is less jagged while keeping the same tree.** Its height, trunk and leaf are within about 5% of A, and its spread is somewhat wider. It is less jagged at both ages on both seeds.
- **B, C and D do not move what reads as jagged at 40.**
- **F and G smooth by growing a different tree.**
- **On this sheet, the jaggedness rides on the angle at which branch and twig wood leaves its bearer.** It does not ride on relays, wander or sag.
- **How the drawn wood treats a lateral's joint (next section) is part of why that angle shows as a hard corner.**

## How the drawn limb is made from the axis's points (read, no run)

1. **Points.** `crates/telperion-core/src/pipeline/space.rs` `convert` makes the tree's points.
   - **Spacing.** It makes one node per phytomer at its tip, merged into the last node only while the wood grown since is under `SHORTEST`, 1,000 µm. So nearly every phytomer is a point: an oak branch internode is about 1.7 cm.
   - **Thin wood.** Wood under `FINEST` (0.2 mm) is not drawn.
2. **Runs.** `pipeline/surface/paths.rs` `paths` chains the nodes into runs. Each run carries on into its widest child by `start_radius` that is not a codominant sibling.
   - **The bearer's own wood always wins.** Its next node takes `start_radius = inherited.max(own)`, the girth below the joint. A lateral's first node takes its own radius (`space.rs` near line 487).
   - **So a lateral always starts a run of its own,** even when it is thicker than the bearer's wood past the joint, as at every take-over joint R1 counted.
   - **Relays and continuations stay in their parent's run** (`space.rs` module doc: "A lateral starts a branch run; a continuation or relay carries its parent's on").
3. **Rings.** `pipeline/surface/samples.rs` puts the run's ring centres exactly at its node positions.
   - **A branch run's first ring** is its attach node pulled back into the bearer by `sink` (`fork_socket` 0.5 of the bearer's radius, held inside its inscribed circle), along the run's own first direction (`away`). Its radius swells near the base by `fork_swell`.
   - **A lateral of 0.4 to 0.7 of its parent's radius is codominant** (`FORK_FROM`, `FORK_AT`). It parts without a socket flange.
4. **Frames and tangents.** `pipeline/surface/frames.rs` gives each ring a tangent: the normalised mean of the segments before and after it (one segment at a run's ends).
5. **Between rings.** The wood is not straight tubes between points.
   - **The curve.** `pipeline/surface/curve.rs` keeps the points in clusters of up to 32 (`CLUSTER`) with 6 nested levels (`LEVELS`).
   - **The draw.** The render crate draws it on the GPU: `telperion-render/src/curve.rs`, `shaders/curve_walk.wgsl`, the same function as core's `Curve::tessellate`.
   - **Which rings are kept.** Each view keeps the coarsest level whose error stays within half of `ERROR`, 0.5 px (`curve.rs`).
   - **How a stretch is cut.** Between kept points a stretch is cut along its cubic Hermite curve, from the end tangents above, until the chords sag within that half (`curve/tessellate/plan.rs` `hermite`; `curve_walk.wgsl` `hermite`). The centreline inside a run is therefore interpolated and smooth.
6. **A joint between two axes is drawn differently from one inside an axis.**
   - **Inside a run** (within an axis, or through a relay or continuation), the corner is one point whose tangent bisects the two segments, so a Hermite curve rounds it over the neighbouring segments.
   - **At a lateral's joint,** the lateral is a separate run. Its tube starts inside the bearer and leaves straight along its first segment, with nothing carrying the bearer's direction into it.
   - **What the eye sees.** The 34° to 57° insertion angle R1 measured is drawn as a hard corner between two tubes. At a take-over this happens on the thicker of the two woods. The bearer's thinner own wood runs on smoothly through the joint, as the run's line.

## Pending, by the rules

Nothing is pending. G at 120 is refused by the machine's buffer limit, which is not overridden here.
