# A unit lays the nodes its vigour and presence give

Child of fn-351 (P1, P2, the speed aim). Host-written, 12 Nov 2026. Builds on the phase's top branch (`fn-355-a-year-is-a-state-the-grower-reads-no`) and lands with fn-354 and fn-355.

## Problem

Measured (`.flow/evidence/fn-356-a-unit-lays-the-nodes-its-vigour-and/SURVEY.md`): once the oak no longer collapses with age, its count of shoots has no ceiling. The oak at 200 on seed 1002 holds 29.4 million phytomers and peaks at 29.6 GiB (base 1.78 million, 5.9 GiB); the scoreboard's panel dies on it. On the oak at 120, 47% of held phytomers stand at a presence under 0.05 and carry 0.8% of the leaf; half the living tips are under 8% present. Light holds the crown's presence-weighted leaf at a ceiling (the summed presence of tips is flat from year 100), presence only ever multiplies downward along a lineage, every unit lays its full count of nodes whatever its size or presence (4.1 to 4.4 in every bin), and no rule ends a tip for being small or faint. So the same leaf is carried by ever more, ever fainter complete shoots.

## The design (host)

A growth unit lays a share of its nodes: `min(1, size x presence)` of them, counted from the unit's base, a whole number of nodes with the last one growing in by degree, as `lineage::laid` already lays a zone's quota.

- `size` is the unit's shade size (the vigour its light gives it against its kind's mean) and `presence` the presence its unit is grown at today. A unit at or above 1 lays every node, as now.
- Nothing else about a node changes: its scale, its leaf, its buds are as today. A unit that lays no node bears no bud; its apex lives on by the rules it lives by today.
- Source: Pałubicki et al. 2009, section 4.2: "The integer part of the amount v of the resource reaching a bud determines the number of metamers produced by this bud: n = ⌊v⌋", read in the paper's text. Departures, named: their v is an absolute amount and ours is relative to the kind's mean, capped at a whole unit; our last node grows in by degree where theirs is cut off.
- No new value.

**Expected on paper** (the survey's first-order sums): held phytomers on the oak at 120, seed 1002, from 9.7 million to about 2.7 million; on the oak at 200 from 29.4 million to 4 to 6 million; leaf area down by a few percent at most (units under size 0.5 carry 6% of it, units under presence 0.2 carry 2.4%); time and memory down with the phytomers. Species with no size response change only through presence. The faint lineages should die out, since a unit that lays no distal node bears no lateral.

**What would refute it:** a census at 120 that still shows the tip count rising against a flat summed presence (then a tip that lays nothing must also end, and the host designs that from Borchert and Honda's F_min rule); the oak's leaf area falling by more than a tenth; a crown hollowed or a tree visibly barer than before on the sheets; a walk failing its bound.

## Acceptance Criteria

- **R1.** The rule is in the grower for every species, with no branch by species; a unit at `size x presence >= 1` is byte-equal to today's (tested on a fixture).
- **R2.** A walk across a unit's quota moves by degree: a node enters at vanishing size (tested as the other node-count walks are).
- **R3.** Paused and one-call growth agree; fn-355's year tests (a tree at y is the tree asked for at y) pass unchanged.
- **R4.** Census on the oak at 80 and 120, seed 1002, before and after: living tips, summed presence, held phytomers, phytomers under 0.2 mm, leaf area, by decade for the tip count.
- **R5.** Measured on seeds 1001, 1007, 1012 beside the branch's head: the oak at 40 and 120, the ordinary tree, beech, birch, spruce at 40, Telperion at 82: height, spread, trunk, drawn leaf, new shoots, held phytomers, seconds and peak memory in one load window; sheets whole and bare. Then the oak at 200 on seeds 1001, 1002 and 7 in a 16 GB scope: drawn or not, peak memory, seconds, sheets.
- **R6.** Docs say what a unit lays; every repinned digest is listed before and after.

## Stops (return to the host with the numbers)

- Any of the refutations above.
- The quota needing a definition the design does not give (which presence, at which moment; a zone order that is not base to tip).
- Any change to what ends a tip.

## Boundaries

- No new value, no per-species value, no cap on any count.
- What ends a tip, what wakes a bud and the balance are untouched.

## Rigor

Product code, Opus to this design; narrow tests while building; the full gate and one codex review at the landing with fn-354 and fn-355.
