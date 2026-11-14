# fn-332 result: what bends the old oak's trunk

Protocol: `PROTOCOL.md`. Probe: `crates/telperion-core/examples/trunk.rs`. Raw reads: ignored `raw/`. The still at 200: `../fn-330-how-far-the-full-cycle-grows-an-oak/oak-age200-seed1.png`.

## R1: the trunk year by year

The one cycle on the CPU, the oak as shipped. "Off the base" is the horizontal distance from the trunk's foot. Remeasured 3 Nov 2026 after the codex review: the probe now leaves a parent at the phytomer a relay joins (it had followed the parent to its tip, then jumped), and prints the path's lean from vertical over each metre. Logs: ignored `raw/v2/`, run by `probes/rerun.py`. The re-review then had the probe leave a parent where the relay's girth joins it (`Axis::joined`, a share of a span) rather than at its stand's node; every row was grown again on the MacBook (`raw/v3/`, `RERUN_CLAIM=0`), and every path, line by line, is the one in `raw/v2/`: on these oaks each relay on the path joins at the end of the node it stands on.

| Seed | Age | Highest tip | Crown's mean off the base | Trunk path ends: high, off the base | Relays on the path | Steepest lean over a metre |
|---|---|---|---|---|---|---|
| 1 | 40 | 10.6 m | 0.6 m | 10.6, 0.7 m | 2 | 20 degrees |
| 1 | 60 | 13.3 | 1.0 | 13.0, 3.4 | 4 | 56 |
| 1 | 80 | 16.5 | 1.6 | 14.6, 6.5 | 7 | 60 |
| 1 | 100 | 19.7 | 3.6 | 15.3, 9.2 | 9 | 67 |
| 1 | 120 | 23.2 | 9.0 | 15.1, 10.3 | 16 | 96 |
| 1 | 160 | 33.0 | 9.6 | 11.5, 13.4 | 24 | 134 |
| 1 | 200 | 24.9 | 15.7 | 22.9, 15.7 | 32 | 80 |
| 7 | 40 | 11.1 | 0.4 | 9.9, 1.7 | 4 | 31 |
| 7 | 60 | 15.1 | 1.0 | 10.3, 2.8 | 9 | 57 |
| 7 | 80 | 20.6 | 1.8 | 10.2, 3.9 | 12 | 70 |
| 7 | 100 | 26.1 | 2.8 | 9.3, 5.8 | 17 | 89 |
| 7 | 120 | 29.1 | 3.9 | 7.0, 7.8 | 21 | 121 |
| 7 | 160 | 23.9 | 6.2 | 12.7, 9.1 | 25 | 95 |
| 7 | 200 | 33.0 | 8.5 | 9.1, 12.4 | 31 | 119 |
| 42 | 40 | 10.0 | 0.4 | 9.9, 2.4 | 3 | 71 |
| 42 | 60 | 13.2 | 1.3 | 11.7, 5.8 | 5 | 73 |
| 42 | 80 | 17.0 | 2.3 | 13.7, 8.6 | 8 | 87 |
| 42 | 100 | 22.6 | 3.4 | 14.9, 12.6 | 12 | 95 |
| 42 | 120 | 26.9 | 3.2 | 16.8, 14.6 | 13 | 91 |
| 42 | 160 | 27.3 | 16.9 | 16.3, 22.2 | 19 | 95 |
| 42 | 200 | 23.6 | 11.9 | 18.0, 5.7 | 25 | 116 |

A lean over 90 degrees is a metre that runs downhill. On seed 1 at 200 the path rises within 4 degrees of vertical to 6 m, leans 28 degrees by 9 m high, and over its next three metres runs nearly level (79 degrees), from 8.9 to 10.0 m high while it moves 2.6 m out; it then climbs at 50 to 60 degrees and straightens near the top. The stem is the seed axis, its PA 3 and PA 4 continuations and a first PA 6 continuation. Above it, the thickest line through the crown is a chain of PA 6 relays: PA 6's apex stops, a relay of the same PA takes over, and so on, 32 times by 200 on seed 1, many laid under 30 degrees. The bend begins between 40 and 60, when the first relays take over, and grows with age.

