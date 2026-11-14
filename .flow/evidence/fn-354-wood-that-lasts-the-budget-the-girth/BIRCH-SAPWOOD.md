# fn-354: the birch's sapwood row, source check and decision, 11 Nov 2026

The check is a Fable agent's (web, read-only), condensed by the host; the decision is the host's. [text] read in the paper's text through an extraction, [secondary] cited by another paper, [inferred] arithmetic.

## Why

Under the wood rule the birch at 40 has a 17 to 19 cm trunk (three seeds), against 26 to 30 cm on the base and 22 to 27 cm for dominant stand trees. Its `sapwood` (3.02 cm² per m² of leaf) was the beech's, copied, with "no source was found for the birch".

## What the sources give

| Source | Value | Trees | Mark |
|---|---|---|---|
| Repola 2008, eq. 12, Table 4 | ln(foliage, kg) = −29.566 + 33.372 × dS/(dS + 2), dS = 2 + 1.25 d | 21 forest-grown birches, 11 to 26 cm dbh, 66% *B. pendula*; sample mean 16.5 cm, 17.1 m, 37 years | [text] |
| Lintunen et al. 2011 | specific leaf area 13.55 m² per kg | birch, Finland | [secondary] |
| Hölttä et al. 2013, Table 2 | xylem area = 0.67 × d^2.01 mm² (d in mm) | birch, 555 sections | [text] |
| Hölttä et al. 2013 | brackets "all xylem conducting" against "outer 2 cm"; no measurement | 2 silver birches | [text] |
| Sumida et al. 2023, Table 1 | leaf area = 3,371 × stem area at crown base^1.006 | 201 deciduous broadleaves pooled, not sapwood | [text] |
| Niemistö & Huuskonen 2025 | dominant trees 22.7 to 25.3 cm twenty years after first thinning | planted, southern Finland | [text] |
| Dubois et al. 2021 | 34.6 ± 3.1 cm at 32 years released at age 1; 22.9 ± 4.1 cm released at 19 | 704 crown-released birches | [text] |

No source read gives a measured leaf area per sapwood area at breast height for silver birch, and none a sapwood ring count for any birch.

## The arithmetic [inferred]

Repola's foliage is 4.68 kg at 20 cm and 7.04 kg at 25 cm: 63 and 95 m² at 13.55 m² per kg. Hölttä's xylem area is 283 and 443 cm². Xylem per leaf: 4.5 and 4.7 cm² per m². From Sumida's crown-base figure (2.97) over Repola's mean crown ratio 0.58: 5.1. The weak link is the specific leaf area: at 18 to 20 m² per kg the figure falls to 3.1 to 3.5.

## Decision

- **`sapwood` for the birch: 4.6 cm² per m² of leaf**, derived, not measured: Repola 2008's foliage over Hölttä 2013's xylem area at 20 and 25 cm, with Lintunen 2011's specific leaf area (secondary). It is the whole xylem, on the reading that a birch of this age has no heartwood. The preset's comment says "derived" and names all three.
- **`sapwood_rings` stays 40, estimated:** no birch measurement; Gebauer et al. 2008 found 29 to 88 rings on other diffuse-porous species.
- **Expected:** sections 4.6/3.02 = 1.52 times, the trunk 1.23 times: 21 to 23.5 cm at 40, at the low end of dominant stand trees. Nothing else of the birch moves but what bends less.
- **What would refute it:** a birch foliage-to-sapwood ratio at breast height far from 4.5 to 5 (Ilomäki et al. 2003 was not readable); a dye or sap-flow profile showing conduction stops within fewer rings than the tree's age.
