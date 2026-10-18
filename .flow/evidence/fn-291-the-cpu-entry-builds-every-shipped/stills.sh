#!/usr/bin/env bash
# fn-291 task 4: the stills of Telperion and Laurelin from the CPU entry's
# own mesh. Seed 1: the base flat dist against the chunked dist. Seeds 7 and
# 42 (the base dist never built them): the chunked dist against the base
# native flat mesh. Each step is admitted on its own: a dist still needs
# about 7 GB, a native one 10 (the driver's mesh, then Node's copy).
#   stills.sh <base-dist> <new-dist> <flatmesh-base binary> <out-dir>
set -euo pipefail
here=$(dirname "$(realpath "$0")")
base=$1 new=$2 native=$3 out=$4
mkdir -p "$out"
still() { telperion-heavy "$1" --threads 2 -- node "$here/still.mjs" "${@:2}"; }
for id in telperion laurelin; do
  still 8 dist "$base" "$id" 1 "$out/$id-1-before.png" "$out/$id-1.camera.json"
  still 8 dist "$new" "$id" 1 "$out/$id-1-after.png" "$out/$id-1.camera.json"
  node "$here/still.mjs" compose "$out/$id-1-before.png" "$out/$id-1-after.png" "$out/$id-1.png"
  for seed in 7 42; do
    still 8 dist "$new" "$id" "$seed" "$out/$id-$seed-after.png" "$out/$id-$seed.camera.json"
    still 10 native "$native" "$id" "$seed" "$out/$id-$seed-before.png" "$out/$id-$seed.camera.json"
    node "$here/still.mjs" compose "$out/$id-$seed-before.png" "$out/$id-$seed-after.png" "$out/$id-$seed.png"
  done
done
