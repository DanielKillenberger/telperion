#!/usr/bin/env bash
# fn-351 speed round 2: one change read against its base. The two binaries
# (raw/speed2/<before>/early, raw/speed2/<after>/early) grow each tree in
# turn, `pairs` times on one thread and on four, so the desk's load falls on
# both alike (fn-368 FRICTION.md, 03:18). Run inside one queue admission:
#
#   TELPERION_HEAVY_RESERVE=8 telperion-heavy 4 --threads 4 -- \
#     .flow/evidence/fn-351-020-the-release-candidate/speed2-ab.sh <before> <after> [pairs] [rows]
#
# Each run goes through count.py (instructions, cycles, cache misses). The
# runs land in raw/speed2/<after>/ab/ and read back with speed2-ab.py.
set -u
here=$(cd "$(dirname "$0")" && pwd)
before=$1
after=$2
pairs=${3:-5}
rows=${4:-"oregon-white-oak:40:1001"}
out="$here/raw/speed2/$after/ab"
mkdir -p "$out"
echo "start $(date '+%F %T') $(uptime)" >> "$out/STATUS"
for row in $rows; do
  IFS=: read -r preset age seed <<< "$row"
  for k in $(seq 1 "$pairs"); do
    for threads in 1 4; do
      for side in "$before" "$after"; do
        f="$out/$side-$preset-$age-$seed-t$threads-$k.txt"
        RAYON_NUM_THREADS=$threads python3 "$here/count.py" \
          "$here/raw/speed2/$side/early" three "$preset" "$age" "$seed" cycle >> "$f" 2>&1
        echo "rc=$? $side $preset $age $seed t$threads $k $(date '+%F %T')" >> "$out/STATUS"
      done
    done
  done
done
echo "done $(date '+%F %T') $(uptime)" >> "$out/STATUS"
