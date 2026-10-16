---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-13-tree-detail-by-viewing-distance.4 Integrate and stabilize the renderer in the normal viewer

## Description
Integrate and stabilize the renderer in the normal viewer.

## Acceptance
- Normal viewer uses the shared WebGPU runtime with exact inspection, stable detail transitions and correct shared depth.

## Done summary
GrowerDev used the shared WebGPU runtime by default with exact inspection and transactional camera/identity state; canonical detail transitions to exact poses by station correspondence. Deep approach/reversal fell from 6.57 s to 1.87 s; a rejected history blend was reverted.

## Evidence
- Commits: 0e40da7c162347ff62832a0f201ba45b4e7ad081, 6e62f5fc06516fc1916668acba5623f908a8a9a6, 72982b0d02fb3997c29109ebef3b3cdde1d96ac4, 991f803896ac20d62de76ffa5e36621f022ef9b4, 315023ad0ce306568698096f3460d02cdf245688
- Tests: npm run typecheck; vitest run; npm run build
