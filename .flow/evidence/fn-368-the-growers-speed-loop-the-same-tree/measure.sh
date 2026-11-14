#!/usr/bin/env bash
# fn-368: one round's measurement. Builds the `early` example, then grows
# each of the four trees three times on `threads` cores (default 1) through
# the queue, each run under peak.py. Raw output in raw/<label>/; read it with
# summary.py.
#
#   .flow/evidence/fn-368-the-growers-speed-loop-the-same-tree/measure.sh <label> [threads] [rows]
set -u
here=$(cd "$(dirname "$0")" && pwd)
root=$(git -C "$here" rev-parse --show-toplevel)
label=$1
threads=${2:-1}
# Each row's last field is the memory the queue holds for it, GB: the
# small trees' measured peaks are far under it (FRICTION.md, 02:43).
rows=${3:-"oregon-white-oak:40:1001:2 european-beech:80:1:2 norway-spruce:40:1:2 oregon-white-oak:120:1001:8"}
out="$here/raw/$label"
mkdir -p "$out"
cd "$root" || exit 1
export TELPERION_HEAVY_RESERVE=8
echo "build $(date '+%F %T')" >> "$out/STATUS"
if ! telperion-heavy 8 --threads 8 -- cargo build --profile ci -p telperion-core --example early > "$out/build.log" 2>&1; then
  echo "build failed $(date '+%F %T')" >> "$out/STATUS"
  exit 1
fi
for row in $rows; do
  IFS=: read -r preset age seed mem <<< "$row"
  for k in 1 2 3; do
    f="$out/$preset-$age-$seed-t$threads-$k.txt"
    echo "start $preset $age $seed t$threads $k $(date '+%F %T')" >> "$out/STATUS"
    telperion-heavy "${mem:-8}" --threads "$threads" -- \
      python3 "$here/peak.py" target/ci/examples/early three "$preset" "$age" "$seed" cycle > "$f" 2>&1
    echo "end rc=$? $preset $age $seed t$threads $k $(date '+%F %T')" >> "$out/STATUS"
  done
done
echo "done $(date '+%F %T')" >> "$out/STATUS"
