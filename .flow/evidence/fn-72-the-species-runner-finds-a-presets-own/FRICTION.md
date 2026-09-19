# Friction

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
