#!/bin/bash
# fn-236: the compare sheets of a round, run from this folder:
#   ./compare_sheet.sh <prev round> <round>
# Per age (40, 80) one sheet, a row per seed (1, 7, 42): O-WHOLE, the 0.1.4
# oak (raw/old/, one tree per seed: the old generator ignores age), round
# <prev> in leaf, round <round> in leaf, bare and crown.
# Writes raw/compare/compare<round>-<age>.png.
set -e
prev=$1 round=$2
cd raw
mkdir -p compare
for age in 40 80; do
  args=()
  for s in 1 7 42; do
    args+=(-label "O-WHOLE" ref/o-whole.jpg)
    args+=(-label "0.1.4 s$s" "old/old-$s-whole.png")
    args+=(-label "r$prev $age s$s" "sheet$prev/oak-$age-$s-whole.png")
    for v in whole bare crown; do
      args+=(-label "r$round $age s$s $v" "sheet$round/oak-$age-$s-$v.png")
    done
  done
  montage "${args[@]}" -tile 6x3 -geometry 480x360+4+4 -pointsize 18 \
    "compare/compare$round-$age.png"
done
echo "raw/compare/compare$round-40.png raw/compare/compare$round-80.png"
