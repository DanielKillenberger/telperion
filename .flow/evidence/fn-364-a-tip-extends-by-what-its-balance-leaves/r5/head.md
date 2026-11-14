# fn-364 R5: grower and settle, fn-363's branch against this one (builder, Opus, 14 Nov 2026)

**The spec's second Stop fires before any drawing, so R6 is not drawn.** Laurelin at its preset age (80) and Telperion at its preset age (82) both lose more than 15% of their height and more than 25% of their leaf, on both seeds:

| tree | height m, fn-363 to this | leaf m², fn-363 to this |
|---|---|---|
| Laurelin 80, seed 1 | 43.6 to 32.1 (−26%) | 3,473 to 1,393 (−60%) |
| Laurelin 80, seed 7 | 45.1 to 33.3 (−26%) | 5,217 to 2,417 (−54%) |
| Telperion 82, seed 1 | 43.8 to 32.4 (−26%) | 3,393 to 1,338 (−61%) |
| Telperion 82, seed 7 | 45.5 to 33.7 (−26%) | 5,181 to 2,299 (−56%) |

Both were over their potential height of 40 m on fn-363's branch (43.5 to 45.5 m), and are under it on this one (32.1 to 33.7 m). Between 43% and 68% of their living tips are laid at s = 0 at 80 and 82. The natural presets at their preset age (40) stay inside the Stop: the largest height fall is the birch's (16.7 to 15.2 m, −9%), the largest leaf move also the birch's (−11%). At 80, which is past every natural preset's age, the birch falls 18% in height and 32% in leaf.

**The host's reading of the Stop (14 Nov 2026):** the rule does to the Two Trees what it was designed to do, against a potential height that is not theirs. Their values carry the beech's 40 m ("it may reach over 40 m", a lower bound for the beech, fn-345), and the base's Two Trees stand at 45.5 and 46.4 m at 80 and 82 (fn-352's SCOREBOARD.md) where the beech at 80 stands at about 17 to 18 m. Their potential height is an open value, the host's to settle, not a fault of the rule. How they get their size from the beech's rows: TWO-TREES-HEIGHT.md. Their draws wait for that value.

**The oak at 40** is within 5% of fn-363's, as the spec expects: height 10.7 to 10.4 m and 11.1 to 10.8 m (−3%), leaf 115 to 112 m² and 180 to 175 m² (−3%).

## Method

- **fn-363's rows** are its builder's (`scratchpad/fn363/logs/after-*`, fn-363's product at `c2d897a37` with the probe's reader). None were grown again.
- **This branch's rows** come from `.worktrees/fn-364-after`, a detached checkout of `6c7093f96` with the same reader applied (never committed). Its grower and settle are those of `f6c00cc49`: the later commits change only the pipeline's conversion and foliage, which these rows do not read. The runner, the scripts and the table are in `r5/`.
- **Drawn leaf stations** are the pipeline's own station plan (`Expansion::stations`, no drawing): fn-363's from a runner built on its checkout, this branch's on `f6c00cc49`. "-" means not yet measured when this was written.
- **Seconds** are wall time inside the queue scope. "not timed" means a PAUSE of the yield watcher fell inside the tree's window.
- **Living tips at s = 0** counts the living apexes laid at no length in the age's own cycle. **Zero-length phytomers** counts settled phytomers with scale above 0 and length 0.
- **White's open-grown ranges** for the oak's trunk at 1.3 m are as in fn-363's MEASURES.md. The potential height is each species' `1 / inverse height`.

## R3

The oak grown to 80 at once equals the oak paused at 40, settled there and grown on, to the bit (seed 1001, the preset: `tree_digest` `b6f1d607a86ea1c6` both ways; at 40, direct and paused `980c7b3fe04dffc4` both ways).

## Table

