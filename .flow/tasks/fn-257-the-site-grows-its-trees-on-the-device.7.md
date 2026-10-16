---
satisfies: [R3, R4]
---
# fn-257-the-site-grows-its-trees-on-the-device.7 Speed through the package entry, sizes and the release's last decisions

## Description
TBD

## Acceptance
- Per-tree warm times through the package entry for every preset against 0.1.4 and against before; the harness's build times not worse; a probe page under src/probe.
- The catalogue's unread scaffold pins deleted (fn-237 R11); artifact ceilings raised to the measured sizes, naming fn-242.
- Gate and npm test green; merged into F4.

## Done summary
R3' measured natively per preset (RESULTS.md): oak 253-306 ms against 0.1.4's 273-332, birch 220-250 against 292-354, spruce 509-523 against about 240 (owner's about 0.35 s not met), palm 79-83 against 17; field queries made order-independent (fixed-point sums) so the per-bucket sort is gone (about 85 ms a tree). Harness builds within noise, the birch 151-172 ms faster. Sizes measured and ceilings raised per the owner (render 3.70 MB, telperion.js 200 KB, the shared chunk 70 KB new; telperion.wasm under its 1.6 MB). R11's unread scaffold pins deleted. The owner's probe page field-probe.html. Gate (after merging F4 9bd93e422): 1404 passed, the 2 guard failures fixed and re-run green; npm test 169/169. Codex reviews: empty first round (NEEDS_WORK) -> re-review SHIP; guard fixes SHIP.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: cc31606b9, bdf5c51b3, 6dfd38fec, 7334944e3
- Tests: cargo test --profile ci --workspace --no-fail-fast, npm test
- PRs: