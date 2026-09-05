# Botanical references — frozen 2026-09-06

Both profiles are **ready for implementation**, not validated generated species. The machine-readable targets, definitions, uncertainties and rubric live in [profiles.json](profiles.json). Selection followed source review; Scots pine was considered but Norway spruce supplies stronger architectural contrast and clear single-needle anatomy without a fascicle abstraction.

## Source manifest

All sources accessed 2026-09-06. University identification/horticultural pages and USFS silvics are authoritative descriptions, not joint statistical distributions of tree dimensions.

| ID | Source and attribution | Use / limitation |
|---|---|---|
| OSU-OAK | [Quercus garryana, OSU Landscape Plants](https://landscapeplants.oregonstate.edu/plants/quercus-garryana), Patrick Breen contact, Oregon State University | Leaf length and habit; photographs below. Copyright OSU; photographer not individually identified on the page. |
| USFS-OAK | [Oregon White Oak, Silvics](https://research.fs.usda.gov/silvics/oregon-white-oak), William I. Stein, 1990 | Mature height/DBH and open-versus-forest form. HTML contains OCR errors: “15 to 27 in (50 to 90 ft)” means metres, confirmed by OWIC; do not interpret inches literally. |
| OWIC-OAK | [Oregon White Oak, Oregon Wood Innovation Center](https://owic.oregonstate.edu/node/79), Niemiec, Ahrens, Willits & Hibbs, *Hardwoods of the Pacific Northwest*, 1995 | Independently presented units corroborate mature sizes and width comparable to height. “May equal” is not a numerical crown-width distribution. |
| OSU-OAK-ID | [Oak species, OSU Common Trees of the Pacific Northwest](https://treespnw.forestry.oregonstate.edu/broadleaf_genera/species/oak_spp.htm) | Blade width 2–5 inches; rounded irregular lobes. Distinguish the Oregon oak section from preceding California black oak. |
| OSU-SPRUCE | [Picea abies, OSU Landscape Plants](https://landscapeplants.oregonstate.edu/plants/picea-abies), Patrick Breen contact, Oregon State University | Ordinary landscape size, needle length/cross-section, drooping secondaries and photos. Cultivar descriptions excluded. |
| NCSU-SPRUCE | [Picea abies, NC State Extension Plant Toolbox](https://plants.ces.ncsu.edu/plants/picea-abies/) | Species-form landscape spread and peg attachment. Photo cultivar captions must not silently become species targets. |

## Inspected images

Original downloads reside only under ignored `.refs/fn9/`. Each image below was opened and visually inspected. No reference pixel has been copied into committed evidence. Copyright/reuse rights are not inferred from download access: keep OSU images local for reference; obtain permission before redistribution. These photographs are uncalibrated, different individuals and seasons; none supplies branch totals, actual leaf area, exact age or measured dimensions.

Image URLs have the prefix `https://landscapeplants.oregonstate.edu/sites/plantid7/files/plantimage/`.

| ID | File | Page caption | Inspected observation |
|---|---|---|---|
| O-WHOLE | `quga788B.jpg` | plant habit | Open-grown leaf-on broad irregular crown, substantial low limbs and internal windows; target context. |
| O-BARE | `quga999A.jpg` | plant habit, winter | Crooked ascending scaffold limbs subdivide into much finer axes; winter anatomy only, not summer density. |
| O-LEAF | `quga28.jpg` | leaf | Rounded lateral/terminal lobes and narrowed petiole at twig; asymmetric outline, not a pointed ellipse. |
| S-WHOLE | `piab977.jpg` | plant habit | Two species-form landscape trees; persistent leader, tapering tiers, lower branches retained. Left tree's crown is more open; right denser. |
| S-BRANCH | `piab428B.jpg` | branch | Near-horizontal primary has an upturned tip; slender secondary branchlets visibly hang underneath. |
| S-NEEDLE | `piab347A_0.jpg` | branchlet and needles | Upper/underside and peg close-ups; individual needles surround twig, upper needles lean forward. |

Reproduce retrieval (replace FILE with the manifest filename):

```bash
mkdir -p .refs/fn9
curl --fail --location --user-agent 'Mozilla/5.0' \
  'https://landscapeplants.oregonstate.edu/sites/plantid7/files/plantimage/FILE' \
  --output '.refs/fn9/FILE'
```

Verify the downloaded file decodes as an image, then inspect it; an HTTP success alone is not a visual assessment. Raw Python urllib initially received HTTP 403 for the page. Ordinary curl with the stated user agent retrieved the public pages and original images; no authentication or consent controls were bypassed. All six required images were available. If a future retrieval fails, preserve the unavailable status and withhold an unqualified visual pass until an equivalent attributed reference is inspected.

## Interpretation limits

Oak identification sources disagree about exact lobe count (and count conventions); rounded lobes and attachment are the target, not a fabricated exact count. OSU's broad oak height description includes smaller trees; the narrower mature favorable-site range is deliberately selected. Spruce dimensions describe a mature landscape specimen, not the much taller forest maximum. The spruce reference includes variable gaps and lower-branch retention, so an absolutely solid cone and a long bare forest bole both miss this context.

No suitable context-matched branch-count, twig-count or leaf/needle-total dataset was located. Those fields remain unknown and contextual. Crown-ratio, spruce diameter and needle-width estimates are explicitly low confidence; they cannot create a botanical pass. Independent new evidence is required to promote them to gates. Geometry integrity and the identifying visual anatomy remain required despite those unknown totals.
