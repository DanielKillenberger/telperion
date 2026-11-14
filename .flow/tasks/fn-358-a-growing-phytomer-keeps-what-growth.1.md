# fn-358-a-growing-phytomer-keeps-what-growth.1 The grower's phytomer store holds what growth reads; the tree equal to the bit

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
The grower holds each phytomer as a 72-byte `Growing` record (the fields growth reads, each at the f32 width growth wrote it, and no radius), and the settle widens it into the tree's 136-byte `Phytomer` to the bit. On the oak at 120, seed 1002, the phytomers' store falls from 2.018 to 1.069 GiB and the grower's footprint from 4.795 to 3.845 GiB (−19.8%). Every digest is unchanged. Tip, heading and side stay in the grower, narrowed, because the rough layout reads them (READING.md). The next bytes are closed axes' pencils and phytomers (TASK1-DONE.md).

Tier: product code, built by Opus to the host's written design (model: claude-opus-5-5).
Existing test edited: grow/cycle/paused/tests.rs `livings` widens the grower's axes before `presence::assign` (a fixture following the type).

stage: impl-review - skipped(config: REVIEW_MODE=none; one codex review at the phase's landing)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 355ff78292acbe8985726d9fdf4a00bc48fb8e27, ad011cdced4fa84f78666ed7e55f2418b56b37e6, 89fa7b33b494da96746a6a4d4375a5ba178cfb81
- Tests: telperion-heavy 16 --threads 4 -- cargo nextest run --cargo-profile ci -p telperion-space -p telperion-core --no-fail-fast -j 3 -E 'not test(every_setting_changes_the_tree_by_degree_and_branches_come_and_go_at_vanishing_size) & not test(the_largest_tree_and_the_palm)', cargo nextest run --cargo-profile ci -p telperion-space -E 'test(growing_phytomer)'
- PRs: