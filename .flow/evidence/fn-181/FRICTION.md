# fn-181 friction

## 2026-10-02 — species metrics baseline for R2

Doing: recording master's species metrics at seeds 1 and 7 for every preset (R2).
Slowed: `species_measure` takes one profile set per run and refuses the catalogue packets'
`profile.json` for european-beech, silver-birch and date-palm ("invalid or unready profile
manifest"); the fn-9 set carries only oak and spruce. Three probe runs, about 5 minutes.
Would remove it: a `species_measure` mode that measures a preset with no profile gate (metrics
only), or a `--profiles` that accepts a packet's unfrozen profile for measurement.

## 2026-10-02 — each byte-identity check costs a fat-LTO release build

Doing: proving R2 (every preset byte-identical at seeds 1 and 7) after each removal phase.
Slowed: `generation_digest` only runs as `cargo run --release`, so every check rebuilds core,
render and wgpu under the release profile's fat LTO before a 3.5-minute run: about 10 minutes
wall per check, three checks (master, phase 1, phase 2). Source edits wait while it compiles.
Would remove it: document running the digest under `--profile ci` (no LTO, the gate's own
profile, already built by the gate), or a `scripts/` wrapper that does, so a byte check shares
the gate's artifacts.

## 2026-10-02 — the dcg hook refuses heredocs with backticks or `<` in them

Doing: multi-line Rust and Markdown edits through `python3 - <<'PY'` heredocs.
Slowed: dcg blocked three heredocs whose bodies held backticks or `<value` as an unverifiable
shell launcher or redirect; each cost a rewrite into a scratch script, about 5 minutes total.
Would remove it: a local setup matter (the owner's dcg allowlist), reported here, not specced.
