# fn-58 tuning-loop probe (2026-09-18)

Question: can a loop of code and Jev move a preset toward its reference
photographs faster than the hand rounds fn-34 and fn-62 ran, and what is
Jev's part in it?

Subject: the European beech as shipped at fn-62's round 22, seed 1, judged
on its two matched stills, B-WHOLE (leaf-on) and B-BARE. The photograph's
numbers are the ones `rounds.tsv` records for those references. A candidate
is a partial wire object laid over the preset (`--family` on the species
example and the headless renderer, new today). Code measures it, renders
still and twin per reference, reads the compare script's five numbers off
them (width over height, crown base, occupied, outline deviation, centre
brightness) and scores the mean relative distance from the photograph. The
gates and the node cap make a candidate infeasible. No still was looked at.

Twelve dials, one authored step each: `loop.py` holds the table. Three
framings, all from the shipped rows:

| Arm | What decides | Result |
|---|---|---|
| sweep | Code evaluates every single-step move on every dial, keeps the best, tries the compound of improving moves | 0.2453 → 0.1870 → 0.1527 → 0.1451 in three rounds, 74 evaluations, 18 minutes |
| jev | One Noul per move over the owner's notes and both numbers tables; code evaluates Jev's top four | 0.2453 → stop. Jev put both directions of `twig_tip_taper` above 0.5 and the move that worked at 0.45 |
| direction | One Choice per dial over up, down, hold (the owner's framing); code evaluates the four dials Jev moves most confidently | 0.2453 → 0.1870 → 0.1724 → 0.1542 in three rounds, 13 evaluations, 3 minutes |
| choice | After the sweep: one Choice over all measured candidates, numbers only, no score | flat, 0.15 to 0.28 over 22 options; `none` in rounds 1 and 3, `spread_down` in round 2 where code's best was `crown_base:up` |

The sweep's three accepted moves: one limb per station instead of two,
crown base 0.06 → 0.10, spread 0.36 → 0.32. The leaf-on still went from
0.532 occupied, 0.170 outline and 56 centre brightness to 0.449, 0.238 and
81 against the photograph's 0.457, 0.227 and 80. The bare still lost density
(0.41 → 0.31 against 0.28) but stayed smooth (0.15 against 0.29) and grew
brighter (116 → 136 against 100), which the mean tolerated. The compound of
all improving moves scored worse than the best single move in rounds 1 and 2:
the dials interact, so the loop steps one at a time.

The direction arm's accepted moves: one limb per station (the same first
move the sweep found), then twigs per station 4 → 3 → 2. It never proposed
the crown base or the spread, which the notes do not mention and which gave
the sweep its rounds 2 and 3; its round-2 pick was the sweep's fourth best.
Per round, Jev's four moves cost 40 to 60 seconds of evaluation against the
sweep's five to six minutes.

What this says:

- The loop is code: measure, render, read the numbers, step, and the number
  decides. One sweep round found the move the birch needed twenty-five hand
  rounds to reach.
- The framing decides whether Jev helps. A Noul per move said yes to both
  directions of one dial and stalled. A Choice over measured candidates was
  flat: comparing twenty tables of numbers is arithmetic. A Choice per dial
  over up, down and hold, read against the owner's notes, proposed the right
  first move and reached 85 percent of the sweep's gain at a sixth of its
  evaluations. Jev routes a note to a direction; it does not optimise and it
  does not compare numbers.
- Jev proposes only what the notes name. The sweep's best rounds moved dials
  the notes never mention. The loop that uses both is: Jev's four directions
  first, the sweep when Jev's proposals stall, the number always deciding.
- The answers are not yet calibrated. Every direction question ran without a
  labelled set; fn-34's rounds (which dial moved for which note, and whether
  the owner accepted) are the labels the project rule requires before a
  selection is trusted.
- The bigger lever is measurement. Five numbers per still miss what the
  owner's notes name: fine twigs at the edge, density by height. Each added
  measurement improves the loop more than any judge. The eye still gates: a
  single limb per station scores well and may read wrong.

Files: `loop.py` (the probe), `jev-choice.py` (the choice framing),
`trials.tsv` (every evaluation), `arms.json` and `arms-direction.json`
(trajectories with ledger references), `jev-choice-round*.json`. Ledger
entries under `.flow/ledger/jev-tune-probe/`. Stills stayed in a scratch
directory.

Run (GPU, key in the interactive shell):

```sh
cargo build --release -p telperion-core --example species_measure
cargo build --release -p telperion-render --example headless
uv run experiments/fn58-tuning-loop/loop.py --stills /tmp/stills --arms sweep --rounds 3
```
