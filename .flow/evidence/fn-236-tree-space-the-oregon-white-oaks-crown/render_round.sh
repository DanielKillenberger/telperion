#!/bin/bash
# fn-236: one round's stills and measures, run from the worktree root:
#   .flow/evidence/fn-236-tree-space-the-oregon-white-oaks-crown/render_round.sh <round>
# Builds space_oak from the current preset, renders ages 40 and 80 at seeds
# 1, 7, 42 under the GPU lock into raw/sheet<round>/, and logs each tree's
# nodes, build time and bounds to raw/sheet<round>/render.log.
set -e
round=$1
dir=.flow/evidence/fn-236-tree-space-the-oregon-white-oaks-crown/raw/sheet$round
lock=/tmp/claude-1000/-home-daniel-Projects-telperion/dd1a391e-8087-41d5-8d26-b3ea78d9cb37/scratchpad/gpu.lock
export RAYON_NUM_THREADS=4 CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=4
mkdir -p "$dir"
cargo build --profile ci -p telperion-render --example space_oak 2>"$dir/build.log"
echo "load before: $(cut -d' ' -f1-3 /proc/loadavg)" >"$dir/render.log"
flock -w 1800 "$lock" target/ci/examples/space_oak "$dir" 40 80 --seeds 1,7,42 \
  --shots bare,whole,crown,limb >>"$dir/render.log" 2>&1
echo "load after: $(cut -d' ' -f1-3 /proc/loadavg)" >>"$dir/render.log"
grep -E "^age|^load" "$dir/render.log"
