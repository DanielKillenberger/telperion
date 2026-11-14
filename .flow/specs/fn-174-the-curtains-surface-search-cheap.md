# The curtain's surface search, cheap

## Goal

The silver birch grew in 136 ms at seed 1 and 267 ms at seed 7 on master f9487810, with fewer nodes than the oak's 51 ms; on seed 7 the curtain's admission search (`Curtain::admits` → `in_band` → `lower_surface`, up to 32 climb steps and 32 halvings of lobed `radius_toward` probes) took about 80% of growth. Asked as one of the focused growth-speed specs split from the owner's request to speed up skeleton generation.

## Decisions

- user (2026-09-27): "we have found many optimizations but skeleton generation seems to still take a lot of time. Can we investigate how we can speed that up?"
- user (2026-09-27): "i have said many times as we build the engine we don't need things byte identical. If we get massive improvements for different generators that aren't identical that's fine. If it being identical helps verifying at no downside then it's good req to have."
- user (2026-09-27, on splitting the growth work into focused specs): "yes and make fn-125 depend on B if that's the most efficient way"
- Depended on fn-173 so its gain would be measured on the cheaper shell.
- Closed 2026-10-02 as superseded by fn-183 (#137): the direct build no longer admits curtain strides, so the search was deleted rather than made cheap (`docs/principles.md`, "Question, delete, then optimise").

## Acceptance Criteria

- **R1:** Birch growth at seeds 1 and 7 at most half the base median, no preset more than 2% slower — not built (superseded; fn-183 cut birch skeleton time 87%).
- **R2:** The new `in_band` held to an independent dense-scan reference — not built (superseded).
- **R3:** Owner's visual verdict on the birch at seeds 1 and 7 — not built (superseded).

## Result

Superseded. After fn-183, `in_band` and `lower_surface` run only on the hidden growth path, which fn-181 removes. The birch's skeleton fell from 183 to about 25 ms at seed 1 by deleting the search.

## Open

None.

## Evidence

- Growth profile cited: `.flow/evidence/fn-173-growth-reads-a-prepared-crown-shell/PROFILE.md`.
