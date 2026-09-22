#!/usr/bin/env bash
set -eu
measure=${1:-target/release/examples/species_measure}
profiles=${2:-.flow/evidence/fn9/profiles.json}
scratch=$(mktemp -d /tmp/tuning-empty-overlay-XXXXXX)
printf '{}\n' > "$scratch/empty.json"
for mode in absent empty; do
  family=()
  if [ "$mode" = empty ]; then family=(--family "$scratch/empty.json"); fi
  rc=0
  timeout 120 "$measure" --case golden:oregon-white-oak:oregon-white-oak:1 \
    --profiles "$profiles" --output "$scratch/$mode.jsonl" "${family[@]}" || rc=$?
  if [ "$rc" -gt 1 ]; then exit "$rc"; fi
  jq -s -e '[.[] | select(.event == "completed")] | length == 1' "$scratch/$mode.jsonl" >/dev/null
  jq -S -s '[.[] | select(.event == "completed") | {metrics,counts,output_bytes,gates}]' \
    "$scratch/$mode.jsonl" > "$scratch/$mode.summary.json"
done
diff -u "$scratch/absent.summary.json" "$scratch/empty.summary.json"
echo "empty-overlay measurement golden passed: $scratch"
