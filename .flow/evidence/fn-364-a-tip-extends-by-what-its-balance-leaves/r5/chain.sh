#!/bin/bash
# fn-364: build this branch's runner (its checkout's own target) and
# fn-363's station runner (a target of its own, fn-363-after's sources read
# only), then R3's pause check, then R5's trees.
D=/home/daniel/.cache/claude-tmp/claude-1000/-home-daniel-Projects-telperion/d6d5cb56-440a-44f4-803a-6a595b8a3532/scratchpad
O=$D/fn364
export TELPERION_HEAVY_RESERVE=8
mkdir -p "$O/bin" "$O/logs"
cd "$O/runner" || exit 1
CARGO_TARGET_DIR=/home/daniel/Projects/telperion/.worktrees/fn-364-after/target telperion-heavy 8 --threads 4 -- cargo build --profile ci >> "$O/build-runner.log" 2>&1
echo "build rc=$? $(date '+%F %T')" >> "$O/times.txt"
cp /home/daniel/Projects/telperion/.worktrees/fn-364-after/target/ci/heldrun "$O/bin/heldrun" || exit 1
echo "pausecheck start $(date '+%F %T')" >> "$O/times.txt"
PAUSECHECK=40 telperion-heavy 6 --threads 4 -- "$O/bin/heldrun" oregon-white-oak 80 1001 > "$O/logs/pausecheck.out" 2>&1
echo "pausecheck end rc=$? $(date '+%F %T')" >> "$O/times.txt"
bash "$O/all.sh"
