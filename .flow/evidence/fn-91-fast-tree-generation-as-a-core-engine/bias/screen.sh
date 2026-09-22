#!/usr/bin/env bash
set -euo pipefail
repo=$(pwd -P)
evidence="$repo/.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/bias"
scratch=$(cat "$evidence/scratch-path.txt")
mkdir -p "$scratch/baseline-output" "$scratch/candidate-output"
sha256sum "$scratch/baseline" "$scratch/candidate" > "$evidence/native-binaries.sha256"
if [[ -f "$scratch/cpu-profile-preserved" ]]; then sha256sum "$scratch/cpu-profile-preserved" >> "$evidence/native-binaries.sha256"; fi
: > "$evidence/baseline.jsonl"
: > "$evidence/candidate.jsonl"
for species in oregon-white-oak norway-spruce; do
 for seed in 1 7; do
  for mode in baseline candidate; do
   timeout 120s "$scratch/$mode" "$species" "$seed" "$scratch/$mode-output" >> "$evidence/$mode.jsonl"
  done
  cmp "$scratch/baseline-output/$species-$seed.bin" "$scratch/candidate-output/$species-$seed.bin"
 done
done
sha256sum "$scratch"/{baseline,candidate}-output/*.bin > "$evidence/outputs.sha256"