## R2: the causes, one at a time

Age 200, seed 1, one setting changed on every PA, all else as shipped:

| Run | Highest tip | Crown's mean off the base | Trunk path ends: high, off the base | The path's relays, rise when laid |
|---|---|---|---|---|
| As shipped | 24.9 m | 15.7 m | 22.9, 15.7 m | 6 to 84 degrees, many under 30 |
| No sag | 32.1 | 7.9 | 28.8, 7.7 | 22 to 84, most over 40 |
| No wander | 31.7 | 10.1 | 29.7, 15.3 | 15 to 84 |
| Tropism times 3 | 35.0 | 10.7 | 29.1, 11.6 | 24 to 80 |
| No shading (extinction 0) | 33.6 | 6.9 | 20.8, 17.3 | -18 to 71 |

Tropism here is `Form::tropism`, the turn toward a set elevation, not the light. Growth toward light is the shade field; `TRUNK_SHADE=0` switches its extinction off. On the desk it was killed at 3, 6 and 12 GB; on the MacBook it grew in 224 s at 17.2 GiB resident, where the shaded oak peaks at 2.2 GB. Shade is what sheds the inner wood, so with none the tree keeps far more of it, a different and larger tree. Its crown's mean sits nearer the base (6.9 m) only because the kept inner shoots pull the mean in; the trunk's path ends further out than as shipped (17.3 m against 15.7), its relays are laid as low (down to 18 degrees below level) and one metre runs downhill (lean 97). Switching the light off does not straighten the trunk: the bend is not the turn toward light. What the light does to the bend inside a tree that still sheds stays unmeasured, since the engine has no switch that removes only the turn toward light.

- **Sag is the largest single cause of those measured.** It halves the crown's lean (15.7 to 7.9 m) and raises the relays: a relay is laid at its angle on a parent that sag has already turned down, so each low relay grows from a drooping limb and droops in turn. The oak's PA 6 sag (0.0003) works as a beam with no reaction wood: it bends every year's added load into wood that never rights itself, so its effect grows with the limbs' length and age.
- **No single cause accounts for all of it.** With no sag the crown's mean still sits 7.9 m off the base, and with wander off or tropism tripled 10 m or more; with no shading the trunk's path ends 17.3 m out. The tree carries its crown on one dominant chain of relays rather than on several limbs around the stem, so the crown is one-sided whatever bends it. The engine does not let this child switch that off, so what makes one chain dominate is unknown here.
- **Height depends on the bend.** As shipped the oak is 24.9 m at 200, inside Silvics' mature 15 to 27 m only because the crown hangs; with no sag it is 32 m, over the range, and seed 1 reaches 33 m at 160.

## R3: against a real oak

The goal's sources (Silvics, Clackamas County): a mature Oregon white oak is 15 to 27 m tall, with a short trunk dividing into several large spreading limbs and a broad, rounded crown; a heritage tree of 200 to 300 years is about 21 m tall with a 27 m canopy. The decurrent form, a stem ending near 10 m in scaffold limbs, matches. What does not: one limb carrying the crown, the crown hung 10 to 17 m to one side, and limbs drooping near horizontal from their base.

## R4: the next step

Two changes, each a rule in the one cycle and not a per-species setting, measured on the oak first and then every preset:

1. **Wood rights itself.** Sag bends new load into wood, and wood older than its first years keeps the angle it had, as reaction wood holds a real limb. That keeps a limb's droop from compounding with age. It changes the oak at its preset age of 40 only a little, since the bend there is still small (0.6 m), and that will be read.
2. **Several scaffold limbs share the crown.** Why one PA 6 relay chain takes the crown needs reading first: apical control, dominance or the relays' light. The next child measures the scaffold limbs' shares at 80 and 200 before choosing.

Both are judged by the owner's eye on stills at 120, 200 and 300 beside the measures here, and by R4's statistics at the preset age.

## R5

The probe adds one example file under `crates/telperion-core/examples`. Codex's re-review of the fix (1ae030d69) was SHIP; the gate is recorded in the spec's close.
