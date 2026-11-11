#!/usr/bin/env bash
# fn-352 R4: telperion 0.1.4 beside the scoreboard. Its stills come from the
# v0.1.4 tag in a checkout of its own (`git worktree add --detach <dir>
# v0.1.4`) with its own target/, through that tag's own headless example at
# the hero pose, whole view, the scoreboard's size and sun; its Wasm times
# from the published package (`npm pack telperion@0.1.4`) through wasm014.mjs.
#
#   bash v014.sh <label> <v0.1.4 checkout> <0.1.4 package dir> [build|stills|wasm|sheet]...
set -u
S="$(cd "$(dirname "$0")" && pwd)"
E="$(dirname "$S")"
LABEL="$1" V="$2" PKG="$3"
shift 3
OUT="$S/$LABEL"
RAW="$E/raw/$LABEL/v014"
mkdir -p "$OUT" "$RAW"
PRESETS="oregon-white-oak norway-spruce silver-birch date-palm ordinary laurelin telperion"
SCENE='{"sunAzimuth":115,"sunElevation":60}'

build() {
  ( cd "$V" && telperion-heavy 8 --threads 16 -- cargo build --profile ci \
      -p telperion-render --example headless ) > "$RAW/build.log" 2>&1
  echo "build rc=$?"
}

stills() {
  # One heavy job, every still serially, one process each.
  telperion-heavy 12 --gpu 2 --threads 24 -- bash -c '
    for p in '"$PRESETS"'; do
      seeds="1001 1007 1012"; [ "$p" = oregon-white-oak ] && seeds="7 1001 1007 1012"
      for s in $seeds; do
        out="'"$RAW"'/$p-$s-whole.png"
        [ -s "$out" ] && continue
        python3 "'"$S"'/peak.py" "'"$V"'/target/ci/examples/headless" --preset "$p" --seed "$s" \
          --out "$out" --size 960x720 --view whole --scene '"'$SCENE'"' > "'"$RAW"'/$p-$s.log" 2>&1
        echo "$p $s rc=$?"
      done
    done'
}

wasm() {
  telperion-heavy 4 --threads 1 -- taskset -c 10 node "$S/wasm014.mjs" "$PKG/dist" | tee "$OUT/wasm-0.1.4.jsonl"
}

sheet() {
  tiles=()
  for p in $PRESETS; do
    seeds="1001 1007 1012"; [ "$p" = oregon-white-oak ] && seeds="7 1001 1007 1012"
    for s in $seeds; do
      png="$RAW/$p-$s-whole.png"
      label="0.1.4 $p seed $s (preset age)"
      if [ ! -s "$png" ]; then
        label="$label: NOT DRAWN $(grep -h -o 'Error.*\|error.*' "$RAW/$p-$s.log" 2>/dev/null | head -1 | cut -c1-80)"
        png="xc:gray80[960x720]"
      fi
      tiles+=(-label "$label" "$png")
    done
  done
  montage "${tiles[@]}" -tile 4x -geometry 480x360+4+4 -font Adwaita-Sans -pointsize 13 \
    -title "telperion 0.1.4 (tag v0.1.4), whole view, leafed, its own headless still" \
    -quality 80 "$OUT/sheets/v014-whole.jpg"
  echo "$OUT/sheets/v014-whole.jpg"
}

for stage in "${@:-build stills wasm sheet}"; do
  for s in $stage; do "$s"; done
done
