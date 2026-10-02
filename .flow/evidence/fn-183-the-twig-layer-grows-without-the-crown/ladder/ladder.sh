#!/usr/bin/env bash
# fn-183 R1: counts, stats and interleaved timings per preset, seed and rung.
# Appends only; remove the three outputs by hand before a fresh run.
set -euo pipefail
R=/home/daniel/Projects/telperion/.worktrees/fn-183/.flow/evidence/fn-183-the-twig-layer-grows-without-the-crown/raw
PRESETS="oregon-white-oak european-beech silver-birch norway-spruce telperion"
for p in $PRESETS; do for s in 1 7; do for r in 0 1 2 3 4; do
  LADDER=$r GROWTH_SAMPLES=1 $R/bin/gp-count $p $s | sed "s/^{/{\"rung\":$r,/" >> $R/counts.jsonl
  LADDER=$r LADDER_STATS=1 GROWTH_SAMPLES=1 $R/bin/gp-count $p $s 2>&1 >/dev/null \
    | sed "s/^/$p $s $r /" >> $R/stats.txt
done; done; done
for round in 1 2 3; do for p in $PRESETS; do for s in 1 7; do for r in 0 1 2 3 4; do
  LADDER=$r GROWTH_SAMPLES=5 $R/bin/gp-time $p $s \
    | sed "s/^{/{\"rung\":$r,\"round\":$round,/" >> $R/times.jsonl
done; done; done; done
