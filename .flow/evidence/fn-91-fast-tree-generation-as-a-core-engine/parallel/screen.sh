#!/usr/bin/env bash
set -euo pipefail
repo=$(pwd -P)
ev="$repo/.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/parallel"
scratch=$(cat "$ev/scratch-path.txt")
mkdir -p "$scratch/baseline-output" "$scratch/candidate-output"
sha256sum "$scratch/baseline" "$scratch/candidate" > "$ev/native-binaries.sha256"
: > "$ev/baseline.jsonl"
: > "$ev/candidate.jsonl"
for species in oregon-white-oak norway-spruce; do
 for seed in 1 7; do
  for mode in baseline candidate; do
   timeout 120s "$scratch/$mode" "$species" "$seed" "$scratch/$mode-output" >> "$ev/$mode.jsonl"
  done
  cmp "$scratch/baseline-output/$species-$seed.bin" "$scratch/candidate-output/$species-$seed.bin"
 done
done
sha256sum "$scratch"/{baseline,candidate}-output/*.bin > "$ev/outputs.sha256"
