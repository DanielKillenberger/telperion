---
satisfies: [R2]
---
# fn-291-the-cpu-entry-builds-every-shipped.2 R2a: free what the surface stage does not read and write the wood once; measure the floor

## Description
R2a and the plan's proof point. Lower the CPU entry's Wasm high-water mark without changing what is drawn: free what the surface stage does not read before it allocates, and write the wood's buffers once at final size. Then measure with task 1's probe and compute the floor R2b can reach. Byte-identical output is expected here (AGENTS.md "Generator evolution": prove it with an equality check on every preset's wood, native, serial and parallel).

**Size:** M
**Files:** `crates/telperion-core/src/pipeline/drawn.rs`, `crates/telperion-core/src/pipeline/surface/rings.rs`, `crates/telperion-core/src/pipeline/surface/build.rs`, `crates/telperion-core/src/pipeline/space.rs`, `crates/telperion-wasm/src/generate.rs`, `.flow/evidence/fn-291-the-cpu-entry-builds-every-shipped/PROBE.md`
**Touches:** [crates/telperion-core/src/pipeline/**, crates/telperion-wasm/src/**, .flow/evidence/fn-291-the-cpu-entry-builds-every-shipped/**]

### Approach
- Measure first: what is alive when `rings()` and `faces()` reserve on the wasm (serial) path. Use the traced-wasm method of `trace-reserved.patch` (Wasm pages at each reservation) and `woodsize.rs` (native live set). Record the numbers in PROBE.md before changing code.
- Candidates the scout named, each kept only if measured to lower the high-water mark:
  - The grown `Structure` and the grower's working state are dropped before `rings()` (`pipeline/space.rs:58-73`).
  - `edges` (40 B a node, `surface/rings.rs:88`) and anything else held in the stage's `OnceLock` (`drawn.rs:27,54`) is released as soon as its last reader is done.
  - Order the wood's largest reservations so they can reuse the freed growth region. Wasm memory never shrinks, and the allocator needs contiguous space.
  - No buffer grows or is copied after its exact reservation (`build.rs:112-113`, `copied()` at `build.rs:152`, `into_mesh`).
- Change serial and parallel paths together: `parallel/tests.rs:92` asserts their equality bit for bit.
- Floor: from the measured sizes, compute the live set at the wood's peak with R2b's per-vertex compaction applied: normals oct-encoded at 4 B a vertex, bark coords at 2×u16. Positions and indices stay as they are. Add the Wasm slack measured here. Write it in PROBE.md for Telperion seed 7 and Laurelin seed 42.
- **Stop condition:** if the floor is over 3.5 GiB (3,584 MiB), finish this task with its measured changes and report `NEEDS_HUMAN` with the gap. Changing indices or sides is the host's call, not this run's.

### Investigation targets
**Required**:
- `crates/telperion-core/src/pipeline/drawn.rs:20-160` — stage order on wasm32 (`a()` wood, then `b()` leaves)
- `crates/telperion-core/src/pipeline/surface/rings.rs:67-143`, `surface/build.rs:94-160`
- `crates/telperion-core/src/pipeline/space.rs:55-75` — growth and the `Structure`'s lifetime
- `crates/telperion-wasm/src/generate.rs:107-130` — what the entry keeps
- `.flow/evidence/fn-291-the-cpu-entry-builds-every-shipped/PROBE.md`, `trace-reserved.patch`, `woodsize.rs`
**Optional**:
- `crates/telperion-core/src/pipeline/surface/parallel/tests.rs:92`

### Key context
Wasm memory pages never return. A native counting allocator measures the live set, not the Wasm high-water mark. The bar is `engine.memoryBytes`.
## Acceptance
- [ ] PROBE.md records, before and after, the Wasm pages at each wood reservation and the native live set, for Telperion seeds 7 and 42 and Laurelin seeds 7 and 42
- [ ] Every preset's wood is bit-identical before and after (positions, normals, coords, indices; native serial and parallel), shown by a test or an equality check run
- [ ] The probe's high-water mark for the four failing cases and Telperion/Laurelin seed 1 is recorded after R2a
- [ ] PROBE.md states the floor with R2b's per-vertex compaction plus the measured slack; over 3,584 MiB means NEEDS_HUMAN with the gap
## Done summary
Blocked:
NEEDS_HUMAN: stop condition met (PROBE.md, "Gap (stop condition met)").

R2a is built and measured in 9d344192c: `fitted()` gives back growth's spare tree capacity, every preset's dist outputs are byte-identical, and the test went red, then green. It lowers no failing case's Wasm high-water mark.

R2b cannot reach 3.5 GiB:
- Its live floor for telperion is 3,551 MiB, and that holds only at zero slack.
- The 1,651 MiB index run cannot land in growth's fragmented 2.69 GiB, where no free run of 831 MiB exists. The high-water mark is therefore at least 4,344 MiB.

The levers are the index layout (a run table, or chunks) or growth's peak and fragmentation. Both are the host's call.

The codex impl-review is at its round cap of 2. Round 1 was NEEDS_WORK and fixed. Round 2 was NEEDS_WORK (the floor used the pre-R2a tree) and fixed in b9e4377fb. That fix is not re-reviewed, because flowctl escalated at the cap.
## Evidence
- Commits:
- Tests:
- PRs:
