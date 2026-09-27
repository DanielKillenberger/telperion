---
satisfies: [R1, R2, R3]
---
# fn-61-trolls-model-limbs-pitch-by-height-and.1 Implement Troll's model limbs: pitch by height and a ragged reach

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
Two habit rows declared once in the catalogue: `pitchByHeight` (degrees the lateral pitch moves from the crown's base at `trunkHeight` to the envelope's top) and `raggedReach` (the most a first-order axis stops short of its room, each axis drawing its own share from its own key). Extension (owner, 2026-09-27): a shortened primary keeps everything it bears (its deeper axes and their twigs and leaves, sleeping-shoot wakes and hanging curtains included) inside the crown's shell scaled about its station by the share it kept (`pipeline/branching/limbs.rs`). Both rows neutral at zero: the generation digest of every shipped table at seeds 1 and 7, growth view included, is identical to the base.

R1: rails refused by name (`InvalidValue { field: "pitchByHeight" | "raggedReach" }`) on the build, the wire and a walk: `suite/troll_limbs.rs`; harness rails in `harness/rows.test.ts`. R2: `pipeline/branching/scaffold/troll_tests.rs`, one test per property (base pitch, top pitch, graded, growth path grades over the same crown, shortfall within the row, differing per axis and seed, same seed same shares); the extension in `pipeline/branching/specimen/limb_tests.rs` (descendants within the share, hanging curtain within the share, coincident limbs keep their own shares, a whole limb names no bound). Red-first checked: with the bound's enforcement disabled in the scaffold edge and the local planner, both containment tests fail on "left its limb's share"; restored, all four pass.

R3, beech bare view, seed 1, rows stated by `--family` overlay only (shipped `beech.values` untouched), 900x900, beside the neutral beech (`.flow/evidence/fn-61-.../raw/beech-bare-{neutral,troll,strong}.png`). At `pitchByHeight -20, raggedReach 0.35` the lower limbs spread wider than the upper and the upper crown reads upright and narrower; the edge is now broken in places (notches on both flanks where one limb system ends short of its neighbour), but the outline still reads mostly as one continuous ovoid. At `-45, 0.6` the crown edge is clearly ragged: it breaks into separate limb systems with gaps between them, each ending at its own reach, though the crown becomes narrow and near-columnar at the top. So yes, the crown edge is now ragged, strongly at the higher values and only partly at the moderate ones; which values the beech takes is the beech spec's value round.

Decisions for the host: the top pitch is a signed offset from `lateralPitch`, not an absolute top pitch, because an absolute row could only be neutral on every table as an optional row, which blends discontinuously. The pitch grade applies at every order, as `lateralPitch` does. The specimen snapshot is schema 5 (core, browser, harness, README): schema 4 carried the two rows and 5 the limb bounds; no schema 4 ever reached master, so 5 skips a number there. The catalogue identity digests, dial counts, sweep held rows and limit inventory were re-pinned; the beech replay tape was extended live through tune with `--extend`.

Follow-ups: `scaffold.rs` is over the 400-line guideline. Friction: 5 entries in `.flow/evidence/fn-61-.../FRICTION.md`.

Tier: session model (worker dispatched in-host; no bridge).

stage: impl-review - ran (extension a1530c4d..HEAD: codex fan-out 3 draws, NEEDS_WORK on 1 merged finding (snapshot schema), fixed in 857085fa; re-review SHIP)
## Evidence
- Commits: d72cfdd1f3a27f8053c854929b338dfa618a14ac, bb100b5a2c751aa7edfa39da50b4b8ec2bafa5f4, 708464dc91f860ee40b521225352d5f3792265ed, 8e4e1cf8ca93dfbf808fa1063d37c6237f44919b, 15eb36c0842cdd7224ed055a763a590d7d023e6e, b860dad85026c37bd0eab2eb3df0f3544972e1a9, a1530c4d4cc83bba32711da7e9b9ab92294bc702, 257a6fc259d86ad3faac3aba246afc19ee0c94cd, 624394817717bc541a5f94f6d3d24b04f5028458, ffd03108bcb1b19942896c2a53de8076ac618f62, b49a494d10506c0e65f9dd4c21ed17919a6a0932, 857085fa41d83574f1776687acc9377d6548b592, b5ea090cadc5681bfc5f797824fa12c0c4df4ad6
- Tests: baseline: none (spec defines no Quick commands; the gate runs once at the end per AGENTS.md), cargo test --profile ci --workspace --no-fail-fast (green at 857085fa, 1062 passed; also green at b49a494d before the review fix), npm test (green at 857085fa, 13 files, 112 tests), cargo test --profile ci -p telperion-core --lib limb_tests (4 tests; red-first: 2 containment tests fail with the bound enforcement disabled, green restored), cargo test --profile ci -p telperion-core --all-targets old_snapshots (schema 5 refuses 4), cargo run --profile ci -p telperion-render --example generation_digest: base c71ef3e1 and change identical on 8 tables x seeds 1 and 7, growth view included (.flow/evidence/fn-61-trolls-model-limbs-pitch-by-height-and/raw/digest-*.jsonl), target/ci/examples/headless --preset european-beech --seed 1 --view bare --size 900x900 --family raw/beech-{troll,strong}.json (R3 renders)
- PRs: https://github.com/DanielKillenberger/telperion/pull/130