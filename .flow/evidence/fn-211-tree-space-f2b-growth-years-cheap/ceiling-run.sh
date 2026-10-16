#!/usr/bin/env bash
# fn-211.7: the stepped executor's phases on the lit oak at 80, seeds 1
# and 7, all cores and one (taskset -c 0). Waits while the 1-minute load
# is MAXLOAD or more (at most WAIT s), and records the load either side.
set -u
cd "$(dirname "$0")/../../.."
OUT=.flow/evidence/fn-211-tree-space-f2b-growth-years-cheap/raw/task7/${TAG:-run}
mkdir -p "$OUT"
BIN=target/ci/examples/hybrid
MAXLOAD=${MAXLOAD:-8}; WAIT=${WAIT:-1800}
load() { cut -d' ' -f1 /proc/loadavg; }
for cores in ${CORES:-all one}; do
  for seed in 1 7; do
    waited=0
    while awk -v l="$(load)" -v m="$MAXLOAD" 'BEGIN{exit !(l>=m)}' && (( waited < WAIT )); do sleep 15; waited=$((waited+15)); done
    f=$OUT/oak80-s$seed-$cores.txt
    echo "LOAD_START $(load) waited ${waited}s $(date -Is)" > "$f"
    if [ $cores = one ]; then HYBRID=1 taskset -c 0 $BIN 80 $seed 2 >> "$f"
    else HYBRID=1 $BIN 80 $seed 2 >> "$f"; fi
    echo "LOAD_END $(load) $(date -Is)" >> "$f"
    grep -E "LOAD|END" "$f"
  done
done
