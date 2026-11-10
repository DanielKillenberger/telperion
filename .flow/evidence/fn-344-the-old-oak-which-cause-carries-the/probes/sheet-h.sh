#!/usr/bin/env bash
# fn-344 follow-up 3: sheet-H.jpg, rows seed 1001 at 40, 120, 200, 260 and 300,
# columns A beside H leafless, leafed and the lower trunk. A tree not drawn
# is a grey tile with the reason from its log.
#   bash probes/sheet-h.sh
set -eu
E=.flow/evidence/fn-344-the-old-oak-which-cause-carries-the
S="$E/raw/stills"
label() {
  local log="$S/oak-$1-$2.log"
  if grep -q 'above the device' "$log" 2>/dev/null; then
    grep -o 'needs [0-9]* bytes, above the device.s limit of [0-9]*' "$log" | head -1
  else
    grep -o '[0-9.]* m tall, [0-9.]* x [0-9.]* m' "$log" 2>/dev/null || echo "not drawn"
  fi
}
tiles=()
for t in 40-1001 120-1001 200-1001 260-1001 300-1001; do
  for shot in bare whole bole; do
    for w in A H; do
      f="$S/oak-$t-$w-$shot.png"
      lab="$t $w $shot: $(label "$t" "$w")"
      [ -f "$f" ] || f="xc:gray85[960x720]"
      tiles+=(-label "$lab" "$f")
    done
  done
done
montage "${tiles[@]}" -tile 6x5 -geometry 480x360+4+4 -pointsize 15 -quality 82 \
  -title "fn-344 old oak, H (B + internodes shortened by 1 - L/34.8 m) beside A (today): leafless, leafed, lower trunk" \
  "$E/sheet-H.jpg"
echo "$E/sheet-H.jpg"
