#!/bin/bash
# fn-364 R6: the sheets. Columns: base-2026-11-11 (the oak and the Two
# Trees), the top and fn-363's branch (fn-363's builder's tiles, reused, not
# redrawn), this branch (the Two Trees with R8).
W=/home/daniel/Projects/telperion/.worktrees
F=$W/fn-364/.flow/evidence/fn-364-a-tip-extends-by-what-its-balance-leaves
S=$W/fn-364/.flow/evidence/fn-352-the-scoreboard-a-fixed-panel-of-trees
O=$W/fn-363/.flow/evidence/fn-352-the-scoreboard-a-fixed-panel-of-trees
BASE="base-2026-11-11=$O/scoreboard/base-2026-11-11:$W/fn-319/.flow/evidence/fn-352-the-scoreboard-a-fixed-panel-of-trees/raw/base-2026-11-11/stills"
TOP="top=$O/scoreboard/fn363-top:$O/raw/fn363-top/stills"
FN363="fn-363=$O/scoreboard/fn363-height:$O/raw/fn363-height/stills"
THIS="this=$S/scoreboard/fn364-this:$S/raw/fn364-this/stills"
R8="this, R8=$S/scoreboard/fn364-r8:$S/raw/fn364-r8/stills"
TIP363="fn-363=$S/scoreboard/fn364-363tip:$S/raw/fn364-363tip/stills"
mkdir -p "$F/sheets"
sheet() { python3 -I "$F/sheet.py" "$F/sheets/$1.jpg" "${@:2}"; }
sheet oak oregon-white-oak 40,80,120,200,300 1001,1007 whole,bare "$BASE" "$TOP" "$FN363" "$THIS"
sheet oak-limb-tip oregon-white-oak 200 1001 tip "$TIP363" "$THIS"
for p in telperion laurelin; do
  sheet "$p" "$p" preset 1,7,1001,1007 whole,bare "$BASE" "$TOP" "$FN363" "$R8"
done
for p in norway-spruce european-beech silver-birch date-palm ordinary; do
  sheet "$p" "$p" preset 1001,1007 whole "$TOP" "$FN363" "$THIS"
done
