#!/usr/bin/env bash
# fn-345 R3 sheets: today's tree (A) beside the probe (N), leafless, leafed
# and the lower trunk, one row per tree; a tree not drawn is a grey tile.
#   bash probes/sheets.sh
set -eu
E=.flow/evidence/fn-345-what-bounds-a-tree-vigour-falls-with
S="$E/raw/stills"
label() {  # name-age-seed, variant
  local log="$S/$1-$2.log"
  grep -o '[0-9.]* m tall, [0-9.]* x [0-9.]* m' "$log" 2>/dev/null | head -1 || echo "not drawn"
}
sheet() {  # output, title, trees...
  local outfile="$1" title="$2"; shift 2
  local tiles=()
  for t in "$@"; do
    for shot in bare whole bole; do
      for w in A N; do
        f="$S/$t-$w-$shot.png"
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
T="fn-345: today (A) beside the probe (N: node count by the tip's path, dead wood not charged): leafless, leafed, lower trunk"
sheet "$E/sheet-oak-1001.jpg" "$T, oak seed 1001" oak-40-1001 oak-120-1001 oak-200-1001 oak-300-1001
sheet "$E/sheet-oak-1007.jpg" "$T, oak seed 1007" oak-40-1007 oak-120-1007 oak-200-1007 oak-300-1007
sheet "$E/sheet-others.jpg" "$T, the other presets" beech-40-1001 beech-80-1001 birch-40-1001 spruce-40-1001 palm-40-1001 ordinary-40-1001
