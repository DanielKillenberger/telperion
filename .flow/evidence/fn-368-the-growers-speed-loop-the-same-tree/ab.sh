#!/usr/bin/env bash
# fn-368: a round read against its base, the two binaries run in turn on
# each tree so the desk's load falls on both alike (FRICTION.md, 03:18).
# raw/<before>/early and raw/<after>/early are the two rounds' binaries;
# the runs land in raw/<after>/ab/ and read back with ab.py.
#
#   .flow/evidence/fn-368-the-growers-speed-loop-the-same-tree/ab.sh <before> <after> [pairs] [rows]
set -u
here=$(cd "$(dirname "$0")" && pwd)
before=$1
after=$2
pairs=${3:-5}
rows=${4:-"oregon-white-oak:40:1001:2 european-beech:80:1:4 norway-spruce:40:1:2 oregon-white-oak:120:1001:4"}
out="$here/raw/$after/ab"
mkdir -p "$out"
export TELPERION_HEAVY_RESERVE=8
for row in $rows; do
  IFS=: read -r preset age seed mem <<< "$row"
  for k in $(seq 1 "$pairs"); do
    for side in "$before" "$after"; do
      f="$out/$side-$preset-$age-$seed-$k.txt"
      echo "start $side $preset $age $seed $k $(date '+%F %T')" >> "$out/STATUS"
      telperion-heavy "$mem" --threads 1 -- \
        python3 "$here/peak.py" "$here/raw/$side/early" three "$preset" "$age" "$seed" cycle > "$f" 2>&1
      echo "end rc=$? $side $preset $age $seed $k $(date '+%F %T')" >> "$out/STATUS"
    done
  done
done
echo "done $(date '+%F %T')" >> "$out/STATUS"
