#!/usr/bin/env bash
# fn-226 task 3: both models on the lit oak at 40 and 80, seeds 1, 7, 42, best of 3,
# with `r3-measure.rs` built as <binary>. Waits for a one-minute load
# under 8 before each tree. Output: raw/r3/<model>.jsonl.
# Usage: r3-run.sh <binary> <model> (today or outline)
set -u
cd "$(dirname "$0")/../../.."
export RAYON_NUM_THREADS=8
out=.flow/evidence/fn-226-light-read-from-the-crowns-outline-per/raw/r3
mkdir -p "$out"
rm -f "$out/$2.jsonl"
wait_load() { while awk '{exit !($1 >= 8)}' /proc/loadavg; do sleep 15; done; }
for age in 40 80; do
  for seed in 1 7 42; do
    wait_load
    "$1" "$2" "$age" "$seed" 3 >> "$out/$2.jsonl"
  done
done
