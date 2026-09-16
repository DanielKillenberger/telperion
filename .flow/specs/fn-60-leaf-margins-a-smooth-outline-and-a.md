## Conversation Evidence

> user (2026-09-16): "Also i haven't seen any leaf side by side with a reference?"
> user (2026-09-16, on the host's proposal to spec leaf margins, a smoother outline and a shoot view): "yes"
> user (2026-09-16, on the round-24 leaf pairs): "why is there no texture to the leaf? i thought we have that capability?"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 25% [user], 35% [paraphrase], 40% [inferred] -->

No leaf had been judged beside a photograph. The references B-LEAF, B-LEAVES, S-LEAF and S-SHOOT carry no shot block, so the runner rendered one leaf and paired it with nothing. Paired by hand on 2026-09-16 (`.flow/evidence/fn34/measure/pairs-leaf22/`), both leaves fall short. [paraphrase]

- The beech's blade has the right proportions but a visibly polygonal outline, flat yellow-green colour where the photograph is dark glossy blue-green, and almost none of its strong, straight, parallel side veins. [inferred]
- The birch's blade is a teardrop with a long spike tip; the photograph's is triangular, with a broad, nearly straight base, a shorter point and a double-toothed margin. The element has no teeth: its only margin row is the oak's lobes. [inferred]
- The shoot photographs show many leaves on a twig; there is no generated shoot to stand beside them. [inferred]

This spec gives the leaf element a toothed margin, a smooth outline and a lit surface texture, makes sure the leaf view draws under the tree's own light and material rows so its colour can be judged, and adds a shoot view the runner pairs with the shoot photographs. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **Teeth.** Element rows for the teeth along each margin: their count, their depth as a share of the half-width, and a second order of smaller teeth between them (the birch's double serration). Neutral is an entire margin, and every shipped element is byte-identical there. Teeth sit on top of the existing lobes (`lobe_count`, `lobe_depth`), so an oak can carry both. [inferred]
- **A smooth outline.** The outline is sampled finely enough that no straight facet shows at the leaf view's size, and the element's levels of detail (`crates/telperion-core/src/foliage/levels.rs`) simplify it for distance as they do now, so a crown's cost at distance does not rise. [inferred]
- **A shape the birch can reach.** If `widest_at`, `base_fullness` and `tip_sharpness` cannot draw a broad straight base under a short point, a base-shape row covers it; values first. [inferred]
- **A judged leaf view and a shoot view.** The Leaf view's flat colour is checked first: whether it draws under the tree's own sun, sky and material rows, and made to if it does not. A Shoot view draws one generated shoot with its leaves, as the local law grew it at its placement. Reference records at leaf and shoot scale gain a shot block with a scale, and the runner pairs them like every matched still. [inferred]
- **Leaf surface texture.** fn-26's blade detail is colour only: `vein_tone` (`crates/telperion-render/src/shaders/foliage.wgsl:70-84`) draws the midrib and side veins as hairlines of a fixed 0.015 and 0.018 of the leaf, brightened by up to `0.8 * vein_contrast`, and `blade_mottle_strength` adds a faint noise. The photographs' texture is mostly light: dark, sunken veins and a blade that puckers between them. Rows for vein width and for vein tone (darker as well as lighter), and a footprint-faded normal relief along the veins and between them, native and browser, neutral byte-identical, so a lit leaf reads textured at the leaf and shoot views and converges to its mean at distance. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** Teeth rows exist with rails, neutral byte-identical on every shipped element, refused by name off their rails, on the wire, blended, in the regenerated metadata and on the harness. Errors: a value off a rail is refused naming the field. [inferred]
- **R2:** The outline shows no facet at the leaf view's size, and the element's levels keep their deviation bounds and the crown's drawn triangle count at the protocol's distances within the fidelity band. Errors: a level outside its deviation bound fails. [inferred]
- **R3:** The runner pairs B-LEAF, B-LEAVES, S-LEAF and S-SHOOT with a leaf still or a shoot still under their records' shot blocks, and compares them. Errors: a record without a still fails the run. [paraphrase]
- **R5:** The vein width, vein tone and blade relief rows exist with rails, neutral byte-identical, native and browser, footprint-faded so the distance and resolution tests hold at their bounds. Errors: a distance or resolution test outside its bound fails. [inferred]
- **R4:** The birch and the beech state their leaf rows against their leaf photographs, the pairs are rendered, the 48-case protocol passes, the implementer answers after looking whether each leaf reads as its photograph's, and the owner judges in fn-34. Errors: a rejecting verdict stops the spec with the owner's words. [user]

## Boundaries
<!-- scope: business -->

- No compound leaves; that is fn-33's, and the ash waits on it. [inferred]
- No change to how leaves are placed in the crown. [inferred]
- Material highlights stay fn-55's. [inferred]

## Resolved via Codebase

- Element rows: `crates/telperion-core/src/foliage/element.rs` (`length`, `width`, `widest_at`, `base_fullness`, `tip_sharpness`, `cup`, `curl`, `lobe_count`, `lobe_depth`, `section_roundness`, `axial_segments`, `cross_segments`, `card`); outline in `foliage/outline.rs`, levels in `foliage/levels.rs`.
- Shipped elements: beech `axial_segments: 8`, `tip_sharpness: 0.7`; birch `widest_at: 0.28`, `base_fullness: 0.55`, `tip_sharpness: 1.8`, `axial_segments: 8` (`crates/telperion-core/src/presets/species.rs`).
- Leaf references without shot blocks: `.flow/evidence/fn34/european-beech/references.json` (B-LEAF, B-LEAVES), `.flow/evidence/fn34/silver-birch/references.json` (S-LEAF, S-SHOOT).
- The runner's leaf view: `tests/species.mjs` (whole, bare and leaf jobs; "one placed element at generated scale").
