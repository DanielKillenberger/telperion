#!/bin/bash
# fn-236 / fn-286: the oak's limb lines at 40, seeds 1-50, through
# seed7/oak_lines.rs (copied into telperion-core's examples for the build
# and removed after). Run from the worktree root:
#   .flow/evidence/fn-236-tree-space-the-oregon-white-oaks-crown/seed7/lines.sh <tag> [values file]
# Writes raw/seed7/oak7-<tag>.jsonl and prints seed7/summary.py's table.
set -u
here=.flow/evidence/fn-236-tree-space-the-oregon-white-oaks-crown
tag=$1
example=crates/telperion-core/examples/oak_lines.rs
mkdir -p "$here/raw/seed7"
cp "$here/seed7/oak_lines.rs" "$example"
trap 'rm -f "$example"' EXIT
telperion-heavy 4 --threads 8 -- env CARGO_BUILD_JOBS=8 cargo build -q --profile ci -p telperion-core --example oak_lines \
  2>"$here/raw/seed7/build.log" || { echo "build failed"; tail -20 "$here/raw/seed7/build.log"; exit 1; }
rm -f "$example"
seeds=$(seq -s, 1 50)
OAK_VALUES=${2:-crates/telperion-core/presets/oregon-white-oak.values} \
  telperion-heavy 8 --threads 8 -- env RAYON_NUM_THREADS=8 target/ci/examples/oak_lines 40 "$seeds" 200 \
  >"$here/raw/seed7/oak7-$tag.jsonl"
python3 "$here/seed7/summary.py" "$here/raw/seed7/oak7-$tag.jsonl"
