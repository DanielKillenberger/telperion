#!/usr/bin/env bash
# One core, interleaved: base (4ef2aebe) against the worktree's engine now.
# Arguments: "tree age runs" triples; `wasm` as the first to time the Wasm build.
set -euo pipefail
S=/tmp/claude-1000/-home-daniel-Projects-telperion/dd1a391e-8087-41d5-8d26-b3ea78d9cb37/scratchpad/t5
mkdir -p "$S/work/crates/telperion-space" "$S/wwork/src"
rsync -rc --delete --out-format=%n /home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-space/src/ "$S/work/crates/telperion-space/src/" \
  | while read -r f; do [ -f "$S/work/crates/telperion-space/src/$f" ] && touch "$S/work/crates/telperion-space/src/$f"; done
cp /home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-space/Cargo.toml "$S/work/crates/telperion-space/"
mkdir -p "$S/work/crates/telperion-space/examples"
cp "$S/probe.rs" "$S/staged.rs" "$S/work/crates/telperion-space/examples/"
cp "$S/base/Cargo.toml" "$S/base/Cargo.lock" "$S/base/rust-toolchain.toml" "$S/work/"
sed 's#\.\./head/#../work/#' "$S/whead/Cargo.toml" > "$S/wwork/Cargo.toml.new"
mv "$S/wwork/Cargo.toml.new" "$S/wwork/Cargo.toml"
cp "$S/whead/src/lib.rs" "$S/wwork/src/lib.rs"
cp "$S/base/rust-toolchain.toml" "$S/wwork/"
(cd "$S/work" && cargo build --release --example probe --example staged -q)
(cd "$S/wwork" && cargo build --release --target wasm32-unknown-unknown -q)
uptime
if [ "${1:-}" = wasm ]; then
  shift
  for t in "$@"; do
    set -- $t
    case $1 in palm) w=0;; beech) w=1;; oak) w=2;; *) w=3;; esac
    for v in wbase wwork; do (cd "$S" && taskset -c 0 node wrun.mjs $v $w $2 $3); done
  done
else
  # Three interleaved rounds a tree; the best of each side is the reading.
  for t in "$@"; do
    for v in base work; do : > "$S/round-$v.txt"; done
    for r in 1 2 3; do
      for v in base work; do
        RAYON_NUM_THREADS=1 taskset -c 0 "$S/$v/target/release/examples/probe" $t >> "$S/round-$v.txt"
      done
    done
    for v in base work; do
      echo "$v $(sort -t' ' -k4 -n "$S/round-$v.txt" | head -1)"
    done
  done
fi
uptime
