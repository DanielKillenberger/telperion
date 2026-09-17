## Conversation Evidence

> user (2026-09-14, on the tree set for fn-10): "Mallorn (Lothlórien), White Tree of Gondor, Yggdrasil (Norse ash)"
> user (2026-09-15, on closing fn-34): "yes move ash to its own spec"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 30% [user], 50% [paraphrase], 20% [inferred] -->

European ash (Fraxinus excelsior) joins the catalogue the way the oak, the spruce, the beech and the birch did: a frozen botanical profile with cited ranges, catalogued references, a preset that is a value table, the fixed and fresh seed protocol, numeric gates, and matched pairs the owner judges. It is the natural base of Yggdrasil in fn-10. [paraphrase]

It was one of fn-34's three species. Its profile, its references and its closed-schema species record were gathered there on 2026-09-14 and are complete; its template could not be written, because an ash's leaf is pinnately compound and the generator draws only simple blades until fn-33 delivers compound leaves. The owner moved it here so fn-34 can close on the beech and the birch. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **The evidence is already gathered.** `.flow/evidence/fn34/european-ash/` holds `profile.json`, `references.json` and `species.json` from fn-34; the reference photographs are under the ignored `.refs/fn34/european-ash/`. This spec adds a shot block to each whole, bare and base reference so it is judged on matched pairs the way the beech and the birch are (fn-36's rig). [paraphrase]
- **The template waits for fn-33.** The ash's preset uses fn-33's compound-leaf rows: an ash leaf is one placement and its leaflets are instances, with leaflet count 7 to 13 and whole-leaf length 0.20 to 0.30 m gated against the profile and the metrics reporting placements, instances and units per instance. A simple-blade ash is not a pass. [user]
- **Everything the beech and the birch taught.** The fn-34 rounds are the method: values first against the matched pairs with the quick look (`npm run species:quick`), a generator gap becomes its own spec, the implementer does visual QA before the owner sees anything. The ash starts from the fn-34 generator: pendulous rows, clumps, outline irregularity, short shoots, canopy lighting, smooth bark. [paraphrase]
- **Catalogue identity.** `european-ash` selectable natively, through wasm and from the browser, every identity site updated as fn-34 did for the other two. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The ash's profile, references and species record from fn-34 are carried over and each whole, bare and base reference carries a shot block. Errors: a reference whose photograph hash does not match its record is refused. [paraphrase]
- **R2:** The ash ships as one named preset that is a value table using fn-33's compound-leaf rows, with leaflet count and leaf length gated against the profile and the metrics reporting placements, instances and units per instance, selectable natively, through wasm and from the browser. Errors: an unknown id fails naming it; a simple-blade ash is not a pass. [user]
- **R3:** The fixed and fresh seed protocol passes the ash's gating ranges with per-seed discrepancies retained, the identity pins and sweep bands exist, and the same seed and parameters yield a byte-identical tree. Errors: a numeric failure is a retained case, never a resample. [paraphrase]
- **R4:** The ash's matched pairs are rendered, the implementer does visual QA before returning, and the owner judges and records the verdict in this spec. Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker. [user]

## Boundaries
<!-- scope: business -->

- No Yggdrasil; fn-10's. No compound-leaf engine work; fn-33's. [paraphrase]

## Decision Context

- Split from fn-34 on 2026-09-15 at the owner's word, so fn-34 closes on the beech and the birch without waiting for fn-33. [user]

## Resolved via Codebase

- The ash's gathered evidence: `.flow/evidence/fn34/european-ash/` (profile, references, species record) and fn-34's REPORT.md "Ash" section.
- fn-34's R4, which this spec takes over: `.flow/specs/fn-34-beech-ash-and-birch-as-real-species.md`.
- Compound leaves: `.flow/specs/fn-33-flowers-cones-and-compound-leaves-as.md`.
