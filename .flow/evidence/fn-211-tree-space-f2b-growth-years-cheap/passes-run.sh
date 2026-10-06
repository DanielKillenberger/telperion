#!/bin/bash
# timing.sh <binary> <out> : per-pass timings, ages 40 and 80, four species, seeds 1 and 7 (each built twice), 1 and all cores
B=$1; OUT=$2
for age in 40 80; do for sp in palm beech oak spruce; do
  light=neutral; [ $sp = oak ] && light=lit
  for cores in all 1; do
    while :; do l=$(cut -d' ' -f1 /proc/loadavg); awk "BEGIN{exit !($l<4)}" && break; echo "load $l, waiting" >&2; sleep 15; done
    if [ $cores = 1 ]; then taskset -c 0 $B passes $age 1,1,7,7 $sp $light >> $OUT; else $B passes $age 1,1,7,7 $sp $light >> $OUT; fi
  done
done; done
