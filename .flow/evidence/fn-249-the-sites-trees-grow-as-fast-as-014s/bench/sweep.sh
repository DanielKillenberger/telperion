#!/usr/bin/env bash
# fn-249 sweep: argv wasm out.jsonl [cut]. Per species, per rep, two Node
# processes, seeds in both orders, so each seed has a cold (first) and a warm
# (second) grow in one instance. One thread: Node runs Wasm on one.
cd "$(dirname "$0")" || exit 1
W=$1; OUT=$2; CUT=${3:-1024}
SP=("oregon-white-oak 0.08 tuft 0.4" "silver-birch 0.08 tuft 0.32" "norway-spruce 0.1 density 0.8")
for rep in 1 2 3; do
for line in "${SP[@]}"; do
  set -- $line
  for seeds in 1234,5678 5678,1234; do
    timeout 600 node bench.mjs "$W" $1 $seeds 24,30,36,48 $CUT $2 $3 $4 | while read -r l; do
      echo "{\"rep\":$rep,\"load\":$(cut -d' ' -f1 /proc/loadavg),\"run\":$l}"; done >> "$OUT"
  done
done
done
