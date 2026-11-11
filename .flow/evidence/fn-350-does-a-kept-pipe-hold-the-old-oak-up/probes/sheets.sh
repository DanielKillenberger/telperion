#!/usr/bin/env bash
# fn-350 R3: today's build (A, fn-345 and fn-346's raw stills), then the
# fix branch at retained 0 (fn-349's own stills, tag N), 0.5 and 1 (this
# spec's raw stills), rows by age; leafless and leafed sheets per seed,
# and the beech at 40. Labels: drawn height, spread and the trunk at 1.3 m
# from each tree's log; a tree refused is a grey tile labelled with its
# error, a tree never drawn a grey tile "not drawn".
#   bash sheets.sh   (from anywhere)
set -eu
F=/home/daniel/Projects/telperion/.worktrees/fn-319/.flow/evidence
E="$F/fn-350-does-a-kept-pipe-hold-the-old-oak-up"
S="$E/raw/stills"
T345="$F/fn-345-what-bounds-a-tree-vigour-falls-with/raw/stills"
T346="$F/fn-346-the-carbon-balance-charges-living-wood/raw/stills"
N=/home/daniel/Projects/telperion/.worktrees/fn-349/.flow/evidence/fn-349-the-corrected-balance-in-the-product/raw/stills
A_dir() { if [ -f "$T345/$1-A-whole.png" ]; then echo "$T345"; else echo "$T346"; fi; }
png() {  # <tree> <col> <shot>
  case "$2" in
    A) echo "$(A_dir "$1")/$1-A-$3.png" ;;
    0) echo "$N/$1-N-$3.png" ;;
    *) echo "$S/$1-R$2-$3.png" ;;
  esac
}
log() {  # <tree> <col>
  case "$2" in
    A) echo "$(A_dir "$1")/$1-A.log" ;;
    *) echo "$S/$1-R$2.log" ;;
  esac
}
label() {  # <tree> <col> <age> <seed>
  local l; l=$(log "$1" "$2")
  [ -f "$l" ] || { echo "not drawn"; return; }
  local err; err=$(grep -o 'below the ground\|Error: .*' "$l" | head -1 || true)
  if [ -n "$err" ]; then
    grep -o 'axis [0-9]* (PA [0-9]*[^)]*) reaches [0-9.]* m below the ground' "$l" | head -1 \
      | sed 's/^/REFUSED: /' || echo "REFUSED"
    return
  fi
  local hw dbh
  hw=$(grep -h "age $3 seed $4" "$l" | grep -o '[0-9.]* m tall, [0-9.]* x [0-9.]* m' | head -1 || true)
  dbh=$(grep -o 'trunk [0-9.]* cm' "$l" | head -1 || true)
  echo "${hw:-not drawn}${dbh:+, $dbh}"
}
tile() {  # <tree> <col> <shot>
  local f; f=$(png "$1" "$2" "$3")
  local age seed; age=$(echo "$1" | cut -d- -f2); seed=$(echo "$1" | cut -d- -f3)
  local name="$2"; [ "$2" = A ] && name="today"
  [ "$2" != A ] && name="kept $2"
  local text; text=$(label "$1" "$2" "$age" "$seed")
  [ -f "$f" ] || f="xc:gray85[960x720]"
  tiles+=(-label "$1 $name $3: $text" "$f")
}
T="fn-350: today, then the fix branch with retained (kept pipe) 0, 0.5 and 1"
for seed in 1001 1007 1012; do
  ages="120 200 300"; [ "$seed" = 1001 ] && ages="40 80 120 200 300"
  for shot in bare whole; do
    tiles=()
    for age in $ages; do for c in A 0 0.5 1; do tile "oak-$age-$seed" "$c" "$shot"; done; done
    montage "${tiles[@]}" -tile 4x -geometry 480x360+4+4 -pointsize 13 -quality 82 \
      -title "$T, oak seed $seed, $shot" "$E/kept-oak-$seed-$shot.jpg"
    echo "$E/kept-oak-$seed-$shot.jpg"
  done
done
tiles=()
for shot in bare whole; do for c in A 0 0.5 1; do tile beech-40-1001 "$c" "$shot"; done; done
montage "${tiles[@]}" -tile 4x -geometry 480x360+4+4 -pointsize 13 -quality 82 \
  -title "$T, beech 40 seed 1001, leafless then leafed" "$E/kept-beech-40-1001.jpg"
echo "$E/kept-beech-40-1001.jpg"
