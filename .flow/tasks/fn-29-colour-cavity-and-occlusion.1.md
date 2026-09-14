---
satisfies: [R1, R2, R3, R4, R5, R6, R7]
---
# fn-29-colour-cavity-and-occlusion.1 Implement Colour, cavity and occlusion

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:

## NEEDS_HUMAN (host, 2026-09-14)

NEEDS_HUMAN: R5 owner judgment on .flow/evidence/fn29/stills — and first the R6 number: the oak's native total p50 is 4.0005 ms (p95 4.3791) against the 3.8 ms bound, 0.2005 ms over, on the exact fn-26 protocol (RTX 3080, Vulkan, seed 7, 1600x1000, 120 measured frames, verdict valid; fn-26 final 3.6879/4.0284). The first valid run was 3.9549 ms; the cost cut in 47b1af8 did not recover it. The browser orbit was not run and the eight R5 stills are unrendered: codex stopped at the spec's R6 rule before capture, so the owner-verdict table in .flow/evidence/fn29/REPORT.md is blank and stills.json is empty.

Implementation is complete and every gate is green at a0fc5b1 (fmt, clippy, cargo test 314/0 with zero adapter skips, npm 77/0, typecheck). The owner's choices: accept a bound of about 4.0 ms for the oak with every term on; ask for a further cost cut in the same codex session (six of ten commits used); or order the eight stills rendered first so the R5 judgment informs the R6 decision. No bound, protocol, tolerance or pinned number was moved.

Still round 1 of 2 (owner's decision: look first; round 3 of the codex session, commit 8ffb672): the eight stills are in .flow/evidence/fn29/stills/ under fn-26's names, sha256-verified against stills.json, the "Stills" section in REPORT.md pairs each with its fn-26 counterpart and reference, and the verdict table stays blank. Browser orbit: wall p50/p95/max 10.00/10.10/10.10 ms over 999 frames, valid, 60 fps holds; GPU total p50 4.1009 ms (fn-26 3.7379). Gates at 8ffb672: fmt 0, clippy 0, npm 0; no code under crates/ or src/ changed. For the R5 look, one number beside the images: centre-crop means (0-255) fn-26 -> fn-29 are oak-trunk 169,167,160 -> 79,89,115 and spruce-trunk 148,105,72 -> 87,49,29; the oak's blue-dominant fissure endpoint (.025,.038,.082) follows from its row (base x cavity .4 plus fissure offset at .65), a value the owner can move without renderer code. NEEDS_HUMAN stands on R5 and R6; one still round remains, eight of ten commits are used.

Still round 2 of 2 (owner-approved one value change; round 4 of the codex session, commit d7c44b3): the oak fissure offset moved from (-0.1, -0.075, 0.005) to (-0.04, -0.03, 0.002) at unchanged strength 0.65, so the fissure endpoint is near (0.064, 0.068, 0.080) instead of (0.025, 0.038, 0.082); only presets/materials.rs and the regenerated browser mirror changed. The eight stills were re-rendered as in round 3 and overwritten under the same names, sha256-verified against stills.json; six hashes are unchanged (the row touches oak bark only). Oak-trunk centre-crop mean is now 109,110,114 (round 3: 79,89,115; fn-26: 169,167,160). Timing and orbit records untouched. Host gates at d7c44b3: fmt 0, clippy 0, cargo test 0 (314 passed, zero device skips), npm 0 (77), typecheck 0; no pin, test or tolerance moved. NEEDS_HUMAN stands on R5 (verdict table blank in REPORT.md) and R6 (native p50 4.0005 ms > 3.8; orbit 60 fps holds). Both still rounds and all ten budget commits are used: no further round without the owner.
