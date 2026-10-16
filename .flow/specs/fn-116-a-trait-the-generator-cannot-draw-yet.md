# A trait the generator cannot draw yet never vetoes an adoption

## Goal

In fn-80's live run (2026-09-23) the date palm's second tuning revision rolled back 12 adoptions and kept 2; 10 of the 12 were "trait fruit-clusters-pendent went from Unknown to Fail" — the date cluster is fn-111, not landed. Every time the crown grew enough for the reviewer to see it, the absent fruit flipped the trait to fail and the closing review rolled the move back (~3.5 M tokens for two small moves). A trait the generator cannot express yet must not veto moves that answer the owner's priorities. Owner (2026-09-23): "i would have thought we run this until we have a palm unless we have an issue/friction that needs fixed".

## Decisions

- Host design (2026-09-23): the tuning config gains `unexpressed: [{trait, spec}]`. `worsened` skips a listed coverage trait and records `trait <id> went from <a> to <b>; not a veto: the generator cannot draw it until <spec> lands`. Required cells, `ready()` and the core-coverage gate are untouched, so a species with an unexpressed core trait never reads ready. Validation refuses a trait missing from the inventory, a blank spec, or a listing with no inventory.
- Fixed under fn-80's boundary (a loop defect the live run proves), into the palm's stack.

## Acceptance Criteria

- **R1:** An adoption whose only backwards change is a listed trait is kept, with the not-a-veto note — met.
- **R2:** An unlisted trait, or a required cell going pass to fail, still rolls back — met.
- **R3:** A species with a listed failing core trait is not ready — met.
- **R4:** Unknown trait or empty spec refused at load — met.
- **R5:** Gate green — met (937 passed).

## Result

Landed in `c1002d60`; later carried by the fn-149 runner rewrite (#121, `c2ac430b`).

## Open

None.
