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
- After each batch: `flowctl validate --all --json` must show `total_errors` 1 (the fn-82 error that predates the spec); `flowctl list --json`, `flowctl show fn-237-release-the-tree-space-engine-replaces --json`, and `flowctl spec chain <id> --json` for open specs succeed (fn-82 exits 2 on the base too).
- Then grep every tracked file outside `.flow/` for each removed path's `.flow/evidence/...` suffix; any hit is a keep-rule miss: restore the file with `git checkout HEAD~1 -- <path>` and fix the rule.
- Kept readers' inputs: the script keeps every file a kept script or JSON names by basename or relative path, and a kept script's whole directory (R1). After the first batch run the documented example, one cargo command at a time: `cargo build --release -p telperion-core --example geometry_benchmark`, then `python3 .flow/evidence/fn19/onboarding-examples/verify.py`; on the base (`0a62c7ec2`) it already fails with `ValueError: unsupported protocol definitions/schema` at `runner.py:115`, after it has read its inputs; after the prune it must fail the same way, never with `FileNotFoundError`.

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
- [ ] `flowctl validate --all` reports exactly the one pre-existing error after each commit; `list` and `show` succeed; `spec chain` succeeds on every open spec except fn-82, whose exit 2 for the missing fn-111 is unchanged from the base.
- [ ] `python3 .flow/evidence/fn19/onboarding-examples/verify.py` (after the release `geometry_benchmark` build) ends exactly as on the base (the same `ValueError` at `runner.py:115`), not with a missing file.
- [ ] No tracked file outside `.flow/` names a removed path.
- [ ] The task's done summary gives `git diff --shortstat origin/master HEAD` before and after, and the files and lines removed per batch.

## Done summary
Closed specs fn-1..fn-99 (3f82a0c09) and fn-100+ (f4f9a69c2) are pruned to their result and verdicts. 358 files and 13,648 lines went in batch 1 and 346 files and 16,608 lines in batch 2. d3faccf31 restored 27 evidence files that live records cite by file name next to the spec, after the round-1 P1. prune_list.py now keeps those files, and R1 states the rule.

Numbers (files / insertions / deletions): before 4,165 / 255,825 / 406,703; after batch 2 4,297 / 242,201 / 423,335; after the restore 4,300 / 245,060 / 422,110. Diff f6d89317d..HEAD: 680 files, 53 insertions, 26,225 deletions.

Checks after the restore: `flowctl validate --all` reports 1 error, the fn-82 -> fn-111 dependency that predates this spec. `flowctl list` returns 295 specs, and `show fn-237` returns 0. Of the 98 open specs, only fn-82 fails `spec chain`, for the same reason. No tracked file outside .flow/ names any of the 677 removed paths. `gate classify` puts the diff in tier B (docs-only).

Follow-up for the conductor: memory capture was not run. The task's FORBIDDEN scope excludes `.flow/memory`, and the lesson to capture is that a prune keep-rule must also match citations by file name next to the spec.

Tier: (model: claude-opus-5-5)

stage: impl-review - ran [round 1 NEEDS_WORK rid 6e9b1828bad845359f687d2167941385 .. re-review SHIP 2026-10-24 14:08]

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 3f82a0c0910ea76762340750ace2ed4a6438eeb3, f4f9a69c236ab932f2b623a1bca131f5f5ba8ece, 3e8dbcf2146978d32541bbcf4844a229f5c20714, 548c4b55c9de5262056cb785ec618eecd34724a2, 8b4b57db994f9bd22d2bf0019d49da686596eb23, d3faccf31994940c842623fb2f96baa5ced98af1
- Tests: flowctl validate --all --json (1 error: fn-82, predates spec), flowctl list --json (295 specs), python3 check312.py removed-now.txt (outside-.flow hits: none; chain failures: fn-82 only)
- PRs: