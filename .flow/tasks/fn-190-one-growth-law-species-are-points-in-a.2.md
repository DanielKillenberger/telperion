---
satisfies: [R1]
---
# fn-190-one-growth-law-species-are-points-in-a.2 Oak (Rauh): model specification

## Description
Stage A for the oak, reading only, in parallel with the beech (.1), spruce and palm.

**Size:** S
**Files:** `.flow/evidence/fn-190-one-growth-law-species-are-points-in-a/MODEL-OAK.md`
**Touches:** [.flow/evidence/fn-190-one-growth-law-species-are-points-in-a/MODEL-OAK.md]

### Approach
- Rules of Rauh's model for *Quercus* from LITERATURE.md (catalogue table, §1) and the source texts in the research worktree's `.firecrawl/lit/` (`hot_ch3.md` for *Q. rubra*, `key.md` or the K19 source, `halle.md`, `bc2007.md`), each quoted: rhythmic growth, orthotropic branches equivalent to the trunk, acrotonic clusters at the top of each annual shoot, forks from the terminal bud cluster, reiteration in the mature crown.
- Map each rule to proposed vocabulary settings with values (see `crates/telperion-render/examples/growth_law/params.rs` for today's settings); name the engine features missing.
- List three to five traits visible in `.flow/references/oak-quercus-robur/` (S1, S2, S3) the owner will judge.

### Investigation targets
**Required:**
- `.flow/evidence/fn-188-one-branching-law-from-trunk-to-twig/LITERATURE.md`
- `crates/telperion-render/examples/growth_law/params.rs`
- `.flow/references/oak-quercus-robur/README.md`

## Acceptance
- [ ] `MODEL-OAK.md` lists every Rauh rule for the oak with its quote, proposed settings and values, and missing engine features
- [ ] Unsourced rules are marked unsourced
- [ ] Three to five reference-visible traits are listed with the photograph each is read from
- [ ] No code changed

## Done summary
MODEL-OAK.md written: ten Rauh rules quoted and mapped to settings, unsourced items listed (clustering at shoot tops, forks from the terminal bud cluster), four missing engine features (polycyclism, apical control by age, reiteration without tip death, shade death), five traits, four design questions for the host (chiefly the low fork).

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: a71017012dd06eba8a703ecd3d1a9d3bfa4ae0ba
- Tests:
- PRs: