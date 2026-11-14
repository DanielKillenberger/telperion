# fn-360-a-runs-girth-eases-wherever-it-drops.1 A run's girth eases wherever it drops

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
A run's drawn girth now eases wherever the wood drops, by one stateless rule (host's final form): every segment across which the radius falls lays a hold of that fall, fading from the ring it fell from over twice the radius it fell from, and the holds add, `g(s) = r(s) + Σ δ_j · w((s − s_(j−1)) / 2a_j)`, with the 1-in-1 floor kept (a twig of the oak at 200 needs it). The fork's separate window and the surface's fork flags are gone. Tests: `ease_tests.rs` (two forks, growth-unit steps, never-thinning run, creeping stem, statelessness); `shoulder_tests` and `fork_tests` changed to the ease. Oak 200: forks at the 1-in-1 floor 4 → 0, the trunk's largest fall 9.3% at the second fork → 4.4% at k 125, as on paper. Stem 120's growth-unit bands 4.0% and 3.9% → 0.3% and 0.4%. Wood vertices unchanged on all four trees. The sheet shows no ledge, shoulder or band round the trunk (host viewed and accepted). Laterals: none floats off; newly inside at most 2.3 mm; those already sunk sit up to 3.5 cm deeper (host accepted). Numbers, decisions and the two widest girth/radius sites are in `.flow/evidence/fn-360-a-runs-girth-eases-wherever-it-drops/MEASURED.md`; the upstream cycle pin fails identically on base (pre-existing, recorded there).

Tier: you execute; you do not reason about how the system should work (model: claude-opus-5-5)

stage: impl-review - skipped(policy: host-deferred - conductor owns the gate)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: a8079b8f5, 50a1b5b68, c35da63b7, b5ebcb506, c526b1531, 5dac5943a, 6ba252b0b, e0c7930fb, b73e5f371, e7444be0f, e42e5154d, e964cf7c6
- Tests: cargo test --profile ci -p telperion-core --lib -- surface fork bark the_axes_move_no_leaf (83 passed, 0 failed, on e7444be0f), cargo test --profile ci -p telperion-core --lib -- every_preset_grows_the_tree_it_grew (fails identically on base b93621b5b: pre-existing, not this change), red first: ease_tests two_forks and growth-unit steps failed on base (50a1b5b68)
- PRs: