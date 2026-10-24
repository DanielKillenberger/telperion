#!/bin/bash
# fn-236: the owner's oak defects of 2026-10-14 as numbers and close-ups;
# run from the worktree root:
#   .flow/evidence/fn-236-tree-space-the-oregon-white-oaks-crown/defects.sh <tag> <ages> <seeds> [<path>=<value>...]
# Copies oak_defects.rs into telperion-render's examples for the run and
# removes it after; stills in raw/defects-<tag>/, measures in
# raw/defects-<tag>.jsonl. OAK_NO_STILLS=1 takes the measures alone.
set -e
tag=$1; ages=$2; seeds=$3; shift 3
here=.flow/evidence/fn-236-tree-space-the-oregon-white-oaks-crown
example=crates/telperion-render/examples/oak_defects.rs
mkdir -p "$here/raw/defects-$tag"
cp "$here/oak_defects.rs" "$example"
trap 'rm -f "$example"' EXIT
telperion-heavy 4 --threads 8 -- cargo build -q --profile ci -p telperion-render --example oak_defects \
  2>"$here/raw/defects-$tag/build.log"
gpu=(); [ -z "$OAK_NO_STILLS" ] && gpu=(--gpu 2)
telperion-heavy "${OAK_MEM:-4}" "${gpu[@]}" --threads 4 -- target/ci/examples/oak_defects "$here/raw/defects-$tag" "$ages" "$seeds" "$@" \
  >"$here/raw/defects-$tag.jsonl"
cat "$here/raw/defects-$tag.jsonl"
