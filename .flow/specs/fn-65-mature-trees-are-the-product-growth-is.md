# Mature trees are the product; growth is hidden

## Conversation Evidence

> user (2026-09-17): "i swear i see some completely different build in the browser again"
> user (2026-09-17): "If this is seed 1. My seed 1 in the harness is 100% a different tree. More messy it has a ring the one you rendered in the pic doesn' thave. it's a different tree.."
> user (2026-09-18): "hm it seems growing the tree as of now seems not mature enough to produce high quality trees just yet. Should we just make it a hidden feature for now and work on just mature trees."
> user (2026-09-18): "alright let's do that"

## Goal & Context
<!-- scope: business -->

Telperion has two ways to make a tree from one preset and one seed. The direct build (`mesh::build`, `branching::generate`) makes the mature tree in one pass; it is what the headless stills, `species:qa`, the numeric protocol and every matched-shot verdict since fn-34 round 2 use. The growth path (fn-11, fn-30) builds a specimen and grows it to an age; it is what the harness draws by default through `buildSpecimen` in `harness/GrowerDev.tsx`. On 2026-09-17 the owner opened the round-26 birch in the harness and saw a different tree from the judging page. Natively, `node_buffer silver-birch` gives 73,337 nodes at seed 1 and `node_buffer silver-birch 100` gives 590,410. Every species verdict so far was taken on a path the harness never showed. [user]

The owner's decision on 2026-09-18: the mature tree is the product for now, and growth is a hidden feature until it can produce trees of judging quality. Everything calibrated on master lives on the direct build; the growth path has no calibrated species on master and no judging rig. [user]

## Architecture & Data Models
<!-- scope: technical -->

- **The harness draws the direct build by default.** `GrowerDev` calls the stage's `setTree(familyJson(params))` unless the page was opened with `?growth=1`. The growth controls (age, frontier, seek) are not rendered on the mature path; on the growth path the component behaves exactly as today. The query flag is parsed beside `normalizeSeed` in `harness/params.ts` and tested there. [paraphrase]
- **Nothing else moves.** The headless renderer, `species_measure`, `species:qa`, the pairs rig, the identity pins and the Wasm bindings keep both paths as they are. The growth path stays buildable and pinned; it stops being a default and stops gating species work. [paraphrase]
- **The rule is written down.** `CLAUDE.md` gains an owner rule dated 2026-09-18: the mature tree is the product; the harness and every judging surface draw the direct build; growth is reachable only behind `?growth=1`; a spec that routes production through growth needs the owner's word. The fn-31 branch's "route production through growth" is noted there as superseded for master, so an agent resuming fn-31 knows growth calibration is a feature in waiting, not the shipped path. [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- A link that carries `?growth=1` with a species and seed opens the specimen at the preset's age, as today. Any other value of `growth`, or its absence, is the mature path. [inferred]
- The direct build has no age to seek; the age panel is absent rather than disabled, so a reader does not look for a control that cannot do anything. [inferred]
- fn-28, the tree that grows as the owner's website scrolls, depends on the growth path and waits for it. fn-30 and fn-31 continue on their branch as growth calibration. Neither is changed by this spec. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** Opening the harness with `?species=<id>&seed=<n>` draws the direct build of that preset at that seed, the same tree `target/release/examples/headless --preset <id> --seed <n>` renders, and shows no growth controls. [user] Errors: a build failure keeps the tree already on the canvas beside the renderer's message, as today.
- **R2:** Opening the harness with `?growth=1` added draws the specimen grown to the preset's age with the growth controls, byte-for-byte the behaviour before this spec. [paraphrase] Errors: a `growth` value other than `1` is the mature path.
- **R3:** The flag parser has a unit test beside the seed parser's, and `npm test` and `npm run typecheck` pass. [inferred]
- **R4:** `CLAUDE.md` carries the dated owner rule and the note on the fn-31 branch. [user]

## Boundaries
<!-- scope: business -->

- No change to the generator, the renderer, the stills protocol, the presets or the identity pins. [paraphrase]
- No removal of the growth path or its tests. Hidden means not default, not deleted. [user]
- fn-28, fn-30 and fn-31 are not rescoped here. [inferred]

## Decision Context
<!-- scope: both -->

### Motivation
<!-- scope: business -->

The owner wants one path every verdict is taken on, and the calibrated one is the direct build. Two paths meant the birch's acceptance at fn-34 round 25 and the round-26 settings page judged a tree the harness cannot show. [paraphrase] The growth path is kept behind a flag rather than removed because fn-28 is the reason it exists and fn-31's thirteen rounds of calibration are not to be thrown away. [paraphrase]

## Strategy Alignment

- Serves "Growth and botanical fidelity": the species the owner judges and the species the harness shows become the same tree, so a verdict means one thing. [strategy:Growth and botanical fidelity]
