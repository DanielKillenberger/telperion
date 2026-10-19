---
satisfies: [R1, R2, R3]
---
# fn-72-the-species-runner-finds-a-presets-own.1 Implement preset reference-profile resolution

## Description
Make the species runner resolve a preset to the profile set holding its records.

## Acceptance
Every R-ID in the parent spec's Acceptance is satisfied.

## Done summary
Beech and birch resolve to fn-34 profiles, oak and spruce stay on fn-9, explicit `--profiles` bypasses discovery, and a missing record names the preset and the sets searched. Tests cover every listed and in-work native preset; 22 script tests and the full Vitest suite (107) pass. Headless captures not validated (binary unbuilt).

## Evidence
- Commits: 3d84fd648717e10a50c8b35c932b598fdd3217be, e48cdf38134f33f545ba09ba1215bf0917d52072, 73b084db90d4b7dd899cb9bafd22cf8ef3009da4
- Tests: npx vitest run, npm run catalogue:check
- PRs: #45
