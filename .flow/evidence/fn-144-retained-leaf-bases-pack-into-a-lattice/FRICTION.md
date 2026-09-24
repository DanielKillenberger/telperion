# fn-144 friction

## 2026-09-24 - R1 measure read a single-girth lattice as crowding

Doing: first green run of the R1 test at 32 bases. Hindered by: the palm's newest base sits in the apex internode, where the stem is 0.34 m against 0.48 m below, so a per-base cell (laid at each base's own girth) did not tile with its neighbours; the unwrapped-ring bug in the test itself (a 32-base cell spans more than half the trunk) hid it for one run. Cost: about 10 minutes, three test runs. Would have removed it: nothing missing in the repo; a note in fn-110's leaf_bases doc that girth steps at the apex internode would have pointed at it first.

## 2026-09-24 - the gate ran twice

Doing: the one end-of-task gate. Hindered by: 35 failures, most of them one cause: `skip_serializing_if` on the new `Tree::sections` field breaks bincode (not self-describing), so every cached specimen and snapshot failed to decode. The others were count pins (capability names, dial rows), limit-inventory entries for new loops and clamps, and one float32 build-only value. None of them runs in a targeted `cargo test -p telperion-core --test <file>` pass without knowing their names in advance. Cost: one extra gate of about 12 minutes. Would have removed it: a note in tree.rs or CLAUDE.md that `Tree` is bincode-cached (a new field must be `serde(skip)` or versioned), and a fast "pins" target that runs the count and inventory tests (capability, dial_table, tuning_engine counts, generation_limit_guard, family build-only) before the full gate.
