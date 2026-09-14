---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-36-reference-matched-stills-and-the.1 Implement Reference-matched stills and the species QA pass

## Description
Reference-matched stills: a shot block per reference record, --camera and --no-figure on the headless target, one matched job per record in the species runner with a horizon-sun twin, the compare script and the first measured comparison, run on fn-34's beech and birch.

## Acceptance
- [x] R1 shot block on the fn-19 reference record, optional and closed; existing records validate; out-of-range refused
- [x] R2 --camera and --no-figure on the headless target; fixed views byte-identical; fill on any aspect; device test
- [x] R3 one matched job per record at the first fixed seed plus a twin; pair composite and comparison JSON per job
- [ ] R4 beech and birch carry shot blocks on whole, bare and base references and round 2 is rendered and tabulated; the owner's verdict on the pairs is open
- [x] R5 template and onboarding doc judge on matched pairs (PR #24); tests for parsing, ranges, fill on three aspects, the figure's frame, schema validation and the compare self-test

## NEEDS_HUMAN

R4's last clause is the owner's: the six round-2 pairs for the beech and the birch are rendered, measured and on the judging page; fn-34 records the verdict. Gates on the branch: cargo test --release -p telperion-render (all pass, shot device test included), cargo test --release -p telperion-core (no failures), npm run typecheck, cargo clippy -D warnings, uv run scripts/compare-references.py --self-test. PR #25.

## Done summary
TBD, after the owner's verdict on the round-2 pairs.

## Evidence
- Commits:
- Tests:
- PRs:
