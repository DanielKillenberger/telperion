# fn-364: why the bough chain relays every year, what turns a relay, and what makes the balance inert (builder, Opus, 14 Nov 2026; no rule changed)

**Tooling.** `curvature/relays.py` instruments a checkout and is never committed:
- `STOPLOG=1` prints every apex stop: lineage, cycle, PA and why. This is `grow/stop.rs`'s `stop`, plus the balance's shed in `grow/cycle/passes.rs`.
- `LAYWATCH=<lineage>` prints every settled lay step of that lineage with its turn split by term (`geometry/lay.rs`, `Layer::step`).

It ran on two checkouts:
- this branch: `.worktrees/fn-364-after`, `8e22c8a17`;
- fn-363's branch: a fresh `.worktrees/fn-364-base363` at `c2d897a37`, its own target. fn-363's builder's checkouts were not touched.

Both grow the preset oak at 300 on seed 1007.

**The chain read is lineage `12456632203837092862`.** A relay carries its axis's lineage, so one lineage is the whole chain together with the relays' own relays. It starts as a PA 7 lateral born at 20 on axis 7's node 32. It holds the relays of births 211 to 268 on CURVATURE.md's first tip. Its stops are `curvature/chain-<branch>.stops.txt`, and its lay steps on this branch `curvature/chain-this.lay.txt`.

## 1. Why the chain relays every year

**The two branches' chain is the same, year for year, until 127** (measured):

| cycles | this branch | fn-363's branch |
|---|---|---|
| 24, 25, 56, 78, 79 | Aborted | Aborted |
| 80 to 126 | Ended, one a year | Ended, one a year |
| 127 | Ended | Ended and **Shed by the balance** |
| 128 to 300 | Ended, one a year (221 Ended in all) | none: the chain is gone, and no axis of the lineage is laid at 300 |

**The yearly rate is the same on both branches: one relay a year from 80.** What differs is that fn-363's balance sheds the chain at 127, and this branch's does not, so it relays on for 173 more years.

**What ends each apex is its PA's lifespan, not a draw.** The bough (PA 7) lives 60 cycles (`oregon-white-oak.values`, the bough row's lifespan 60). The lateral was born at 20, so its age ends at 80.
- `grow/stop.rs`, `move_on`, stops the apex with `Stop::Ended`.
- `relay_ended` is 1 on the bough, so `make_relay` always makes the relay.
- `make_relay` hands the relay the axis's time: `draws.aged = grown`, "it carries ... the time the axis has grown, which its abortion hazard counts". So the relay is born with its PA's lifespan already spent. Its age ends after its first unit, `Stop::Ended` again, and so on every cycle.

That is code read. The table above shows it: every stop from 80 on is `Ended`.

**Where the stretch enters:**
- **Not in the stop.**
  - `Ended` is decided by time spent against the lifespan.
  - `Aborted` is the abortion draw against the time grown.
  - `Failed` is a unit's survival, which reads the light at the tip, the ground and the kept share. No `Failed` stop occurs on the chain on either branch.
  - No unit laid short counts as failed. Node counts do not read the stretch: it never enters `draws.nodes` or `Unit`.
- **Only through the balance's shed** (`passes.rs`, `balance_one`):
  - Charged by its height against its light, the chain's tip on fn-363's branch was shed at 127.
  - On this branch its units are laid at stretch 0.04 to 0.6, so its tip rises little and its charge stays under its light. It is not shed.
  - The stretch enters through the tip's height and light, which come from where it was laid. That is inferred from the code and the two branches' tables, not measured per tip.

**Over the whole tree** (counted from the stop logs, PA 6 limbs, PA 7 boughs and PA 9 branches):

| stops | this branch | fn-363 |
|---|---|---|
| PA 7 Ended | 21,954 | 33,973 |
| PA 7 Aborted | 2,571 | 6,008 |
| PA 7 Shed | 692 | 1,701 |
| PA 6 Aborted / Shed | 473 / 76 | 1,188 / 155 |
| PA 9 Shed | 46,972 | 157,632 |

This branch makes fewer bough relays in all than fn-363's. Its chains live on, because its balance sheds fewer of them: about a third of fn-363's sheds in PA 9 and two fifths in PA 7.

## 2. What turns a relay

