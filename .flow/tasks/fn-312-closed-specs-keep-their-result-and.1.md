---
satisfies: [R2, R3]
---
# fn-312-closed-specs-keep-their-result-and.1 Prune the closed specs' task files and evidence, in two commits

## Description
Apply R1's keep rules to the 157 done specs outside fn-237's direct dependencies: `git rm` their task pairs and the evidence the rules do not keep. One commit for fn-1..fn-99 (66 specs), one for fn-100 and above (91 specs), so each diff reads on its own.

**Size:** M
**Files:** `.flow/tasks/<pruned spec>.*`, `.flow/evidence/<pruned spec dirs>/**`
**Touches:** [.flow/tasks/**, .flow/evidence/**]

### Approach
- The delete list comes from `/home/daniel/.cache/claude-tmp/claude-1000/-home-daniel-Projects-telperion/dd1a391e-8087-41d5-8d26-b3ea78d9cb37/scratchpad/prune_list.py <lo> <hi>`, run from the repo root (prints paths to stdout, a count to stderr, and writes `prune_keep.json` beside itself with each kept file's reason). Recompute it at commit time, never reuse an old list.
- Record before each batch: `git diff --shortstat origin/master HEAD` and the per-dir numstat for `.flow/evidence`, `.flow/tasks`, `.flow/specs`.
- Delete with `git rm -q --pathspec-from-file=<list file in scratchpad>`; the dcg hook blocks `rm -rf` and variable-path redirects, so write the list with a Python helper or a literal path.
- After each batch: `flowctl validate --all --json` must show `total_errors` 1 (the fn-82 error that predates the spec); `flowctl list --json`, `flowctl show fn-237-release-the-tree-space-engine-replaces --json`, and `flowctl spec chain <id> --json` for open specs succeed.
- Then grep every tracked file outside `.flow/` for each removed path's `.flow/evidence/...` suffix; any hit is a keep-rule miss: restore the file with `git checkout HEAD~1 -- <path>` and fix the rule.

### Investigation targets
**Required:**
- `.flow/specs/fn-312-closed-specs-keep-their-result-and.md` — Checked, R1 and its keep list
- `docs/evidence-retention.md` — what a cleanup must check
**Optional:**
- `.flow/evidence/ARCHIVE-MANIFEST.md` — the 2026-10-16 precedent

### Key context
- Four FRICTION.md files (fn-186, fn-209, fn-211, fn-214) stay: no host has disposed of them. The script holds them.
- fn-209's evidence dir is named `fn-209-tree-space-f1-trees-resolved-on-demand` but its spec id differs; legacy dirs like `fn9`, `fn19`, `fn-185` map to their spec by number. The script handles both.
- Commit trailers per the conductor's brief.

## Acceptance
- [ ] Two commits remove only files `prune_list.py` lists for their range; nothing of an open or fn-237-dependency spec changes.
- [ ] `flowctl validate --all` reports exactly the one pre-existing error after each commit; `list`, `show` and `spec chain` on open specs succeed.
- [ ] No tracked file outside `.flow/` names a removed path.
- [ ] The task's done summary gives `git diff --shortstat origin/master HEAD` before and after, and the files and lines removed per batch.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
