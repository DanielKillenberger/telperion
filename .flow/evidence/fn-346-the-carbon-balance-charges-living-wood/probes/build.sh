#!/usr/bin/env bash
# fn-346: build the space examples with probes/pipes.patch applied (repo root).
set -u
R=.flow/evidence/fn-346-the-carbon-balance-charges-living-wood/raw
telperion-heavy 10 --threads 4 -- cargo build --profile ci -p telperion-render \
  --example space_oak --example space_beech --example space_birch --example space_spruce \
  --example space_palm --example space_ordinary > "$R/build.log" 2>&1
echo "build rc=$?"; grep -E "^error" -A12 "$R/build.log" | head -60; tail -2 "$R/build.log"
