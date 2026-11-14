#!/bin/bash
# fn-364 CURVATURE: build both branches' readers, then the oak at 300 seed
# 1007 and at 200 seed 1001 on each (grower and settle only).
D=/home/daniel/.cache/claude-tmp/claude-1000/-home-daniel-Projects-telperion/d6d5cb56-440a-44f4-803a-6a595b8a3532/scratchpad/fn364
export TELPERION_HEAVY_RESERVE=8
for side in this 363; do
  t=/home/daniel/Projects/telperion/.worktrees/fn-364-after/target; [ $side = 363 ] && t=$D/target-leaf363
  cd "$D/curve/$side" || exit 1
  CARGO_TARGET_DIR=$t telperion-heavy 8 --threads 4 -- cargo build --profile ci >> "$D/curve/build-$side.log" 2>&1
  echo "curve build $side rc=$? $(date '+%F %T')" >> "$D/times.txt"
  cp "$t/ci/curve" "$D/bin/curve-$side" || exit 1
done
for job in 300:1007:8 200:1001:6; do
  IFS=: read -r age seed gb <<< "$job"
  for side in this 363; do
    out=$D/curve/$side-a$age-s$seed.md
    grep -q "^Totals" "$out" 2>/dev/null && continue
    echo "curve $side $age $seed start $(date '+%F %T')" >> "$D/times.txt"
    telperion-heavy "$gb" --threads 4 -- "$D/bin/curve-$side" oregon-white-oak "$age" "$seed" > "$out" 2> "$D/curve/$side-a$age-s$seed.log"
    echo "curve $side $age $seed end rc=$? $(date '+%F %T')" >> "$D/times.txt"
  done
done
echo "CURVE DONE $(date '+%F %T')" >> "$D/times.txt"
