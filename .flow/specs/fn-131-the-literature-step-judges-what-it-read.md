# The literature step judges what it read, and never passes a gap silently

## Conversation Evidence

> host audit of the literature step against the palm's live run, 2026-09-23 (items 3 to 7, 9 and 10), after the host's convergence guard; companion to fn-130, which fixes what the stages read.
> user (2026-09-23): "yes pls fix the frictions if they're obvious."

## Goal & Context
<!-- scope: business -->

Once the stages read the whole source (fn-130), the judging and gating after it still lose correct values, accept wrong ones and let a required field go missing without a stop. This spec makes screen, select, appearance, verify and quality agree on what a source says and stop, or act, on every gap. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, checked by the host's audit on 2026-09-23 on the fn-80 branch (artifacts under `.flow/evidence/date-palm/pipeline/`).** [checked]
  - *Line numbers re-checked on this branch (above fn-130 and fn-129) by the worker, 2026-09-23.*
  - *Screen and select classes.* `data/questions/screen.json` puts cones and needles under `not_about_tree_size`; `stages/select.rs:135` keeps only `measured_size_at_age` and `mature_size_range`; P4's, P5's and A1's leaflet and frond sentences were `not_about_tree_size`; `sprout_cultivar_or_nursery` drops M1's frond sizes.
  - *Select.* One document for every field, the field's terms ignored (`select.rs:131-144`); spans deduped as bare strings and credited to the first row containing them (`src/extract.rs:110-120`, `select.rs:163-172`: frond length "20 feet" credited to A1's growth-rate sentence); no confidence floor (`select.rs:157`, picks at 0.34 and 0.36 accepted); `parse_span` (`select.rs:194-231`) misses glued units ("6–10m", so crown width dropped), `mm`, thousands separators, em-dash ranges, and reads "in" the preposition as a unit.
  - *Appearance and sufficiency levels.* An expected score rounded over unordered categories with `unstated` last (`sets.rs:138-144`, `appearance.rs:214-221`, `quality.rs:70-71`): `leaf_back_colour` became `silvery_white` at P=0, `leaf_brightness_range` `strongly_varied` at P=0.
  - *Verify.* The measurement check is asked without its field (`verify.rs:208-217`); `excerpt_for` falls back to the document start (`verify.rs:295`); claim decisions are keyed by source (`verify.rs:77`) so two values of one source share one; `accept`, `replace-source`, `drop-value` are consumed by no stage (`consume.rs:31-40`) and not routine (`conductor-policy.json`); the appearance claim is its own sentence, so it always supports.
  - *Retirement.* A person's resolution with changed inputs reopens a superseded decision (`decision.rs:219-244`; `verify/claim-unsupported/A1` open with a void 2026-09-22 resolution).
  - *Age.* Only table rows carry ages (`quality.rs:257`, `:289-294`); `measured_points` needs `open_grown` (`:270`) while screen left 78 of 92 rows `unstated`; A1's "reaching 5 m (20 feet) in 15 to 20 years" cannot count.
  - *Gate gaps.* A field quality passed and select could not fill files nothing (`select.rs:76-78`; crown width); quality's `mature()` counts rows select drops (`quality.rs:169-183`); field terms match substrings ("width" pulls leaflet sentences, "leaf" matches "leaflet"); a `proxy_only` bar passes with gap `no_mature_size`.
  - *Keys.* A stage's idempotence key carries the crate version and landed gap fixes, not its code (`stage.rs:292-297`); fn-128's fix left three stages `current` until the host moved their records aside.
- **Shape.** [host design]
  - *Organ sizes are classes.* Screen gains organ-size classes (leaf, leaflet, frond, needle, cone) and a cultivar class that counts toward a species' mature size; select reads the rows quality counted, per field, with whole-word terms.
  - *Select is exact.* One document per field; spans keyed to their sentence and source; a confidence floor from labelled cases; one number and unit grammar shared with the extractor.
  - *Levels are choices.* Appearance and sufficiency levels are the most probable level above a floor, never an average; `unstated` wins when it is most probable.
  - *Verify acts.* The measurement check names its field; claims are keyed by pointer; `drop-value` removes the value and re-files the field's `requirements-unmet` when it was required; `replace-source` goes to fn-129's search; both are routine; the appearance claim checks the level against the sentence.
  - *Retired stays retired.* A superseded decision ignores a stale resolution.
  - *Ages from sentences.* Code parses a stated age or age range from a candidate sentence into a point; `unstated` condition counts at a lower weight than `open_grown`.
  - *No silent gap.* A required field select cannot fill files `requirements-unmet`; quality counts only rows select can use; a mature field fails on `no_mature_size`.
  - *Keys carry the code.* A stage's key includes the pipeline's build identity, so changed code reruns its stages.

## Acceptance Criteria
<!-- scope: both -->

- **R1:** Red tests on the palm's rows: P4's, P5's and A1's leaflet and frond sentences reach select; M1's cultivar frond sizes count; crown width's "6–10m" parses. [inferred]
- **R2:** Frond length is credited to F1's frond sentence as a range; a pick below the calibrated floor is not filled. [inferred]
- **R3:** `leaf_back_colour` and `leaf_brightness_range` take their most probable level or `unstated`, never a zero-probability level. [inferred]
- **R4:** A flagged value dropped by `drop-value` leaves the packet and re-files its requirement; claims are per pointer; the appearance claim can fail. [inferred]
- **R5:** A superseded decision stays resolved under a stale resolution. [inferred]
- **R6:** A1's "reaching 5 m in 15 to 20 years" is a height point at 15 to 20 years. [inferred]
- **R7:** An unfilled required field files `requirements-unmet`; `no_mature_size` fails a mature field. [inferred]
- **R8:** A code change to a stage expires its key. [inferred]
- **R9:** New question classes and floors have labelled cases and a no-match answer. [inferred]
- **R10:** The gate is green: `cargo test --profile ci --workspace --no-fail-fast`. [paraphrase]

## Boundaries

- What the stages read is fn-130's. Source admission is fn-129's. [inferred]
- No live run here; the host reruns the palm. [inferred]

## Decision Context

- One of two specs the host's whole-step review produced (fn-80 ECONOMICS.md guard). [paraphrase]

## Open Questions

- None.

## Settled

Closed as landed (2026-09-26, fn-149 R8): carried by the fn-149 runner rewrite, merged in #121 (c2ac430b).
