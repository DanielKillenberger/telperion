# fn-252 R2: before code (worker, 2026-10-14)

## The birch's value, stated before code

No source reached says whether a wild silver birch's hanging shoots end at the ground or run on along it. The texts are in `raw/lit/`:
- Hynynen 2010 and Atkinson 1992 are paywalled.
- Václav 1974 says birch roots only when layered by hand.
- *Betula nana* layers naturally (UW protocol).

The one text on a weeping birch's skirt points the other way, for a grafted garden cultivar. Hammersley Inlet Farm nursery, on 'Youngii' (`hammersley_weeping_birch.md`): "the cascading branches naturally grow toward the ground and will eventually root in the soil or create an untidy mat on the ground if not lifted periodically; trimming the tips of the lowest branches to maintain them 6 to 12 inches above the ground keeps the form attractive".
- It is a nursery care page with no cited evidence, about a cultivar, so it is weak.
- But it is the only text, and it says the shoots go on along the ground.

The silver birch's value is therefore the host's declared stand-in, **1**, on every one of its ages. The stem is exempt by the rule. Every other species is 0.

## The mechanism: what the host's design leaves to settle

The engine has no node ended inside a living axis. The design asks for three things, and each one is a choice about how the system works, so I bring them up rather than guess:

1. **"Not drawn, no leaves" for nodes past contact.**
   - Leaves are counted from each node's grown wood, `internode × scale` (`pipeline/space.rs:424-429`; `foliage/placement.rs`), not from the laid positions. So shortening a laid step alone still leaves the stations and the short-shoot clusters on it.
   - The engine's by-degree idiom for how much of a node exists is the phytomer's `scale`. Ending by presence is one way to meet the design: the lay multiplies wood laid along the ground by (1 - h), and what it bears inherits the factor. At h = 1 that is scale 0, so the wood is not drawn and bears no leaves.
   - The cost: girth (the pipe) and the sag levers are computed from scale before the final lay (`geometry::scale`, `place_bearers`, then the bent lay). So the pipe would need a pass after the lay, or the bearers keep the girth of wood that no longer stands.
   - The other way is truncating the structure after the lay. That is a switch at h = 1 rather than a degree.
2. **Growth's answer.** "In the rough lay, their apex dead" can be one more factor on the apex's survival, beside light's: `viability × (1 - h)` while the apex's rough tip rests (`grow/advance.rs:72-75`). Growth's existing draws then end it, by degree.
   - The device needs the same contact fact from its rough lay (`lay_walk.wgsl`), carried to its advance, as light is carried.
3. **What counts as "along the ground".** Proposed: from the first step the clamp catches, every later step of that axis, and of what it bears.

## What the build touches

- **CPU:**
  - `geometry/lay.rs` (`Layer` gains its contact state).
  - `geometry/place.rs` and the pipe pass.
  - `grow/draw.rs` (the rough lay's contact, per apex).
  - `grow/advance.rs`.
  - `species.rs`, `species/validate.rs`, `species/scale.rs` and `presets/space.rs` (the row).
  - `birch.rs` and `silver-birch.values`.
- **Device:**
  - `lay_walk.wgsl` and the column packing for the new row (`columns.rs` R-rows).
  - The device advance, which reads the contact.
  - The settle's rows, if the pipe is redone after the lay.
- **Tests:**
  - h = 0 byte-identical on every shipped preset, and fn-200's resting test.
  - h = 1: no axis extends along the ground past its contact node, on both lays.
  - CPU and device agree.
