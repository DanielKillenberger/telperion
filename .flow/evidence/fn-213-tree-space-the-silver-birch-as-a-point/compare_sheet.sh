#!/bin/bash
# fn-213: the compare sheets of a round, run from this folder:
#   ./compare_sheet.sh <prev round> <round> [prev dir]   e.g. ./compare_sheet.sh 10 11
# [prev dir] (under raw/) defaults to sheet<prev>; round 12 passes
# sheet12/r11b, round 11b's rows rendered on round 12's code.
# Rows per seed: the old birch (raw/compare/old-*), round <prev>, round <round>;
# columns bare, in leaf, crown, limb. Writes raw/compare/compare<round>-<age>.png.
set -e
prev=$1 round=$2 prevdir=${3:-sheet$1}
cd raw
for age in 40 80; do
  args=()
  for s in 1 7 42; do
    for v in bare whole crown limb; do args+=(-label "old s$s $v" compare/old-$s-$v.png); done
    for v in bare whole crown limb; do args+=(-label "r$prev $age s$s $v" $prevdir/birch-$age-$s-$v.png); done
    for v in bare whole crown limb; do args+=(-label "r$round $age s$s $v" sheet$round/birch-$age-$s-$v.png); done
  done
  montage "${args[@]}" -tile 4x9 -geometry 480x360+4+4 -pointsize 18 compare/compare$round-$age.png
done
