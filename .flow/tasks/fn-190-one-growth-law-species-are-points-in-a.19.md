---
satisfies: [R3, R4]
---
# fn-190-one-growth-law-species-are-points-in-a.19 Probe tooling: age stills, walk stills and the palm's organs

## Description
Tooling the point and walk tasks need and may not edit themselves: stills at a chosen age, still strips for walks, and the palm's retained leaf bases reaching expansion. Serial after the beech task, which owns the probe files until then.

**Size:** M
**Files:** `crates/telperion-render/examples/growth_law/main.rs`, `tree.rs`, `raw/run.sh`, `raw/sheet*.sh`; the smallest exposure in `crates/telperion-core` that lets the probe clothe leaf bases
**Touches:** [crates/telperion-render/examples/growth_law/main.rs, crates/telperion-render/examples/growth_law/tree.rs, .flow/evidence/fn-190-one-growth-law-species-are-points-in-a/raw/*.sh, crates/telperion-core/src/pipeline/branching/leaf_bases.rs, crates/telperion-core/src/pipeline.rs, crates/telperion-core/src/pipeline/contract.rs, crates/telperion-core/src/pipeline/executor.rs]

### Approach
- Age: a grow argument that stops at a given year, so sheets show 5, 10, 20 years and mature.
- Walk: render each step's tree with the same interpolated settings and site as its metric row; apply the collapsed-tree and below-ground errors to each step.
- Palm organs: production adds retained leaf bases in `pipeline::skeleton` through `clothe_leaf_bases` before expansion; `tree::to_tree` produces none. Expose that step to the example with the narrowest visibility and verify an organ-bearing tree reaches expansion without the old stem generator.

#- Framing: the `today` and grow stills frame the camera on the stem's bounds, which crops the palm's frond crown (fn-190.4, Q4); frame every still on the whole tree including foliage, and re-render the palm bar (`raw/palm-today/`).

## Investigation targets
**Required:**
- `crates/telperion-render/examples/growth_law/main.rs:150-281`
- `crates/telperion-core/src/pipeline/branching/leaf_bases.rs`
- `crates/telperion-render/examples/growth_law/tree.rs`

## Acceptance
- [ ] Stills at a chosen age work for every model
- [ ] Walks write a still per step from the same trees as their metric rows, with the error checks applied
- [ ] A palm tree from the probe carries retained leaf bases through expansion, shown on a still
- [ ] Production behaviour is unchanged (the exposure only widens visibility)

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
