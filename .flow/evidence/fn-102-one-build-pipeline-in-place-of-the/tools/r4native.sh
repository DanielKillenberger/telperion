#!/usr/bin/env bash
# R4 native: generation_stages per catalogue preset at seed 1, base and
# candidate interleaved over five rounds so load drift on a shared machine
# falls on both alike. Each process builds twice; the first, cold build is
# dropped. Peak RSS of each process is recorded beside its samples.
set -u
S=/tmp/claude-1000/-home-daniel-Projects-telperion/464c173f-e1a8-46dd-b41b-cc247c8ef785/scratchpad
W=/home/daniel/Projects/telperion/.worktrees/fn-102-one-build-pipeline-in-place-of-the
cd "$S/out"
for id in ${IDS:-ordinary oregon-white-oak norway-spruce silver-birch telperion laurelin}; do
  for round in 1 2 3 4 5; do
    for side in base cand; do
      if [ "$side" = base ]; then bin=$S/base/target/release/examples/generation_stages; else bin=$W/target/release/examples/generation_stages; fi
      GENERATION_SAMPLES=2 python3 "$S/rss.py" "$id" "$bin" "$id" 1 >> "r4-$side.jsonl" || echo "FAIL $side $id"
    done
  done
  echo "done $id"
done
