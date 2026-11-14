# fn-366-the-growth-paths-download-measured-and.1 The growth paths download measured and brought down

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
The growth path is measured (scripts/growth-path.mjs, its test, a line in the budget script) and levers a, c and e are applied: 419,983 to 364,870 Brotli bytes, 114,870 over P11's 250,000. No lever moved the oak's tree digest. Readings and tables: .flow/evidence/fn-366-the-growth-paths-download-measured-and/READING.md and LEVERS.md.

Not applied: b stopped (the native catalogue writes maxInstances as a 64-bit usize::MAX, so presets.generated.ts is not byte-equal); d has no narrow way; f(s) wins 48,013 on top of a and c but makes the oak about 20% slower; g, measured only, saves 75,885 alone with a stub that serves no field. Held for the host: telperion.wasm's ceiling, because the budget tests replay fn-170's 1.41 MB module under it. module.js's ceiling went from 70,000 to 63,000. test-dist's refused case (telperion seed 7 at 64 sides) is red on the base too.

baseline: red (scripts/test-dist.mjs refused case, inherited)
stage: impl-review - skipped(policy: host-deferred - conductor owns the gate)
Tier: opus (model: claude-opus-5-5)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: e3c5d3e7f71229559a0faedabceebeb6646a8c40, 5321d1b8572d4af0203689b28a3ef2d51bfc0763, 055489df08f1e4c177b6b459cdee6256bd7a55c9, ef7acb165e0f80cb523988a4cc717abe16e48984, 6dba9930f26fe54639d65a2b2c8192430eb12e04, 51651530998ac365a190b33e95a17de30702d15c
- Tests: npx vitest run src/browser/presets.test.ts src/field/field.test.ts src/field/compile.test.ts scripts/growth-path.test.mjs scripts/artifact-budgets.test.mjs, cargo test --profile ci -p telperion-wasm --lib, cargo test --profile ci -p telperion-space --lib cores, npm run build && node scripts/artifact-budgets.mjs, node scripts/test-dist.mjs (24 of 25 cases green; the refused case red on the base too), cargo test --profile ci -p telperion-core --test generation_limit_guard, cargo test --profile ci -p telperion-render --lib compile_size
- PRs: