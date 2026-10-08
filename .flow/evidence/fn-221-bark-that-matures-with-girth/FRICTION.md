# fn-221 friction

## 2026-10-07, task 1: formatting only the touched files

- Doing: formatting the files the task touched, since `cargo fmt --all` rewrites unrelated files.
- Hindered: `cargo fmt -- <file>` still checks the whole workspace (it printed `build.rs` diffs), and `rustfmt <file>` follows `mod` declarations, so formatting `scene.rs` rewrote the untouched `scene/row.rs`. The safety hook then blocked `git checkout -- row.rs`, so the change was undone by reversing its diff.
- Cost: about 3 minutes.
- Would remove it: a workspace that is already `rustfmt`-clean, so `cargo fmt --all` is safe again; or a `scripts/fmt-changed` that runs `rustfmt --config skip_children=true` (nightly) on `git diff --name-only`.

## 2026-10-07, task 1: no trunk radius to state a test range against

- Doing: choosing a test radius range for the demonstration stills, which needs the specimen's trunk and limb radii.
- Hindered: `species_measure --case beech-1:european-beech:european-beech:1` printed only `fail/unassessed`, with no radii. The beech's range was guessed from a still, missed the first time (its trunk is under 0.1 m), and took a second render to correct.
- Cost: about 5 minutes and one extra pair of stills.
- Would remove it: a headless flag or a small example that prints the root radius and the radius at breast height of a preset at a seed.

## 2026-10-07, task 1: shell redirects to variable paths blocked

- Doing: writing a test-row JSON with `cat > $R/...` inside a render command.
- Hindered: the safety hook refuses redirects to a path built from a variable, so the whole render batch was refused and rewritten as a script with literal paths.
- Cost: about 2 minutes.
- Would remove it: nothing in the repository; a local hook setting (reported, not specced).
