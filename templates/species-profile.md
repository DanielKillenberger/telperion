# Species packet: <stable-id>

Copy into a species-owned evidence directory. Replace every `<...>` before handoff; mark unknown facts `unavailable` with a reason. This packet is a botanical template and work assignment, not a generated specimen or a second status tracker. Task status lives in the linked Flow tasks.

## Dispatch contract

- Flow spec/task: <existing IDs>; research owner: <agent/task>; profile owner: <agent/task>; implementation owner: <agent/task>.
- Worktree / pinned base commit: <absolute path> / <SHA>.
- Own writes: <species-only paths>; read-only inputs: <paths + hashes>.
- Shared integration owner/task: <owner/ID>; dependencies: <core capability task IDs>.
- Stop and hand back evidence if: identity conflicts, attribution is missing, required anatomy is unsupported, or another task owns a required write path. Do not copy another taxon's preset and rename it.
- Deliver: this packet, `profile.json`, `references.json`, `species.json`, `specimens.json`, source commits and validation receipts. Return paths, unresolved prerequisites and next owner.

## Species evidence — research/profile owners

| Field | Value |
|---|---|
| Stable ID / scientific name / rank / cultivar | <lowercase-id> / <exact name> / <rank> / <null or cultivar> |
| Target age / context / exclusions | <years if known, otherwise maturity class; habitat, light, climate, leaf state; excluded cultivars/forms> |
| Profile path / SHA-256 | <repository-relative path> / <exact file-byte hash> |
| Preset mapping / implementation verified at | <explicit ID or unsupported> / <source commit and support receipt> |

Use `profile.json` with `{"schema_version":1,"definitions":{...},"profiles":[{"id":"<stable-id>","scientific_name":"<name>",...}]}`. Preserve source observations separately from engineering choices.

| Dimension or distribution | Unit + operational definition | Target or null | Source IDs | Confidence | Gating / contextual / unavailable |
|---|---|---|---|---|---|
| Height, DBH and its plane | <m; actual bounds / model proxy> | <range> | <IDs> | <level> | <classification> |
| Crown width/base | <m; retained foliage bounds> | <range/null> | <IDs> | <level> | <classification> |
| Organ length/width and count | <single blade/needle; exclude connector> | <range/null> | <IDs> | <level> | <classification> |
| Axis/order/taper/angle and foliage bins | <frozen definition IDs> | <range/null> | <IDs> | <level> | <classification> |

For each reference record, fill the frozen `reference` schema: ID, species ID, source ID, kind, anatomical scales, matching disposition, URL, attribution, context, dimensions/age or null, usage rights, access date, asset hash or null, observation and limitations. Inventory whole, bare, base, fork and **connected** shoot separately. A page description is not an inspected image; a missing scale stays unassessed.

| Trait | Botanical observation + source | Required capability | Existing support or owning core task | Per-trait evidence / disposition |
|---|---|---|---|---|
| Crown and scaffold habit | <...> | <...> | <...> | <unassessed + reason> |
| Base/fork continuity and taper | <...> | <...> | <...> | <unassessed + reason> |
| Organ identity and attachment | <blade/needle/fascicle; arrangement and connector> | <...> | <...> | <unassessed + reason> |
| Surface detail / rendering artifacts | <separate observations> | <...> | <...> | <unassessed + reason> |

## Template implementation — implementation owner

`species.json` follows `protocol.schema.$defs.species`: `id`, `scientific_name`, `taxon_rank`, `cultivar`, `context`, `profile_id`, `profile_path`, `profile_sha256`, `preset`, `required_capabilities`, **complete** `parameters`, `fixed_seeds`, `holdout_seeds`, `reference_ids`. Do not add workflow keys to that closed schema.

- Parameter snapshot path/hash: <full family, no implicit current defaults>.
- Parameter choices and source/engineering rationale: <trait → field → reason>.
- Native support and parameter-parser receipts: <paths>; unmet anatomy: <capabilities>.
- Shared capability: <one owner/task, affected files, dependent species, integration commit>.
- Registry/binding handoff: <coordinator and proposed mapping; workers do not concurrently edit central registration>.

## Specimens and validation — separate from the template

- Benchmark/reference version and exact hashes: <...>; immutable previous cohort: <path/hash>.
- `specimens.json`: explicit case IDs, species/parameter IDs, u32 seeds and roles; at least three fixed + three distinct audited holdouts. Resolve a full parameter copy with only `skeleton.seed` replaced.
- Seed audit: <scope, hashes, concurrent reservations, draw time, source commit, exclusions>; inspection history: <fresh at freeze / now regression>. Never replace a failed seed.
- Resource window: <fn13/fn18 coordinator, UTC start/end, host, CPU/GPU/memory budget, exclusive or observation-only>; no window means no qualifying timing run.
- Generation output: <new immutable directory>; source/tool/binary hashes: <...>; all case failures: <...>.
- Numeric receipt / capture receipt / frozen conditions: <paths/hashes>.
- Visual trait findings: <supported/contradicted/unassessed + evidence IDs and author>.
- Independent reviewer: <role, expertise, relation, original feedback or unavailable>; expert disposition: <unassessed until supplied>.
- Integration recheck of existing species: <commands/receipts>; unresolved gates and next Flow task: <...>.
