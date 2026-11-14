# A claim quoted word for word is accepted by code

## Goal

In fn-80's live run (2026-09-24) the palm's height "50 - 100 feet", verbatim in A1's size table, was flagged `claim-unsupported` (Jev "supports" at 0.65 against the 0.8 auto-accept cut); the cheap tier could only replace or drop it, the search rounds were spent, and it reached the owner. A value whose exact span stands in the cited sentence is supported by the text itself. Owner (2026-09-24): "Accept, and fix (Recommended)".

## Decisions

- Host design: before the confidence cut, code checks whether the claim's span occurs verbatim in the cited sentence (whitespace normalisation only); a verbatim `supports` claim is not listed on confidence and the record says `verbatim`. Other listing rules (site-quality criterion, height-at-age kind) still apply.

## Acceptance Criteria

- **R1:** The palm's height claim (supports 0.65) is not listed after the fix and reads `verbatim` — met (red then green).
- **R2:** A non-verbatim span below the cut and a verbatim site-quality criterion are still listed — met.
- **R3:** Gate green — met (1049 passed).

## Result

Landed in `f8fdcb24` (`cite.rs` `list_reason`, `is_verbatim()`); carried by the fn-149 rewrite (#121).

## Open

None.
