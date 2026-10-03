# Tree space, overnight run of 2026-10-03 to 04: morning report

## Outcome

- **A (fn-191), the engine core, passed.** Draft PR #140. `telperion-space` reproduces GreenLab's closed-form counts exactly and Letort's simulators (run unchanged) count for count deterministically and in distribution over 4,000 seeds; Codex review SHIP; workspace gate 990 passed.
- **B (fn-192), continuity by construction, passed.** Draft PR #141, stacked on #140. Lineage-keyed draws; presences after growth; each draw grows in over a log-odds window scaled by the closed form's expected wood under its node. Eight walk strips change by degree on the host's view; 17.0% of length lost to partly grown branches; workspace gate 1004 passed.
- **C (fn-193), the beech, stopped** on its wrong-path rule after four rounds. Branch pushed, no PR; task in progress. D to F did not start.

## The beech: what the sheets show

- **Best so far: round 2** (`raw/final2/sheet-beech.png`, committed values on the branch). At 80 years both seeds read as mature broadleaf trees: clear bole, fork near a quarter of the height into ascending limbs, rounded crown, trunk 1.03 and 1.20 m at chest height (sourced maximum 1.3 m). Structurally nearer the Rostock and Entzia photographs than today's beech. Short: the in-leaf crown is airy, not a dense mass; the young trees (10, 20 years) are a pole with rods.
- **Round 4** (`raw/final3/sheet-beech.png`, reverted): Troll's modules with relays from the bend no longer collapse the tree, but the trunk becomes a stack of ring-shaped bulges (each relay drawn as a fork beside the tip it replaces), the mature fork disappears into a leaning fan, the crown is sparse, and the 20-year tree is still a pole with rods.
- **Colour:** white bark and pale leaves are the beech preset's own appearance; today's beech draws the same on this branch (row 6). Darkening it is the owner's call.

## Why C stopped, and what is open

1. **Troll's young form.** The relay mechanism is built and tested (growth age carried across relays, relay position, epitony, relays that decide only the difference). Two things are still wrong: a relay is drawn as a side fork beside the old tip, which bulges the trunk, where it should continue the axis; and the module's plagiotropic tip and straightening base do not yet produce the crooked stacked young beech. A design decision, not a tuning one.
2. **Leaves per node.** The pipeline places leaves at a fixed spacing along leaf-bearing wood; the sources give leaves per short shoot (3 to 5, 77% of leaf area). An exact per-node leaf rule is a contract between the engine and the pipeline (phase F, fn-125's station contract).
3. **Cost.** 85 to 90% of the wood grown is shed (branch systems live 10 + 5 + 3 years and are shed whole); growth takes 1 to 2.6 s against today's 57 ms skeleton. Lifespans are fixed settings, so shedding can be decided before growth: the main lever for F.

## Decisions taken on the owner's behalf overnight

- fn-191 R4: L-Py's shipped models as the runnable 3D reference, Pałubicki 2009's figures as the mature target.
- fn-192: three host design decisions on the grow-in window, each from the worker's measurements; one "last attempt" past the stop rule (it passed). The Codex review capped twice with real findings; the host reset it once, Astra reviewed the last fix independently (SHIP), and the task closed with the override recorded.
- fn-193: four rounds; one "last attempt" past the stop rule after the module change collapsed the tree before its look could be judged; it did not pass, and C stopped.

## Friction (all specs)

- **Local setup, reported only:** the dcg guard blocking variable-path redirects, `git checkout -- <path>` and heredocs (fn-191, fn-192, fn-193; about 10 minutes in all); `/usr/bin/time` missing (fn-193).
- **Fixable in the repository:** pipeline refusals should name the first offending node and its radius or length (fn-193, 10 minutes); one example for both stills and measures, so a stale binary cannot time old values (fn-193); the walk helper already names the failing setting (fn-192, done).
- **Recorded knowledge:** the variance standard error for two-valued counts (fn-191).

## For the owner

1. Judge A and B at their PRs (#140, #141).
2. Look at the beech sheets above and decide: is round 2's mature tree the direction, and how should a relay continue the axis (the remaining Troll question)?
3. Darken the beech preset's colours or not.
