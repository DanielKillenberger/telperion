#!/usr/bin/env bash
# fn-346 sheets: today's tree (A) beside P1 and P2, leafless then leafed,
# one row per tree; a tree not drawn is a grey tile. Today's stills come
# from fn-345's raw/stills (same crates; fn-345 drew A with its probe
# dormant), but for the oak at 80, drawn here.
#   bash probes/sheets.sh
set -eu
E=.flow/evidence/fn-346-the-carbon-balance-charges-living-wood
S="$E/raw/stills"
T345=.flow/evidence/fn-345-what-bounds-a-tree-vigour-falls-with/raw/stills
src() {  # tree, variant -> directory holding it
  if [ "$2" = A ] && [ -f "$T345/$1-A.log" ]; then echo "$T345"; else echo "$S"; fi
}
label() {  # tree, variant
  local log; log="$(src "$1" "$2")/$1-$2.log"
  grep -o '[0-9.]* m tall, [0-9.]* x [0-9.]* m' "$log" 2>/dev/null | head -1 || echo "not drawn"
}
sheet() {  # output, title, trees...
  local outfile="$1" title="$2"; shift 2
  local tiles=()
  for t in "$@"; do
    for shot in bare whole; do
      for w in A P1 P2; do
        f="$(src "$t" "$w")/$t-$w-$shot.png"
        lab="$t $w $shot: $(label "$t" "$w")"
        [ -f "$f" ] || f="xc:gray85[960x720]"
        tiles+=(-label "$lab" "$f")
      done
    done
  done
  montage "${tiles[@]}" -tile 6x -geometry 480x360+4+4 -pointsize 15 -quality 82 \
    -title "$title" "$outfile"
  echo "$outfile"
}
T="fn-346: today (A) beside P1 and P2 (a living tip costs size x path / L_max, dead wood nothing): leafless, then leafed"
sheet "$E/sheet-oak-1001.jpg" "$T, oak seed 1001" oak-40-1001 oak-80-1001 oak-120-1001 oak-200-1001 oak-300-1001
sheet "$E/sheet-oak-1007.jpg" "$T, oak seed 1007" oak-120-1007 oak-300-1007
sheet "$E/sheet-others.jpg" "$T, the other presets" beech-40-1001 beech-80-1001 birch-40-1001 spruce-40-1001 palm-40-1001 ordinary-40-1001
