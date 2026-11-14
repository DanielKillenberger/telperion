#!/usr/bin/env bash
# fn-345 R1: build space_oak with fn-344's oldoak.patch and probes/stem.patch
# applied (from the repo root), inside the heavy queue.
set -u
R=.flow/evidence/fn-345-what-bounds-a-tree-vigour-falls-with/raw
telperion-heavy 10 --threads 4 -- cargo build --profile ci -p telperion-render --example space_oak --example space_beech --example space_birch --example space_spruce --example space_palm --example space_ordinary > "$R/build.log" 2>&1
echo "build rc=$?"; grep -E "^(error|warning)" -A8 "$R/build.log" | head -60
