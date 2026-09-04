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

- [ ] README no longer states space colonization as the whole skeleton technique, and the skeleton row names the depth input
- [ ] `src/index.ts` header carries the same correction
- [ ] `colonize.ts`'s header describes its actual scope and points at the local pass
- [ ] `grow.ts`'s `defaultGrowth` comment describes the derivation that exists
- [ ] `package.json` description reflects both branching methods
- [ ] `preset.ts`'s "only bending stiffness" claim is checked and corrected only if task .2 falsified it
- [ ] fn-1's frame budget, shell-cull fractions and leaf count are re-measured and updated in its spec body, with superseded figures marked as such
- [ ] `npx vitest run`, `npx tsc --noEmit` and `npm run build` green

## Acceptance
- [ ] TBD

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