Five consecutive relays of the chain, born 230 to 234 (settled lay, this branch). "Pull" is the angle between the running direction and the phytomer's laid direction: the pull towards the vertical and the take-over's run back onto its line together.

| birth | phytomers | laid m | reach (`total`, scaled) | own bend | tropism deg | wander / bend memory deg | sag deg | pull at first / last phytomer, deg | take-over at first | landing deg | rest deg |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 230 | 5 | 0.020 | 0.663 | 0 | 0.1 | 0.0 | 0.0 | 24.5 / 2.6 | 0.675 | 0.0 | 0.0 |
| 231 | 6 | 0.011 | 0.367 | 0 | 0.1 | 0.0 | 0.1 | 22.0 / 1.9 | 0.688 | 0.0 | 0.0 |
| 232 | 6 | 0.020 | 0.665 | 0 | 0.0 | 0.0 | 0.0 | 25.0 / 2.1 | 0.688 | 0.1 | 0.1 |
| 233 | 5 | 0.012 | 0.386 | 0 | 0.0 | 0.0 | 0.0 | 24.4 / 2.5 | 0.675 | 0.0 | 0.0 |
| 234 | 6 | 0.020 | 0.678 | 0 | 0.2 | 0.0 | 0.1 | 22.0 / 1.9 | 0.688 | 0.1 | 0.1 |

Over all 454 relays of the lineage laid at 300, the median per relay is:
- tropism 0.12°, wander and bend memory 0.06°, sag 0.03°, landing 0.04°, rest 0.04°;
- pull 23.5° at the first phytomer and 2.0° at the last;
- 6 phytomers, 0.014 m laid.

**The turn inside a relay is the take-over** (measured):
- The terms that scale with the internode's length (tropism, wander and bend memory, sag) together come to well under a degree per relay.
- The bough's own bend is 0. The take-over starts at 0.68 of the way back onto the line at the relay's first phytomer.
- It fades to almost none at its last, because `over = take × fade` and `fade` runs from 1 to 0 over the relay's own reach (`(run + scale/2) / total`).
- The reach is the relay's own laid length in scaled units: it shrinks with the stretch, and the fraction run through it does not. So the whole take-over, about 22° from the first phytomer to the last, is spent inside each relay, however short.
- Its size follows neither the laid length nor the node count. It follows the take-over row (`take`) and the share of the reach run.

**The junction** (about 10° in CURVATURE.md's tables) is the relay's frame against its bearer at its node. It was not split by term: this instrumentation reads `Layer::step` only, not `geometry::frame`.

Its sense is not one hand. Over the 206 junctions on each of CURVATURE.md's most tortuous tips at 300, seed 1007, the mean resultant of the unit turn axes is 0.064 and 0.061, against 1 for one hand. On a least tortuous tip it is 0.081. The axes are near random or alternating, not of one hand.

**fn-363's branch shows no lay terms for this lineage**: the chain was shed at 127 and nothing of it is laid at 300. The same take-over, spent over relays of whole length (a unit of 0.07 to 0.12 m a year before the stretch), is not measured there.

## 3. `light_no_bud_reads_leaves_every_passed_species_to_the_bit`

**What it zeroes** (`telperion-space/tests/light.rs`), on every age of the beech, the spruce and the oak:
- `shade_hazard`, `shade_size` and `balance_hazard`;
- every `shade_short` entry.

It keeps `inverse_height`, and grows each tree at 30 on seeds 1 and 7 under neutral light, no sky and three shading lights. It holds each tree to the bit.

**What makes the balance inert on fn-363's branch is `balance_hazard`** (code read, `contract/shed.rs`, `balance`):
- The hazard is `b.hazard * (b.tolerance - memory).max(0.0)`, where `b.hazard` is the PA's `balance_hazard`.
- At 0 it returns `undecided`: no shed, `kept` the bearer's keep, untouched.
- The charge and the light still reach `balance_one` and its memory, but nothing they make there reaches the tree.

**The surplus share has that same quantity at hand**: `allot_one` reads the tip's PA state (`species.states[self.axes[i].pa]`) for `inverse_height`, and the same state holds `balance_hazard`. As built, the share reads the charge and the light whatever the hazard, so under this test it lays a tip in less light shorter, and the tree changes with light. That is measured: the test fails with "beech seed 1: Light { extinction: 0.5, sky: 0.0 } changed the tree".
