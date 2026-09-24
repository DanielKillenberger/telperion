# One species runner: set stages from a name to an accepted tree

## Conversation Evidence

> owner (2026-09-24): "We also want the simplest lightweight species adding runner that can take a species or a picture of a tree gather documentation, compile it into a catalog entry and start and finish the tuning and gap analysis quickly and produce specs to close those gaps. Differentiate between identity defining specs that are part of developing the species or global specs that apply to many/most species. Bloat has to be gone. Design has to be lean. How we now refactored the generator pipeline having simplified all to one path."
> owner (2026-09-24): "i agree with cleaning up first before merging to master. Photo input is a follow up spec"
> owner (2026-09-24): "don't finish until we agree on complexity is adequate and it's lean and elegant"

## Goal & Context
<!-- scope: business -->

The date palm run (fn-80, fn-82) proved the method: literature sets the starting tree, a reviewer compares renders with photographs, tuning moves dials, and a stall names a real generator gap. Four gaps made the palm identifiable (fn-108, fn-109, fn-110, fn-144). The machinery around the method cost more than the method. On the run's last day about 30 stops came up and four were real work; the host wrote 49 decision files by hand, and the three subsystems that carry a run hold about 34,000 lines. This spec does to the species runner what fn-102 did to the build: one path with set stages, each producing one artifact the next reads, and nothing that asks permission to do its job. The only stops left are a source claim a person must settle, an identity gap waiting on its spec, and the owner's eye. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

**What exists, checked 2026-09-24 on the fn-80 branch** (inventory with file:line in `.flow/evidence/fn80/RUNNER-INVENTORY.md`). [checked]
- `species-conductor step` plans one hop at a time over 13 ordered checks (`conductor/plan.rs:215`), spawns `species-pipeline <stage>` for 12 stages and `tuning-loop run` for a revision, and routes gap specs through a dependency state machine whose design and implementation dispatches Jev routes by a policy table (`conductor/dependency.rs:300`, `conductor/policy.rs`).
- A run can stop at 6 conductor pause sites, 5 continuation reasons, 6 tuning pause reasons and about 14 further "reassess or diagnose" stops, and 26 pipeline decision kinds.
- Duplicates: two reviewer comparisons (`tuning/sheet/`, `tuning/progress/`), two visual assessment paths (`vision::Adapter::assess`, `reference_first::assess`), four result writers, two route tables with the same design (`data/conductor-policy.json`, `data/gap-routes.json`), labelled-case scoring in three places, and Claude/Codex twin adapter scripts.
- The gap loop (`pipeline/gap/`, 2,636 lines) is run by a dispatched agent, never by the conductor, and `metrics.json`, which the packet requires, is written only by it.

**The runner.** One binary, `species <id>`, reruns only the stages whose inputs changed (a content hash of the input artifacts, never the build id), and each stage writes one artifact: [inferred]

1. **Sources.** Discover, fetch and self-admit sources by rights → `sources.json` and the fetch cache.
2. **Profile.** Extract, screen, select, verify and fit → `packet/profile.json` (values with their ranges) and `packet/references.json` (reference photographs). A contradicted or unsupported claim is the one decision a person settles here.
3. **Catalogue.** The article, source copies and README in `catalogue/<id>/`, as the catalogue scripts write them today.
4. **Capability.** The species' traits against the generator's vocabulary → `packet/capability.json`: expressed, missing identity traits, missing global traits.
5. **Start.** The profile's values mapped onto dials → the starting overlay (today's `conductor/derive.rs`).
6. **Tune.** Rounds over the live dials of the current tree (fn-148's gates): render, the reference-first reviewer against the photographs, proposals, keep or roll back. Each revision starts from the last kept tree. It stops when rounds stop keeping anything → `tuning/result.json` with the tree, its stills and the traits still failing.
7. **Gaps.** Each failing trait becomes one line in `gaps.md`: reachable (the dial and a two-value A/B render the runner made), an identity gap (a spec draft the species waits on) or a global gap (a spec draft for the backlog). The host reviews and mints; the runner never mints.
8. **Accept.** The owner looks in the harness; accepting writes the overlay into the preset as a value table and refreshes the catalogue pins.

**Kept, because they earned it this run.** The literature stages and their Jev selections; the reviewer's receipt with one repair and code trimming (fn-80, a4b3d056); the no-progress stop; the stage-idempotence records; the catalogue scripts. Jev's labelled sets stay as the tests that set its thresholds, never as runtime gates. [inferred]

**Removed.** The conductor's plan/step/state/dependency/policy/dispatch/adopt/handoff/questions/cases modules; the pipeline gap loop and its route table; pilot authority, priority re-approval, continuation, caps, preflight identity pauses and routing; the second reviewer path (`progress/`, `vision::Adapter::assess`) and the Codex twin scripts; three of the four result writers. A changed input reruns what it touched and never asks for a decision file. [inferred]

**Unknown.** The line count this lands at, which the implementer measures and reports; whether `veto/` and the contact sheet earn their keep, which the implementer decides from what they caught on the palm and the proof species. [unknown]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** `species <id>` runs the eight stages in order, each writing its artifact, and a second run with no changed input reruns nothing. [inferred]
- **R2:** A run stops only for a contradicted or unsupported claim, an identity gap and the owner's look; every other condition reruns or is logged. [inferred]
- **R3:** A revision starts from the last kept tree, reads the dial table live and offers only live dials; the render tools it uses are rebuilt from the same commit. [inferred]
- **R4:** `gaps.md` classes every failing trait as reachable, identity or global, each with its evidence, and the runner writes no spec. [inferred]
- **R5:** Accepting writes the preset and the catalogue pins with no hand-editing. [inferred]
- **R6:** The date palm reruns end to end from its recorded sources to the same accepted tree class, and one new proof species runs from a name to the owner's look; stops and tokens for each are reported against the palm's first run (about 30 stops on its last day, about 12M tuning tokens). [inferred]
- **R7:** The removed modules are gone, not disabled; the workspace gate is green; the lines before and after are reported. [inferred]

## Boundaries
<!-- scope: business -->

- Not photo input (fn-147), variation ranges (fn-146) or continuity gates beyond what R3 reads (fn-148). Not a generator change. The fn-80 stack is not merged to master; this spec's branch is cut from master after the palm ships and reuses the stack's code by rewrite, keeping what the removals above leave.

## Strategy Alignment

- Serves "The catalogue": every species by a lean, repeatable run. [strategy:The catalogue]
