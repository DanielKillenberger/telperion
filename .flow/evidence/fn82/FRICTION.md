# Friction reports, fn-82 (date palm as a real species)

Entries follow the friction rule in CLAUDE.md; the owner decides which become specs.

## 2026-09-18 23:32 discover needs a seed the skill does not name

- **Doing:** starting skill step 2, `discover` for `date-palm`.
- **Hindered by:** `species-pipeline discover` reads `DIR/manifest.json` and will not start without it, but `.claude/skills/add-species/SKILL.md` step 2 says "run discover, read its proposal, draft DIR/manifest.json" and never says to write the seed first. The seed shape (species, taxon, two fields, no sources) lives in `crates/telperion-jev/tests/pipeline_fixes.rs` as a comment on the 2026-09-18 ash run, and the admitted ash manifest is on another branch. This worktree's `DIR` held only an empty `driver-1.log`.
- **Cost:** about ten minutes of reading the discover stage, the ash test fixture, and `git show fn-56-european-ash-as-a-real-species:.../manifest.json` before any Firecrawl call.
- **What would remove it:** the skill's step 2 names the seed write, and a one-file seed template sits next to the runbook (species, taxon, the two growth fields, empty sources), so a cheap agent copies it instead of reconstructing it from a test.
- **Early return:** not taken; the seed is now written and discover can run.

## 2026-09-18 23:33 known sources and Wikipedia force a hand edit of the draft

- **Doing:** reading `discover.json` and drafting `DIR/manifest.json` from the proposal.
- **Hindered by:** each field's candidate list opened with 11 known hits (oak, spruce, fn-11 growth papers) that cannot be date-palm evidence. Jev then ranked Wikipedia first for `height_m`, which the species spec forbids as a citation. The research index for `dbh_m` returned avocado biochar, an insecticide paper, olive radiocarbon, and a Canary Island date palm page (`Phoenix canariensis`). The machine draft therefore named Wikipedia and the UA arboretum; a person still has to drop Wikipedia, skip the wrong taxon, and pick IFAS plus the Al-Madinah morphology paper by hand.
- **Cost:** the discover itself was 10.6 s; the hand edit was a few minutes of reading 42 hits. No extra credits.
- **What would remove it:** known sources filtered by taxon before ranking; a citation-policy prior so Wikipedia cannot be `ranked_first`; research queries that drop social posts and a sister species.
- **Early return:** not taken; the edit is the skill's draft step and the credit spend was already done.

