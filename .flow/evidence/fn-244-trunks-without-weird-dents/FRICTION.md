# Friction, fn-244

## 2026-10-09: The still runner cannot switch a row off

While switching each surface candidate off in turn (R1), I found that the `space_*` still runner has no flag for a wire overlay on the family (lobes, flare, bark rows) and none for a forced level. The tuning flags only reach the engine species. To get around it I wrote a scratch example with `params::overlay` and a probe that prints the trunk's radius profile.

- **Cost:** about 15 minutes and two extra builds.
- **What would remove it:**
  - an `--overlay <json>` flag (and `--level N`) on `space/still.rs`;
  - a `two_trees` base shot.

## 2026-10-09: The command hook blocks loop redirects and `git checkout --`

The `dcg` hook refused a loop whose redirect target was built from a variable. It also refused `git checkout -- <file>` when I tried to revert a diagnostic edit. I worked around both with a script file and `git show HEAD:<file>`.

- **Cost:** about 3 minutes.
- **What would remove it:** this is a local setup matter, reported and not specced.
