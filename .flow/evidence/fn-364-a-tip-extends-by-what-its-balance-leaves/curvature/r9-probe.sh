#!/bin/bash
# fn-364 R9 probe: the oak at 300 seed 1007 and at 200 seed 1001 on this
# branch with R9 patched in (.worktrees/fn-364-r9, never committed): R5's
# columns, CURVATURE's ten-tip table, and the two trees drawn whole and bare
# (the tip shot at 200).
D=/home/daniel/.cache/claude-tmp/claude-1000/-home-daniel-Projects-telperion/d6d5cb56-440a-44f4-803a-6a595b8a3532/scratchpad/fn364
W=/home/daniel/Projects/telperion/.worktrees
B=$W/fn-364/.flow/evidence/fn-352-the-scoreboard-a-fixed-panel-of-trees
T=$W/fn-364-r9/target
export TELPERION_HEAVY_RESERVE=8
for r in runner-r9 curve/r9; do
  cd "$D/$r" || exit 1
  CARGO_TARGET_DIR=$T telperion-heavy 8 --threads 4 -- cargo build --profile ci >> "$D/build-r9.log" 2>&1
  echo "r9 build $r rc=$? $(date '+%F %T')" >> "$D/times.txt"
done
cp "$T/ci/heldrun" "$D/bin/heldrun-r9p" && cp "$T/ci/curve" "$D/bin/curve-r9" || exit 1
for job in 300:1007:8 200:1001:6; do
  IFS=: read -r age seed gb <<< "$job"
  name=r9p-oregon-white-oak-a$age-s$seed
  echo "$name start $(date '+%F %T')" >> "$D/times.txt"
  OLDOAK=1 SURPLUS=1 telperion-heavy "$gb" --threads 4 -- python3 "$D/../peak.py" "$D/bin/heldrun-r9p" oregon-white-oak "$age" "$seed" > "$D/logs/$name.out" 2> "$D/logs/$name.log"
  echo "$name end rc=$? $(date '+%F %T')" >> "$D/times.txt"
  telperion-heavy "$gb" --threads 4 -- "$D/bin/curve-r9" oregon-white-oak "$age" "$seed" > "$D/curve/r9-a$age-s$seed.md" 2> "$D/curve/r9-a$age-s$seed.log"
  echo "r9 curve $age $seed rc=$? $(date '+%F %T')" >> "$D/times.txt"
done
cd "$W/fn-364" || exit 1
[ -x "$B/raw/target-fn-364-r9/ci/scoreboard-probe" ] || CRATES=$W/fn-364-r9 bash "$B/scoreboard/run.sh" fn364-r9 build
out=$B/raw/fn364-r9/stills
mkdir -p "$out" "$B/scoreboard/fn364-r9"
for t in 300:1007:whole,bare 200:1001:whole,bare,tip; do
  IFS=: read -r a s shots <<< "$t"
  log=$out/oregon-white-oak-$a-$s.log
  [ -s "$log" ] && grep -q '^PEAK .* exit 0$' "$log" && continue
  telperion-heavy 8 --gpu 2 --threads 16 -- python3 "$B/scoreboard/peak.py" "$B/raw/target-fn-364-r9/ci/scoreboard-probe" tree oregon-white-oak "$a" "$s" "$out/oregon-white-oak-$a-$s" "$shots" > "$log" 2>&1
  echo "r9 draw $a $s rc=$? $(date '+%F %T')" >> "$D/times.txt"
  grep -h '^TREE ' "$out"/*.log | sed 's/^TREE //' > "$B/scoreboard/fn364-r9/measures.jsonl"
done
echo "R9 PROBE DONE $(date '+%F %T')" >> "$D/times.txt"
