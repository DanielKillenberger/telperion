#!/usr/bin/env bash
set -euo pipefail
repo=$(pwd -P)
evidence="$repo/.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/cpu-profile"
scratch=$(cat "$evidence/scratch-path.txt")
export CARGO_TARGET_DIR="$repo/target"
python3 "$evidence/followup.py" "$scratch" prepare
for mode in coarse sampled; do
 if [[ "$mode" == sampled ]]; then python3 "$evidence/followup.py" "$scratch" sample; fi
 cd "$scratch"
 timeout 600s cargo build --release -p telperion-core --example cpu_profile > "$evidence/followup-$mode-build.log" 2>&1
 sha256sum "$CARGO_TARGET_DIR/release/examples/cpu_profile" > "$evidence/followup-$mode-binary.sha256"
 : > "$evidence/followup-$mode.jsonl"
 for species in oregon-white-oak norway-spruce; do
  for seed in 1 7; do
   timeout 120s "$CARGO_TARGET_DIR/release/examples/cpu_profile" "$species" "$seed" >> "$evidence/followup-$mode.jsonl"
  done
 done
 cd "$repo"
done
