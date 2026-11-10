#!/usr/bin/env bash
# fn-346 R1: build space_oak with probes/census.patch applied (repo root).
set -u
R=.flow/evidence/fn-346-the-carbon-balance-charges-living-wood/raw
telperion-heavy 10 --threads 4 -- cargo build --profile ci -p telperion-render --example space_oak > "$R/build.log" 2>&1
echo "build rc=$?"; grep -E "^(error|warning)" -A8 "$R/build.log" | head -60
