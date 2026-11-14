#!/bin/bash
# fn-364 R6: this branch's column of the sheets, drawn by fn-352's probe
# (this checkout's copy, with fn-364's `tip` shot) on this branch's crates,
# one tree a heavy scope, one GPU job at a time; and the old limb's outer
# metre on fn-363's branch, the one tile fn-363's builder does not draw.
# The other two columns are fn-363's builder's tiles, never redrawn.
#   bash draw.sh            (resumable: a tree whose log ends exit 0 is kept)
W=/home/daniel/Projects/telperion/.worktrees
B=$W/fn-364/.flow/evidence/fn-352-the-scoreboard-a-fixed-panel-of-trees
S=$B/scoreboard
D=/home/daniel/.cache/claude-tmp/claude-1000/-home-daniel-Projects-telperion/d6d5cb56-440a-44f4-803a-6a595b8a3532/scratchpad/fn364
export TELPERION_HEAVY_RESERVE=8
cd "$W/fn-364" || exit 1
[ -x "$B/raw/target/ci/scoreboard-probe" ] || bash "$S/run.sh" fn364-this build
[ -x "$B/raw/target-fn-363-after/ci/scoreboard-probe" ] || CRATES=$W/fn-363-after bash "$S/run.sh" fn364-363tip build
trees=""
for p in norway-spruce european-beech silver-birch date-palm ordinary; do
  for s in 1001 1007; do trees="$trees fn364-this:$p:preset:$s:whole:8"; done
done
for a in 40 80 120 200 300; do
  gb=16; [ $a -le 80 ] && gb=8; [ $a = 120 ] && gb=12
  for s in 1001 1007; do
    shots=whole,bare; [ $a = 200 ] && [ $s = 1001 ] && shots=whole,bare,tip
    trees="$trees fn364-this:oregon-white-oak:$a:$s:$shots:$gb"
  done
done
trees="$trees fn364-363tip:oregon-white-oak:200:1001:tip:24"
for t in $trees; do
  IFS=: read -r label p a s shots gb <<< "$t"
  probe=$B/raw/target/ci/scoreboard-probe
  [ $label = fn364-363tip ] && probe=$B/raw/target-fn-363-after/ci/scoreboard-probe
  out=$B/raw/$label/stills
  mkdir -p "$out" "$S/$label"
  log=$out/$p-$a-$s.log
  [ -s "$log" ] && grep -q '^PEAK .* exit 0$' "$log" && continue
  echo "$label $p-$a-$s start $(date '+%F %T')" >> "$D/draw-times.txt"
  telperion-heavy "$gb" --gpu 2 --threads 16 -- python3 "$S/peak.py" "$probe" tree "$p" "$a" "$s" "$out/$p-$a-$s" "$shots" > "$log" 2>&1
  echo "$label $p-$a-$s end rc=$? $(date '+%F %T')" >> "$D/draw-times.txt"
  grep -h '^TREE ' "$out"/*.log | sed 's/^TREE //' > "$S/$label/measures.jsonl"
done
echo "DRAW DONE $(date '+%F %T')" >> "$D/draw-times.txt"
