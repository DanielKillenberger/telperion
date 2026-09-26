# A species' literature covers what the generator needs, or the run stops

## Conversation Evidence

> user (2026-09-23): "so what's wrong with fn-82 that it didn't write the material reference docs? ... We need to make sure that the first step of gathering resources and creating the docs fulfills the requirements we have and fails with NEEDS_HUMAN if it doesn't."
> user (2026-09-23), on the palm's height: "yes make sure we have enough data."

## Goal & Context
<!-- scope: business -->

The date palm reached three tuning revisions with a manifest that asked the literature for two numbers, height and trunk diameter, and nothing about how the tree looks: no bark colour or roughness, no leaf colours, no frond or leaflet size, no crown width. Its height rests on one garden page's "50 to 100 feet" with no age, passed only because the manifest set its own bar at `proxy_only`; its trunk diameter never met its bar. The tuning loop then had no data to aim the palm's proportions or materials at. `docs/species-onboarding.md` already says a profile owes appearance; nothing enforces it. [paraphrase]

The first step of a species run, gathering sources and writing the documentation, must deliver every field the generator needs at a stated bar, and stop the run with NEEDS_HUMAN when it cannot. [user]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, checked 2026-09-23 on the fn-80 branch.**
  - The palm's manifest (`catalogue/date-palm/manifest.json`) lists `fields` `height_m` (bar `proxy_only`) and `dbh_m` (bar `partial`), and `described` is empty; no shipped manifest uses `described`. [checked]
  - `manifest::validate` (`pipeline/manifest.rs:229`) checks shape, not coverage; the bar is the manifest author's. [checked]
  - `quality` files `data-insufficient` per failing field; its resolutions are `admit-proxy`, `add-sources`, `lower-bar` (`docs/species-pipeline.md:207`); the palm's `dbh_m` was resolved `add-sources` by the cheap driver under the routine policy and stayed unavailable. [checked]
  - Extraction saw appearance in the palm's sources ("rough gray", "diamond pattern", "up to ½ m in diameter" in A1) with no field to carry it. [checked]
- **The design (host, 2026-09-23).** [host design]
  - *A requirements table.* `crates/telperion-jev/data/species-requirements.json`, keyed by `growth_form`, lists the fields a manifest must ask for and the minimum bar each must reach: the size fields (height at a stated age or maturity, trunk diameter, crown width, crown base) and for each foliage organ its size (leaf or frond length, leaflet length and width where compound), plus the appearance fields the material row reads (bark colour and roughness, leaf front and back colour, the hue and brightness ranges) as `described` traits with level tables. The bar is the table's, not the manifest author's.
  - *Admission refuses a short manifest.* `manifest::validate` refuses a manifest whose `growth_form` requires a field it does not list, or lists below the table's bar, naming each.
  - *Quality stops, it does not lower.* A required field below its bar after `quality` files a blocking `requirements-unmet` decision owned by the owner (NEEDS_HUMAN); `lower-bar` is refused for a required field and the routine policy may not resolve it. `add-sources` stays, and a resolution that adds none leaves the decision open.
  - *Documentation follows the data.* `document` writes an appearance section from the described traits, so the article carries what the material row is authored from.
- **Host decisions on the worker's escalation (2026-09-23).** [host design]
  - *Appearance is copied, not rendered.* An appearance trait is a manifest entry whose level maps, by code, to a range per colour channel (or a scalar for roughness); `select` fills it from the chosen span and code copies the range into the profile; `generate` skips it with a recorded note (no render, no measurer metric). fn-82 authors the material row from the range and tuning aims at it.
  - *Coverage at admission, by schema version.* The coverage check runs only for a manifest at the new `schema_version` this spec introduces; every existing manifest and fixture loads unchanged. New manifests (`discover`'s proposal) are written at the new version.
  - *The palm's manifest is the host's.* The tracked manifest is `.flow/evidence/date-palm/pipeline/manifest.json` on the fn-80 branch; `catalogue/date-palm/manifest.json` is untracked (#57). Rewriting it to the table and rerunning the literature stages is done by the host in the fn-80 layer, not here.
- **Settled (host, 2026-09-23).** The first table covers `broadleaf`, `conifer` and `palm`. Each appearance trait's level table is derived from the material row field it feeds (`MaterialParams`: bark colour and roughness, leaf front and back colour, hue and brightness ranges), in coarse named levels with a no-match level, so a described level maps to a value range code owns. [host design]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The requirements table exists for broadleaf, conifer and palm growth forms, and a test reads it. [inferred]
- **R2:** A manifest missing a required field, or listing it below the table's bar, is refused at admission with every missing field named. [inferred]
- **R3:** A required field below its bar after `quality` files a blocking owner decision; `lower-bar` on it is refused and the routine policy cannot resolve it; the run reports NEEDS_HUMAN. [inferred]
- **R4 (host, in fn-80):** The palm's manifest is rewritten to the table (height at an age, trunk diameter, crown width, frond length, leaflet size, bark colour and roughness, leaf colours) and its literature stages rerun; each field reaches its bar or the run stops with NEEDS_HUMAN naming it. [inferred]
- **R5:** The gate is green: `cargo test --profile ci --workspace --no-fail-fast`. [paraphrase]

## Boundaries

- No change to Jev's role: it judges which candidate span answers a field; code copies every number. [paraphrase]
- The palm's preset values are fn-82's; this spec delivers the data they are authored from. [inferred]

## Decision Context

- The owner asked on 2026-09-23 that the first step fail with NEEDS_HUMAN rather than pass thin data forward. [user]

## Open Questions

- None.

## Settled

Closed as landed (2026-09-26, fn-149 R8): carried by the fn-149 runner rewrite, merged in #121 (c2ac430b).
