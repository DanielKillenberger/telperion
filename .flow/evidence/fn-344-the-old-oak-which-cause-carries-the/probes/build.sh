#!/usr/bin/env bash
# fn-344: build space_oak with probes/oldoak.patch applied (from the repo root).
set -u
R=.flow/evidence/fn-344-the-old-oak-which-cause-carries-the/raw
telperion-heavy 10 --threads 4 -- cargo build --profile ci -p telperion-render --example space_oak > "$R/build.log" 2>&1
echo "build rc=$?"; grep -E "^(error|warning)" -A8 "$R/build.log" | head -60
