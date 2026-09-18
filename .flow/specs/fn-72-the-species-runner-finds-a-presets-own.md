# The species runner finds a preset's own reference records

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 20% [user], 60% [paraphrase], 20% [inferred] -->

`node tests/species.mjs --quick european-beech` fails with `No matched reference records` because the runner's default profiles path is fn-9's, and the beech's and birch's records live in the fn-34 profiles; the agent has to know to pass `--profiles .flow/evidence/fn34/profiles.json` by hand. fn-55's build paid one failed pair of runs and about three minutes to find that out, and filed it as friction on 2026-09-18. [paraphrase]

The owner's rule is that a friction entry is specced before its spec closes so the next build does not pay it again. This spec makes the runner resolve a preset id to the profile set that carries its records, with the flag kept as an override. [user]

## Architecture & Data Models
<!-- scope: technical -->

- **One index from preset id to profile set.** A small table beside the profiles, or a scan of the known profile files for the preset's records, gives the runner the set to load when `--profiles` is absent. The fn-9 path stays the fallback for the oak and the spruce. [inferred]
- **The flag wins.** `--profiles` keeps its meaning; the resolution only fills the default. [paraphrase]
- **No pipeline change.** fn-58's pipeline consumes the runner as it is; this is the runner's own default. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** `node tests/species.mjs --quick european-beech` and `--quick silver-birch` run without `--profiles` and match the same reference records the fn-34 profiles carry. [paraphrase] Errors: a preset with records in no known profile set fails naming the preset and the sets searched.
- **R2:** `--profiles` still overrides the default, and the oak and the spruce resolve to the fn-9 set unchanged. [inferred] Errors: a resolution that changes the oak's or spruce's matched records fails naming the record.
- **R3:** A test covers the resolution for every shipped preset. [inferred] Errors: none beyond R1 and R2.

## Boundaries
<!-- scope: business -->

- No change to what a profile set contains or how a record matches. [inferred]
- No new species. [paraphrase]
