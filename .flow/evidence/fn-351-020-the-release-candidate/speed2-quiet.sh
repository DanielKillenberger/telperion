#!/usr/bin/env bash
# fn-351 speed round 2: several binaries (raw/speed2/<label>/early) read in
# turn on pinned cores, five rounds (or `ROUNDS`), each run under count.py: one thread on
# core 4, or four on cores 4 to 7. Inside one queue admission:
#
#   speed2-quiet.sh <tag> <threads> "<preset:age:seed>" <label>...
#
# The runs land in raw/speed2/quiet-<tag>/ and read back with speed2-ab.py,
# every label against the first.
set -u
here=$(cd "$(dirname "$0")" && pwd)
tag=$1; threads=$2; row=$3; shift 3
out="$here/raw/speed2/quiet-$tag"
mkdir -p "$out"
IFS=: read -r preset age seed <<< "$row"
cores=4; [ "$threads" -gt 1 ] && cores=4-7
echo "start $(date '+%F %T') $(uptime)" >> "$out/STATUS"
for k in $(seq 1 "${ROUNDS:-5}"); do
  for side in "$@"; do
    f="$out/${side}__$preset-$age-$seed-t$threads-$k.txt"
    RAYON_NUM_THREADS=$threads taskset -c "$cores" python3 "$here/count.py" \
      "$here/raw/speed2/$side/early" three "$preset" "$age" "$seed" cycle >> "$f" 2>&1
  done
done
echo "done $(date '+%F %T') $(uptime)" >> "$out/STATUS"
python3 "$here/speed2-ab.py" "$out"
