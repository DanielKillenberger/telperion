# The pipeline finds and admits its own sources

## Conversation Evidence

> user (2026-09-23), asked to admit a source for the palm's height and trunk diameter: "why can only i admit sources? the pipeline should be able to do this"

## Goal & Context
<!-- scope: business -->

Twice on 2026-09-23 the palm's run stopped for the owner to admit sources: the version 2 manifest's seven, then a study of height and trunk diameter by age that the host found for the two fields still unmet. Admission was the owner's for two reasons: someone had to confirm a source's rights before its numbers reach a preset, and "admitting a manifest is never routine" kept a cheap agent from admitting junk. Both can be met by the pipeline with guards, so the owner is asked only when the pipeline has run out of sources. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, checked 2026-09-23 on the fn-80 branch.**
  - `discover` (`pipeline/stages/discover.rs`) lists known sources, then searches the web and the research index with a plain query per field (`plain_query(taxon, field, condition)`), and Jev ranks the hits; its draft marks a new source's rights "unstated: a person confirms the rights before admission" (the palm's P4 to P7). [checked]
  - `data/conductor-policy.json` makes `manifest-proposed` admission and `requirements-unmet` the owner's ("its sources are the owner's to add"). [checked]
- **The design (host, 2026-09-23).** [host design]
  - *Rights by code, classified by Jev.* Code fetches each proposed source's licence evidence (a CC licence in the page's metadata or text, a PMC or DOAJ open-access record, a public extension or arboretum page); Jev classifies it among `open-licence`, `public-cite-only`, `restricted` and no-match. `open-licence` and `public-cite-only` are admitted with their class recorded; the numbers are cited and no text is reproduced, as for every source today. `restricted` and no-match are not admitted.
  - *Relevance by Jev.* A proposed source is admitted only if the ranking judged it to state the field it was found for.
  - *Admission by the pipeline.* `manifest-proposed` is resolved `admit` by the pipeline (`by: pipeline`) when every source in the draft passes both checks and nothing else in the manifest changed but its sources; a draft that changes fields, bars or appearance traits still goes to the owner.
  - *A requirement unmet searches again.* `requirements-unmet`'s `add-sources` is taken by the pipeline: it reruns `discover` with a query aimed at the field's dominant gap (for `no_age_indexed_points`, the field at stated ages), admits what passes, and reruns the stages. After two such rounds with no newly admitted source for a field, the decision goes to the owner (NEEDS_HUMAN) with the sources tried.
  - The bars stay the requirements table's; nothing here lowers one.

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A draft whose new sources all carry an open licence or a public cite-only page and state their field is admitted by the pipeline and recorded with each source's rights class; one with a restricted or unclassifiable source goes to the owner. [inferred]
- **R2:** A draft that changes a field, a bar or an appearance trait goes to the owner. [inferred]
- **R3:** A `requirements-unmet` field is searched again with a gap-aimed query, twice at most; a newly admitted source reruns the stages; two empty rounds hand the owner the decision with the sources tried. [inferred]
- **R4:** The rights classes have labelled cases (the palm's F1, A1, M1, P4 to P7 among them) and a no-match answer. [inferred]
- **R5:** The gate is green: `cargo test --profile ci --workspace --no-fail-fast`. [paraphrase]

## Boundaries

- No bar lowered, no manifest field changed by the pipeline. [inferred]
- Text of a source is never reproduced; numbers are cited. [paraphrase]

## Decision Context

- The owner asked on 2026-09-23 that the pipeline admit sources itself. [user]

## Open Questions

- None.
