---
satisfies: [R1, R2, R3]
---
# fn-61-trolls-model-limbs-pitch-by-height-and.1 Implement Troll's model limbs: pitch by height and a ragged reach

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
Two habit rows declared once in the catalogue: `pitchByHeight` (degrees the lateral pitch moves from the crown's base at `trunkHeight` to the envelope's top) and `raggedReach` (the most a first-order axis stops short of its room, each axis drawing its own share from its own key). Both neutral at zero: the generation digest of every shipped table at seeds 1 and 7, growth view included, is identical to the base.

R1: rails refused by name (`InvalidValue { field: "pitchByHeight" | "raggedReach" }`) on the build, the wire and a walk: `suite/troll_limbs.rs`; harness rails in `harness/rows.test.ts`. R2: `pipeline/branching/scaffold/troll_tests.rs`, one test per property (base pitch, top pitch, graded, growth path grades over the same crown, shortfall within the row, differing per axis and seed, same seed same shares).

R3, beech bare view, seed 1, rows stated by overlay only (`pitchByHeight -20`, `raggedReach 0.35`; the shipped `lateralPitch 28` untouched), beside the neutral beech (`.flow/evidence/fn-61-.../raw/beech-bare-{neutral,troll}.png`): the lower limbs do spread wider than the upper; the upper crown reads more upright and narrower. The crown edge is not ragged: the outline is still a smooth, regular dome, because the deeper orders and twigs still fill to the shell. Whether that needs a relay, a raggedness on deeper orders, or a beech value move is the host's call.

Decisions for the host: the top pitch is a signed offset from `lateralPitch`, not an absolute top pitch, because an absolute row could only be neutral on every table as an optional row, which blends discontinuously. The pitch grade applies at every order, as `lateralPitch` does. Adding two `HabitParams` fields bumped the specimen snapshot to schema 4 (core, browser, README). The catalogue identity digests, dial counts, sweep held rows and limit inventory were re-pinned (the identity digests reproduce the d6a3405c values with the two rows stripped); the beech replay tape was extended live through tune with `--extend`, adding 9 Jev and 2 adapter answers. These re-pins landed after the review's SHIP.

Follow-ups: `scaffold.rs` is 450 lines, over the 400 guideline (was 435). Friction: 4 entries in `.flow/evidence/fn-61-.../FRICTION.md`.

stage: impl-review - ran (codex fan-out 3 draws, NEEDS_WORK: 3 findings fixed; re-dispatched after a finalize-order error; NEEDS_WORK: 1 finding fixed; re-review SHIP)
## Evidence
- Commits: d72cfdd1f3a27f8053c854929b338dfa618a14ac, bb100b5a2c751aa7edfa39da50b4b8ec2bafa5f4, 708464dc91f860ee40b521225352d5f3792265ed, 8e4e1cf8ca93dfbf808fa1063d37c6237f44919b, 15eb36c0842cdd7224ed055a763a590d7d023e6e, b860dad85026c37bd0eab2eb3df0f3544972e1a9
- Tests: baseline: none (spec defines no Quick commands; the gate runs once at the end per AGENTS.md), cargo test --profile ci --workspace --no-fail-fast (green at b860dad8, 1058 passed; first run at 15eb36c0 red on 7 pins this task moved, repaired in b860dad8), npm test (green at 8e4e1cf8, 13 files, 112 tests), cargo run --profile ci -p telperion-render --example generation_digest: base c71ef3e1 and change identical on 8 tables x seeds 1 and 7, growth view included (.flow/evidence/fn-61-trolls-model-limbs-pitch-by-height-and/raw/digest-*.jsonl), cargo test --profile ci -p telperion-core --lib troll (10 tests)
- PRs: