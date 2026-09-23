# A palm's size is its growth rate and mature range; the literature rerun's last defects

## Conversation Evidence

> host, 2026-09-23, the palm's literature rerun after fn-129 to fn-131: leaflet length and width `sufficient` with 7 points yet failing on `no_mature_size`; height and trunk diameter with one age point each (`age_range_uncovered`); the conductor searched for sources before rerunning stages whose code had changed.
> user (2026-09-23), asked what a palm row should require since no source measures an old date palm: "Growth rate + mature range (Recommended)".

## Goal & Context
<!-- scope: business -->

The requirements table asks every growth form for height and trunk diameter at 20, 50 and 80 years. Palm literature gives growth rates, a few age classes and mature ranges, and a palm grows near-linearly through most of its life, so the ages are the wrong requirement for palms. The owner chose: a palm's height is a stated growth rate plus its mature range, and its trunk diameter its mature range. Two defects found on the same rerun go with it. [user]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, checked 2026-09-23 on the fn-80 branch.** [checked]
  - `quality.json` (the palm, after fn-131): `leaflet_length_m` and `leaflet_width_m` level `sufficient`, 7 points each, `passed: false`, gap `no_mature_size`; `height_m` and `dbh_m` level `proxy_only`, 1 point each, gap `age_range_uncovered`.
  - The palm's sources state growth rates (A1: "30-45 cm (1 to 1.5 feet) a year"; P5: "1-1.5 ft (30-45 cm) a year") and mature ranges (P8: "height ranges from 15 to 25 m"; A1: trunk "up to ½ m").
  - The conductor's next action was `search_again` while `fetch` to `verify` were stale after fn-130 and fn-131; two empty rounds then went to the owner before the stages had read P8's raw body.
  - Where the gap `no_mature_size` is computed after fn-131, and where the conductor's plan orders search before stale stages: unknown; the implementer confirms both first. [unknown]
- **Shape.** [host design]
  - *A rate field kind.* The requirements table gains a field kind `rate` (a size per year, e.g. `height_growth_m_per_year`), judged by a rate question set (a stated growth rate for the taxon, with the same no-match answer and labelled cases before its floor is trusted).
  - *The palm row.* Palm requires `height_growth_m_per_year` (rate) and `height_m` and `dbh_m` as `mature` fields (a mature range, no age); other growth forms keep 20/50/80.
  - *The gap follows the level.* A mature field whose points make its level `sufficient` or `partial` is never given `no_mature_size`; that gap is only for a field with no mature-size point.
  - *Stale stages first.* The conductor reruns stages whose key expired before it searches for sources or pauses for the owner on a literature decision.

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A red test on the palm's leaflet rows: `sufficient` with 7 points passes and carries no `no_mature_size`. [inferred]
- **R2:** The palm row requires a height growth rate and mature height and trunk-diameter ranges; A1's and P5's rate sentences fill the rate, P8's range fills mature height, A1's "up to ½ m" fills trunk diameter. Broadleaf and conifer rows are unchanged. [inferred]
- **R3:** The rate question set has labelled cases (the palm's rate sentences among them) and a no-match answer. [inferred]
- **R4:** With stale stages, the conductor's next action is the stages, never `search_again` or the owner. [inferred]
- **R5:** The gate is green: `cargo test --profile ci --workspace --no-fail-fast`. [paraphrase]

## Boundaries

- The palm's manifest is rewritten to the row by the host in fn-80, not here. [inferred]

## Decision Context

- The owner chose the palm's requirement on 2026-09-23. The two defects are obvious fixes under the owner's standing rule. [user]

## Open Questions

- The two code locations marked unknown, confirmed first.
