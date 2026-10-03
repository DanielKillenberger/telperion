---
satisfies: [R2]
---
# fn-190-one-growth-law-species-are-points-in-a.5 The settings vocabulary (host, Astra review, owner sees it)

## Description
Stage B. The host merges the four model specifications into one vocabulary; this is a design decision (AGENTS.md, "Dispatch and escalation"), so a worker only assembles drafts the host edits.

**Size:** M
**Files:** `crates/telperion-render/examples/growth_law/params.rs`, `.flow/evidence/fn-190-one-growth-law-species-are-points-in-a/VOCABULARY.md`
**Touches:** [crates/telperion-render/examples/growth_law/params.rs, .flow/evidence/fn-190-one-growth-law-species-are-points-in-a/VOCABULARY.md]

### Approach
- Starts only after the beech task's recorded positive owner verdict (the early proof point), or the owner's authorisation of a revised approach.
- One table: setting, neutral value, range, source quote, models using it, engine feature behind it.
- Astra reviews against `docs/principles.md` (no switch, a neutral value, a source per setting); findings answered in the file.
- Settings that cannot vary by degree are listed as vocabulary gaps for the owner.
- Split the engine work into one serial task per missing feature (`flowctl task create`, deps on this task), each naming the models that need it.

### Investigation targets
**Required:**
- `.flow/evidence/fn-190-one-growth-law-species-are-points-in-a/MODEL-*.md`
- `crates/telperion-render/examples/growth_law/params.rs:7-172`
- `docs/principles.md`

## Acceptance
- [ ] `VOCABULARY.md` holds every setting with neutral value, range, source and users
- [ ] Astra's review is recorded and answered
- [ ] Vocabulary gaps are listed for the owner
- [ ] One engine task per missing feature exists, serial
- [ ] The owner has seen the vocabulary

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
