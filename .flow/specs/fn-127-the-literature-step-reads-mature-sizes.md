# The literature step reads mature sizes, keeps appearance sources, and stops first

## Conversation Evidence

> fn-80 live run, 2026-09-23: the palm's first pass through fn-118's requirements (seven admitted sources, manifest version 2) filed seven `requirements-unmet` decisions and four `structural-unmet` on appearance, then the conductor went on to the gap loop.
> user (2026-09-23): "yes pls fix the frictions if they're obvious."

## Goal & Context
<!-- scope: business -->

The palm's sources hold the data the new requirements ask for: A1 states "The leaflets are ½ m (18 inches) long", fronds "5-7 m (15 to 20 feet) long", "**Width:** 20 - 50 feet" and "reaching 5 m (20 feet) in 15 to 20 years". The literature step still scored leaflet length, leaflet width and crown width `none` and height and trunk diameter `proxy_only`, all with dominant gap `no_age_indexed_points`; it copied four appearance values with no source, which `verify` then refused; and the conductor went past the owner's stop to the gap loop. The stop the owner asked for is right; these three defects made it fire on data the run had. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, checked 2026-09-23 on the fn-80 branch (the stack's merge).**
  - `.flow/evidence/date-palm/pipeline/quality.json`: `crown_width_m`, `leaflet_length_m`, `leaflet_width_m` level `none`, `height_m`, `dbh_m` `proxy_only`, every one `no_age_indexed_points`; `frond_length_m` passed at `proxy_only` from A1's "20 feet". `extract.json` holds A1's leaflet, frond, width and height-at-age sentences. [checked]
  - `manifest::validate` requires a non-empty `required_ages_years` on every field; the host gave the mature sizes `[50.0]` to pass (fn-80 FRICTION.md). The sufficiency question set judges sizes at stated ages. [checked]
  - `decisions.json`: four `structural-unmet` on `/profiles/0/appearance/{bark_roughness,leaf_back_colour,leaf_front_colour,leaf_hue_range}`, payload `{"rule":"source id resolves","source":""}`. [checked]
  - The conductor's next action after the stages was `gap_loop` on the capability gate while seven owner-only `requirements-unmet` decisions were open. [checked]
- **Shape.** [host design]
  - *Mature sizes.* The requirements table marks a field `mature` (crown width, frond, leaf, needle and leaflet sizes; not height or trunk diameter). A mature field needs no required age; its sufficiency is judged by a mature-size question set (a stated mature value or range for the taxon, at its bar), with the same no-match answer and labelled cases before its threshold is trusted.
  - *Appearance sources.* `select` records the chosen span's source id on every appearance value it copies; a value with no source is not written.
  - *Owner first.* The conductor acts on an open owner-only decision (`requirements-unmet`, `manifest-proposed`) before the gap loop, the stages or tuning, and pauses with its handoff.

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A red test on A1's sentences reproduces `none` for leaflet length and crown width before the fix; after it, a `mature` field with no age reaches `proxy_only` or better from a stated mature size. [inferred]
- **R2:** A red test reproduces the empty appearance source; after the fix every appearance value carries its span's source and `verify` holds. [inferred]
- **R3:** With a `requirements-unmet` decision open, the conductor's next action is the owner's pause, never the gap loop. [inferred]
- **R4:** The mature-size question set has labelled cases (A1's leaflet and width sentences among them) and a no-match answer. [inferred]
- **R5:** The gate is green: `cargo test --profile ci --workspace --no-fail-fast`. [paraphrase]

## Boundaries

- Height and trunk diameter stay age-indexed. [inferred]
- No live literature run here; the host reruns the palm's stages after. [inferred]

## Decision Context

- Owner's standing rule of 2026-09-23: obvious friction fixes are the host's to spec. These are defects fn-118's first live run proved. [user]

## Open Questions

- None.

## Settled

Closed as landed (2026-09-26, fn-149 R8): carried by the fn-149 runner rewrite, merged in #121 (c2ac430b).
