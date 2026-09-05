# fn-5-branch-until-the-tips-bear-leaves-one.6 The docs that still say one algorithm, and fn-1's numbers that no longer hold

## Description
Retires every claim that the skeleton is space colonization alone, and re-measures the three numbers fn-1 recorded on a tree that no longer exists. Folded into one task because they are the same edit landing in several places, and because leaving fn-1 quietly stale is how a spec's evidence rots.

**Size:** M
**Files:** `README.md`, `src/index.ts`, `package.json`, `src/skeleton/grow.ts`, `src/skeleton/colonize.ts`, `.flow/specs/fn-1-the-canopy-real-leaf-geometry-culled-to.md`
**Touches:** [README.md, src/index.ts, package.json, src/skeleton/grow.ts, src/skeleton/colonize.ts, .flow/specs/fn-1-the-canopy-real-leaf-geometry-culled-to.md]

### Approach

- `README.md:40` states space colonization as the skeleton technique, unqualified. It is now one of two, handed off at a crossover. The pipeline table's stage count is unaffected: this is all inside the `skeleton` stage, which stays one of five. The skeleton row's inputs gain the depth parameter.
- `src/index.ts:9` mirrors the same claim in the header comment and needs the same correction. The repo deliberately keeps both descriptions in sync.
- `src/skeleton/colonize.ts:3-53` presents `colonize()` as producing the skeleton's topology outright. It now produces the upper structure; the header needs a pointer to the local pass and where the crossover lives.
- `src/skeleton/grow.ts:59-65` is the most directly falsified comment in the repo: it claims an influence radius of nine steps is "narrow enough that the crown does not collapse to a single mast," which task .1 measured as false and fixed. If .1 did not already rewrite it, do it here.
- `package.json` description names space colonization as one of the library's techniques. Light edit; the keyword itself stays accurate.
- **fn-1's Measured section carries three numbers taken on the pre-fn-5 tree**: the frame budget, the 14.8% and 6.5% shell-cull fractions, and the 2,904-leaf count. All three are invalidated by this spec. Re-measure and update fn-1's spec body, marking clearly which figures were superseded and why.

### Investigation targets

**Required** (read before coding):
- `README.md:36-56` - the "four things follow" list and the pipeline table
- `src/index.ts:1-25` - the header block mirroring the README
- `src/skeleton/colonize.ts:3-53` - the algorithm header whose scope changed
- `src/skeleton/grow.ts:56-79` - `defaultGrowth`'s rationale comment
- `.flow/specs/fn-1-the-canopy-real-leaf-geometry-culled-to.md` - the Measured section

**Optional** (reference as needed):
- `src/presets/preset.ts:10-15` - "four argument objects", confirmed still accurate and not to be changed

### Key context

- The repo has no CHANGELOG. Release notes live in commit bodies: a Conventional Commits subject, a prose paragraph of reasoning, then the two-line trailer. Write the landing commit accordingly.
- README and `src/index.ts` deliberately hold the same pipeline description in two formats. Update both in one commit or they drift.
- `src/presets/preset.ts:10-15` says a preset is four argument objects. That stays true: depth is a field inside the existing skeleton argument, not a fifth call. Do not "fix" it.
- `src/presets/preset.ts:41-44` claims `growth` carries only bending stiffness. Whether that is now false depends on where task .2 put the depth parameter. Check before editing.
- fn-1 is open only because its R8 owner sign-off is a no. Updating its numbers does not close it and must not pretend to.

### Acceptance
## Acceptance
- [ ] README no longer states space colonization as the whole skeleton technique, and the skeleton row names the depth input
- [ ] `src/index.ts` header carries the same correction
- [ ] `colonize.ts`'s header describes its actual scope and points at the local pass
- [ ] `grow.ts`'s `defaultGrowth` comment describes the derivation that exists
- [ ] `package.json` description reflects both branching methods
- [ ] `preset.ts`'s "only bending stiffness" claim is checked and corrected only if task .2 falsified it
- [ ] fn-1's frame budget, shell-cull fractions and leaf count are re-measured and updated in its spec body, with superseded figures marked as such
- [ ] `npx vitest run`, `npx tsc --noEmit` and `npm run build` green
- [ ] The clay comparison shows both presets at their shipping depth with every supernatural term at its preset value, so R8 can judge that Telperion still reads as Telperion and Laurelin as Laurelin (R8)
## Done summary
Every claim that the skeleton is space colonization alone is retired: the README bullet and pipeline table, the index.ts header, package.json's description and colonize.ts's header now describe colonization as the upper structure handed at a crossover to local rules that branch on to leaf-bearing twigs, and the skeleton row names the depth inputs. src/index.ts re-exports the whole depth surface five tasks left reachable only by module path (growReport, resolveGrowth, influenceRadiusFor, DEFAULT_STEP, GrowthReport; branchTwigs, resolveTwigs, DEFAULT_TWIGS, TwigParams, TwiggedSkeleton; shedTwigs, DEFAULT_SHED; DEFAULT_TWIG_TAPER), under the one-line-banner-per-group convention, and the README quickstart was extracted and typechecked against that barrel (rc 0, with a negative control that fails). fn-1's Measured section keeps its three pre-fn-5 figures marked superseded and carries a "Re-measured after fn-5" table taken on the CPU at this commit: shedding removes 8-9% of twig nodes on both presets, the leaf cull then finds 0.4-3.3% on Telperion and under 1% on Laurelin, leaves are 136,808 / 766,435 at eight orders, and the frame budget is stated as awaiting the owner's GPU sweep with no number invented. fn-1 stays open on its R8.

Verified rather than redone: grow.ts's defaultGrowth comment (rewritten by .1, no falsified text remains), preset.ts's "only bending stiffness" (still true; step is a sibling of growth), and the R8 comparison (buildComparison builds each preset wholesale, so both stand at their stated depth with every bias term at preset value; the shipping depth is the owner's call and both presets state zero orders until then).

Commit: 831692a on base 4f24688 (workspace wt/fn5-6). Files: README.md, src/index.ts, package.json, src/skeleton/colonize.ts, .flow/specs/fn-1-the-canopy-real-leaf-geometry-culled-to.md. grow.ts unchanged after verification. Nothing outside Touches.

baseline: green via handoff (conductor: 21 files / 325 tests at 4f24688; no gate receipt existed for HEAD, so gate check returned RUN for all three)
verify: green (npx vitest run 21 files / 325 tests; npx tsc --noEmit; npm run build 48.25 kB); gate classify: FULL (package.json); receipts written for unittest, typecheck, build at 831692a3

Run note: /home/daniel/Projects/telperion/.git/flow-notes/fn-5-branch-until-the-tips-bear-leaves-one-20260904T203339Z-685936/fn5-t6-docs.md
Measurements: scratchpad fn5-t6-measure.tsv (script fn5-t6-measure.ts), fn5-t6-quickstart-tsc.log, fn5-t6-verify-{vitest,tsc,build}.log

stage: impl-review - skipped(policy: parallel-wave - conductor reviews after integration; REVIEW_MODE=none)
## Evidence
- Commits: 831692a, 60805ae
- Tests: npx vitest run (21 files / 325 tests on the joined target), npx tsc --noEmit clean, npm run build clean, README quickstart extracted and typechecked against the real barrel by the conductor: clean, all thirteen depth-work exports present in src/index.ts, checked by name
- PRs: