#!/bin/bash
# fn-236: the crown measures of the current preset, run from the worktree root:
#   .flow/evidence/fn-236-tree-space-the-oregon-white-oaks-crown/measure.sh <round>
# Copies oak_measure.rs into telperion-core's examples for the run, removes
# it after, and writes raw/measure-r<round>.jsonl.
set -e
round=$1
here=.flow/evidence/fn-236-tree-space-the-oregon-white-oaks-crown
example=crates/telperion-core/examples/oak_measure.rs
export RAYON_NUM_THREADS=4 CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=4
cp "$here/oak_measure.rs" "$example"
trap 'rm -f "$example"' EXIT
cargo run -q --profile ci -p telperion-core --example oak_measure -- 40,80 1,7,42 \
  >"$here/raw/measure-r$round.jsonl"
cat "$here/raw/measure-r$round.jsonl"
