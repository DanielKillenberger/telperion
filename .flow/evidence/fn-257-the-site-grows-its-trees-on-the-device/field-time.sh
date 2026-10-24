#!/bin/bash
# fn-257 task 11: the site field's time, paired warm A/B. Two test binaries
# (the base and the change, each with `field_time`) run alternately, 16
# pairs, the order flipped every pair; each run grows the oak, birch and
# spruce at seed 1 twice and asks the warm field a 36-cell query.
# Usage: field-time.sh <base binary> <changed binary>; output in raw/t11/.
set -u
here=.flow/evidence/fn-257-the-site-grows-its-trees-on-the-device/raw/t11
mkdir -p "$here"
run() {
  telperion-heavy 10 --gpu 6 --threads 4 -- "$1" field_time --ignored --nocapture 2>&1 \
    | grep FIELD_TIME | sed "s/^/$2 /" >> "$here/ab.txt"
}
for i in $(seq 1 16); do
  nvidia-smi --query-gpu=utilization.gpu --format=csv,noheader | sed "s/^/load $i /" >> "$here/ab.txt"
  if [ $((i % 2)) -eq 1 ]; then run "$1" base; run "$2" after; else run "$2" after; run "$1" base; fi
done
echo done
