---
title: Higher fidelity at lower cost
date: "2026-09-21"
focus_hint: Higher fidelity at lower cost
volume: 22
survivor_count: 3
rejected_count: 19
rejection_rate: 0.86
artifact_id: higher-fidelity-at-lower-cost-2026-09-21
promoted_ideas: []
status: active
---

## Focus

Higher fidelity at lower cost. The search is not limited to another generation-speed win in the class of the recent engine work. Open specs are excluded. Survivors have to rest on a measurement or a named gap already recorded in the repo.

## Grounding snapshot

focus_kind: concept

git_log_30d: fn-91 evidence archive dominates the file list; source commits are fast generation, species presets, the twelve-byte leaf, and PR format
  - crates and specs around generation, foliage, and the species catalogue

open_specs: 43
  fidelity-open: fn-4 intersections and blunt ends; fn-10 legendary traits; fn-15 wind; fn-16 damage; fn-20 woody anatomy; fn-21 developmental shoots; fn-30 calibrated growth; fn-33 flowers cones compound leaves; fn-41 surface lighting; fn-59 bend under leaves; fn-60 leaf margins; fn-61 troll pitch; fn-64 leaf outlines; fn-66 birch fork ring; fn-90 bark flakes
  cost-open: fn-53 caps; fn-91 fast generation (oak browser still about 22ms over 10x; CPU-output reported separately); fn-100 field from the plan not placed leaves; fn-101 slim wasm; fn-102 one pipeline, byte-identical, no algorithm change
  product-open: fn-17 engine integration; fn-28 scroll growth; fn-56 ash; fn-62 beech; fn-67 growth parity; fn-68 tuning; fn-75 ash fixes; fn-77 registry; fn-78 test cost; fn-79 spruce fixture; fn-80 gap loop; fn-82 date palm; fn-83 jev in CI; fn-89 conductor; fn-92 device crash; fn-93 dirty tests; fn-94 to fn-98 studio; fn-99 archive fn-91 evidence
done_relevant: fn-1 instanced canopy; fn-13 distance detail; fn-23 frame budget; fn-50 short shoots; fn-52 leaf mass

changelog_recent: scanned: none (no CHANGELOG.md)

memory_matches: 5
  - [knowledge/decisions] The far draw is the near draw minus what the eye cannot resolve — tags: lod, filtering, bark, fn-71
  - [bug/runtime-errors] Zero width is not no constraint — tags: envelope, grower
  - [knowledge/decisions] Tuning loops are code; Jev routes a verdict — tags: tuning, jev, fn-68

memory_audit_stale: scanned: none (audit not run)

strategy:
  name: Telperion
  last_updated: 2026-09-20
  tracks: Growth and botanical fidelity; The catalogue; The core and integration; Surface and rendering at scale
measured_tags:
  - spruce cpu-output warm median 4516ms and 4308ms after parallel wood (1.24x); oak cpu-output 477ms and 645ms
  - browser completed frames about 179-204ms oak and 179-195ms spruce
  - birch crown-shell cull reads every leaf vertex and costs 3.1s of a 3.3s build; segment cull was deferred and not specced
  - deferred, unspecced: CPU expander over stations; GPU station path as renderer default; station form for short shoots
  - field-only wasm: oak 1.0s/96MB, birch 3.5s/32MB, spruce 8.9s/668MB; skeleton 124/163/58ms (owned by open fn-100)
  - full wasm 1242383 bytes raw (owned by open fn-101)

## Survivors

### High leverage (1-3)

#### 1. Cull foliage by segment before expanding vertices
**Summary:** Birch crown cull spends 3.1s of a 3.3s build reading every leaf vertex; reject a whole station segment first.
**Leverage:** Small-diff lever because the crown cull already walks station segments and the 3.1s is the per-vertex test inside that walk; impact lands on the birch build and every preset whose shell cull dominates.
**Size:** M
**Affected areas:** crates/telperion-core/src/foliage
**Risk notes:** A segment test can keep a leaf the current vertex test would drop, so the shell has to stay conservative.
**Persona:** senior-maintainer
**Next step:** /flow-next:refine

#### 2. Station descriptor for short shoots
**Summary:** Beech interior leaves exist only as CPU placements; a station form lets GPU and field consumers clothe that wood.
**Leverage:** Small-diff lever because short shoots are already placements on solved wood and only lack the station record the GPU path and the field already consume; impact lands on beech interior foliage for every consumer that skips the CPU mesh.
**Size:** M
**Affected areas:** crates/telperion-core/src/foliage
**Risk notes:** A short shoot is a cluster of 1 to 8 leaves, not a run of stations, so a segment encoding can mis-count it.
**Persona:** senior-maintainer
**Next step:** /flow-next:refine

### Worth considering (4-7)

#### 4. Renderer default uses the GPU station path
**Summary:** The renderer still builds the CPU mesh by default while qualified GPU delivery is already 179-204ms on oak.
**Leverage:** Small-diff lever because qualified GPU delivery already completes the oak frame in 179-204ms and the default is one renderer call; impact lands on every viewer that still pays the CPU mesh, including the 4.3-4.5s spruce output.
**Size:** S
**Affected areas:** crates/telperion-render
**Risk notes:** fn-91 is still open on the same delivery path, and CPU-output tests without a device still need the mesh.
**Persona:** senior-maintainer
**Next step:** /flow-next:refine

### If you have the time (8+)

_(none)_

## Rejected

- Engine export is one needle mesh plus instances — out-of-scope-vs-strategy: fn-1 already ships an instanced culled canopy, and the strategy points engines at stations or a field.
- Darken recorded limb contacts in the shader — duplicates-open-epic: fn-4 already owns limb intersections, and a darkening would mask that defect.
- Fork collar from the two radii on every species — duplicates-open-epic: fn-20 and fn-66 already own fork junction anatomy.
- Move skeleton growth onto a GPU compute shader — too-large: A whole-generator GPU rewrite targets 51-56ms of advance while foliage and field builds consume seconds.
- Open the Unreal integration now — duplicates-open-epic: fn-17 is already the open spec for the first external engine.
- Cache wood-ring sine and cosine — insufficient-signal: That cache already shipped inside fn-91.
- Delete the CPU leaf placer — backward-incompat: fn-102 keeps the CPU leaf placer for the growth timeline and for families with no station form.
- Homepage serves a curated seed cache — out-of-scope: A curated seed list is forbidden as a substitute for generation.
- Restore Three.js pixel parity — backward-incompat: The owner dropped pixel parity and the Three.js path was removed.
- Rebuild only wood that changed this month — duplicates-open-epic: fn-28, fn-30, and fn-67 already cover growth over time.
- Directional overlapping bark flakes — duplicates-open-epic: fn-90 is the open spec for that look.
- Qualify phone generation at 100ms — duplicates-open-epic: fn-91 already requires phone evidence and the 100ms stretch.
- Emit voxel cubes from telperion-core — out-of-scope-vs-strategy: Voxel meshing stays in the consumer; the core answers occupancy.
- Replace Jev with a local judge — out-of-scope-vs-strategy: The catalogue track routes species claims through Jev.
- Repair limb crossings with mesh CSG — duplicates-open-epic: fn-4 owns intersection topology on the skeleton.
- Add wind as a second skeleton solver — duplicates-open-epic: fn-15 is the open wind spec.
- Smooth every leaf outline in the mesh — duplicates-open-epic: fn-60 and fn-64 already own leaf outlines.
- Ship the studio dial page — duplicates-open-epic: fn-96 is that page.
- One function for the copied build chains — duplicates-open-epic: fn-102 is that consolidation.
