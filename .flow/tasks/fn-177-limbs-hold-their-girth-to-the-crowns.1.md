---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-177-limbs-hold-their-girth-to-the-crowns.1 Implement Limbs hold their girth to the crown's edge

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
Added `girthHold` and `girthFall` to `/radii`. Every structural axis holds the pipe model's radius where it leaves its parent over `girthHold` of its own reach, then falls to the pipe radius by its tip. The rows are built to the host's decisions, and the spec's Architecture and Acceptance sections are corrected. Codex reached SHIP after one review-round reset, which the host authorised. The host also accepted four refinements, recorded in the spec: the root is a point, an axis is measured from where it leaves its parent, leaf decisions read the pre-hold radii, and the `girthHold` rail stops at 0.9.

- R1: .flow/evidence/fn-177-limbs-hold-their-girth-to-the-crowns/R1-MEASUREMENT.md
- R2: rows in the catalogue, wire, blend, dials, docs/parameters.md and browser metadata; specimen schema 7; at zero hold, 16 of 16 preset/seed builds are identical to master (zero-hold-vs-master.json), and species digests are unchanged
- R3: crates/telperion-core/src/pipeline/radius/hold_tests.rs: girth held within 1e-9 of its base, the pipe radius from the end of the fall to the tip, a whorl-ended leader not left blunt, continuity of start and distal radii under taper across a sibling and a root fork, rails refused by name, leaves placed unchanged
- R4: plane candidate with girthHold 0.75 and girthFall 4, 8 seeds; raw/stills/compare-bare.png and compare-whole.png, before on top and held below. Owner question: do the limbs hold their girth to the crown's edge now?
- R5: gate and npm test green. Candidate median build is 771 ms against 769 ms before (fn-170: 769 ms); peak memory is 619 MB against 618 MB (fn-170: 619 MB). Nodes and placed leaves are identical at all 8 seeds.

Left for the host:
- A fork's primary keeps its axis's girth past the fork, while a sibling starts thinner at its pipe share. Worth checking on the stills.
- Wood 10 cm and thicker grows only from 33 m to 41 m (5 cm and thicker: 116 m to 307 m), because the pipe model still gives every limb a thin base.
- The owner's R4 look is with the host.

stage: impl-review - ran (codex: round 1 NEEDS_WORK 2 P2, round 2 NEEDS_WORK 1 P2, cap reset by the host, round 3 SHIP)
Tier: not provided by the conductor

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 7513162a961b6a95e81f2f8cc9e599ea9cf76f47, 4e938d65bb0d9ee6d760f922a6664983918d9500, 792fc840f954a05c79d0045d2435ee0773bc53ac, 6adef593c1c23d2847472d1d98bc9faadf3e5af1, 03d55a719591bfb9152c18cc3e50d6f3f67709c3, 6ad2c7095656b6cb1262148f1191afffde00df18, d515486ea5244df9901de71329bed67a5b20e932
- Tests: cargo test --profile ci --workspace --no-fail-fast (1082 passed, 0 failed, 22 ignored at 03d55a71; later commits touch .flow/ only), npm test (13 files, 124 tests), cargo test --profile ci -p telperion-core --lib hold_tests (7 tests, red-first), cargo test --profile ci -p telperion-jev --test replay, zero hold: 16 preset/seed builds identical to master (zero-hold-vs-master.json)
- PRs: