#!/usr/bin/env bash
# R3: every family x seed x output combination through the base and the
# candidate binding, one fresh instance a build. Appends; clear out/ first.
set -u
S=/tmp/claude-1000/-home-daniel-Projects-telperion/464c173f-e1a8-46dd-b41b-cc247c8ef785/scratchpad
W=/home/daniel/Projects/telperion/.worktrees/fn-102-one-build-pipeline-in-place-of-the
BASE=$S/base/target/wasm32-unknown-unknown/release/telperion_wasm.wasm
CAND=$W/target/wasm32-unknown-unknown/release/telperion_wasm.wasm
cd "$S/out"
for id in ${IDS:-ordinary oregon-white-oak norway-spruce silver-birch telperion laurelin european-beech date-palm}; do
  [ -n "${CAND_ONLY:-}" ] || node "$S/binding.mjs" "$BASE" families.jsonl "$id" >> "r3-base-$id.jsonl" || echo "FAIL base $id"
  node "$S/binding.mjs" "$CAND" families.jsonl "$id" >> "r3-cand-$id.jsonl" || echo "FAIL cand $id"
  echo "done $id"
done
