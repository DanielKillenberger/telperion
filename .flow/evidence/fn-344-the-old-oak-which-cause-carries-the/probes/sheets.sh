#!/usr/bin/env bash
# fn-344 ablation sheets: per variant B..F one sheet; rows the trees
# (1001 at 120, 200 and 300; 1007 at 300), columns A beside the variant
# leafless (whole), leafed (whole) and the lower trunk (bole, leafless).
# A tree the variant did not draw (stopped by the census, or refused by the
# device) is a grey tile saying why.
#   bash probes/sheets.sh
set -eu
E=.flow/evidence/fn-344-the-old-oak-which-cause-carries-the
S="$E/raw/stills"
declare -A NAME=([B]="B dead wood not charged" [C]="C stem line pulled vertical as the leader"
  [D]="D the leader lives past the run" [E]="E trunk-line sleeping buds wake as limbs"
  [F]="F = B + C + E")
label() {  # tree, variant: a short measure line from its log
  local log="$S/oak-$1-$2.log"
  if grep -q '^STOP' "$log" 2>/dev/null; then
    grep '^STOP' "$log" | sed 's/^STOP /stopped: /'
  else
    grep -o '[0-9.]* m tall, [0-9.]* x [0-9.]* m' "$log" 2>/dev/null || echo "not drawn"
  fi
}
for v in B C D E F; do
  tiles=()
  for t in 120-1001 200-1001 300-1001 300-1007; do
    for shot in bare whole bole; do
      for w in A "$v"; do
        f="$S/oak-$t-$w-$shot.png"
        # Stopped by the spec's census rule: the second reading
        # (probes/stills-peak.sh, tagged <v>p) where one was drawn.
        if [ ! -f "$f" ] && [ -f "$S/oak-$t-${w}p-$shot.png" ]; then
          w="${w}p"; f="$S/oak-$t-$w-$shot.png"
        fi
        lab="$t $w $shot: $(label "$t" "$w")"
        [ -f "$f" ] || f="xc:gray85[960x720]"
        tiles+=(-label "$lab" "$f")
      done
    done
  done
  montage "${tiles[@]}" -tile 6x4 -geometry 480x360+4+4 -pointsize 15 -quality 82 \
    -title "fn-344 old oak, ${NAME[$v]} beside A (today): leafless, leafed, lower trunk" \
    "$E/sheet-$v.jpg"
  echo "$E/sheet-$v.jpg"
done
