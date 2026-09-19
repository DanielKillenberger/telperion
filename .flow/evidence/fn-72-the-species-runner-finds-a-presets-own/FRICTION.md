# Friction

## 2026-09-20 — Receipt conflict was only its final newline

On renewed owner direction to merge, an exact comparison showed the add/add task-receipt conflict was solely a missing final newline on the PR branch. The main checkout's concurrent merge was also cleared. Normalizing the receipt's final newline preserves all evidence and avoids manual content selection. Diagnosis took less than a minute; normalizing generated receipt files at write time would prevent this landing interruption. No new spec was created.

## 2026-09-20 — Landing held by concurrent checkout work and a receipt conflict

The owner requested a squash merge of PR #45. CI has three passing and three skipped checks. GitHub reports DIRTY; a read-only merge-tree check identifies only an add/add conflict in `.flow/tasks/fn-72-the-species-runner-finds-a-presets-own.1.md`. Meanwhile another session started an fn-87 merge in the master checkout, with unresolved source conflicts. The host left that session's work untouched and did not merge or push fn-72. Diagnosis cost roughly two minutes, beyond the landing skill's instruction read. NEEDS_HUMAN: reconcile the receipt on fn-72 and finish the other session's use of master, then re-check and squash-merge PR #45. Serializing merges into the shared checkout and reconciling task receipts before landing would remove this friction. No new spec was created; fn-68 remains paused for tomorrow.

## 2026-09-19 — Host full gate lacked the generated Wasm artifact

The host's first full Vitest run in the fresh worktree passed 96 tests and failed 11 because src/browser/telperion.wasm was absent (ENOENT). The ignored artifact had not been built. The host started npm run wasm:build before rerunning; compile cost reported at this point was 21.9 seconds. This is gate preflight friction, not evidence of a generator regression. Running the repository's prerequisite build before a direct Vitest command would avoid the failed gate pass.

## 2026-09-19 — PR skill preparation outweighed this change (host report)

The host reached make-pr preparation: workflow.md alone reports 30,665 tokens, before mandatory cognitive-aid/create-finalize references. Its first read truncated after about 14k tool-output tokens. For this three-criterion path-resolution fix, the host paused that PR stage under the owner inefficiency rule. An owner-approved lightweight PR path for tiny specs would remove the cost. No friction spec was created.

## 2026-09-19 — SSH diagnostic repeated the setup delay (host report)

A second SSH diagnostic stalled for about 30 seconds. Bounded HTTPS using the existing gh credential helper completed in under one second and confirmed origin master at 3345b07f. The host will use a per-command HTTPS override for publication; no permanent configuration changed. Avoiding further SSH calls removes this local setup delay.

## 2026-09-19 — The browser's generated preset list is not the runner's

Writing the coverage case for every shipped preset, the only preset list a JS
test can read was `src/browser/presets.generated.ts`, and it omits the European
beech: `params.rs` splits the listed `CATALOGUE` from `IN_WORK`, and the beech
sits in the second, unlisted in the browser but reachable by the runner and by
`Preset::from_id`. The test failed on the missing id before that split was
visible. Cost: one test run and one read of `params.rs`, roughly three minutes,
no pilot tick. The case now reads both native tables directly. A single exported
list of every preset the runner may be handed - or a note in the generated file
that it carries only the listed half - would have removed the detour.

## 2026-09-19 — Optional worktree fetch stalled

The installed manager was found under the worktree skill's scripts directory. Its optional SSH fetch of `origin/master` stalled without output for roughly 30 seconds. The host terminated that specific SSH subprocess; the manager's documented optional-fetch fallback created the worktree from existing `origin/master` at `3345b07f`. Cost: roughly half a minute, no pilot tick. Bounded optional fetches or a no-fetch option would avoid this. Remote publication remains unverified.

## 2026-09-19 — Worktree manager missing from documented install path

While setting up the owner-requested isolated worktree, the skill's prescribed `/home/daniel/.codex/scripts/worktree.sh` command failed because the file is absent. Cost: one failed command and a local installation-path lookup; no implementation or pilot tick spent. A working installed manager path would remove this setup friction. This is a local installation issue, not a repository feature to spec.
