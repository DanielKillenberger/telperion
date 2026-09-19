---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-42-bark-depth-level-and-a-fair-capture.1 Implement bark depth, level and a fair capture

## Description
TBD

## Acceptance
Every R-ID in the parent spec's Acceptance Criteria is satisfied; judge this task against the spec directly.

## Done summary
Blocked:
# fn-42 checkpoint and blocker

NEEDS_HUMAN: Restore the original owner-white-oak-bark.png and owner-norway-spruce-bark.png photographs (or identify their actual local paths). The fn-32 catalogue has no source URL for them, and repository plus sibling-worktree caches contain only historic measurements and rendered stills. Reference comparison and colour calibration cannot be accepted against absent source pixels. Reference crop scale must be documented as measured or explicitly estimated when the sources are restored.

Prepared a test-only production-renderer fixture that captures a flat 0.4 m square of oak and spruce bark with an explicit material radius. The host inspected two initial backlit images and the two corrected images, the full four-image budget. Final lighting is azimuth 45 degrees, elevation 55 degrees. Both final images are usable baseline captures, not a fidelity improvement or owner acceptance.

Verified one physical-coordinate test and one explicit hardware capture, including deterministic redraw. Formatting and diff checks passed after module ordering was formatted. No production shader, material, species-pipeline or leaf-layout change. Timing is not claimed under GPU contention. R1 is partial; R2-R5 remain incomplete. Task must not be marked done.

Friction reviewed by host: stale fn-42 design reconciled with accepted fn-71 filtering; missing local source cache needs restoration; flat-patch radius requirement resolved by the internal fixture; cold compilation cost at least 30 seconds and can be reduced by retaining the warmed build; wrong initial light orientation resolved and recorded in the capture recipe. No new friction specs created. The source cache and build-cache issues are local setup, not repository work.

Route taken: fn-42 -> reconcile newer owner constraints -> direct work -> missing reference photographs.
stage: work - ran (partial; calibrated capture checkpoint)
stage: impl-review - skipped(config: review.backend=none; host inspected the fixture diff)
stage: completion-review - skipped(policy: task incomplete)
stage: qa - skipped(policy: reference comparison and implementation incomplete)
stage: make-pr - skipped(policy: task blocked before implementation acceptance)
Tracker sync: n/a (bridge inactive)
Next: restore original reference photographs, confirm their crop scale basis, and resume this task.

Blocked:
# Reference decision after the bounded search

The original source photographs remain unavailable. Replacement retrieval produced one usable spruce structural candidate: MONGO's Norway Spruce bark detail, 2816 by 2112 pixels, mostly exposed bark with small scattered growth. It is frame-filling and substantially cleaner than the previously rejected growth-covered photograph. Flash illumination and unknown photographed width prevent claims of absolute colour or physical scale calibration. Syrio's 4000 by 3000 spruce close-up is useful secondary evidence of lifted plate edges, but its oblique angle and varying focus prevent a square-on score comparison. Sources and file hashes are in spruce-reference-candidates.json.

The best oak candidate, SelecTree's labelled bark original, is 650 by 567 pixels. It shows the relevant mature plate/furrow morphology but fails the high-resolution requirement. Three larger Commons originals were rejected for subject/framing; details are in REFERENCE-SEARCH.md. Originals remain in the local .refs cache, not redistributed.

Host recommendation: continue the fidelity change with reference-guided visual acceptance for exposed bark; preserve the calibrated render fixture, filtering continuity, deterministic redraw and measured-cost reporting. Replace the unavailable-photo colour target and strict photo-scale comparison with an explicit qualitative reference contract. This is a proposed material acceptance change, not approved yet; do not claim current R1 or R3 is satisfied. The alternative is to retain those requirements and wait for suitable calibrated sources.

NEEDS_HUMAN: choose the acceptance contract. The request to continue licensed the replacement search; it did not make absent scale or colour calibration real. The task remains blocked until this choice resolves the contract.

Friction: one earlier unnecessary user continuation turn; one original download rate-limited and not retried; approximately seven minutes/four inspections on the bounded oak search. Recommended fixes are to persist inspected source metadata and actual dimensions, and keep rejection-driven searches active within their bound. No new friction specs created. Source pixels remain locally cached with recoverable URLs.

stage: reference-search - ran (host spruce; worker oak; both complete)
stage: implementation - skipped(policy: material reference acceptance contract unresolved)
Next: accept reference-guided visual targets, or retain photo calibration and wait for suitable sources.
## Evidence
- Commits:
- Tests:
- PRs:
