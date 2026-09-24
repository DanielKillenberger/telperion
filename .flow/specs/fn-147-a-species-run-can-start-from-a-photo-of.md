# A species run can start from a photo of a tree

## Conversation Evidence

> owner (2026-09-24): "We also want the simplest lightweight species adding runner that can take a species or a picture of a tree gather documentation, compile it into a catalog entry and start and finish the tuning and gap analysis quickly"
> owner (2026-09-24): "Photo input is a follow up spec"

## Goal & Context
<!-- scope: business -->

The lean runner takes a species name. This follow-up lets a person start a run from a photograph instead: the runner identifies the species, or names the few candidates and asks, and the photo joins the run's reference photographs beside the sourced ones. Everything after identification is the name path unchanged. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **Unknown.** Which model identifies the species and how its answer is checked before a literature run is spent on it; whether a user photo's rights let it ship in the catalogue or only guide tuning. [unknown]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A photo of a species the catalogue holds starts a run that names that species, or stops with its candidates for a person to pick. [inferred]
- **R2:** The photo is a reference the reviewer compares against, and its rights are recorded. [inferred]

## Boundaries
<!-- scope: business -->

- Follows the lean runner. Not a replacement for sourced reference photographs.
