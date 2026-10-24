#!/bin/bash
# fn-257 task 15: the site field, paired warm A/B. Two render test binaries,
# the release tip before R8 (23fb243e3) and R8 (d9193b9ce), each carrying
# the scratch `site_time` test (site-time-base.patch, site-time-after.patch),
# run alternately, 16 pairs, the order flipped every pair. Each run grows the
# oak, birch and spruce at seeds 1, 1234 and 5678 and reads the warm second
# field at 36 cells a side. The GPU's and CPU's load (1-minute average) are
# sampled before every pair.
# Usage: site-time.sh <base binary> <after binary>; output in raw/t15/ab.txt.
set -uo pipefail
here=.flow/evidence/fn-257-the-site-grows-its-trees-on-the-device/raw/t15
mkdir -p "$here"
run() {
  telperion-heavy 10 --gpu 6 --threads 4 -- env FIELD_SEEDS=1,1234,5678 "$1" site_time --ignored --nocapture 2>&1 \
    | grep SITE_TIME | sed "s/^/$2 /" >> "$here/ab.txt" || { echo "site-time: the $2 run failed" >&2; exit 1; }
}
for i in $(seq 1 16); do
  echo "load $i gpu $(nvidia-smi --query-gpu=utilization.gpu,memory.used --format=csv,noheader) cpu $(cut -d' ' -f1 /proc/loadavg)" >> "$here/ab.txt"
  if [ $((i % 2)) -eq 1 ]; then run "$1" base; run "$2" after; else run "$2" after; run "$1" base; fi
done
echo done
