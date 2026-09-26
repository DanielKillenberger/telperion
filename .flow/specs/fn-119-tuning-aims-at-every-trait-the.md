# Tuning aims at every trait the generator can draw

## Conversation Evidence

> user (2026-09-23): "And why was there no goal? we should tune all params that are relevant to a species."

## Goal & Context
<!-- scope: business -->

The date palm's three tuning revisions aimed at three objectives, the owner's written priorities (fronds, straight trunk, bare trunk). The reference inventory the reviewer wrote holds sixteen traits, among them `frond-colour-range`, `trunk-colour-and-weathering`, `trunk-fibrous-matting`, `crown-silhouette-proportions` and `leaflet-arrangement-stiff-narrow`; none was an objective, so the materials track found "no supported dial" in 13 rounds and kept nothing in three revisions. A species is tuned toward everything its references show that the generator can draw, with the owner's priorities first. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, checked 2026-09-23 on the fn-80 branch.**
  - The priority approval (`tuning/priority.rs`) is an ordered list the pause asks for; the palm's approvals listed only the owner's three notes. [checked]
  - The inventory (`.flow/evidence/fn80/reference-first/inventory.json`) lists each trait with a priority (`core`, `secondary`, `variation`). [checked]
  - fn-114's stride takes one lead tuning priority for every track (`tuning/stride/decide.rs:87`), so the materials track drew at the fronds' "far off". [checked]
- **The design (host, 2026-09-23).** [host design]
  - *Objectives.* The approval's ordered list is the owner's priorities first, then every inventory trait at `core` and `secondary` that is not listed `unexpressed` (fn-116), in inventory order; `variation` traits are recorded, not objectives. The pause proposes that list; the approving decision may reorder or remove, never needs to add.
  - *Tracks.* Each objective is routed to the dials that answer it (the existing routing); a track's objectives are those routed to its dial groups, and its stride class comes from its own lead objective, not the run's.
  - *No objective, no turn.* A track with no objective routed to it is skipped and recorded, not drawn.
- **Host decisions on the worker's escalation (2026-09-23).** The worker found no objective-to-track routing in code (`routing.rs` routes a priority to `tuning` or elsewhere only; proposals carry no priority). [host design]
  - *Track on a gap.* Each approved gap may carry an optional `track`. An unassigned gap belongs to every track, which is today's behaviour. A track's objectives are its named gaps plus the unassigned ones, and its stride class comes from its own lead objective. No new Jev question.
  - *Per-track requests and caps.* Each track's sheet and progress request carries only its own objectives. The per-request cap rises from 8 to 16, and `Approval::verify` accepts up to 32 gaps.
  - *Inventory trait to gap.* Code turns an inventory trait into a gap: its views are the shots of its `reference_ids` restricted to the required cells, and its `evidence_ids` are the current render of each such view plus its references.
  - *R2's palm test* assigns `trunk-colour-and-weathering`, `frond-colour-range` and `trunk-fibrous-matting` to the materials track.

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A run's proposed approval lists the owner's priorities then every expressible `core` and `secondary` inventory trait; an `unexpressed` trait and a `variation` trait are left out and recorded. [inferred]
- **R2:** Each track's stride class is judged on its own lead objective; the palm's materials track, given `trunk-colour-and-weathering`, no longer inherits the fronds' class (test on recorded state). [inferred]
- **R3:** A track with no routed objective is skipped with a route note. [inferred]
- **R4:** The gate is green: `cargo test --profile ci --workspace --no-fail-fast`. [paraphrase]

## Boundaries

- No live run here; the palm's next revision exercises it. [inferred]
- Readiness and the owner's acceptance are unchanged. [inferred]

## Decision Context

- The owner asked on 2026-09-23 that tuning cover every parameter relevant to the species. [user]

## Open Questions

- None.

## Settled

Closed as landed (2026-09-26, fn-149 R8): carried by the fn-149 runner rewrite, merged in #121 (c2ac430b).
