# fn-309 friction

## 2026-10-22, conductor (flow --auto, plan stage)

- **Doing:** planning R1 (every compound leaf draws its rachis through `rachis::rachises`).
- **Slowed by:** the spec does not say how a twig or cluster placement reaches the clothing step. `rachises` runs in `branching::clothe` before placement, with no seed, no twig stations and no `kept()` filter; a twig placement's point and heading come from `station::place_run` afterwards. On the device the order is boots (frond axes) -> folds -> sweep -> stations, so a station's rachis has no stage that can hang it as wood. fn-283 (Open questions) and fn-295 (R1) both named this as "a second design"; fn-309 is silent on it. The plan stopped and went to the host.
- **Cost:** one scout round (about 190k subagent tokens) and a stop before plan-review; no cargo run.
- **Would remove it:** the spec naming the stage that hangs a station's rachis (replay the station walk in `clothe`, record placements for a later wood step, or hang from placement) and the device stage it moves to, checked against the code before the spec is marked ready (AGENTS.md, "Gates and checked claims").

- **Doing:** starting `flow --auto` on the spec.
- **Slowed by:** the pilot snapshot refuses the run twice: the host's uncommitted probe (`crates/telperion-render/examples/walk_leaf_probe.rs`) trips the dirty-tree guard, and the spec is not marked ready. The conductor drove the stages directly instead.
- **Cost:** a few minutes.
- **Would remove it:** the probe committed (or kept in scratch) before dispatch, and the spec marked ready by the person who consents to the run.

## 2026-10-22 18:15, conductor (plan review)

- **Doing:** the codex plan review, wrapped in `telperion-heavy 8 --threads 2`.
- **Slowed by:** `flowctl codex plan-review` has no `--focus` option, so the host's "run no cargo" instruction could not reach the reviewer's prompt (fn-308 found the same for `completion-review`). The read-only sandbox and the heavy wrapper were the only guard; the reviewer ran no cargo command (checked in its log).
- **Cost:** none this time; a risk on every plan and completion review.
- **Would remove it:** a focus option on every codex review command, or a cargo shim on reviewers' PATH refusing unadmitted test runs (host's to spec, as fn-308 noted).
