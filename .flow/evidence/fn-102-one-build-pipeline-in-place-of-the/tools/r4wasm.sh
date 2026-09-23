#!/usr/bin/env bash
# R4 Wasm: the binding's own stage timings, five builds per side, interleaved
# per preset, for the mesh request, the field request and every output at once.
set -u
S=/tmp/claude-1000/-home-daniel-Projects-telperion/464c173f-e1a8-46dd-b41b-cc247c8ef785/scratchpad
W=/home/daniel/Projects/telperion/.worktrees/fn-102-one-build-pipeline-in-place-of-the
BASE=$S/base/target/wasm32-unknown-unknown/release/telperion_wasm.wasm
CAND=$W/target/wasm32-unknown-unknown/release/telperion_wasm.wasm
cd "$S/out"
grep '"seed":1' families.jsonl > families-seed1.jsonl
for id in ${IDS:-ordinary oregon-white-oak norway-spruce silver-birch telperion laurelin}; do
  for combo in 3 8 15; do
    node "$S/binding.mjs" "$BASE" families-seed1.jsonl "$id" 5 "$combo" >> r4w-base.jsonl || echo "FAIL base $id"
    node "$S/binding.mjs" "$CAND" families-seed1.jsonl "$id" 5 "$combo" >> r4w-cand.jsonl || echo "FAIL cand $id"
  done
  echo "done $id"
done
