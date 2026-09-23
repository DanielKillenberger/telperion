---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-127-the-literature-step-reads-mature-sizes.1 Implement fn-127-the-literature-step-reads-mature-sizes

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
This task fixes the three defects from the palm's live run. Crown width and the leaf, needle, frond and leaflet sizes are now `mature` in the requirements table. A mature field needs no required age, and quality judges it with a new mature-size question set that has 14 labelled cases and a no-match answer. The appearance route records the source and span of the section Jev placed on a level, so verify's structural check holds. The conductor now pauses with a handoff on any open requirements-unmet or manifest-proposed decision, before the gap loop, the stages or tuning.

- R1: `tests/mature_sizes.rs` runs quality on A1's live screen rows. On the base it scored leaflet length `none`. It now scores leaflet length, crown width and frond length `partial`, with the ages `[50]` or with none at all. Height and dbh stay age-indexed and file requirements-unmet, as does leaflet width, since A1 states only the leaflet's length.
- R2: `tests/requirements.rs::every_appearance_value_carries_its_sources_id_and_verify_holds`. On the base the source was null. The test now requires every appearance entry to carry `source` S1 and a span, and verify to file no decision and no structural failure.
- R3: `tests/owner_first.rs`. On the base the conductor opened `dispatch-1` for the gate halt. It now pauses under `pause-owner-<hash>`, whose handoff lists the owner's decisions, and a second step adds nothing.
- R4: `data/questions/mature_size.json` and `data/cases/mature_size.json`: 14 cases, 4 held out, 4 negative. A1's leaflet and width sentences are among them, and the case runner scores the set.
- R5: `cargo test --profile ci --workspace --no-fail-fast` is green: 129 test result lines, 958 passed, 0 failed, 21 ignored.

Follow-ups for the host, none of them built here:
- `select` builds its document only from rows the screen called `measured_size_at_age` or `mature_size_range`. A1's leaflet sentence is `not_about_tree_size`, so a mature field that passes quality can still end up unavailable in select with "no admissible candidate span". Expect this on the palm rerun.
- Appearance spans now reach verify's citation check and its measurement obligation, which were left unchecked while the source was empty. The fixture test holds. A live section that carries numbers may list a claim (FRICTION.md).
- The palm rerun needs a manifest edit, such as dropping the mature fields' ages, to re-key quality. The tool version is unchanged.
- `crown_base_m` stays age-indexed, because the spec's list left it out.

stage: impl-review - skipped(config: REVIEW_MODE=none)
Tier: session (host-settled design)
## Evidence
- Commits: 28341393aa882013983965f77bd1639f52099d16
- Tests: cargo test --profile ci -p telperion-jev --test mature_sizes (red on base: leaflet_length_m none; green after), cargo test --profile ci -p telperion-jev --test requirements (R2 red on base: appearance source Null; green after), cargo test --profile ci -p telperion-jev --test owner_first (R3 red on base: dispatch-1 routine on cheap; green after), cargo test --profile ci -p telperion-jev --test sets, cargo test --profile ci --workspace --no-fail-fast (129 test result lines: 958 passed, 0 failed, 21 ignored), baseline: not run pre-edit (owner rule: the gate runs once, at the end of a task)
- PRs: