#!/usr/bin/env bash
# usage: measure-build.sh <checkout>; cold package build (fresh target), timed per step, then sizes.
set -euo pipefail
cd "$1"
t() { local s=$(date +%s%N); "$@"; echo "WALL $* : $(( ($(date +%s%N) - s) / 1000000 )) ms"; }
uptime
t npm run wasm:build
t npm run render:build
t npx tsc -p tsconfig.build.json
t npx vite build
for f in dist/telperion.wasm dist/telperion-render.wasm dist/telperion.js dist/field.js dist/wasm-source.js dist/voxelize.js; do
  echo "SIZE $f raw $(stat -c %s "$f") gzip $(gzip -9 -c "$f" | wc -c)"
done
