#!/usr/bin/env bash
# fn-343: build the core test binary with probes/stops.patch applied (from the repo root).
set -u
R=.flow/evidence/fn-343-a-line-that-changes-hands-carries-on/raw
telperion-heavy 8 --threads 4 -- cargo test --profile ci -p telperion-core --lib --no-run > "$R/build.log" 2>&1
echo "build rc=$?"
grep -E "^(error|warning: unused)|Executable" "$R/build.log" | head -20
