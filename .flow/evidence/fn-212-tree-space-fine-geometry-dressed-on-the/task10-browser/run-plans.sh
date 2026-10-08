#!/bin/bash
# Drives each plan named after the output directory, with the RSS sampler beside it.
#   target/run-plans.sh <out-dir> <plan>...
set -u
cd /home/daniel/Projects/telperion/.worktrees/readers
out=$1; shift
mkdir -p "$out"
for sp in "$@"; do
  rm -f target/rss.stop
  python3 target/rss.py 1200 target/rss.stop >> "$out/rss-$sp.txt" &
  sampler=$!
  sleep 1
  timeout 1200 node target/drive.mjs http://127.0.0.1:5212/ "$out" "target/plans/$sp.json" 2>&1 | cut -c1-700 | tee "$out/log-$sp.txt"
  sleep 2
  touch target/rss.stop
  wait "$sampler"
  cat "$out/rss-$sp.txt"
done
