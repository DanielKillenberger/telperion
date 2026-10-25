# Points between species stay viable trees

## Goal & Context

The tree space's claim is that every point between species is itself a tree, changing by degree (STRATEGY.md, "The tree space"). fn-215 found it fails at age 80: the raw equal-weight centroid of beech, oak and spruce grows past the 20M phytomer budget (light-neutral prediction 21.4M kept, against corners of 2.2M-4.7M), and walk midpoints at 80 between existing species are refused or do not finish (oak to spruce, beech to spruce; beech to oak builds). Averaging growth rates of differently built species compounds over decades. fn-215 fixed its own point by matching size: centroid of every row, then one factor on the per-age lateral probabilities so the light-neutral predicted kept phytomers at 80 equal the corners' geometric mean (host, 2026-10-08).

## Requirements

- **R1:** Every blend between species (`blend::families` and the walk) keeps shape by degree and size typical of its endpoints: the blended rows, then one factor on the lateral production probabilities solved so the light-neutral predicted kept phytomers at the family's age equal the endpoints' weighted geometric mean; the factor computed, never authored; neutral (exactly 1) at every endpoint, so every shipped species is unchanged. Amended (host, 2026-10-08): (1) the factor only shrinks, min(1, solved), so a blend predicted at or below the mean keeps its rows; palm blends stay as they are, and some come out smaller than either end, a known property and not a defect. (2) The rule is defined for N points (`blend::centroid`): the weighted centroid of every row, then one factor against the points' weighted geometric mean, with no sequential sizing; a two-way walk is the N = 2 case. (3) A walk sets both ends to its age before blending, so each frame is sized at its own age. (4) The solve time per blend at 40 and 80 is reported next to the build time. (5) Kept phytomers do not bound grown ones, since a blend can shed a larger share: a factor is solved against both the predicted kept and the predicted grown phytomers (the shed included, which the budget counts), and the smaller is taken (option A).
- **R2:** Every walk frame at 40 and 80 between every pair of passed branching species (beech, oak, spruce, birch when passed, the ordinary tree) builds within the budget (this spec); walk tests extended to age 80. No visible pop: owned by fn-231, "the engine's walk is continuous at its ends", and re-judged when it lands (host, 2026-10-08).
- **R3:** The ordinary tree (fn-215) re-derived through the same rule, so it carries no rule of its own; its age-80 walks (deferred from fn-215) pass.
- **R4:** Gate and npm test green; build time of the slowest frame at 80 reported.

## Boundaries

The blend rule only. The cost of lit points is fn-226's open item. Depends on fn-198 and fn-215. R2's visible-pop clause depends on fn-231.

