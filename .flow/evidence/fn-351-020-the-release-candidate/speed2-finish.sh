#!/usr/bin/env bash
# fn-351 speed round 2: the bundle built (with and without the floor), the
# quiet reads, then each member proved alone. One queue admission.
set -u
here=$(cd "$(dirname "$0")" && pwd)
root=$(git -C "$here" rev-parse --show-toplevel)
cd "$root" || exit 1
c="$here/speed2-candidates"
build() {  # label, patches...
  local label=$1; shift
  local out="$here/raw/speed2/$label"; mkdir -p "$out"
  for p in "$@"; do git apply "$c/$p.patch" || echo "patch $p failed" >> "$out/STATUS"; done
  if cargo build --profile ci -p telperion-core --example early > "$out/build.log" 2>&1; then
    cp target/ci/examples/early "$out/early"; echo "built $(date '+%F %T')" >> "$out/STATUS"
  else
    echo "build failed" >> "$out/STATUS"; grep -A12 "^error" "$out/build.log" | head -40
  fi
  local k; for (( k=$#; k>0; k-- )); do git apply -R "$c/${!k}.patch"; done
}
build bundle r3r4-deposit r5-node-wood r2-wood-leaf
build bundle6 r3r4-deposit r5-node-wood r2-wood-leaf r6-floor
oak="oregon-white-oak:40:1001"
"$here/speed2-quiet.sh" one 1 "$oak" r0-base r4-deposit-count r5-node-wood r2-wood-leaf r6-floor bundle bundle6
"$here/speed2-quiet.sh" four 4 "$oak" r0-base r3-deposit-parts bundle bundle6
for p in r3-deposit-parts r3r4-deposit r5-node-wood r2-wood-leaf r6-floor; do
  git apply "$c/$p.patch" && "$here/speed2-tests.sh" "alone-$p"
  git apply -R "$c/$p.patch"
done
git status --short crates
