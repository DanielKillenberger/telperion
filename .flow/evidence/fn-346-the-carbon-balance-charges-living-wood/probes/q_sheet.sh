#!/usr/bin/env bash
# fn-346 old-tree step 5: today (A) beside Q on the other presets,
# leafless then leafed, one row per tree. Today's stills are fn-345's.
#   bash probes/q_sheet.sh
set -eu
E=.flow/evidence/fn-346-the-carbon-balance-charges-living-wood
S="$E/raw/stills"
T345=.flow/evidence/fn-345-what-bounds-a-tree-vigour-falls-with/raw/stills
src() { if [ "$2" = A ] && [ -f "$T345/$1-A.log" ]; then echo "$T345"; else echo "$S"; fi; }
label() {
  local log; log="$(src "$1" "$2")/$1-$2.log"
  grep -o '[0-9.]* m tall, [0-9.]* x [0-9.]* m' "$log" 2>/dev/null | head -1 || echo "not drawn"
}
tiles=()
for t in beech-40-1001 beech-80-1001 birch-40-1001 spruce-40-1001 palm-40-1001 ordinary-40-1001; do
  for shot in bare whole; do
    for v in A Q; do
      f="$(src "$t" "$v")/$t-$v-$shot.png"
      [ -f "$f" ] || f="xc:gray85[960x720]"
      tiles+=(-label "$t $v $shot: $(label "$t" "$v")" "$f")
    done
  done
done
montage "${tiles[@]}" -tile 4x -geometry 480x360+4+4 -pointsize 15 -quality 82 \
  -title "fn-346: today (A) beside Q (the charge by size and path, L_max doubled), leafless then leafed: the other presets" "$E/q-others.jpg"
echo "$E/q-others.jpg"
