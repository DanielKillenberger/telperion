---
satisfies: [R1, R2, R3, R4]
---
# fn-109-the-apical-rosette-fronds-borne-only-at.1 Implement The apical rosette: fronds borne only at the apex of an unbranched stem

## Description
Add the apical rosette of pinnate fronds as a shared generator capability, off by default.

## Acceptance
Spec R1-R4.

## Done summary
`foliage/rosette.rs` places N fronds on a phyllotactic spiral at each stem apex; `fan` expands a placement into leaflets along a rachis; ten canopy rows with ranges and dial rows; rosette apices bear no twig. apical-rosette, pinnate-frond and pinnate-compound moved to the expressed list. Presets byte-identical. The one gate failure (tuning proposal cap with 358 bytes headroom) was resolved by the host raising the cap to 32 KiB (`31f1f763`); gate then green, 894 passed.

## Evidence
- Commits: 90b61031, 9c08aa55, 31f1f763
- Tests: cargo test --profile ci --workspace --no-fail-fast; cargo test --profile ci -p telperion-core --test rosette; cargo test --profile ci -p telperion-jev --test tuning_engine
- PRs:
