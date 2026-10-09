#!/bin/bash
# fn-236: the oak from four azimuths and its projected foliage gap share,
# from the preset's value file on disk with overrides; run from the
# worktree root:
#   .flow/evidence/fn-236-tree-space-the-oregon-white-oaks-crown/views.sh <tag> [<ages> <seeds> [<path>=<value>...]]
# Copies oak_views.rs into telperion-render's examples for the run, removes
# it after; stills in raw/views-<tag>/, measures in raw/views-<tag>.jsonl.
# OAK_NO_STILLS=1 takes the measures alone, without the GPU.
set -e
tag=$1
ages=${2:-80}
seeds=${3:-1,7,42}
shift $(( $# < 3 ? $# : 3 ))
here=.flow/evidence/fn-236-tree-space-the-oregon-white-oaks-crown
example=crates/telperion-render/examples/oak_views.rs
lock=/tmp/claude-1000/-home-daniel-Projects-telperion/dd1a391e-8087-41d5-8d26-b3ea78d9cb37/scratchpad/gpu.lock
export RAYON_NUM_THREADS=4 CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=4
mkdir -p "$here/raw/views-$tag"
cp "$here/oak_views.rs" "$example"
trap 'rm -f "$example"' EXIT
cargo build -q --profile ci -p telperion-render --example oak_views 2>"$here/raw/views-$tag/build.log"
# With OAK_NO_STILLS set the run takes measures only and needs no GPU lock.
run=(target/ci/examples/oak_views "$here/raw/views-$tag" "$ages" "$seeds" "$@")
if [ -n "$OAK_NO_STILLS" ]; then "${run[@]}"; else flock -w 1800 "$lock" "${run[@]}"; fi \
  >"$here/raw/views-$tag.jsonl"
cat "$here/raw/views-$tag.jsonl"
