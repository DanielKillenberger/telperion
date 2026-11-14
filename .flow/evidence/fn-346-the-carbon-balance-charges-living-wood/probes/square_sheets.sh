#!/usr/bin/env bash
# fn-346 the square charge: today (A) beside QR, SR and PR. One sheet per
# view for the oak on seed 1001 (40, 80, 120, 200, 300) and on seed 1007
# (120, 200, 300); one sheet for the other presets, today beside SR,
# leafless then leafed. Today's stills are fn-345's
# (same crates), the oak at 80 drawn in this spec's first run. A tree not
# drawn is a grey tile.
#   bash probes/square_sheets.sh
set -eu
E=.flow/evidence/fn-346-the-carbon-balance-charges-living-wood
S="$E/raw/stills"
T345=.flow/evidence/fn-345-what-bounds-a-tree-vigour-falls-with/raw/stills
src() { if [ "$2" = A ] && [ -f "$T345/$1-A.log" ]; then echo "$T345"; else echo "$S"; fi; }
label() {
  local log; log="$(src "$1" "$2")/$1-$2.log"
  grep -o '[0-9.]* m tall, [0-9.]* x [0-9.]* m' "$log" 2>/dev/null | head -1 || echo "not drawn"
}
tile() {
  local f; f="$(src "$1" "$2")/$1-$2-$3.png"
  [ -f "$f" ] || f="xc:gray85[960x720]"
  tiles+=(-label "$1 $2 $3: $(label "$1" "$2")" "$f")
}
T="fn-346: today (A) beside QR (straight charge, L_max doubled), SR (square charge, L_max undoubled) and PR (straight charge, L_max undoubled), all with R"
for seed in 1001 1007; do
  ages="40 80 120 200 300"; [ "$seed" = 1007 ] && ages="120 200 300"
  for shot in bare whole; do
    tiles=()
    for age in $ages; do for v in A QR SR PR; do tile "oak-$age-$seed" "$v" "$shot"; done; done
    montage "${tiles[@]}" -tile 4x -geometry 480x360+4+4 -pointsize 15 -quality 82 \
      -title "$T, oak seed $seed, $shot" "$E/square-oak-$seed-$shot.jpg"
    echo "$E/square-oak-$seed-$shot.jpg"
  done
done
tiles=()
for t in beech-40-1001 beech-80-1001 birch-40-1001 spruce-40-1001 palm-40-1001 ordinary-40-1001; do
  for shot in bare whole; do for v in A SR; do tile "$t" "$v" "$shot"; done; done
done
montage "${tiles[@]}" -tile 4x -geometry 480x360+4+4 -pointsize 15 -quality 82 \
  -title "fn-346: today (A) beside SR (square charge, L_max undoubled, with R), leafless then leafed: the other presets" "$E/square-others.jpg"
echo "$E/square-others.jpg"
