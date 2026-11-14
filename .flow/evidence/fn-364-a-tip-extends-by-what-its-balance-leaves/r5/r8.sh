#!/bin/bash
# fn-364 R8: the Two Trees with their potential height lengthened, at 40
# and their preset ages on seeds 1 and 7 (grower, settle and drawn
# stations), the browser's ceiling test, and their draws for R6 (seeds 1,
# 7, 1001, 1007, as fn-363's builder drew theirs), all on 8e22c8a17.
D=/home/daniel/.cache/claude-tmp/claude-1000/-home-daniel-Projects-telperion/d6d5cb56-440a-44f4-803a-6a595b8a3532/scratchpad/fn364
W=/home/daniel/Projects/telperion/.worktrees
B=$W/fn-364/.flow/evidence/fn-352-the-scoreboard-a-fixed-panel-of-trees
R=$W/fn-364/.flow/evidence/fn-364-a-tip-extends-by-what-its-balance-leaves/raw
export TELPERION_HEAVY_RESERVE=8
cd "$D/runner-r8" || exit 1
CARGO_TARGET_DIR=$W/fn-364-after/target telperion-heavy 8 --threads 4 -- cargo build --profile ci >> "$D/build-r8.log" 2>&1
echo "build r8 rc=$? $(date '+%F %T')" >> "$D/times.txt"
cp "$W/fn-364-after/target/ci/heldrun" "$D/bin/heldrun-r8" || exit 1
cd "$D/runner-leafr8" || exit 1
CARGO_TARGET_DIR=$D/target-leafr8 telperion-heavy 8 --threads 4 -- cargo build --profile ci >> "$D/build-leafr8.log" 2>&1
echo "build leafr8 rc=$? $(date '+%F %T')" >> "$D/times.txt"
cp "$D/target-leafr8/ci/heldrun" "$D/bin/leafr8" || exit 1
two=""
for p in laurelin telperion; do
  ages="40 80"; [ $p = telperion ] && ages="40 82"
  for a in $ages; do for s in 1 7; do two="$two $p:$a:$s:4"; done; done
done
bash "$D/runs.sh" "$two" r8
bash "$D/runs.sh" "$two" leafr8
cd "$W/fn-364" || exit 1
echo "ceiling start $(date '+%F %T')" >> "$D/times.txt"
telperion-heavy 8 --threads 4 -- cargo test --profile ci -p telperion-core --lib -- pipeline::space::ceiling_tests --nocapture >> "$R/r8-ceiling.log" 2>&1
echo "ceiling end rc=$? $(date '+%F %T')" >> "$D/times.txt"
[ -x "$B/raw/target-fn-364-after/ci/scoreboard-probe" ] || CRATES=$W/fn-364-after bash "$B/scoreboard/run.sh" fn364-r8 build
out=$B/raw/fn364-r8/stills
mkdir -p "$out" "$B/scoreboard/fn364-r8"
for p in telperion laurelin; do
  for s in 1 7 1001 1007; do
    log=$out/$p-preset-$s.log
    [ -s "$log" ] && grep -q '^PEAK .* exit 0$' "$log" && continue
    echo "fn364-r8 $p-preset-$s start $(date '+%F %T')" >> "$D/draw-times.txt"
    telperion-heavy 8 --gpu 2 --threads 16 -- python3 "$B/scoreboard/peak.py" "$B/raw/target-fn-364-after/ci/scoreboard-probe" tree "$p" preset "$s" "$out/$p-preset-$s" whole,bare > "$log" 2>&1
    echo "fn364-r8 $p-preset-$s end rc=$? $(date '+%F %T')" >> "$D/draw-times.txt"
    grep -h '^TREE ' "$out"/*.log | sed 's/^TREE //' > "$B/scoreboard/fn364-r8/measures.jsonl"
  done
done
echo "R8 DONE $(date '+%F %T')" >> "$D/times.txt"
