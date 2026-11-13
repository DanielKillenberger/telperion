# A tip is charged by its height

Child of fn-351 (P1's old trees; Phase 3). Host-written, 13 Nov 2026. Builds on `fn-351-phase-top` (the phase's top with fn-362 and fn-361 merged).

## Problem

The carbon balance (fn-349; `crates/telperion-space/src/grow/cycle/passes.rs`, `balance_one`, `charge`) charges a living tip its size times the square of its PATH from the root collar over the species' potential HEIGHT. fn-349 listed that reading as the host's estimate, with the exponent. Measured since:

- With it the Oregon white oak's limb tips run until their path passes 34.8 m times the square root of their light and then die unrelayed; settled leaf on seed 1001 is 739 m² at 120, 199 at 200, 0.9 at 300, a bare post (`PHASE3-SURVEY.md` 1; `SURPLUS-PROBE.md`, V0).
- fn-361's limb dominance row keeps the old oak only by drawing every subordinate limb, leaves included, at about half its bearer's size at each limb birth (`OLD-OAK-WOOD.md`, table J: 0.75 of the 1.04 fall in ln leaf scale), thins the young oak (leaf 119 against 187 m² at 40) and puts 7 of 12 seeds at 300 past 24 GB (`fn-361-…/PANEL.md`). Held by the host's verdict; it is not the answer at any dose.
- Probe variant V4 (`SURPLUS-PROBE.md`, rounds 2 and 3; the charge reads the tip's height above the collar, nothing else changed; no limb row), settled leaf in m² at 40, 80, 120, 200, 300: seed 1001 113.5, 514, 968, 1,041, 590; seed 1007 181, 1,063, 1,974, 2,198, 1,111. The tree at 40 equals today's to the digit; leaf per leafed phytomer holds at 15 to 12 cm² through 300; phytomers per m² of leaf stay within 1.06 (seed 1001) and 1.38 (seed 1007) of the young tree's.

## The design (host)

A potential height is a height. The charge reads it against the tip's height:

- `charge = size * (h * inverse_height)²`, h the height of the tip's last laid node above the root collar's base, never below 0 (the probe's V4: the z of the tip's last laid node above axis 0's base, already at hand in `balance_one`). The exponent 2 stays and stays listed as the host's estimate (McDowell et al. 2002, abstract only).
- Nothing else in the balance, the allotment or the settle changes. No new value, no new record.
- The Oregon white oak's limb dominance row (`/space/ages/6/form/dominance = 0.5`, fn-361) is removed with its mirror lines and comment, by reverting fn-361's product commit `8f079717c` in substance (the preset row, `oak.rs`, `presets.generated.ts` by its own generator); fn-361's evidence stays.

Known and accepted into this spec as open, not solved here: under V4 the tallest tip passes the potential height (36.1 to 38.8 m at 200 and 300 on two seeds against 34.8 m), because a tip in full light past the limit extends at full rate until the hazard takes it. Slowing extension by the balance was probed three ways (V1, V3, V7, V8) and each has a cost recorded in `SURPLUS-PROBE.md`; it is a later decision of the host's, not this spec's.

## Acceptance Criteria

- **R1.** The reading first, in the evidence, before the product change: every reader of a tip's path in the balance and its tests (`balance_tests.rs`, `sketch.rs`'s whole-tree engine and anything that holds the two engines to each other, the closed form, docs that state the path form: `docs/pipeline.md`, the species comments on `inverse_height`, fn-349's text in any doc that ships), and whether `path_end` is still read by anything once the charge leaves it (if nothing reads it, say what carrying it costs and leave it; deleting it is not this spec).
- **R2.** Tests, red before and green after: the charge of a hand-built tip is its size times the square of its height over the potential height and does not change when the same tip is reached by a longer path at the same height; a tip below the collar's base is charged nothing; a species with no potential height is charged nothing (as today). The whole-tree test engine and the one cycle agree as they did before (if the test engine computes the charge separately, it changes with it; no second form anywhere).
- **R3.** Year-independence: the five tests of `grow/cycle/paused/tests.rs` pass, `every_year_is_the_horizons_prefix` run by name.
- **R4.** Every pin that moves is repinned by its own procedure and listed with the tree it pins; presets whose trees do not move (to the bit) are named as such. A walk or closed-form bound that moves is listed with its old and new value and what it checks; none is loosened to pass: return to the host with the numbers if a bound has to move for a reason other than the tree having changed by this rule.
- **R5.** Measured before and after (the top branch against this one), grower and settle only, no drawing: every shipped preset at its preset age and at 40 and 80 on seed 1 and one other seed; the oak also at 120, 200, 300 on seeds 1001, 1007, 1012: height, mean crown spread, trunk diameter at 1.3 m, settled leaf, leafed phytomers, phytomers, tallest living tip against the species' potential height, apexes shed by the balance in the last 20 cycles, peak memory, seconds (only with no pause inside). The oak's trunk beside White's open-grown ranges. A tree that cannot be grown in 24 GB is a row saying so.
- **R6.** Stills for the host's verdict, on one sheet per species, the top branch beside this one: the oak whole and leafless at 40, 80, 120, 200, 300 on seeds 1001 and 1007 beside the base's `base-2026-11-11` tiles; every other preset at its preset age on the scoreboard's first two seeds. Height, trunk and leaf under each tile, a refused tree as a grey tile with the amount it reached.
- **R7.** `docs/pipeline.md` (or wherever the balance is stated for readers) and the preset comments say height; the generation-limit inventory is updated if the guard asks.

## Stops

- Any preset other than the oak whose trunk, height or leaf at its preset age moves by more than 25%: return with the numbers before drawing.
- The height not being available for a tip in some mode the pipeline grows presets in.
- A test that asserts the path form as a promise rather than as arithmetic: return with it, do not edit.
- Any question about slowing extension, the exponent, the settle's fade, or a preset row.

## Boundaries

The charge's argument and the removal of one preset row. No extension rule, no change to the hazard, the allotment, the settle, the fade, the drawing, or any other preset value.

## Rigor

Opus builds; narrow tests while building; heavy runs through `telperion-heavy` with `TELPERION_HEAVY_RESERVE=8`, one GPU job at a time, under the yield watcher (jobs in the background, checked by the clock; seconds only with no PAUSE inside). The host views the sheets and gives the verdict; the gate and review at the phase's landing.
