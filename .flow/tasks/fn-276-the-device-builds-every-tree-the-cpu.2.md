---
satisfies: [R3]
---
# fn-276-the-device-builds-every-tree-the-cpu.2 A build's start leaves room for its first move

## Description
Host 2026-10-14 from SWEEP.md: Sizes::margins starts at margin 1.0 whenever the starting tables fit, with no room for a later move that holds a group twice (the spruce at 62 refused, 63-68 built; 71 and 74-80 at the browser cap). The start margin is chosen so the starting tables plus their largest first move fit the budget.

## Acceptance
Red/green test on the spruce at 62; the spruce 60-80 at TELPERION_DEVICE_BUDGET_MIB=6144 with the card's cap reported per build; the sweep's other cases unchanged; cost measured.

## Done summary
A build's start now leaves room for its first move. The start margin is the largest the budget holds beside the largest table group the first move makes again (`Sizes::first_move`). It is checked against the cap and charged to nothing (`Ledger::fits`). This is the same margin ladder, with no species branch.

**The spruce at 62, seed 1:** red without the room (4,203 of 4,096 MiB) and green with it (9,065,481 instances). The test is the ignored device test `the_spruce_at_62_starts_with_room_to_move`.

**The spruce at 60 to 80, three seeds, at `TELPERION_DEVICE_BUDGET_MIB=6144`:** all 63 build. The card's cap was 5,640 to 5,838 MiB on each build, so the spruce to 80 fits the browser's 6 GiB.

**At the native 4 GiB,** the spruce builds to 72, apart from 69 at seed 42. From 73 the budget refuses some or all seeds by name, at later moves that are outside this rule. The other 1,794 sweep cases are unchanged in outcome and instance count.

**Cost:** GPU-stamped time is unchanged (Telperion 82: 693.5 to 693.3 ms; the spruce at 40: 232.6 to 232.5), and peak memory is the base's.

**`buffer.rs`** gains one read accessor, `Ledger::fits`. That is separate from the fn-270-grow-budget fix to `destroy`, which is not merged here. A merge may meet a textual conflict near `unreserve`.

Evidence: `.flow/evidence/fn-276-the-device-builds-every-tree-the-cpu/SWEEP.md`, "Task 2".

Tier: worker (host-dispatched)

stage: impl-review - ran (codex, one reviewer: SHIP)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 3fc6acf65bc8f22ee4d5150ed6a8cf631c6a8277, 615a153bf00bfd07c3901da14159f68eed2f7dbd, 73304a60db6ab1a245aea78bc20bc360e8315ad0, 00a4cb78d82b1f94f13032e9052eaa2cd453ee0a
- Tests: cargo test --profile ci -p telperion-render --lib the_spruce_at_62 -- --ignored (red without the room, green with), cargo test --profile ci -p telperion-render --lib every_tree_builds_on_the_device -- --ignored (spruce 60-80 at 6144; the walk frames, five presets and the Two Trees unchanged), cargo test --profile ci -p telperion-render --lib telperion_is_timed -- --ignored (CYCLE_TIMED telperion:82:42, norway-spruce:40:42)
- PRs: