#!/usr/bin/env bash
# fn-343: build space_oak with probes/ablate.patch applied.
set -u
R=.flow/evidence/fn-343-a-line-that-changes-hands-carries-on/raw
telperion-heavy 10 --threads 4 -- cargo build --profile ci -p telperion-render --example space_oak > "$R/build-stills.log" 2>&1
echo "build rc=$?"; grep -E "^(error|warning)" -A6 "$R/build-stills.log" | head -40
