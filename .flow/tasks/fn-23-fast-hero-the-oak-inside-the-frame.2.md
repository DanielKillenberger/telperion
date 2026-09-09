---
satisfies: [R3]
---
# fn-23-fast-hero-the-oak-inside-the-frame.2 Levels in the core: sections chosen by deviation, and the identity proofs

## Description
Give the core's foliage element its nested levels (R3, spec Architecture first bullet and API Contracts first bullet) and prove the tree is unchanged. Levels are chosen from the element's existing transverse sections by outline deviation, one rule for every anatomy; the finest level is byte-identical to today's index list. No renderer work here.

**Size:** M
**Files:** `crates/telperion-core/src/foliage/levels.rs` (new), `crates/telperion-core/src/foliage/element.rs`, `crates/telperion-core/src/foliage.rs`, `crates/telperion-core/src/mesh.rs` (only if the element's type surface changes), `crates/telperion-core/tests/foliage.rs`, `crates/telperion-core/src/branching/audit.rs` or a new `crates/telperion-core/tests/identity.rs`
**Touches:** [crates/telperion-core/src/foliage/**, crates/telperion-core/src/foliage.rs, crates/telperion-core/src/mesh.rs, crates/telperion-core/tests/**, crates/telperion-core/src/branching/audit.rs]

### Approach
- Add `levels: Vec<Level>` to the element, each `Level { indices: Range<u32>, deviation: f64 }` addressing one shared index buffer, ordered coarsest first; the last level's range is the whole of today's `indices`, unchanged bytes. Keep `positions` shared by every level.
- Build levels in a new `levels.rs` from `AnatomyGeometry.sections` (rings base to tip, see `element.rs:207-333`): the kept set always contains the first section, the last section and the section of greatest lateral extent; then, at tolerance t, greedily keep the section whose vertices deviate most from the surface between its kept neighbours until every dropped vertex is within t. Halve t per level from a starting tolerance that yields the fewest sections down to zero (all sections). Rebuild triangles between consecutive kept sections with the same lateral topology the element uses between adjacent sections (`element.rs:256-297` for blades, :211-252 for needles, `build_element` at :111-205 for the generic grid). Record t as the level's deviation.
- The generic blade path (`build_element`) must also expose sections so it gets levels through the same rule; if it does not today, add them there rather than special-casing.
- The connector's sections are candidates like any other; its tiny extent means the tolerance drops it early. Do not exempt it.
- Validation (`Element::validate`) rejects a level list whose last range is not the full index list or whose deviations do not strictly decrease.
- Identity tests, no device: for oak and spruce at seed 7, hash the placement matrices and the skeleton with the FNV pattern at `crates/telperion-core/src/branching/audit.rs:96-123` and pin the literal hashes; assert `mesh::build` counts and bounds against literal values taken before this task; assert level 0 has fewer triangles than the finest, every level's vertex set is a subset of the next finer, the finest indices equal the pre-task indices byte for byte, and for every level every dropped section vertex lies within the level's deviation of the coarse surface.

### Investigation targets
**Required** (read before coding):
- `crates/telperion-core/src/foliage/element.rs:207-333` — `build_anatomy`, sections for needle and blade
- `crates/telperion-core/src/foliage/element.rs:111-205` — `build_element`, the generic grid
- `crates/telperion-core/src/foliage/element.rs:355-383` — the connector
- `crates/telperion-core/src/mesh.rs:72-103` — the mesh call whose output must not change
- `crates/telperion-core/src/branching/audit.rs:96-123` — the hash-pin test pattern

**Optional** (reference as needed):
- `crates/telperion-core/src/foliage.rs:41-53` — `Instances` layout
- `crates/telperion-core/src/presets.rs:85-95` and `:118-125` — oak and spruce element parameters

### Key context
- Capture the pre-task literal counts, bounds and hashes in a first commit before touching the element, so the pins are honest.
- Files stay under about 400 lines; `element.rs` is the reason `levels.rs` is new.
- Widest-section retention is the difference between a coarse leaf and a sliver; it is not optional.

## Acceptance
- [ ] The element exposes an ordered level list, coarsest first, each with an index range and a deviation in metres; the last level's range is the full index list and its bytes equal the pre-task indices
- [ ] Oak at default parameters yields at least four levels, the coarsest under 12 triangles; spruce yields at least two; the generic blade yields at least two
- [ ] Every level keeps the first, last and widest sections; every dropped section vertex lies within the level's deviation of the coarse surface, asserted in a test over all levels of all three anatomies
- [ ] Level vertex sets are nested and deviations strictly decrease; validation rejects a violating list with a named error
- [ ] Placement and skeleton hashes for oak and spruce at seed 7, and `mesh::build` wood vertex count, wood triangle count, instance count and bounds, are pinned as literals recorded before this task and pass after it
- [ ] `cargo test --release -p telperion-core` passes with no device
- [ ] `mesh::build` signature and `TreeMesh` fields are unchanged; the wasm binding's metadata pin test still passes

## Done summary
The foliage element now carries nested levels chosen by outline deviation, and
the tree it belongs to is pinned as unchanged. A level is a subset of the
element's transverse sections: base, tip and widest are never dropped, the rest
are added one at a time - always the section furthest from the surface spanned
between its kept neighbours - until nothing dropped lies outside the tolerance;
halving the tolerance gives the next level down. Oak gets 13 levels from 4 to
268 triangles, spruce 4 from 14 to 56, the generic grid 3 from 4 to 16.

Two deviations from the approach as written, both deliberate.

The coarse triangles live in a new `level_indices` buffer rather than being
appended to `Element::indices`. The renderer sums its foliage normals over the
whole of `element.indices` and draws all of it (`telperion-render/src/foliage.rs:28`
and `:195`), and that crate is outside this task's Touches, so extending the
element's own index list would have silently changed the rendered normals and
stacked every level into one draw. `level_indices` holds the coarse levels
followed by a copy of today's list, so the finest level's range is those bytes
exactly and nothing existing moved. The duplicate costs 3.2 KB on the oak.

A level's triangles are built by moving every vertex to its nearest kept section
and keeping the element's own triangles that survive, rather than by rebuilding
strips per anatomy. The lateral topology is then the element's by construction
instead of by imitation, and there is genuinely one code path for the lobed
blade, the four-sided needle and the generic grid. The generic grid records its
sections internally to reach that path; it does not gain a public
`AnatomyGeometry`, because `anatomy.is_some()` is what the wasm binding and the
species metrics read as "species anatomy is proven", and flipping it would have
moved output R3 requires unmoved.

The connector falls out of that same rule with no exemption: its vertices sit
nearest the base section, so they collapse onto it and every triangle it owns
dies degenerate. That is what puts the coarsest oak level at 4 triangles rather
than 20. The consequence worth naming is that the connector is absent from every
level but the finest, including fine levels whose tolerance is far below its
12 mm - it returns only at deviation zero. The deviation guarantee the tests
assert is over section vertices, which the connector is not.

Identity holds. `tests/identity.rs` was written and committed before the element
was touched, pinning oak and spruce at seed 7: wood vertex and triangle counts,
instance count, mesh bounds, and FNV hashes of the skeleton, the placement
matrices and the element itself. All eight literals per species pass unchanged
after the levels exist, and `mesh.rs` was not edited at all.

For task 3: the oak's 13 levels are more than the spec's arithmetic assumed
("five lists of 555 thousand indices are 11 MB" becomes about 29 MB and 13
indirect draws). The fine tail buys little - level 11 is 250 triangles against
the finest 268 - so a budget that stops emitting once a level is within some
share of the finest belongs in the selection work, where the memory cost is
known. Also note `telperion_core::foliage::Level` now shares a name with
`telperion_render`'s `Level::{Full, Quad}` enum from task 1; task 3 replaces the
latter.

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: f19af78dd153cefdce6a6644b50c043115ae1a87, d556b54d8e2b2aa5c533a240c4345f4237658542
- Tests: cargo test --release --workspace (113 passed, 0 failed, 29 suites), cargo test --release -p telperion-core, cargo fmt --check, cargo clippy --release --workspace --all-targets (no warnings)
- PRs: