---
satisfies: [R6]
---
# fn-304-the-gate-runs-in-ten-minutes-and-stays.1 Core lib: repeat test subset, packed_leaf merged, row bounds spread

## Description
Rulings 7, 8 and 9 in the core lib, the binary with the most test-seconds (660 under nextest). Proves the subset-and-patch method first (Early proof point).

**Size:** M
**Files:** `crates/telperion-core/src/pipeline/space/repeat_tests.rs`, `crates/telperion-core/src/suite/packed_leaf.rs`, `crates/telperion-core/src/family/tests.rs`, `.flow/evidence/fn-304-the-gate-runs-in-ten-minutes-and-stays/RESULT.md`
**Touches:** [crates/telperion-core/src/pipeline/space/repeat_tests.rs, crates/telperion-core/src/suite/packed_leaf.rs, crates/telperion-core/src/family/tests.rs, .flow/evidence/fn-304-the-gate-runs-in-ten-minutes-and-stays/**]

### Approach
- Repeat test (`repeat_tests.rs:56-70`): move the per-preset body into one helper; the gate test runs it for `telperion` and `date-palm` only (ids from `presets.rs:25-38`); a new ignored test runs every `CATALOGUE` preset, reason naming fn-304. Keep the print-sensitivity precondition call (`:57`) in the gate test.
- Red proof for ruling 7: a temporary, deterministic reorder in a pipeline stage (for example reversing an output order on the one-core path only) that the full catalogue sweep catches; show the two-preset subset also red. Never a random HashMap order (gap analysis c5).
- packed_leaf (`packed_leaf.rs:29-105`): one test, one build per preset, both assertions. Test B's reference comes from the same mesh test A builds (`x.mesh()`); check `mesh::build` and the `executor::grow` path agree on `foliage.instances.reference` before relying on it (A already asserts the reference). Keep B's telperion/laurelin/date-palm skip. Prove each assertion red on its own patch.
- `every_row_past_its_bound_is_refused_on_every_family` (`family/tests.rs:81-93`): spread families with `std::thread::scope` and `resume_unwind`, the fn-271 idiom at `suite/sweep.rs:333-337`. It holds 11 MB, so memory is no concern.
- Patches are never committed; `git diff --stat crates/*/src` shows only test files before the gate.

### Investigation targets
**Required:**
- `crates/telperion-core/src/pipeline/space/repeat_tests.rs:50-110`
- `crates/telperion-core/src/suite/packed_leaf.rs`
- `crates/telperion-core/src/family/tests.rs:38-93`
- `crates/telperion-core/src/suite/sweep.rs:325-390` (spread idiom)

**Optional:**
- `crates/telperion-core/src/pipeline/space/radii_tests.rs:20` (ignore reason with spec id and cost)

### Key context
Under `cargo test` these tests' times include the one-tree turn wait; read their own times under nextest (R1.md). Every heavy run goes through `telperion-heavy 8`.

### Acceptance
- [ ] Repeat test: gate subset is telperion and date-palm; full catalogue ignored with an fn-304 reason; both red on the deterministic reorder patch, green on the engine (RESULT.md).
- [ ] packed_leaf: one test, one mesh per preset, both assertions, each red on its own patch.
- [ ] Row-bound test spread over threads, same assertion, own time measured before and after under nextest.
- [ ] Gate run once at the end; codex impl review.

## Acceptance
- [ ] TBD

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
