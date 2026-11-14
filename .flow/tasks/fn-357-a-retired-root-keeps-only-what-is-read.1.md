# fn-357-a-retired-root-keeps-only-what-is-read.1 A retired root's stores let go; the tree equal to the bit

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
A retired root now leaves every per-axis store at compaction and keeps a 24-byte Stand (bearer, node, birth, vigour, living) that the settle's fades and an ancestor's evaluation carry into its bearer. The tree is byte-equal: narrow suites 770/770, a new let-go test, and the oak 80 digest is unchanged. The oak at 120's grower footprint went from 7.106 to 4.795 GiB (-32.5%). Evidence is in .flow/evidence/fn-357-a-retired-root-keeps-only-what-is-read/TASK1-DONE.md and READING.md.

Tier: product code, built by Opus to the host's written design (model: claude-opus-5-5).

stage: impl-review - skipped(config: REVIEW_MODE=none; one codex review at the phase's landing)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 954543d7d2b9a28ed39d7d252a95e70b84d389bd, 42986038243fa07b92bee8dcdb6cd8fe2ad5b192, 61fcfe5dbb4ba6a657a296af37748c1f0ba9faaa, 0859bcc271a8d3c5540ee3b74064f91202397ebf
- Tests: telperion-heavy 16 --threads 4 -- cargo nextest run --cargo-profile ci -p telperion-space -p telperion-core --no-fail-fast -j 3 -E 'not test(every_setting_changes_the_tree_by_degree_and_branches_come_and_go_at_vanishing_size) & not test(the_largest_tree_and_the_palm)', cargo fmt --all --check
- PRs: