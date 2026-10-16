#!/bin/bash
# fn-286: a preset at its own age, seeds 1, 7 and 42, from the front, from
# a value file with rows replaced or added; run from the worktree root:
#   .flow/evidence/fn-286-a-relay-runs-back-onto-the-line-it/views.sh <tag> <value file> [<path>=<value>...]
# Copies relay_views.rs into telperion-render's examples for the build and
# removes it after; stills in raw/stills/, measures appended to raw/views.jsonl.
set -u
here=.flow/evidence/fn-286-a-relay-runs-back-onto-the-line-it
tag=$1 file=$2
shift 2
example=crates/telperion-render/examples/relay_views.rs
mkdir -p "$here/raw/stills"
cp "$here/relay_views.rs" "$example"
trap 'rm -f "$example"' EXIT
telperion-heavy 6 --threads 8 -- env CARGO_BUILD_JOBS=8 cargo build -q --profile ci -p telperion-render --example relay_views \
  2>"$here/raw/build.log" || { echo "build failed"; tail -20 "$here/raw/build.log"; exit 1; }
rm -f "$example"
telperion-heavy 10 --gpu 4 --threads 8 -- env RAYON_NUM_THREADS=8 target/ci/examples/relay_views \
  "$here/raw/stills" "$tag" "$file" 1,7,42 "$@" | tee -a "$here/raw/views.jsonl"
