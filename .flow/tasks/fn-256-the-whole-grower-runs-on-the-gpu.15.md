# fn-256-the-whole-grower-runs-on-the-gpu.15 Arena hints measured again on the lit presets; tall narrow lattices

## Description
Two follow-ups from tasks 13-14: (1) the arena hints' measured ratios predate fn-264's light for every species, so Telperion and Laurelin now move once a build; measure the ratios again over every preset at seeds 1, 7, 42 so every build is back to zero moves; (2) codex's finding at build_size.rs:65: the sweep limits refuse tall, narrow lattices the old path took; bound them by the swept layers as task 13's fix did for the refusal, with a test on a tall narrow tree.

## Acceptance
Zero moves on every preset at seeds 1/7/42; a tall narrow lattice builds; gate and review.

## Done summary
The arena's ratios are measured again with every species lit. Every preset at seeds 1, 7, 42 and its own now grows with 0 moves (24 of 24). The cause of the move was the records: Telperion's reach 4.63 of the hint, where 1.745 was stated. The sweep's plan bound is now at least 64 ranges (1,024 layers, the least arena's 4,096 nodes stood two wide), and the first two cycles record it whole, so a tall, narrow lattice builds. `a_tall_narrow_lattice_is_swept` and `a_tall_narrow_tree_builds` (a 450 m pole, 902 layers) were red on the base and are green (PROOF.md task 15, DESIGN.md section 19).

Open for the host: Telperion's production build now peaks at 4,209 MB, 85 MB under the 4 GiB native default (task 13 tip: 3,849 MB). The added memory is the records, now 412 MB, and the units.

## Evidence
- Commits: 5d7b49839a67fb1c66faebdd143f1402951b460b, e9693d6e8f2a0b139850fb6f04a48aa83e273d42, b363619f60a950018624168bb5251e45be688d53, e62797a0a294073a79cd22653f945e6bce986eed, c60091375cea708d0fb14e98f2a124828097c4dc
- Tests: cargo test --profile ci --workspace --exclude telperion-render --no-fail-fast, cargo test --profile ci -p telperion-render --no-fail-fast, npm test, cargo test --profile ci -p telperion-render every_preset_grows_in_its_hinted_arena -- --ignored, cargo test --profile ci -p telperion-render arena_ratios -- --ignored
- PRs:
