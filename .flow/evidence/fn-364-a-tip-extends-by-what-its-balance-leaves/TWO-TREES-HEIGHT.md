# How the Two Trees get their size from the beech's rows (builder, Opus, 14 Nov 2026; reading, no change)

Read on `f6c00cc49`. Heights come from R5 (MEASURES.md), seed 1.

## Every row that differs from the beech's and bears on size

**No `/space` row differs.** `telperion.values` and `laurelin.values` carry the beech's `/space` rows to the character: every internode, lifespan, node count, elongation, hazard and `inverseHeight`. The rows that differ and bear on size are all in the supernatural group. `supernatural.rs`, `Supernatural::applied`, writes them over the species' own: an internode or primary radius per age, and the leaf area times the square of the canopy size over the natural `/canopy/size`.

| row | beech (the natural value) | Telperion | ratio | Laurelin | ratio |
|---|---|---|---|---|---|
| internode, age 2 (stem) | 0.07 m | 0.182 m | 2.6 | 0.182 m | 2.6 |
| internode, ages 4 to 8 (leader, fork, limbs, boughs, spurs) | 0.04 m (laurelin.values' comment; the beech's file sets no row for them) | 0.104 m | 2.6 | 0.104 m | 2.6 |
| primary radius, ages 2, 4 to 8 | 0.008 m | 0.024 m | 3.0 | 0.0272 m | 3.4 |
| canopy size (the leaf) | 1.1 | 2.3 | 2.09, so leaf area × 4.37 | 2.4 | 2.18, so leaf area × 4.76 |
| `/age` | 40 | 82 | | 80 | |

The fine ages (9, 12 and 14: branch systems, shoots, short shoots) keep the beech's internodes (0.03, 0.025, 0.006 m) and primary radii. No `/supernatural/scale` row is set in either file, so the wood is drawn at a scale of 1. The rest of the group (bloom, material, reflectance, flowering) does not bear on size.

## `inverse_height` and scaling

- **Every age of both Two Trees carries `inverseHeight = 0.025`**: ages 2, 4, 5, 6, 7, 8, 9, 12 and 14, the beech's own rows, a potential height of 40 m.
- **`species/scale.rs` scales no species as a whole.** It is the space's metric: how each setting mixes between two species on a walk (`Scale::Log` for magnitudes). Lines 129 and 187 only list `inverse_height` among the settings, so a walk mixes it on a log scale (`table.rs`: `Log { floor: 1e-3 }`, per metre).
- **Nothing lengthens a species as a whole, so nothing scales the potential height with lengths.**
  - The supernatural internode rows lengthen the coarse ages' internodes 2.6 times and leave `inverse_height` at the beech's 0.025. The Two Trees' wood is laid 2.6 times longer per node while the balance charges it against the beech's 40 m.
  - The only other multiplier of internodes is `Species::coarsened` (`species/coarse.rs`, a fidelity's coarse tree), which divides node counts by the same factor and leaves `inverse_height` alone.

## Heights, seed 1

| tree | fn-363 | this branch |
|---|---|---|
| beech 40 | 10.0 m | 9.8 m |
| Telperion 40 | 25.8 m (2.58 × the beech) | 22.8 m (2.33 ×) |
| Laurelin 40 | 25.8 m (2.58 ×) | 22.8 m (2.33 ×) |
| beech 80 | 17.4 m | 16.6 m |
| Telperion 80 | 43.5 m (2.50 ×) | 32.2 m (1.94 ×) |
| Laurelin 80 | 43.6 m (2.51 ×) | 32.1 m (1.93 ×) |

On fn-363's branch the Two Trees stand about 2.5 times the beech at both ages, close to their 2.6 times longer coarse internodes. On this branch they hold 2.33 times at 40 and 1.94 times at 80. At 80 the beech stands at 16.6 m against its 40 m, so its tips are charged (16.6/40)² = 0.17 of their light. The Two Trees stand at 32 m against the same 40 m, (32/40)² = 0.64, and 43 to 68% of their living tips are laid at s = 0 (MEASURES.md).
