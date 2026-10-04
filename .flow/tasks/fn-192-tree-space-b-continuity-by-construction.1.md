---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-192-tree-space-b-continuity-by-construction.1 Implement Tree space B: continuity by construction

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
Continuity by construction in telperion-space: lineage-keyed draws (R1), gradual birth and death through presences computed after growth, each draw growing in over a log-odds window scaled by the closed form's expected wood under its node's physiological age (R2, host decision 3), discrete botany as continuous settings (R3), walk tests within the bound and A's oracle green (R4), eight strips viewed by the host with no visible pop (R5). Length lost to partly grown branches: 17.0% over seeds 0-49.
stage: impl-review - OVERRIDDEN: the Codex cycle (reset once by the host) ran two NEEDS_WORK rounds, each finding fixed with a red-then-green test; the cap refused round 3 on the last fix (43efd4b7), which Astra reviewed independently outside the cycle: SHIP. Remaining overflow paths it named are pre-existing extremes (wood() exponentiation, share() reciprocal, age + 1 at u32::MAX), listed for the owner.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 54c280a4795e7869d10173a0d451e52d614a303c, 9cda1fd71a47779670efec79955c9979217030e5, b258cabf857823667a6d16e7df3d7573048dcfd9, 6bda9e02048f4b1d25d24aa3a3cee98934ff2469, a8a3fc721273f070d0b06be6dc34fddba774b552, 03d4a7fb6d137f9d7ec3699a9fbcf769c46559f3, d1668c9689d8227036a492d4ba9fd556a293f8d4, 447e885bf80a043c0a280b4d40825932ba1b7616, d47c8897d6e1a4c10a11e534246b4afacafbca96, c186a4230467de56808aed735d5094366d4a1550, bb4806977a6a4c2be68f73fe1752d1ac1650b3b3, 5d51118ebf08e5bb4609602ac9244ea7977a549c, ebca07143e860d9f25d06fdaf4a1a7a4cbb679cf, f54541205beed1205074a9fd60f50112809879ba, d94becd6ec8a23dd2098288396b6af6749fd83d0, 43efd4b79172be681503ab1dc5ba92e063b0ac32, b00e9856b46c5dff336fe949e19154bb6950d854, 95ab76a2b2b1b44e07c524fe063175c3833ea6ce
- Tests: cargo test --profile ci --workspace --no-fail-fast (1004 passed)
- PRs: