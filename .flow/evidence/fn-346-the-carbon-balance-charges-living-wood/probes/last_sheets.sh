#!/usr/bin/env bash
# fn-346 the last measurement's sheets. Per seed (1001, 1007): rows 120,
# 200, 300, each leafless then leafed; columns today, SR and SRB, each with
# the settle's sag on and off (sag off is an instrument; today's sag-off
# only at 200 and 300). Seed 1012: today beside SR, sag on, leafless and
# leafed, at 40, 120, 200, 300. Young oak and other presets: today, SR
# and SRB, sag on. Today's sag-on stills are fn-345's (same crates), the
# oak at 80 from this spec's first run. A tree not drawn is a grey tile.
#   bash probes/last_sheets.sh
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
for seed in 1001 1007; do
  tiles=()
  for age in 120 200 300; do
    for shot in bare whole; do
      for v in A Au SR SRu SRB SRBu; do tile "oak-$age-$seed" "$v" "$shot"; done
    done
  done
  montage "${tiles[@]}" -tile 6x -geometry 400x300+4+4 -pointsize 13 -quality 82 \
    -title "fn-346 last: today (A), SR and SRB, each sag on beside sag off (u, an instrument), oak seed $seed" \
    "$E/last-oak-$seed.jpg"
  echo "$E/last-oak-$seed.jpg"
done
tiles=()
for age in 40 120 200 300; do
  for shot in bare whole; do for v in A SR; do tile "oak-$age-1012" "$v" "$shot"; done; done
done
montage "${tiles[@]}" -tile 4x -geometry 480x360+4+4 -pointsize 15 -quality 82 \
  -title "fn-346 last: today (A) beside SR, sag on, leafless then leafed, oak seed 1012" "$E/last-oak-1012.jpg"
echo "$E/last-oak-1012.jpg"
tiles=()
for t in oak-40-1001 oak-80-1001 beech-40-1001 beech-80-1001 birch-40-1001 spruce-40-1001 palm-40-1001 ordinary-40-1001; do
  for shot in bare whole; do for v in A SR SRB; do tile "$t" "$v" "$shot"; done; done
done
montage "${tiles[@]}" -tile 6x -geometry 400x300+4+4 -pointsize 13 -quality 82 \
  -title "fn-346 last: today (A), SR and SRB, sag on, leafless then leafed: the young oak and the other presets" "$E/last-others.jpg"
echo "$E/last-others.jpg"
