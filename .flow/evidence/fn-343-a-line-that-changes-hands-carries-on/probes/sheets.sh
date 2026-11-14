#!/usr/bin/env bash
# fn-343 ablation sheets: per variant B..G, a leafless and a leafed sheet;
# rows the trees (40/1001, 40/1007, 120/1001), columns A beside the variant
# at the whole view, then A beside it at the mid view.
#   bash probes/sheets.sh
set -eu
E=.flow/evidence/fn-343-a-line-that-changes-hands-carries-on
S="$E/raw/stills"
declare -A NAME=([B]="B no limb/bough abortion" [C]="C no wander" [D]="D no sag"
  [E]="E branch+twig insertion halved" [F]="F branch never moves on" [G]="G = B+E+F")
for v in B C D E F G; do
  for kind in bare whole; do
    mid=$([ "$kind" = bare ] && echo limb || echo spray)
    tiles=()
    for t in 40-1001 40-1007 120-1001; do
      for shot in "$kind" "$mid"; do
        f="$S/oak-$t-$v-$shot.png"
        # A tree the renderer refused (G at 120: its wood points exceed the
        # device's buffer limit) is a grey tile saying so.
        [ -f "$f" ] || f="xc:gray85[960x720]"
        lab="$t $v $shot"; [ "${f#xc:}" = "$f" ] || lab="$t $v: not drawn, wood over the device buffer limit"
        tiles+=(-label "$t A $shot" "$S/oak-$t-A-$shot.png" -label "$lab" "$f")
      done
    done
    montage "${tiles[@]}" -tile 4x3 -geometry 480x360+4+4 -pointsize 18 -quality 82 \
      -title "fn-343 ablation, ${NAME[$v]} beside A (today); $([ "$kind" = bare ] && echo leafless || echo leafed)" \
      "$E/sheet-$v-$kind.jpg"
    echo "$E/sheet-$v-$kind.jpg"
  done
done
