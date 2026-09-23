# A dial's range is its validated bound; the preset span is a stride, not a wall

## Conversation Evidence

> host, fn-80 `GAPS.md` proposal 1, 2026-09-22: "Tuning loop: dial ranges from validated bounds, the preset span as a stride only."
> user (2026-09-23): "i approve 1 & 2"

## Goal & Context
<!-- scope: business -->

The date palm's first tuning revision (fn-80, seven rounds, about 1.64 M tokens) could not reach a date palm's proportions. A palm leaflet is 20 to 40 cm; the dial table stops `leaf_length` at 0.171 and `leaf_width` at 0.11175 because those rows take their range from the span the shipped presets occupy. The same basis capped `trunk_radius` at 0.023 and `leaf_size` at 1.575. A species whose proportions lie outside every shipped preset cannot be tuned toward itself, and every new species is such a case sooner or later. [paraphrase]

This spec makes a dial's walls the generator's validated bounds, and keeps the preset span as information about where botanical values usually sit. [inferred]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, checked 2026-09-23 on the fn-80 branch.** `crates/telperion-jev/data/dials.json` holds 219 rows: 135 `validated bound`, 75 `preset span`, 9 `authored` (`range_basis`, documented at `tuning/actions.rs:41`). `min` and `max` are hard: `Dial::value` refuses a value outside them (`tuning/actions.rs`) and the bundle's `step` clamps to them (`tuning/bundle.rs:83`). [checked]
- **Unknown.** Whether every one of the 75 `preset span` rows has a validated bound in the generator to replace it with (a doc-commented clamp or a `validate` check in `telperion-core`), or whether some parameters are unbounded by the generator and would need one authored. The row's `source` names the file each range was read off; the audit is part of the work. [unknown]
- **Shape.** A row's `min`/`max` become the validated bound; the preset span stays on the row as a separate, non-limiting field the loop may read (a stride hint, a prior for the reviewer's "usual" range), never as a clamp. A parameter with no validated bound gets one in the generator, with a test, before its row is widened; a row without either stays capped and says so. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** No dial-table row takes its `min` or `max` from the preset span. Each of the former 75 rows reads a validated bound with its `source`, or stays capped and records why. Errors: a row widened past what the generator validates fails. [inferred]
- **R2:** The preset span is kept on each row it came from as a non-limiting field, and a test shows a move past it is accepted where the validated bound allows. [inferred]
- **R3:** The date palm's leaflet length and width, trunk radius and leaf size can reach the palm's measured values (leaflet 20 to 40 cm), shown by a table test on those rows. No capture. [inferred]
- **R4:** The gate is green: `cargo test --profile ci --workspace --no-fail-fast`. [paraphrase]

## Boundaries

- No change to how far a round moves; that is the stride spec approved beside this one. [paraphrase]
- No generator behaviour change except adding a missing bound, each named in the done summary. [inferred]
- No tuning run here; fn-80's next revision exercises it. [inferred]

## Decision Context

- The owner approved this from fn-80's gap list on 2026-09-23, with the stride proposal; proposals 3 to 5 were not approved. [user]

## Open Questions

- Unknown until the audit: how many of the 75 rows lack a generator bound.
