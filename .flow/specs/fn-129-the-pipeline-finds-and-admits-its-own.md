# The pipeline finds and admits its own sources

## Goal

Twice on 2026-09-23 the palm's run stopped for the owner to admit sources. Admission was the owner's because someone had to confirm a source's rights and because "admitting a manifest is never routine" kept a cheap agent from admitting junk. Both can be met by the pipeline with guards, so the owner is asked only when the pipeline has run out of sources. Owner (2026-09-23): "why can only i admit sources? the pipeline should be able to do this"

## Decisions

Host design (2026-09-23):
- Rights by code, classified by Jev: code fetches licence evidence; Jev classifies `open-licence`, `public-cite-only`, `restricted` or no-match. The first two are admitted with the class recorded (numbers cited, no text reproduced); the rest are not.
- Relevance by Jev: admitted only if the ranking judged the source to state its field.
- The pipeline resolves `manifest-proposed` (`by: pipeline`) only for a sources-only draft whose sources all pass; drafts changing fields, bars, appearance traits or the schema version (v1 -> v2) go to the owner.
- `requirements-unmet` `add-sources` is taken by the pipeline: up to two `search-again` rounds with a gap-aimed query, then the owner gets the decision with the sources tried. Bars never lowered.
- Appearance traits (added after the first implementation, R6): searched again the same way; adding an admitted id to a trait's `sources` is a sources-only change; renaming a trait or changing its level table stays the owner's.
- A PMC article's open-access record is Europe PMC's `license` field (the PMC OA service answers 404).

## Acceptance Criteria

- **R1:** A draft of open/public sources stating their field is admitted by the pipeline with rights classes; restricted or unclassifiable goes to the owner — met.
- **R2:** A draft changing a field, bar or appearance trait goes to the owner — met.
- **R3:** An unmet field is searched again twice at most; two empty rounds hand the owner the decision — met.
- **R4:** Rights classes have labelled cases (palm F1, A1, M1, P4-P7) and a no-match answer — met (16 cases from real fetches, scored at the 0.9 bar).
- **R5:** Gate green — met (994 passed).
- **R6:** Unmet appearance traits searched again like fields; owner keeps trait shape changes — met.

## Result

Landed in `461a8c2c`, `a9baf10d`, `93169d74` (`pipeline/admission.rs`, `pipeline/search/`, `data/questions/rights.json`); carried by the fn-149 rewrite (#121). A round ended by an adapter or Jev error still counts, so the search cannot loop.

## Open

None.
