---
satisfies: [R6]
---
# fn-304-the-gate-runs-in-ten-minutes-and-stays.3 Walks merged, wasm flower check folded, device field at one grid

## Description
Rulings 6, 3 and 4: the walks binary (369 test-seconds, its two big tests each build the walks under nextest), the wasm flower binding (128 s, three builds per preset) and the device field (92 s).

**Size:** M
**Files:** `crates/telperion-space/tests/walks.rs`, `crates/telperion-wasm/src/generate/tests.rs`, `crates/telperion-render/src/generation/resident/field_tests.rs`, RESULT.md
**Touches:** [crates/telperion-space/tests/walks.rs, crates/telperion-wasm/src/generate/tests.rs, crates/telperion-render/src/generation/resident/field_tests.rs, .flow/evidence/fn-304-the-gate-runs-in-ten-minutes-and-stays/**]

### Approach
- walks (`walks.rs:19-33`, `:143-181`): one test calls `every_walk()` once and runs `judge` then the vanishing-size body; every assertion kept; no walk narrowed. Check `docs/tree-space.md:52` still reads right. Prove each half red on fn-271's patches (insertion switch, `lineage.rs`; presence window at full size, `contract/presence.rs`).
- wasm (`generate/tests.rs:125-147`, `:168-209`): the main-entry test, which already generates every preset with surface and foliage, asserts slots 27-29 empty and `organReference` null for non-flowering presets; the flower test builds and checks only the flowering presets (telperion, laurelin) and keeps its `flowering == [...]` claim from the main-entry pass or its own. Prove the folded assertion red (a temporary organ slot written for a non-flowering preset).
- Device field (`field_tests.rs:191-207`): the gate runs one grid size (36, unless measurement favours another) at seeds 1 and 5678; an ignored test runs `[24, 36, 48]`. Its reason must not start with `device limits:` or `release check:` (`docs/gpu-tests.md:13-14`). Red proof: a temporary per-cell offset in the device read-out at all grids. fn-302 and fn-303 edit `generation/` in parallel: touch only this test.

### Investigation targets
**Required:**
- `crates/telperion-space/tests/walks.rs`
- `crates/telperion-space/tests/walk/mod.rs:22-59`
- `crates/telperion-wasm/src/generate/tests.rs:120-210`
- `crates/telperion-render/src/generation/resident/field_tests.rs:130-210`
- `docs/gpu-tests.md:1-40`
## Acceptance
- [ ] walks: one merged test growing the walks once, both halves red on their patches.
- [ ] wasm: no non-flowering preset is built twice across the two tests; the flowering presets keep their own extra builds (pipeline reference, foliage and surface-only requests); folded assertion red on its patch.
- [ ] Device field: one grid at both seeds in the gate, all grids ignored, red proof recorded.
- [ ] Gate run once at the end; codex impl review.
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
