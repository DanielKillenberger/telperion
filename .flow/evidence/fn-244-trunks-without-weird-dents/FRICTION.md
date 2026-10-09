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

## 2026-10-09: The decision assumed a single join point, and the girth model has none

While building option 2, I found that the girth model enters a relay's pipe at three places (`relay_parts`), with blends far from one on mature trunks. Any single drawn join therefore leaves a stub or a dip. I found this only after building the change and rendering every species.

- **Cost:** about 25 minutes and one full still set.
- **What would have removed it:** printing `b` and `t` for the trunk relays before the decision. That is one probe line, now in DENTS.md.

## 2026-10-09: An owner requirement surfaced only at the gate

I built option 1 across CPU and device, then ran the space suite. Only then did fn-199 R4 (`relays.rs`, girth moves by degree with `relay_at`) fail. The second decision contradicted it, and nobody had checked.

- **Cost:** about 40 minutes of device work and one full still set, before the conflict was visible.
- **What would have removed it:** a search of the tests for the rule being changed (`relay_parts`, `relay_span`) before deciding. It finds `tests/relays.rs` at once.
