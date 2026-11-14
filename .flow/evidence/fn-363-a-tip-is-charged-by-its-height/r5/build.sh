#!/bin/bash
# fn-363: builds the grower runner against fn-363-<side>, into that worktree's target.
D=/home/daniel/.cache/claude-tmp/claude-1000/-home-daniel-Projects-telperion/d6d5cb56-440a-44f4-803a-6a595b8a3532/scratchpad/fn363
side=$1
cd "$D/runner-$side" || exit 1
mkdir -p "$D/bin"
export CARGO_TARGET_DIR=/home/daniel/Projects/telperion/.worktrees/fn-363-$side/target TELPERION_HEAVY_RESERVE=8
telperion-heavy 8 --threads 4 -- cargo build --profile ci > "$D/build-$side.log" 2>&1
rc=$?
cp "$CARGO_TARGET_DIR/ci/heldrun" "$D/bin/heldrun-$side" && echo "copied heldrun-$side"
echo "build $side rc=$rc $(date '+%F %T')"
