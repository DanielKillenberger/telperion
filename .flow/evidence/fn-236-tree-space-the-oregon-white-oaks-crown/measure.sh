#!/bin/bash
# fn-236: the crown measures of the current preset, run from the worktree root:
#   .flow/evidence/fn-236-tree-space-the-oregon-white-oaks-crown/measure.sh <tag> [<ages> <seeds> [<path>=<value>...]]
# Copies oak_measure.rs into telperion-core's examples for the run, removes
# it after, and writes raw/measure-<tag>.jsonl (ages 40,80 and seeds 1,7,42
# unless named; overrides as oak_measure.rs reads them).
set -e
tag=$1
ages=${2:-40,80}
seeds=${3:-1,7,42}
shift $(( $# < 3 ? $# : 3 ))
here=.flow/evidence/fn-236-tree-space-the-oregon-white-oaks-crown
example=crates/telperion-core/examples/oak_measure.rs
export RAYON_NUM_THREADS=4 CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=4
cp "$here/oak_measure.rs" "$example"
trap 'rm -f "$example"' EXIT
cargo run -q --profile ci -p telperion-core --example oak_measure -- "$ages" "$seeds" "$@" \
  >"$here/raw/measure-$tag.jsonl"
cat "$here/raw/measure-$tag.jsonl"
