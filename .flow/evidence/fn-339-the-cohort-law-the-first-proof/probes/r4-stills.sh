#!/usr/bin/env bash
# fn-339 R4: whole-view stills of the cycle and the coupled probe
# (TELPERION_COHORTS=0.5), seeds 1001 and 1007, fn-336's still examples.
# Run from the repository root with probes/coupled.patch applied and the
# space_* examples built, inside one GPU job:
#   telperion-heavy 10 --gpu 2 --threads 4 -- bash <this file>
set -u
E=.flow/evidence/fn-339-the-cohort-law-the-first-proof
PEAK=.flow/evidence/fn-330-how-far-the-full-cycle-grows-an-oak/probes/peak.py
for mode in base probe; do
  out="$E/raw/stills/$mode"
  mkdir -p "$out"
  if [ "$mode" = probe ]; then export TELPERION_COHORTS=0.5; else unset TELPERION_COHORTS; fi
  for example in space_oak space_spruce space_beech; do
    python3 "$PEAK" "target/ci/examples/$example" "$out" 40 --seeds 1001,1007 --shots whole > "$out/$example-40.log" 2>&1
  done
  python3 "$PEAK" target/ci/examples/space_oak "$out" 80 120 --seeds 1001,1007 --shots whole > "$out/space_oak-old.log" 2>&1
done
