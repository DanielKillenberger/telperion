# fn-89 friction

## 2026-09-22 — the spec's gap-check contract lives on a commit this branch does not carry

The spec's newest decision ("A tuning run ends with a gap list") binds the
conductor to fn-95's reachable/covered/new check, but fn-95 and the specs
fn-94 to fn-106 exist only in commit 57855e4a on the main checkout's branch,
not on this worktree's base (44aaccce). Reading the shared gap-list shape
meant `git show 57855e4a:.flow/specs/fn-95-...md` from another branch's
commit. Cost: about three minutes and one wrong `ls`.

What would have removed it: a spec that binds to another spec's contract
names the commit or branch that holds it when that spec is not yet on
master, or the worktree is cut after the referenced specs land.
