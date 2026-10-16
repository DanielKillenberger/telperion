---
satisfies: [R3]
---
# fn-276-the-device-builds-every-tree-the-cpu.2 A build's start leaves room for its first move

## Description
Host 2026-10-14 from SWEEP.md (archived: see .flow/evidence/ARCHIVE-MANIFEST.md, `git show 8dab10634:.flow/evidence/fn-276-the-device-builds-every-tree-the-cpu/SWEEP.md`): Sizes::margins starts at margin 1.0 whenever the starting tables fit, with no room for a later move that holds a group twice (the spruce at 62 refused, 63-68 built; 71 and 74-80 at the browser cap). The start margin is chosen so the starting tables plus their largest first move fit the budget.

## Acceptance
Red/green test on the spruce at 62; the spruce 60-80 at TELPERION_DEVICE_BUDGET_MIB=6144 with the card's cap reported per build; the sweep's other cases unchanged; cost measured.

## Done summary
A build's start leaves room for its first move: the start margin is the largest the budget holds beside the largest table group the first move makes again (`Sizes::first_move`, checked by `Ledger::fits`); same ladder, no species branch. The spruce at 62, seed 1: red without the room (4,203 of 4,096 MiB), green with it (`the_spruce_at_62_starts_with_room_to_move`). The spruce at 60-80, three seeds, at 6,144 MiB: all 63 build (cap 5,640-5,838 MiB). At native 4 GiB it builds to 72 except 69 at seed 42. Other 1,794 sweep cases unchanged; GPU time unchanged. Codex SHIP.

## Evidence
- Commits: 3fc6acf65bc8f22ee4d5150ed6a8cf631c6a8277, 615a153bf00bfd07c3901da14159f68eed2f7dbd, 73304a60db6ab1a245aea78bc20bc360e8315ad0, 00a4cb78d82b1f94f13032e9052eaa2cd453ee0a
- Tests: cargo test --profile ci -p telperion-render --lib the_spruce_at_62 -- --ignored (red without the room, green with), cargo test --profile ci -p telperion-render --lib every_tree_builds_on_the_device -- --ignored (spruce 60-80 at 6144; the walk frames, five presets and the Two Trees unchanged), cargo test --profile ci -p telperion-render --lib telperion_is_timed -- --ignored (CYCLE_TIMED telperion:82:42, norway-spruce:40:42)
- PRs: