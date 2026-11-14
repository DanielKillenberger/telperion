# The species runner finds a preset's own reference records

## Goal
`node tests/species.mjs --quick european-beech` failed with `No matched reference records` because the runner defaulted to fn-9's profiles while the beech's and birch's records live in fn-34's; fn-55 paid a failed pair of runs to learn to pass `--profiles` by hand (friction, 2026-09-18). Under the owner's rule that a friction entry is specced before its spec closes, the runner resolves a preset id to the profile set carrying its records, the flag kept as an override.

## Decisions
- One resolution from preset id to profile set fills the default; `--profiles` still wins; fn-9 stays the set for oak and spruce. No pipeline change, no change to profile contents or matching, no new species.

## Acceptance Criteria
- **R1:** `--quick european-beech` and `--quick silver-birch` run without `--profiles` and match the fn-34 records; a preset in no known set fails naming it and the sets searched — met.
- **R2:** `--profiles` still overrides; oak and spruce resolve to fn-9 unchanged — met.
- **R3:** A test covers the resolution for every shipped preset (listed and in-work) — met.

## Result
Resolver in `scripts/species-profiles.mjs` with tests in `scripts/species-profiles.test.mjs`; host verification 2026-09-19: `npx vitest run` 107 passed, catalogue check 5 species pass. The CLI reached the right reference ids (B-WHOLE/B-BARE/B-BASE, S-WHOLE/S-BARE/S-BARK); actual headless captures were not run (binary unbuilt). Landed as PR #45 after the owner's merge direction (2026-09-20).

## Open
Proposed, not specced: a lightweight PR path for tiny specs (later `docs/pr-format.md`); one exported list of every preset the runner may be handed.

## Friction
- Browser's generated preset list omits in-work presets (beech) -> test reads both native tables; proposed an exported full list.
- make-pr skill reading (~30k tokens) outweighed a three-criterion fix -> paused under the inefficiency rule; led to the short PR format.
- Fresh worktree lacked the generated Wasm, 11 Vitest failures -> build Wasm before direct Vitest.
- PR landing blocked by a concurrent fn-87 merge in the main checkout and an add/add receipt conflict that was only a final newline -> NEEDS_HUMAN, then normalised; proposed serialised merges.
- SSH fetch stalls and a missing worktree manager path -> local setup, not specced.
