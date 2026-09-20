#!/usr/bin/env bash
set -euo pipefail
repo=$(pwd -P)
evidence="$repo/.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/cpu-profile"
scratch=$(cat "$evidence/scratch-path.txt")
export CARGO_TARGET_DIR="$repo/target"
for mode in control instrumented; do
 if [[ "$mode" == instrumented ]]; then
  python3 "$evidence/instrument.py" "$scratch"
 fi
 cd "$scratch"
 timeout 600s cargo build --release -p telperion-core --example cpu_profile > "$evidence/$mode-build.log" 2>&1
 sha256sum "$CARGO_TARGET_DIR/release/examples/cpu_profile" > "$evidence/$mode-binary.sha256"
 : > "$evidence/$mode.jsonl"
 for species in oregon-white-oak norway-spruce; do
  for seed in 1 7; do
   timeout 120s "$CARGO_TARGET_DIR/release/examples/cpu_profile" "$species" "$seed" >> "$evidence/$mode.jsonl"
  done
 done
 cd "$repo"
done
