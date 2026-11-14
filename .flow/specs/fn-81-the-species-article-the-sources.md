# The species article: the sources distilled, every claim cited

## Goal
The owner wanted to read a species the way a person reads an encyclopedia entry and wanted an agent to reach a source's text without fetching it again. fn-35's species page showed only the bibliography, images and notes. This spec adds to each `catalogue/<id>/` a markdown copy of every admitted source (with its original link) and an `ARTICLE.md` distilled from the sources and packet with a citation on every claim, and makes the onboarding loop fill both every time.

## Decisions
- > user (2026-09-18): "but i can't browse all the info like a human would. i thought we'd copy all the sources also to be able to have them available at an agent's finger tips?"
- > user (2026-09-18): "or at least build the profile in more detail from the sources"
- > user (2026-09-18): "kinda like grokipedia"
- > user (2026-09-18): "that's the better option"
- > user (2026-09-18): "that also needs a loop adjustment to always fill this profile documentation yea?"
- > user (2026-09-18): "should probably distill the essence from the sources but always link to sources. Keep a copy of the sources in .md format to reference to with original link there also. Makes sense?"
- The repository is public, so the source's `rights` field chooses the copy's shape: a full copy only on an explicit open licence or public domain, otherwise the cited passages as quotations; unclassifiable means extract. This amends fn-58's "source bytes stay outside the repository" with the owner's word. No PDF, raw HTML or image bytes committed.
- Numbers are copied by code from the packet, never written by a model. A written article cannot be byte-deterministic, so it is gated on staleness: its front matter records the sha256 of every input and the check fails a moved input.
- `ARTICLE.md` and a copy per admitted source are required files in the structure check; a pipeline `document` stage after packet verification writes them and re-runs fn-57's citation check.

## Acceptance Criteria
- **R1:** Every admitted source has a markdown copy with link, title, attribution, rights, fetch date and checksum; full or extract by rights — met (22 copies: 4 full, 16 extracts, 2 recorded unavailable).
- **R2:** Every species has an `ARTICLE.md` with a citation on every claim and every measurement copied from the packet — met (five articles).
- **R3:** fn-57's citation check runs over every article claim; an unsupported claim becomes a decision — partial: mechanism built and tested, not run over the five backfilled articles (~150 calls; no pipeline fetch records for pre-pipeline species).
- **R4:** An article records its inputs' checksums and the check fails a moved input — met.
- **R5:** A `document` stage runs in the pipeline; the add-species route and both docs name it; the files are required and backfilled — met.
- **R6:** The species page links the article and each source copy; `node scripts/catalogue-article.mjs --species <id>` writes an article end to end — met.

## Result
Commits 8bf062de, 5d65a79b, 9b19d32f. The articles state what the sources would not support rather than smoothing it: KEW-ASH carries no morphology though cited for height and leaflet count; O1 and OSU-ASH are the same page under two ids; NCSU-SPRUCE's two descriptions disagree on secondary branchlet hang; ATKINSON-BIRCH is behind JSTOR and TSO-BIRCH's url now 404s, leaving the birch's crown ratio and both gating blade dimensions on text no run can reach.

## Open
- Whether an article is rewritten wholesale or per section when one input moves.
- Whether rights classification should read a licence identifier rather than the rights sentence.

## Friction
- No fetch cache survives a run, so all 22 sources were re-fetched (the whole budget) -> proposed a durable per-source store, or `document` running in the same run as `fetch`.
- A `verified` date can point at an unreachable or non-supporting source -> proposed `verified` requiring a stored checksum and citation checks at admission.
- Shared tooling changed under four parallel writers, forcing re-runs -> lesson: freeze the contract before fanning out.
