# fn-256-the-whole-grower-runs-on-the-gpu.15 Arena hints measured again on the lit presets; tall narrow lattices

## Description
Two follow-ups from tasks 13-14: (1) the arena hints' measured ratios predate fn-264's light for every species, so Telperion and Laurelin now move once a build; measure the ratios again over every preset at seeds 1, 7, 42 so every build is back to zero moves; (2) codex's finding at build_size.rs:65: the sweep limits refuse tall, narrow lattices the old path took; bound them by the swept layers as task 13's fix did for the refusal, with a test on a tall narrow tree.

## Acceptance
Zero moves on every preset at seeds 1/7/42; a tall narrow lattice builds; gate and review.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
