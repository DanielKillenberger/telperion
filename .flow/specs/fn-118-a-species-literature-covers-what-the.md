# A species' literature covers what the generator needs, or the run stops

## Goal

The date palm reached three tuning revisions with a manifest that asked the literature for only height and trunk diameter and nothing about appearance (bark, leaf colours, frond and leaflet size, crown width). Its height rested on one garden page's "50 to 100 feet" with no age, passing only because the manifest set its own bar at `proxy_only`; trunk diameter never met its bar; tuning had nothing to aim at. The first step of a species run must deliver every field the generator needs at a stated bar, or stop with NEEDS_HUMAN.

Owner quotes (2026-09-23):
> "so what's wrong with fn-82 that it didn't write the material reference docs? ... We need to make sure that the first step of gathering resources and creating the docs fulfills the requirements we have and fails with NEEDS_HUMAN if it doesn't."
> On the palm's height: "yes make sure we have enough data."

## Decisions

Host design and escalation decisions (2026-09-23):
- A requirements table `crates/telperion-jev/data/species-requirements.json`, keyed by growth form (broadleaf, conifer, palm), lists required size fields and appearance traits with the table's minimum bar, not the manifest author's.
- Admission refuses a short manifest naming every shortfall; the check runs only at the new `schema_version` 2, so older manifests load unchanged.
- `quality` files a blocking owner-only `requirements-unmet` decision; `lower-bar` is refused for a required field and the routine policy cannot resolve it; `add-sources` that adds nothing stays open.
- Appearance is copied, not rendered: a described level maps by code to channel ranges derived from `MaterialParams`; `generate` skips it with a note; fn-82 authors the material row from it.
- Rewriting the palm's manifest and rerunning its literature (R4) is the host's, in fn-80. Jev judges which span answers; code copies every number.

## Acceptance Criteria

- **R1:** Requirements table for broadleaf, conifer and palm, read by a test — met.
- **R2:** A short manifest refused at admission with every missing field named — met.
- **R3:** A required field below its bar files a blocking owner decision; NEEDS_HUMAN reported — met.
- **R4:** Palm manifest rewritten to the table and literature rerun — moved to the host in fn-80.
- **R5:** Gate green — met (944 passed).

## Result

Landed in `0118ca37`, `5c4f0a1f`, `281b80ba`; carried by the fn-149 rewrite (#121). First-cut bars: height and DBH at `partial`, crown and organ sizes at `proxy_only`. Docs: "The requirements table" in `docs/species-pipeline.md`.

## Open

- The sufficiency question set's lowest level reads "only mature ranges", so a mature-only organ size may score `none` and stop the run (fails safe); taken up by fn-127.
