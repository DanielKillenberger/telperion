---
satisfies: [R1, R2, R3, R4, R5, R6, R7, R8]
---
# fn-157-a-species-values-are-the-confident.1 Implement A species' values are the confident aggregate of everything written about it

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
The literature stages gather everything written about a species, read each document once, class its kind, and compose each value in code from the best tier of sources that agrees; a value gates only when a flora, forestry or garden tier agrees on it (host decisions of 2026-09-26). R1: the beech from its native range has a height of 40 m from three agreeing forestry sources. R5: the Oregon white oak, run from a bare seed, lands within its catalogue ranges. R8: a photograph found from a name gets a shot chosen from code's candidates; the beech ran live through Tune (capped at three rounds, crown 13.4 m to 17.7 m kept) and Gaps (0 reachable, 7 identity of which 3 wait on fn-61 and 4 are the host's to assess under fn-62, 0 global, 5 unsourced), stopping at the owner's look. The owner does not accept this beech: it waits on fn-61. The gate replays the beech through Start and through Tune's baseline and one round (hardware GPU; CI skips), and the oak through Start. The branch is rebased onto origin/master (fn-152) with no conflicts; Tune's round 1 was re-recorded for the catalogue's dial table.

Known limits (R1, partial): the beech's leaf width is 11.4 cm from one extension page, outside the flora's 2.5-7 cm (contextual, never gating); every oak value rests on one source (contextual). The oak kept no photograph: the photograph search favours European species (follow-up, not built).

- Start gives the measurer a frozen, ready copy with every metric gating or contextual: tests/tiers.rs `start_gives_the_measurer_the_profile_it_derived_from`
- PDF by magic: tests/fetch_pdf.rs (red first)
- Agreement gates, and only a flora, forestry or garden tier gates: tests/tiers.rs `a_value_gates_only_when_its_tier_agrees`; tests/replay.rs (beech height gating; crown width and trunk diameter contextual, with why), red first
- Round cap: tests/tuning_engine.rs `a_round_cap_ends_the_revision_after_its_rounds` (red first)
- Gap classes: tests/runner_stages.rs `every_failing_trait_is_classed_reachable_identity_or_global_with_its_evidence` (reachable only when a judged attempt passes the trait; assessment by `trait` or `covers`; host to assess otherwise) and `gaps_md_keeps_each_gap_to_a_readable_line`, red first
- R5: tests/replay.rs `the_recorded_oak_replays_offline_and_lands_within_its_catalogue_ranges`
- R8: tests/replay.rs `the_recorded_beech_replays_through_tunes_first_revision`
- Chosen shot: runner::shots::candidates tests; reference-first.py `shot` and `outline` stages (test-reviewers); runner::cells `the_kept_views_are_the_required_cells`
- Found references named by view and recorded once: tests/photos.rs (red first)
- Replay stability: compare-references self-test, test-reviewers (unstable keys, rekey, newest answer kept), tests/reference_first.rs, tests/bundle.rs, tape::trim rekey test, each red first
- Trim: tape::trim tests (kind and appearance sentences whole, unread pages empty, PDF magic and parse); `tape_trim --check` fails on a finding (tests/tape_trim_check.rs)
- Tier rule, median Start, best-tier appearance, native-range queries, photograph categories (decisions 1-6): tests/tiers.rs, data/cases/aggregate.json, gather and tests/leads.rs, tests/replay.rs
- R2: tests/aggregate.rs `a_single_specimen_is_the_fields_maximum_never_its_typical_value`; R3: runner::folder `only_a_page_declaring_an_open_licence_is_copied`; R6: the replaced stages, decision kinds and question sets are deleted
- Fixtures: beech 447 files, 11 MB; oak 244 files, 1.4 MB; `tape_trim --check` clean on both
- Follow-ups for the host (FRICTION.md): Tune's proposal calls over every dial, a harness view of a run's kept tree (fn-166 captured), non-European photographs, proposal batches stable under a table reorder
- Gates on the rebased head: `cargo test --profile ci --workspace --no-fail-fast` 1048 passed, 0 failed; `npm test` 110 passed (13 files); `python3 scripts/test-reviewers.py` 16 OK

Tier: session model (Opus 5.5)

