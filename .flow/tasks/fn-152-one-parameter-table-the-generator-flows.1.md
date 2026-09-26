# fn-152-one-parameter-table-the-generator-flows.1 One parameter table the generator flows through

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
Stage 1 of 4 (the parameter catalogue). Every wire row is declared once, beside the code that reads it, through `rows!`. The declaration emits the family field (its doc comment is the row's meaning) and the row's catalogue entry. The wire, scalar validation, ordinary blending, the jev dial table and `docs/parameters.md` are generated from those entries. Output is byte-identical to master.

- **R1.** 249 rows, one entry each (`every_wire_row_has_one_entry`).
  - A row with no meaning, no stage or no bounds does not compile (`.flow/evidence/fn-152-one-parameter-table-the-generator-flows/build-time-checks.txt`).
  - Wire JSON and `presets.generated.ts` are unchanged (rebuilt, no diff).
- **R2.** Validation and blending are generated, with named relational checks kept (age, trunk height, the connector, the envelope, supernatural and hue products, card, supernatural terms, growth overrides).
  - Four digests recorded on master's code before the refactor still pass (`tests/catalogue_identity.rs`): presets, 145 walks, None/Some overrides, and parse+validate over every row off its rail.
  - Checks keep their error names and order through a per-site rank.
- **R3.**
  - `data/dials.json` and `dials.excluded.json` are deleted. The generated table matched them on every field of all 224 dials; the three deprecated dials were dropped.
  - A config names dials by id plus overrides at a table revision; a stale revision, an unknown id or a stray override is refused.
  - The fn-68 pilot config still replays on its own dials, at the calibrated hash.
- **R4.** `canopy.spacing`, `clump` and `clumpSpan` are deprecated, have no dial, and still parse, overlay and serialise.
- **R5.** The workspace gate (129 binaries, 971 passed, 0 failed) and `npm test` (129 passed) are green.
- **Also:**
  - The capability classifier no longer requires a rosette for leaf bases (red, then green).
  - Artifact sizes are within budget: main +15,858 bytes, render +25,041, field +14,787 (2.4 KB of headroom left).

Open:
- The pilot `leaves` dial window [2, 12] exceeds its row's 1..8 and is hash-pinned; it is kept, named as an exception in the test.
- A wire with two malformed-type values now names the first in catalogue order, not the old macro order.

stage: impl-review - skipped(config: REVIEW_MODE=none)
Tier: session model (claude-opus-5-5)
## Evidence
- Commits: 39f6f4bebd1000411af8cf26e19107972b31f8bf, f82d8c4777dd9e578d0f27252a046acd43a6e88f, d6208293cbd74cdabb8c641fb62703fa1c7c16be, 7db21fa408c6c70d74b080a45dd09bbda2aea0a8, 684901e18d96c42299261c22af7b27347c82f345, ba6650c798b2b03615248e167a64cf66831ad388, 3171468dfc28fe630127fa2ff683658e0705d0ae
- Tests: cargo test --profile ci --workspace --no-fail-fast (129 binaries, 971 passed, 0 failed, 21 ignored), npm test (12 files, 129 tests passed), node scripts/artifact-budgets.mjs after npm run build (all six artifacts within budget), baseline: none (spec lists no Quick commands)
- PRs: