---
satisfies: [R1]
---
# fn-292-the-foliages-level-lists-hold-only-what.1 Lay the foliage's level lists end to end, each level's start read by its draw

## Description
One list of `instances` words replaces one list per level; the scatter writes each leaf at its level's start (the sum of the lower levels' final counts) plus its offset, and each level's start word into a start buffer whose slots sit at `min_storage_buffer_offset_alignment`. The draw group binds the whole list and the start buffer with the dynamic offset; the foliage vertex shader reads `list[start + instance_index]`. Leaf order within a level is unchanged.

**Size:** M
**Files:** `crates/telperion-render/src/select.rs`, `crates/telperion-render/src/select/bind.rs`, `crates/telperion-render/src/select/frame.rs`, `crates/telperion-render/src/shaders/select.wgsl`, `crates/telperion-render/src/shaders/foliage.wgsl`, `crates/telperion-render/src/foliage.rs`, `crates/telperion-render/src/submit.rs`, tests
**Touches:** [crates/telperion-render/src/select.rs, crates/telperion-render/src/select/**, crates/telperion-render/src/shaders/select.wgsl, crates/telperion-render/src/shaders/foliage.wgsl, crates/telperion-render/src/shaders/shadow.wgsl, crates/telperion-render/src/foliage.rs, crates/telperion-render/src/submit.rs, crates/telperion-render/tests/submit.rs, crates/telperion-render/src/generation/resident/stalk_tests.rs]

### Approach
- `sizes()` (`select.rs:65-76`): lists = align(instances × 4); a new start buffer = levels × alignment; scratch unchanged. Keep the buffer labels ("foliage level lists", "foliage selection scratch") and add one for the starts so the ledger names it.
- `scatter` in `select.wgsl`: start = sum of `counts[0..level)` (at most 16 reads; counts are final after `prefix`); the leaf with index 0 writes every level's start slot. Keep `leaf_index` / row folding for > 65,535 groups.
- `select/bind.rs`: the draw layout's list binding loses `has_dynamic_offset` and its `stride` window; a new start binding takes the dynamic offset. `bind_level` passes `alignment × level`; the leaf view (`bind_leaf`) and the shadow pass (`foliage.rs` `draw_shadow`, which shares the layout and reads only placements) bind slot 0 / a valid offset.
- `frame::Uniforms.stride` and foliage `frame.stride` (`select.rs:~420-428`): repurpose or remove; check both consumers.
- `submit::fits` (`submit.rs:60-120`) judges the new sizes by the same names.
- `chosen()` / `counted` test helpers (`select.rs:~485-499`) read each level from its start (read the start buffer back).

### Investigation targets
**Required** (read before coding):
- `crates/telperion-render/src/select.rs` — sizes, setup, reset, frame encode, bind_level, chosen
- `crates/telperion-render/src/shaders/select.wgsl` — select / prefix / scatter
- `crates/telperion-render/src/select/bind.rs` — compute and draw layouts
- `crates/telperion-render/src/shaders/foliage.wgsl:12,49-54` — the list read
- `crates/telperion-render/src/foliage.rs:~320-356` — draws and shadow

**Optional**:
- `crates/telperion-render/src/select/tests.rs` — compaction tests to extend
- `crates/telperion-render/tests/submit.rs:~168-195` — admission names

### Key context
- WebGPU core has no indirect first instance; the start must reach the shader through a binding.
- The unseen bucket is counter index `levels`; it has a scratch slot and no list.
- The renderer's still is the proof of identity: render the base and the change at the same views and compare pixels (task 2 does every preset).

## Acceptance
- [ ] Lists sized `align(instances × 4)` once, not per level; a start buffer of `levels` aligned slots; `select/tests.rs`'s size test and `tests/submit.rs`'s admission cases updated to the new sizes and names.
- [ ] Every existing selection test passes reading each level from its start (`compaction_preserves_placement_order_at_every_level`, the > 65,535-group crown, the forced level, inflorescences, thin rachis).
- [ ] New test: a frame where one level gets no leaves, its start equals the next level's and the other levels' lists are unchanged; and a forced level starts at 0.
- [ ] The shadow pass and the leaf view draw with the new layout (`tests/shadow.rs`, `tests/thinning.rs`, `tests/conformance.rs`, `tests/walk.rs` green).
- [ ] Focused crate tests green; no full gate here (task 2).

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
