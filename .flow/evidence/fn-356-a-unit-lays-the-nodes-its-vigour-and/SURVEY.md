# fn-356 survey: why the old oak's shoots have no ceiling, 12 Nov 2026

By a Fable agent (code read, small censuses run on fn-354's head in the `fn-355-base` worktree, two papers read in text), condensed by the host. [code] read in source, [measured] run by the surveyor, [text] read in the paper, [inferred] arithmetic, [U] not determined. Paths are under `crates/telperion-space/src/`. Raw dumps were left in the session's scratch directory and are not kept.

## What started it [measured by the host]

The oak at 200 on seed 1002, fn-354's head, alone in a 40 GB scope: 29.58 GiB peak, 1,091 s, 29.4 million phytomers on 6.3 million axes, 1,024,969 new shoots in the last year, 21.0 million phytomers drawn under 0.2 mm of radius, 604 thousand leaves drawn. The base: 5.87 GiB, 173 s, 1.78 million phytomers, 86 thousand shoots. At 120 the twelve seeds hold 1.0 to 2.2 times the base's phytomers.

## The fields [code]

- `Phytomer.size` is the unit's shade size, `(v/p)^psi` normalised over the living buds of its PA to a presence-weighted mean of one (`allocation.rs:139-183`); what the unit bears does not inherit it.
- `Phytomer.scale` while growing is unit presence x node presence x share of cycle x the axis's base scale (`grow/draw.rs:362-363`); the base scale is the bearer node's unsized scale x the birth draws' presences x dominance (`grow/draw.rs:273-292`), so it compounds down the lineage.
- The balance's "size" is base scale x running, that is presence (`grow/cycle/passes.rs:55`); shade size is in neither income nor charge.

## Census, the oak on seed 1002 at its last cycle [measured]

| | 80 | 120 |
|---|---|---|
| Living tips after the balance | 355,866 | 693,907 |
| Sum of presence over tips | 174 thousand | 216 thousand |
| Shed by the balance that year | 9,224 (2.5%) | 25,407 (3.5%) |
| Tip size, 10th / 50th / 90th percentile | 0.21 / 0.53 / 1.58 | 0.15 / 0.49 / 1.41 |
| Tip presence, 10th / 25th / 50th | 0.009 / 0.044 / 0.48 | 0.0013 / 0.010 / 0.079 |
| Tip light, 10th / 50th / 90th | 0.064 / 0.15 / 0.45 | 0.057 / 0.18 / 0.46 |
| Tips with presence under 0.05 | 27% | 44% |
| Held phytomers | 4.68 million | 9.71 million |
| Phytomers under 0.2 mm | 1.20 million (26%) | 4.15 million (43%) |

- Nodes per unit: 3 to 7, mean 4.1 to 4.4 in every size bin from under 0.05 to over 2.
- At 120, units under presence 0.05 hold 47.2% of the phytomers and 0.8% of the leaf area (1,228 m² in all); under presence 0.2, 56.4% and 2.4%; under size 0.5, 45.2% and 6.1%.
- Of the phytomers under 0.2 mm at 120, 95.5% are faint by presence with a size of 0.2 or more.
- At 80, 94% of faint shoots had a bearer already faint; only 1.5% of laterals are born with their own factor under 0.5. The faint share rises with branch order: 3% at order 1, 35% at order 8.
- By decade the living tips run 1.4, 6.7, 24, 56, 108, 182, 271, 365, 468, 558, 649, 694 thousand, while the summed presence is flat from year 100 (214, 221, 216 thousand).

**Reading [inferred]:** light holds the crown's presence-weighted leaf at a ceiling; presence only multiplies downward along a lineage and no rule reads it; so the same leaf is carried by ever more, ever fainter complete shoots.

## What ends a living tip today [code, arithmetic]

For a tip of size 0.05 in light 0.5 at a 15 m path on 34.8 m of potential height: the shade hazard gives viability squared (0.81 for a short shoot, 0.96 for most kinds); the balance gives +0.46, no shedding, since size cancels; lifespans and abortion end kinds by their own count; the ground does nothing. No rule reads size or presence.

## The papers [text]

- **Pałubicki et al. 2009, section 4.2:** "The integer part of the amount v of the resource reaching a bud determines the number of metamers produced by this bud: n = ⌊v⌋. The length l of these internodes is calculated as l = v/n." A bud with v under 1 lays nothing [inferred]. Section 4.4: a branch whose light per internode falls under a threshold "is considered a liability for the tree and is shed."
- **Borchert & Honda 1984:** "terminals receiving F_i < F_min survive for N_s additional increments during which their flux remains constant; then they die." Figure 10: F_min = 5, N_s = 3. No internode count from flux.

## Options weighed [inferred sums over the census at 120, extrapolated to 200]

| Option | New value | Left at 120 of 9.71 million | Left at 200 of 29.4 million |
|---|---|---|---|
| Nodes by shade size alone | none | 5.9 million | 16 to 18 million |
| Nodes by size x presence | none | 2.7 million | 4 to 6 million |
| A fixed part in a tip's charge | one, no source | 4.2 to 5.1 million | up to 8.4 million |
| A hazard rising as presence falls | one, no source | 4.2 to 5.1 million | up to 8.4 million |

`lineage::laid` (`lineage.rs:254-283`) already lays a quota of a zone's nodes with the last growing in by degree.

## What the base hid [code, inferred]

The base's balance charged an axis its accumulated wood, which shed any lateral whose wood outgrew its light: the collapse with age. It never shed a tip for being small either, and its counts at 120 are of the same order, so the faint population was already there.

## Not determined

Why seed 1001 stays small at 200. Whether the faint population explains the thin old crown (half the tips at 120 are under 8% present).
