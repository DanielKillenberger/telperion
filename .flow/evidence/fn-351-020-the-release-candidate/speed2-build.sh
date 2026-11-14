#!/usr/bin/env bash
# fn-351 speed round 2: each candidate change built on its own from a
# patch against the base (speed2-candidates/<patch>.patch), its `early`
# kept as raw/speed2/<label>/early, and the working tree left as it was;
# then each read against its base (speed2-ab.sh). One queue admission:
#
#   TELPERION_HEAVY_RESERVE=8 telperion-heavy 4 --threads 4 -- \
#     .flow/evidence/fn-351-020-the-release-candidate/speed2-build.sh "<label>:<patch>:<before> ..."
set -u
here=$(cd "$(dirname "$0")" && pwd)
root=$(git -C "$here" rev-parse --show-toplevel)
cd "$root" || exit 1
for row in $1; do
  IFS=: read -r label patch before <<< "$row"
  out="$here/raw/speed2/$label"
  mkdir -p "$out"
  if ! git apply "$here/speed2-candidates/$patch.patch"; then
    echo "patch failed $(date '+%F %T')" >> "$out/STATUS"
    continue
  fi
  if cargo build --profile ci -p telperion-core --example early > "$out/build.log" 2>&1; then
    cp target/ci/examples/early "$out/early"
    echo "built $(date '+%F %T')" >> "$out/STATUS"
  else
    echo "build failed $(date '+%F %T')" >> "$out/STATUS"
    grep -A12 "^error" "$out/build.log" | head -60
  fi
  git apply -R "$here/speed2-candidates/$patch.patch"
done
for row in $1; do
  IFS=: read -r label patch before <<< "$row"
  [ -x "$here/raw/speed2/$label/early" ] || continue
  "$here/speed2-ab.sh" "$before" "$label" 5
  python3 "$here/speed2-ab.py" "$here/raw/speed2/$label/ab"
done
