#!/usr/bin/env bash
# fn-346 rules A and B: one sheet per view for the oak on seed 1001 (rows:
# 40, 80, 120, 200, 300; columns: today, Q, QA, QB, QAB), and a sheet of
# today beside QAB (leafless, leafed) for seed 1007 and the other presets.
# Today's stills are fn-345's (same crates), the oak at 80 drawn in this
# spec's first run. A tree not drawn is a grey tile.
#   bash probes/renew_sheets.sh
set -eu
E=.flow/evidence/fn-346-the-carbon-balance-charges-living-wood
S="$E/raw/stills"
T345=.flow/evidence/fn-345-what-bounds-a-tree-vigour-falls-with/raw/stills
src() { if [ "$2" = A ] && [ -f "$T345/$1-A.log" ]; then echo "$T345"; else echo "$S"; fi; }
label() {
  local log; log="$(src "$1" "$2")/$1-$2.log"
  grep -o '[0-9.]* m tall, [0-9.]* x [0-9.]* m' "$log" 2>/dev/null | head -1 || echo "not drawn"
}
tile() {  # tree variant shot
  local f; f="$(src "$1" "$2")/$1-$2-$3.png"
  [ -f "$f" ] || f="xc:gray85[960x720]"
  tiles+=(-label "$1 $2 $3: $(label "$1" "$2")" "$f")
}
T="fn-346: today (A) beside Q (charge by size and path, L_max doubled), QA (+ fine tips live on), QB (+ sleeping buds wake by light), QAB"
for shot in bare whole; do
  tiles=()
  for age in 40 80 120 200 300; do
    for v in A Q QA QB QAB; do tile "oak-$age-1001" "$v" "$shot"; done
  done
  montage "${tiles[@]}" -tile 5x -geometry 480x360+4+4 -pointsize 15 -quality 82 \
    -title "$T, oak seed 1001, $shot" "$E/renew-oak-1001-$shot.jpg"
  echo "$E/renew-oak-1001-$shot.jpg"
done
tiles=()
for t in oak-120-1007 oak-300-1007 beech-40-1001 beech-80-1001 birch-40-1001 spruce-40-1001 palm-40-1001 ordinary-40-1001; do
  for shot in bare whole; do for v in A QAB; do tile "$t" "$v" "$shot"; done; done
done
montage "${tiles[@]}" -tile 4x -geometry 480x360+4+4 -pointsize 15 -quality 82 \
  -title "fn-346: today (A) beside QAB, leafless then leafed: oak seed 1007 and the other presets" "$E/renew-others.jpg"
echo "$E/renew-others.jpg"
