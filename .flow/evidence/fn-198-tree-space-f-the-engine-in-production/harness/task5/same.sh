#!/usr/bin/env bash
# Every tree unchanged: lineage digests and `measures hash 80` against HEAD's.
set -euo pipefail
S=/tmp/claude-1000/-home-daniel-Projects-telperion/dd1a391e-8087-41d5-8d26-b3ea78d9cb37/scratchpad/t5
cd /home/daniel/Projects/telperion/.worktrees/f4
cargo build --profile ci -p telperion-space --example measures --example digests -q 2>&1 | grep -E "^error" -A5 || true
strip() { grep -v '^real\|^user\|^sys\|^$' | sed 's/, grown in.*//; s/ [0-9.]*s$//'; }
target/ci/examples/measures hash 80 1,7 all both | strip > "$S/now-measures.txt"
target/ci/examples/digests | strip > "$S/now-digests.txt"
strip < "$S/base-measures.txt" | diff - "$S/now-measures.txt" && echo "measures hash 80: same"
strip < "$S/base-digests.txt" | diff - "$S/now-digests.txt" && echo "digests: same"
