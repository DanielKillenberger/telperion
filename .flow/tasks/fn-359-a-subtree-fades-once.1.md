# fn-359-a-subtree-fades-once.1 A subtree fades once: build, tests, measurements

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
A subtree fades once (commits `0289bdd02..19a86ceca`). Living tips the grower holds above scale 0.1 that are drawn under 0.01, oak seed 1001: 2,411 to 38 at 120, 952 to 1 at 200, 2,369 to 6 at 250, 2,304 to 12 at 280, 1,963 to 1 at 300; seeds 1007 and 1012 at 120: 5,296 to 235 and 3,912 to 134. Phytomers drawn rise 3.7% at 120. Heights, trunks and the three largest limbs are unchanged. The girth share is the applied fade (host decision on the builder's stop). No existing test changed; one pin test repinned where nested fades sit (oak 1, beech 1, spruce 1 and 7).

Host's read of the sheet (viewed): at 250 the same dying skeleton with a few more twigs; at 300 the same bare 7.8 m post with three small growths near its top, one a short thick stub (wood at a tenth of its length and 0.36 of its radius). Not worse, and not the cure: the tree at 300 holds 4 m² of leaf in the grower; the crown's renewal is Phase 3's rule.

Accepted short of R4 in one respect: one-thread seconds were read once each in a loaded machine and are missing for six of fourteen rows (115.8 to 130.8 s at 120, 185.4 to 191.3 at 200, 149.6 to 161.0 at 250, 230.1 to 215.8 at 300). The cost follows the 3.7% more phytomers drawn; the phase's scoreboard measures it properly. Not checked: no narrow test compares a drawn tree with its grower's rough layout. Review skipped here; it runs at the phase's landing.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 19a86ceca
- Tests: cargo test --profile ci -p telperion-space (narrow: shed, fade, girth, paused, year, repeat, let-go, closed form)
- PRs: