# A trait the generator cannot draw yet never vetoes an adoption

## Conversation Evidence

> fn-80 live run, 2026-09-23: the date palm's second tuning revision rolled back 12 adoptions and kept 2; 10 of the 12 were "trait fruit-clusters-pendent went from Unknown to Fail". The date cluster is fn-111, not landed.
> user (2026-09-23): "i would have thought we run this until we have a palm unless we have an issue/friction that needs fixed"

## Goal & Context
<!-- scope: business -->

The palm's crown grew in the second revision, and every time it grew enough for the reviewer to see the crown clearly, the reviewer judged the date clusters absent and the trait flipped from unknown to fail. The closing review treats that flip as the candidate getting worse and rolls it back, so the loop spent about 3.5 M tokens and kept two small moves. The fruit is not a thing any dial can draw: its organ is an open spec. A trait the generator cannot express yet must not veto the moves that answer the owner's priorities. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, checked 2026-09-23 on the fn-80 branch.** `tuning/veto/restore.rs` `worsened` lists a required cell that went pass to fail and every coverage trait whose status went backwards (`backwards`: pass to fail, or unknown to fail). A non-empty list rolls the adoption back. The reference-first inventory (`.flow/evidence/fn80/reference-first/inventory.json`) lists `fruit-clusters-pendent` as a core trait. [checked]
- **The design (host, 2026-09-23).** [host design]
  - The tuning config (`tuning::live::Config`) gains `unexpressed`: a list of `{trait, spec}`, a trait id from the inventory and the open spec whose landing will let the generator draw it.
  - `worsened` skips a coverage trait listed there and records it instead, in words a router can read: `trait <id> went from <a> to <b>; not a veto: the generator cannot draw it until <spec> lands`. Required cells are untouched.
  - Readiness is untouched: `ready()` and the core-coverage gate still refuse a failing core trait, so a species with an unexpressed core trait never reads ready.
  - Config validation refuses an entry whose trait is not in the inventory, or whose spec is empty.

## Acceptance Criteria
<!-- scope: both -->

- **R1:** An adoption whose only backwards change is a listed trait going unknown to fail, or pass to fail, is kept, and the route notes carry the not-a-veto line. [inferred]
- **R2:** The same flip on an unlisted trait still rolls the adoption back, and a required cell going pass to fail still does, listed or not. [inferred]
- **R3:** A species with a listed core trait that fails is not ready. [inferred]
- **R4:** A config listing a trait the inventory lacks, or an empty spec, is refused at load. [inferred]
- **R5:** The gate is green: `cargo test --profile ci --workspace --no-fail-fast`. [paraphrase]

## Boundaries

- No change to readiness, the core-coverage gate or the reviewer's verdicts. [inferred]
- No live run here; the palm's next revision exercises it with `fruit-clusters-pendent` listed against fn-111. [inferred]

## Decision Context

- Found by the host in the fn-80 live run and fixed under fn-80's boundary that a loop defect the live run proves is fixed and named in the run's record. It goes into the palm's stack under fn-115. [paraphrase]

## Open Questions

- None.

## Settled

Closed as landed (2026-09-26, fn-149 R8): carried by the fn-149 runner rewrite, merged in #121 (c2ac430b).
