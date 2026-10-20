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

## Unknown, check before the plan

- Where the ground is today. Is it the plane through the stem base (y = 0)? Do any parameter, scene or terrain input move it? Do the CPU executor and the device executor share that definition?
- Which stage should own contact. Candidates are the settle or pose stage, which already bends axes (sag and arch), or the drawn records. The plan must check this against `docs/pipeline.md` and name its choice. It must not add a second chain.

## Requirements

- **R1 (repro):** a test reads, for every shipped preset at ages 6, 8, 10, 12, 15, 20 and its default age, at seeds 1, 7 and 42:
  - the lowest drawn vertex of wood, of leaves and of flowers;
  - the count of instances with any vertex below the ground.

  The test is red on the base for the palm. Record which other presets reach the ground, if any.
- **R2 (contact):**
  - **An axis.** An axis whose path would pass below the ground meets it and lies along it from that point on. It keeps its length, and it carries on in the horizontal direction it was travelling. Its children are attached to the axis as it now lies.
  - **Leaves and flowers.** Leaves and flowers borne on a resting axis rest with it. A leaf instance whose own extent still crosses the ground is posed to lie on the ground; it is never culled and never moved upward through the wood.
  - **The dead skirt.** The skirt rests the same way.
  - **Both executors.** The CPU and the device apply contact in the same stage, through the one pipeline.
- **R3 (dormant):**
  - Trees that never reach the ground build byte-identical meshes to the base. This covers every shipped preset at its default age that R1 finds clear, and the palm at 40.
  - The cost is measured with a paired `build_time` run, and it stays within noise on trees that never touch the ground.
- **R4 (agreement):**
  - The CPU and the device agree on the palm at 12, to `agree_tests`' tolerances.
  - On every R1 row, no vertex lies below the ground.
- **R5 (look):**
  - Stills of the palm at 8, 12, 15 and 20 at seed 1, base against change, viewed by the agent and then judged by the owner.
  - One still from the walk that passes through a young palm, if the film has one.
- **R6:** the gate and `npm test` are green, with a codex review on each task.

## Boundaries

- Not terrain, slopes or the ground's material.
- Not collision between branches.
- Not tuning the palm's frond pose. The owner chose contact over that.
