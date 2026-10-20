# A tree rests on its ground

## Goal & Context

The owner judged fn-283's stills (2026-10-20): "at 12 doesn't look right it's clipping leaves into the floor". Between the two fixes offered, the owner chose ground contact (option A) over tuning the palm's rows.

fn-283 measured the clipping and showed that its own changes did not cause it. The base, 147dae260, clips the same way to within 1 mm. The figures, from `.flow/evidence/fn-283-a-frond-draws-its-rachis/raw/clip-*.tsv`:

| Palm age | Leaflets with a vertex below y = 0 | Of them, on the dead skirt | Lowest leaflet vertex |
|---|---|---|---|
| 8 | 138–145 | | |
| 12 | 1,547–1,560 | 939–948 | −5.4 m |
| 20 | 302–316 | | |
| 40 | 0 | | |

The cause is the young palm's shape. It wears fronds close to full length (7 m, `frondEstablishment` 6.75 years) on a stem still only 3.5 m tall. Those fronds leave the stem 27.5° below horizontal and arch further down (`rachisArch` −0.6), so a frond falls about 0.99 m for every metre of its length. The dead skirt hangs at 165° and falls about 6.6 m.

Real young palms rest their lower fronds and their dead fronds on the ground. So does any tree whose limbs reach the ground. The fix is a property of the pipeline, not of the palm: nothing a tree draws passes below its ground. Contact is dormant wherever nothing reaches the ground. It is never a species branch, and presets stay value tables.

The film grows the palm through ages 8 to 20, so this spec joins the release (fn-237). It also blocks fn-283's R5 verdict on the young palm.

## Checked before the plan (fn-301 conductor, 2026-10-21)

- **The ground** is the plane through the root, y = 0. Nothing moves it. The CPU and the device each hard-code 0 (`tree.rs:63-77`, `scene/room.rs:23-26`).
- **Wood already lands.** The lay walk eases a descending limb toward the ground and clamps any step below it (CPU `telperion-space/src/geometry/lay.rs:357-379`, device `cycle/lay_walk.wgsl:206-223`). The clamp breaks this spec twice:
  - It shortens the step.
  - It sets the centreline, not the bark, at 0.
  - Trunks are exempt.
- **No single stage positions everything a tree draws.** Three things make positions:
  - Engine axes come from the lay walk.
  - Frond axes come from one parabola, `point + heading·L·t + lift·arch·L·t²`. On the CPU that is `foliage/leaflet.rs:26` `rachis`, which both `rachis.rs` and the leaflets call. The device has three copies: `generation/pack.wgsl` `on_rachis`, `resident/fronds.wgsl` `rachises` and `resident/field.wgsl`. The dead skirt is the rosette's dead fronds (`skirtFronds`, `skirtPitch` 165), drawn through the same curve.
  - Leaf and flower instances get their matrices in `station::matrix`, `pack.wgsl` and the bloom matrix.
- **R1 on the base** (`tests/ground_reach.rs`, 24704052c; `r1-base.tsv`): the palm is not the only preset that reaches the ground.

  | Preset | Reaches the ground | Lowest leaf |
  |---|---|---|
  | date palm | ages 8–20, deeply | −5.39 m at 12 |
  | silver birch | most rows | −0.09 m |
  | Norway spruce | age 40 | −0.025 m |
  | ordinary | age 40, seed 42 | −0.124 m |
  | Oregon white oak | age 40, seed 42 | −0.062 m |
  | beech, Telperion, Laurelin | clear on every row | – |

## Design (host, 2026-10-21)

Contact is a rule of the curve each axis is drawn on, plus one pose rule for instances. On each executor it runs where those positions are already made, never as a second pass over the finished tree.

- **Candidates weighed:**
  - **(b)** One contact pass over the drawn records, with the leaflets made to read the hung rachis nodes. This is the cleanest end state, because it deletes the three WGSL copies of the frond curve. It also restructures how leaflets are placed, which is too large for the release. It is a post-release Open item.
  - **(c)** Engine lay contact alone. This leaves the fronds, which are the defect.
  - **Chosen:** (a) together with (c).
- **The frond curve is one function per executor.** The CPU already has one. On the device, the three copies fold into one WGSL function that all three call. If the modules cannot share it, the plan says why and names the copies a test holds equal. Contact lives inside that function, so the rachis, its leaflets' stations and run, and the skirt all agree.
- **Resting keeps length and rests the bark.** On both curves, a resting axis keeps its arc length, and its centreline lies one radius above the ground.

## Requirements

- **R1 (repro):** a test reads, for every shipped preset at ages 6, 8, 10, 12, 15, 20 and its default age, at seeds 1, 7 and 42:
  - the lowest drawn vertex of wood, of leaves and of flowers;
  - the count of instances with any vertex below the ground.

  The test is red on the base for the palm. Record which other presets reach the ground, if any.
- **R2 (contact):**
  - **An axis.** An axis whose bark would pass below the ground meets it and lies along it from that point on, with its centreline one radius above the ground. It keeps its length: on the frond's parabola this is the closed-form arc length, and in the lay walk the step keeps its length. It carries on in the horizontal direction it was travelling; if that direction is vertical, it takes its heading's horizontal direction. Its children are attached to the axis as it now lies. Trunks stay exempt, as they are today.
  - **Engine wood.** The lay walk's clamp becomes this rule. The landing ease is unchanged.
  - **Fronds.** The frond curve function, one per executor, applies the rule.
  - **Leaves and flowers.** Leaves and flowers borne on a resting axis rest with it. The instance pose is one function per executor, applied where each leaf and flower matrix is finished. An instance whose element bounds would cross the ground under its matrix is turned about its attachment point by the least rotation that brings those bounds onto the ground. Lying flat always qualifies, since the attachment sits on bark above the ground. It is never culled, never scaled and never translated.
  - **The dead skirt.** The skirt is the rosette's dead fronds, so it rests through the frond curve.
  - **Both executors.** The CPU and the device apply contact in the same stage, through the one pipeline.
- **R3 (dormant, and what changes):**
  - Trees that R1 finds clear build byte-identical meshes to the base. That covers the beech, Telperion and Laurelin on every row, and the palm at 40.
  - The rows R1 finds touching change, under AGENTS.md "Generator evolution" (host, 2026-10-21): the palm at 6–20, the birch, the spruce at 40, the ordinary tree and the oak at 40, seed 42. Real lower limbs and skirts rest on the ground.
  - The cost is measured with a paired `build_time` run. It stays within noise on trees that never touch the ground. On the palm at 12 and the spruce at 40 it is recorded.
- **R4 (agreement):**
  - The CPU and the device agree on the palm at 12, to `agree_tests`' tolerances.
  - On every R1 row, no vertex lies below the ground.
- **R5 (look):**
  - Stills of the palm at 8, 12, 15 and 20 at seed 1, base against change, viewed by the agent and then judged by the owner.
  - The spruce at 40 and the birch at 40, seed 1, base against change, at the default view.
  - One still from the walk that passes through a young palm, if the film has one.
- **R6:** the gate and `npm test` are green, with a codex review on each task. `tests/ground_reach.rs` is un-ignored and green. `docs/pipeline.md` line 25, which still says four families have no engine species, is corrected.

## Boundaries

- Not terrain, slopes or the ground's material.
- Not collision between branches.
- Not tuning the palm's frond pose. The owner chose contact over that.
- Not (b), the leaflets reading the hung rachis. That is a post-release Open item, which would delete the frond curve's device copies.
