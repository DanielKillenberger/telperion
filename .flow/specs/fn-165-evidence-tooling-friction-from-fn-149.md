## Conversation Evidence

> owner (2026-09-26), on the host's proposal to spec fn-149's obvious friction fixes: "ok go ahead"
> owner (2026-09-26), on adding `species <id> --init-tuning`: "yes"

## Goal & Context
<!-- scope: business -->

fn-149's build recorded fifteen friction entries (`.flow/evidence/fn-149-one-species-runner-set-stages-from-a/FRICTION.md`). Four were fixed in the build, three are local setup on the owner's machine, and five are obvious fixes, joined by a sixth the host left out at first (a generated tuning config), whose cause the entry names and whose remedy changes no product behaviour. This spec carries those five, under the owner's rule of 2026-09-23 that the host specs and builds such fixes itself. Together they cost the build about 55 minutes and one extra gate run, and one of them can break the recorded replay with no gate going red. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

Checked 2026-09-26 on the fn-149 branch (`055a414e`) unless marked otherwise.

1. **The Jev ledger keeps what it hashed.** A ledger entry stores the question, the answer and `state_sha256` (`crates/telperion-jev/src/ledger.rs:43`), never the state, so diagnosing the beech's rights answers meant rebuilding their inputs from the fetch cache and the Commons API (about 15 minutes). The entry keeps the state it hashed, bounded in size, with the hash unchanged. [checked]
2. **The reviewer scripts' tests run in the gate.** `scripts/test-reviewers.py` covers the reviewer adapters and the tape adapter; neither `cargo test --profile ci --workspace` nor `npm test` runs it (no reference in `package.json` or the crates). A workspace test runs it, so a change to an adapter script cannot break the replay silently. [checked]
3. **The memory ceiling is measured alone.** The core species budget tests read the process peak (`VmHWM`, `crates/telperion-core/tests/species/budget.rs`), which the other test threads in the same binary share; four failed once at 3.6 GB against 2.16 GB with core unchanged and passed on the rerun. The ceiling is measured in a process of its own, or on the bytes the test already charges. [checked]
4. **A replay lists what it served.** The tape (`crates/telperion-jev/src/tape/`) keeps no record of which entries a replay read, so pruning the beech fixture needed `inotifywait`. A replay writes the served keys, and `tape_trim --check` or a small command reports entries no replay reads. [checked]
5. **One JSON form for catalogue records.** The catalogue's Node scripts write JSON in insertion order and the jev crate's `serde_json` sorts keys, so a record rewritten with the same values changed its bytes and staled a checksum; fn-149 worked around it by rewriting only on a value change. Both writers produce one canonical form. [inferred]
6. **A run needs only its seed.** A run from a name also needs a hand-written tuning config (`.flow/evidence/<id>/tuning.json`: required views and seeds, reviewer adapters, protocols, ledger and scratch paths); the beech's was rebuilt from the palm's and `tuning::live::Config` in about 15 minutes (fn-149 FRICTION, 2026-09-25). `species <id> --init-tuning` writes the default config for the seed's growth form, so a new species needs only its seed and the host's capability assessment. [checked]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A rights question's ledger entry can be read back with the state it was given, and its hash is unchanged. [inferred]
- **R2:** Breaking a reviewer script's test turns the workspace gate red. [inferred]
- **R3:** The budget tests pass with other test threads loaded, and still fail a real regression past the ceiling. [inferred]
- **R4:** Replaying the beech fixture reports its served keys, and an entry no replay reads is named. [inferred]
- **R5:** A catalogue record written by the Rust runner and by the Node scripts from the same values is byte-identical. [inferred]
- **R6:** The workspace gate and `npm test` are green. [inferred]
- **R7:** `species <id> --init-tuning` on a bare broadleaf seed writes a config the runner accepts through Start, and it never overwrites an existing config. [inferred]

## Boundaries
<!-- scope: business -->

- Not the three local setup entries (the dcg hook on the owner's machine), which are reported, never specced. Not the review-round cap or task sizing, which follow flow-next's defaults (owner, 2026-09-26). Builds on fn-149, so it starts after #121 merges.

## Strategy Alignment

- Serves "Minimalist af, efficient af and beautiful": the tooling around a run costs less than the run. [strategy:Our approach]
