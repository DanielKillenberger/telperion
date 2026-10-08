# Points between species stay viable trees

## Goal & Context

The tree space's claim is that every point between species is itself a tree, changing by degree (STRATEGY.md, "The tree space"). fn-215 found it fails at age 80: the raw equal-weight centroid of beech, oak and spruce grows past the 20M phytomer budget (light-neutral prediction 21.4M kept, against corners of 2.2M-4.7M), and walk midpoints at 80 between existing species are refused or do not finish (oak to spruce, beech to spruce; beech to oak builds). Averaging growth rates of differently built species compounds over decades. fn-215 fixed its own point by matching size: centroid of every row, then one factor on the per-age lateral probabilities so the light-neutral predicted kept phytomers at 80 equal the corners' geometric mean (host, 2026-10-08).

## Requirements

- **R1:** Every blend between species (`blend::families` and the walk) keeps shape by degree and size typical of its endpoints: the blended rows, then one factor on the lateral production probabilities solved so the light-neutral predicted kept phytomers at the family's age equal the endpoints' weighted geometric mean; the factor computed, never authored; neutral (exactly 1) at every endpoint, so every shipped species is unchanged.
- **R2:** Every walk frame at 40 and 80 between every pair of passed branching species (beech, oak, spruce, birch when passed, the ordinary tree) builds within the budget; the walks show no visible pop; walk tests extended to age 80.
- **R3:** The ordinary tree (fn-215) re-derived through the same rule, so it carries no rule of its own; its age-80 walks (deferred from fn-215) pass.
- **R4:** Gate and npm test green; build time of the slowest frame at 80 reported.

## Boundaries

The blend rule only. The cost of lit points is fn-226's open item. Depends on fn-198 and fn-215.