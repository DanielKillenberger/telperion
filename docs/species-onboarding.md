# Species onboarding

Start one existing Flow spec/workstream per species using [the fillable dispatch packet](../templates/species-profile.md). A species template holds botanical evidence, anatomy requirements and parameters; its specimens hold seeds and run receipts. Successful admission or generation does not establish botanical approval.

The [frozen fn19 contract](../.flow/evidence/fn19/PROTOCOL.md) defines species, case, reference and admission schemas. Keep workflow notes in the Markdown packet, outside closed manifest objects. Later inventories may grow without editing the initial oak/spruce protocol, cases, references or receipts.

## Stage handoffs

| Stage / owner | Inputs | Output and evidence gate | Next handoff |
|---|---|---|---|
| Research / species research agent | Stable taxon, maturity/context, source inventory | Attributed observations, rights, confidence, explicit missing scales; no invented measurements | Profile author |
| Profile / species profile agent | Research, dimension definitions and trait rubric | Species-owned hashed profile, references, target/estimate distinction; unique ID and normalized taxon tuple | Capability owner |
| Capability assessment / core coordinator | The habit, element and attachment trait values the species needs, and the ranges the generator accepts them in | One shared capability task with dependent species, or support receipt; a value the trait space cannot reach, or a structure no trait expresses, is unmet | Template implementer after dependencies |
| Template / species implementation agent | Frozen profile, resolved capabilities, assigned worktree | Full parameter family, explicit native preset mapping, parser checks; species-owned changes only | Registry/binding coordinator |
| Support integration / integration owner | Template commits and shared capability commit | Serial central registry/binding edits, support checks and existing-species regression checks | Specimen owner |
| Seed freeze and generation / specimen owner | Admitted manifests, audited seeds, source/tool/binary identity, resource window | Three-plus-three cases per species; fresh seeds persisted before inspection; immutable run directories retain failures | Numeric and visual owners |
| Numeric and visual validation / evidence owners | Same protocol/cases/parameters, frozen baseline conditions | Distribution receipts, matched whole/bare/base/fork/connected-shoot captures, framing/convergence checks and per-trait judgments remain distinct | Independent assessor and integration owner |
| Catalogue acceptance / integration owner | Engineering evidence, unresolved gaps, actual independent feedback | Recheck old species; publish supported scope with failures and expert status; close Flow tasks through ordinary review gates | Next cohort/workstream |

Appearance is part of the research a profile owes: the reference has to supply the bark's colour and roughness, the leaf's front and back colours, and the hue and brightness ranges a leaf of that species varies inside, because those are numeric fields of the family's material row and the renderer has nothing else to colour a tree by. Both ranges are symmetric offsets about no change, so a species whose foliage reads uniform states a zero-width range rather than leaving the field unsourced.

Research and profile ownership can be assigned to different agents or explicitly combined. Neither implies implementation ownership. Parallel species agents use separate worktrees, species directories and run outputs. They may research and author parameters together; they cannot promise independent simultaneous edits to `Preset`, `profile_id`, `from_id`, central bindings or catalogue registration.

Before dispatch, record a single integration owner for shared paths in each Flow task. If two species need the same missing organ primitive, create one core task and make both species tasks depend on it. Species workers provide requirements and tests to that owner; they do not implement competing copies. Merge capability work first, reconcile each template with that commit, then integrate registry/bindings serially. Re-run existing species support, parameter and relevant regression checks after shared integration. This handoff convention uses Flow, not another task database or scheduler.

Generation and captures use windows coordinated with fn13/fn18: owner, UTC interval, host, resource budget and lifecycle conditions. An unassigned/overlapping exclusive window blocks qualifying measurements. Contended costs may be labeled observations only. Never stop another session, shrink mature parameters, truncate foliage or reduce fidelity to obtain a pass. Cheap schema/support checks need no mature generation window.

## Executable examples

[Examples](../.flow/evidence/fn19/onboarding-examples/README.md) contain separate oak and spruce packets and an illustrative Scots pine extension. From the repository root:

```bash
cargo build --release -p telperion-core --example geometry_benchmark
python3 .flow/evidence/fn19/onboarding-examples/verify.py
```

This calls task 2's real manifest/schema/admission helpers and native support/parameter parser. It does not generate trees. Old protocol/reference hashes and the original twelve cases are checked. Oak/spruce are runnable but botanical validation remains unassessed here. Pine parses as an additional manifest in a later version and returns `unsupported-anatomy`, `implemented=false`; it is not admitted runnable catalogue content. Its borrowed parameter payload is a parser fixture only, never pine geometry.

For implemented future species, numeric runs use `target/release/examples/geometry_benchmark --protocol FILE --references FILE --output NEW_DIR`. Stills come from the renderer's headless target: `npm run species:qa` captures every required specimen whole, bare and as a single leaf, one process per still, and `cargo run --release -p telperion-render --example headless` renders one tree on demand. The fn19 browser capture rig retired with the Three.js renderer; see [the migration guide](../tests/migration/README.md#comparative-botanical-benchmark-fn19). Task 4 integrates comparison evidence after numeric/visual runs. Do not measure the illustrative pine cohort.

## Failed prerequisites and recovery

| Example | Disposition / recovery |
|---|---|
| Two packets claim one ID, or the same normalized scientific name/rank/cultivar | `duplicate-identity`; reconcile with catalogue owner before allocating cases. Do not rename the taxon to bypass uniqueness. |
| Reference ID absent | `invalid-manifest`; restore an attributed resolvable record. Missing/unreadable/hash-mismatched profile or no usable real evidence is `missing-evidence`. Explicit missing anatomical scales stay unassessed. |
| Pine requires paired needles in a fascicle | `unsupported-anatomy`; core capability task required. Single spruce needles do not satisfy it. The illustrative profile also lacks sufficient dimensions and per-scale anatomy evidence. |
| Fixed seed reused as holdout | `invalid-manifest`; freeze a new version with audited seeds. Previously inspected holdouts retain their historical role but are regressions for future tuning. Never redraw only failures. |
| Two agents claim one core file | `ownership-conflict`; assign one shared owner and dependent Flow tasks. Native admission does not enforce this; coordinator must gate dispatch. |
| Concurrent exclusive measurement claims | `resource-conflict`; coordinator assigns separate windows. Native admission alone cannot qualify timing. |
| Qualified independent feedback unavailable | `expert_status=unassessed`; engineering packet may ship with that limit, independent biological approval remains unresolved. No automatic external contact. |

Resume a failed task from its last committed profile/template in its own worktree. Preserve old failed/interrupted run directories and seeds. Record the cause and next dependency in its Flow task, use a new output directory for retry, and reuse only identity-verified complete artifacts. A seed conflict or changed protocol/reference/camera rules requires a new version. Another species' evidence and frozen cohort are read-only throughout recovery.