stage: impl-review - ran [codex round 1 NEEDS_WORK (4 findings fixed) .. rounds 2-5 SHIP; round 5 over the six commits after the rebase]
## Evidence
- Commits: b7bc5535b81c1ce37d51812c13396e30ca2f083d, 40a871c737edf2847f483c770758c8bb1cf3b6a0, c90058531e60e2b5bcf86a7a483175964a317e86, 9f220db3ecb757bf689194ae827565e78a275f84, a6651671a8ab645208f335ac958169a528d8d32e, c7faac7b496fef09aa1b28031f56f5a072f9763b, 36fcef3869dca067e0afcdce3229bcb663e2bf1e, 985f0d9d2665ee6eaaafc96395df2490718cde26, ad0f5a9fb64582292ef38db75dbcdf65d7e131eb, 6ad4bb23f2256538b619d090c9fb5e372054b247, 98c4e88e7aafb67e58117a8f32735f416a74c1b3, 74a5c3776b28a6646832cf479d4f911d147716ea, 5337d04b77f1dfc68ff83c1aef5b2bb70a21ed5b, 2399643e683a00f5e683c2f468e66789b416d1a2, 244edcf2787898756fb900f7003eae1f34334393, c045009e5ef5881f214006aff6255e65e390c0ba, 39006824a78e186b6f0a95e5b13f5d16d0aad28a, 5d4849b82125c601bd26845b7b25731724c41841, 2a5fb0d8a2a344abd92350228daf346b8aa64f80, ec5c642dc34c9ebd1b69c208672f709cfb68cabc, cd73a8729068c23f9ffba77d6a20a25860c90f64, 586c4c1f7532c1d7d56180bd7129b191aae8b64b, e01712cc21b1564a94d056d5cb6513b8ad5f1bd0, 43f89c18e04b697ca41a6c67246a5ce74e95d194, feb43bfedc5eb2c53bd9ea54a276c6d49a383973, 4dc197aae8488381b700b996f424638ccb8c5c06, def77edf2e589a25125eff661e7f45b4c7b7b204, 4552733b7004d96feda995b04d1388438fd05869, 4e437ae4837ad7eaf0c87ca41b4753083b7155ef, 7ce335b92dc1cd9d803cf01c5e958810529649aa, 80b8defde4b6914e7fb254d43d28eb7d36ec8858, 5717927857c1f6b7a338b1cbb2bcd237fc6ae96a, 2918ad1ea0f08ac8ae3a1492a1294d4790279b68, 8c6987e3d2f2c4a0983f06a73f8680d21e05e4b1, 2e74bea7ef00513b6138f02571c2593294c0669c, 7fc34ae09c6c1067cdcab1e1dd7314f85bfb550b, ada598cd2081ab63fcf645a44d9f30248edb76df, 65015750d0aa42a75a1cd6c7e839ac58c179c25d, 1ed1fe4c00daab4818fbf3d3659137c07e2c1eba, 8af0a8bc66af0ed5750090b3c7b0a9aae2b24b37, 6079da3faf4f8b63cd3c24b07cb28197b5022aa9, 6ff3d1a2caa455cefa98d3f3ae0fd84ae5e0f430, 26eb6fa11d9dffbcf14012a7b0e98e991f0bce99, 464ad795f01440e15e4e652529fd3b7219b5f272, f7b29058740001ee7f4a34ab17cc9b5c21727832, 9823a801362e9ffbf6b986c22b6a8c236e48af0f, 77c4a163ac864378a1f273973013c2c182445c1c, 22139f0d618b58c3af5fdf516709e46ff876687d, ad564da40344c5a93a97ac8cfbd9a9c23858c0d7, 48eb4405ebe7d1679bd683e8c536441d6e98e074, a3c8f8dedac6db6a6f04b2ebcc2bd555f90d42b0, 0731498311c9b68d6e64df38fc204affa4def789, f9a6a5af5510aa92986c9816f3b507b59997bfa1, deec6dd1c19bfc028a4753cff05132d2f6578196, 9da4ae6b9223c1c28352a28fc2afa9f3260d1a27, 5e1276076ebbd673b0fe0a89d01b07ab1397176b, 0dc21d8c88175c8af1d702b350f68d1de8a84de2, f24863d17b3363a28d9fa5fd496abe81c7190877, c2f86c301099922aa820bb9b69ff1d09266f1649, 1442983938ad4b7fc79f06a89947f7aebb90e44c, 4f19b1f826b80d0cab3bd0c7f0fcc2143b4fe3a9, 09cf5a05868ee2fa2b1f1830a65f604c6407a36f, 3b96ad67b76f6cfb64a41d610ca927947320c8bc, ba1f11948c35602b3143fc9166de4c951ef6a0bd, bbb00659178c539039f2ece478cd88ef0cd6924e, e3b1238b9fbb8b2a1353a16053022a8d05979d8b, a7c126159edbb9fe781250bb86323f345fb16048, 71c419dc3faa0ce83bf4d26da4e00758ecb249cc, a044f6dacc14881d9504ecc4ad60d66ad3b5b9df, 610f29b2dba093cdcbfde0ae5532287b585248e0, 4c5cf6b73cc176bf5e3a7c619adc54653ee790cd, f3d353cedf6f91febe65e30dadd5299c895909d6, 334f02cb31b523281dc90497da2c114f4f21093c, f3a0971637c321c7525b343da0a74295ed51ebb9, 57632c0d4c1ec893618ff945d568b113fa6afbb6
- Tests: cargo test --profile ci --workspace --no-fail-fast, npm test, python3 scripts/test-reviewers.py
- PRs: