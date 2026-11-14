#!/usr/bin/env bash
# fn-371: builds fn-370's leaf probe (copied here) and fn-352's scoreboard
# probe against this checkout's crates, into this spec's ignored raw/target
# (this checkout's own). Run inside one telperion-heavy job.
#   bash build.sh
set -eu
F="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(git -C "$F" rev-parse --show-toplevel)"
S="$ROOT/.flow/evidence/fn-352-the-scoreboard-a-fixed-panel-of-trees/scoreboard"
T="$F/raw/target"
cp "$ROOT/Cargo.lock" "$F/leaf-probe/Cargo.lock"
cp "$ROOT/Cargo.lock" "$S/probe/Cargo.lock"
cargo build --profile ci --manifest-path "$F/leaf-probe/Cargo.toml" --target-dir "$T"
cargo build --profile ci --manifest-path "$S/probe/Cargo.toml" --target-dir "$T"
echo "BUILD DONE $(git -C "$ROOT" rev-parse --short HEAD)"
