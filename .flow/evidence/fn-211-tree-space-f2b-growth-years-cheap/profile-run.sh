#!/bin/bash
# Runs the profile matrix; one JSON line per build into raw/profile.jsonl.
B=/home/daniel/Projects/telperion/.worktrees/f2b/target/release/examples/profile
OUT=$(dirname "$0")/raw/profile.jsonl
for age in 40 80; do for sp in palm beech oak spruce; do for seed in 1 7; do
  for cores in all 1; do
    while :; do l=$(cut -d' ' -f1 /proc/loadavg); awk "BEGIN{exit !($l<4)}" && break; echo "load $l, waiting" >&2; sleep 20; done
    if [ $cores = 1 ]; then taskset -c 0 $B $sp $age $seed 2 >> $OUT; else $B $sp $age $seed 2 >> $OUT; fi
    echo "$sp $age $seed $cores done load $(cut -d' ' -f1 /proc/loadavg)" >&2
  done
done; done; done
