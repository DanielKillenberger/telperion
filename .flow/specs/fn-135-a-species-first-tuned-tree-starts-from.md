# A species' first tuned tree starts from its sourced profile

## Conversation Evidence

> user (2026-09-24): "I thought we have a generalized tuning loop to get to a acceptable looking species based on literature"
> host: the literature fills the profile, but nothing turns the profile into the tree's starting values; the palm's tuning started from registration defaults and its bark was the family default material.
> user (2026-09-24): "it is tomorrow now so yes" (to a spec for the profile-to-preset step before the next tuning pass)

## Goal & Context
<!-- scope: business -->

The loop from literature to an acceptable-looking species has one missing link: the sourced profile (sizes and appearance) never reaches the tree the tuning starts from. The palm was tuned from the preset's registration values with the default bark, and its sourced grey, rough bark and grey-green leaves reached nothing. The conductor must derive the starting values from the profile before the first tuning revision, for any species, so tuning refines a literature-shaped tree instead of searching from defaults. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, checked 2026-09-24 on the fn-80 branch.** [checked]
  - The palm's profile (`.flow/evidence/date-palm/pipeline/packet/profile.json`) holds `metrics` (`crown_width_m`, `dbh_m`, `frond_length_m`, `height_growth_m_per_year`, `height_m`, `leaflet_length_m`, `leaflet_width_m`, each `gating` with a range) and `appearance` entries whose `ranges` are keyed by `MaterialParams` field names (`bark_red`, `bark_green`, `bark_blue`, `bark_roughness`, `leaf_front_*`, `leaf_back_*`, `hue_range_*`, `brightness_range_*`).
  - The tuning config has an `initial_overrides` overlay (the palm's is `{}`); the dial table (`data/dials.json`) gives each wire path's meaning, range and source.
  - `species-requirements.json`'s `set_from` is a provenance note, not a mapping; no code maps a profile to preset values.
  - Unknown: which profile metrics the tuning measurer (`species_measure`) can measure (organ sizes may be unmeasured, and a `gating` metric the measurer cannot read would fail every evaluation); and the exact wire meaning of `envelope.spread` for crown width. The implementer confirms both first. [unknown]
- **Shape.** [host design]
  - *A derivation table, species-agnostic.* `data/profile-to-preset.json` maps a profile entry to a wire path through a code-owned formula: `midpoint` (a range to its middle), `ratio` (one metric over another, e.g. trunk radius as half the trunk diameter over the height), `identity`. Each row may carry a condition on the family's own values (e.g. frond length applies only where the family bears a rosette), never a species name.
  - *Appearance first.* Every appearance range keyed by a material field sets that field to its midpoint.
  - *Sizes.* Height to the envelope height; trunk diameter over height to trunk radius; frond length to the rachis length where a rosette exists; leaflet and leaf sizes to the element; crown width through the envelope's spread as its meaning allows. A row whose meaning the implementer cannot confirm is left out and recorded.
  - *Clamped and recorded.* Each derived value is clamped to its dial's validated range (fn-113) and written, with its profile source and formula, as the tuning config's `initial_overrides` and a provenance file beside it. No preset code changes: shipping the tuned values into the preset stays the species spec's last step.
  - *The conductor runs it.* Before the first tuning revision, and again when the profile changes, the conductor writes the derived overlay; a manual `initial_overrides` entry wins over a derived one.
  - *Measurable targets only.* A profile metric the measurer cannot read is classified `contextual` in the tuning profile, not `gating`.

## Acceptance Criteria
<!-- scope: both -->

- **R1:** On the palm's profile, the derivation sets the bark and leaf colours, bark roughness, hue and brightness ranges to their range midpoints, and the sizes the table maps, each clamped and with its provenance. [inferred]
- **R2:** The table names no species; a row's condition reads only the family's values; a broadleaf profile (the oak's or the ash's) derives without touching rosette rows. [inferred]
- **R3:** The conductor writes the derived `initial_overrides` before tuning revision 1 and again after a profile change; a manual override wins. [inferred]
- **R4:** A profile metric the measurer cannot read is `contextual`, so a tuning baseline on the palm measures without a gating failure (test on the measurer's metric list, no render). [inferred]
- **R5:** The gate is green: `cargo test --profile ci --workspace --no-fail-fast`. [paraphrase]

## Boundaries

- No change to the literature step or the tuning engine's search. [inferred]
- No preset code change; no render or capture here. [inferred]

## Decision Context

- The owner confirmed on 2026-09-24 that the loop should reach an acceptable species from literature; this is the missing link, built before the palm's next tuning pass. [user]

## Open Questions

- The two unknowns in Architecture, confirmed first.
