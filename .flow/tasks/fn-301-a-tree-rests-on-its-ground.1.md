---
satisfies: [R2]
---
# fn-301-a-tree-rests-on-its-ground.1 The device's frond curve is one function

## Description
Fold the device's copies of the frond parabola into one WGSL function that every module calls, with no change in output. This is the ground task .2 builds contact on. It is a behaviour-preserving refactor.

**Size:** M
**Files:** `crates/telperion-render/src/generation/rachis.wgsl` (new), `generation/pack.wgsl`, `generation/sources.rs`, `generation/resident/field.wgsl`, `generation/resident/field.rs`, `generation/resident/fronds.wgsl`, `generation/resident/boots.rs`
**Touches:** [crates/telperion-render/src/generation/rachis.wgsl, crates/telperion-render/src/generation/pack.wgsl, crates/telperion-render/src/generation/sources.rs, crates/telperion-render/src/generation/resident/field.wgsl, crates/telperion-render/src/generation/resident/field.rs, crates/telperion-render/src/generation/resident/fronds.wgsl, crates/telperion-render/src/generation/resident/boots.rs, crates/telperion-render/src/generation/resident/sweep.rs, crates/telperion-render/src/generation/resident/rings.rs]

### Approach
- The copies are `pack.wgsl:208-221` `on_rachis`, its verbatim twin `resident/field.wgsl:283-301`, and the inline point in `resident/fronds.wgsl:386` (`rachises`). The CPU's one function is `crates/telperion-core/src/pipeline/foliage/leaflet.rs:26` `rachis`.
- Modules compose by `format!` concatenation. Follow `generation/sources.rs:3-17` (`LEAF` prepended to `place`), `resident/field.rs:72` and `resident/boots.rs:44`. A new `rachis.wgsl` holds the point (and the tangent the leaflets read). Prepend it where each consumer is built. `pack.wgsl` already reaches sweep and rings (`resident/sweep.rs:18,39`, `resident/rings.rs:15`).
- The function takes its rows as parameters: length, arch, the heading and lift. No module's uniform layout leaks into it. Resolve name clashes (`fronds.wgsl:344` `frame`, `pack.wgsl`'s inline lift).
- Check `fronds.wgsl:381-383` `t = k/count·split` against the CPU's `leaflet.rs:39` `station()`, which `rachis.rs:67` uses. If they differ, record by how much, and take `station()`'s form only if the existing rachis tests stay inside their tolerances. Otherwise leave `t` as it is and note it for .2.
- If one module truly cannot take the shared source, say why in the commit, and add a test that holds the remaining copy equal on sample inputs.

### Investigation targets
**Required:**
- `crates/telperion-render/src/generation/pack.wgsl:175-248`
- `crates/telperion-render/src/generation/resident/fronds.wgsl:327-413`
- `crates/telperion-render/src/generation/resident/field.wgsl:280-301`
- `crates/telperion-render/src/generation/sources.rs`
**Optional:**
- `crates/telperion-render/src/generation/resident/rachis_tests.rs`, `agree_tests.rs:252-530`, `field_tests.rs:108-126`

### Key context
The equivalence harness: the device's drawn records, packed leaves and field for the palm at 12 and 40, seed 1, are bit-identical before and after. Use a scratch digest test run on the base and on the change, recorded in `.flow/evidence/fn-301-a-tree-rests-on-its-ground/`. Then the existing `rachis_tests`, `agree_tests` and `field_tests` pass. GPU tests run through `telperion-heavy 8 --gpu 2 --threads 2 -- ...`; never in a browser.

## Acceptance
- [ ] One WGSL function makes the frond curve's point (and tangent), and `pack.wgsl`, `field.wgsl` and `fronds.wgsl` all call it; or a named copy is held equal by a test, with the reason recorded.
- [ ] The palm at 12 and 40, seed 1: the device's drawn records, packed leaves and field digest are identical before and after (evidence recorded).
- [ ] `rachis_tests`, `agree_tests` and `field_tests` pass.
- [ ] The `station()` versus `k/count·split` comparison is recorded.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
