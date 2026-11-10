#!/usr/bin/env bash
# fn-340 R1: the oak grown one year at a time to 120 as the cycle, the
# probe, and the probe with each switch alone, one seed per call. Run from
# the repository root with probes/coupled2.patch applied and the core test
# binary built (cargo test --profile ci -p telperion-core --lib --no-run):
#   bash .flow/evidence/fn-340-the-cohort-law-carried-forward-the/probes/r1-trace.sh <test binary> <seed> [<modes> <log suffix>]
set -u
B="$1"
R=.flow/evidence/fn-340-the-cohort-law-carried-forward-the/raw
TRACE_PRESET=oregon-white-oak TRACE_SEEDS="$2" TRACE_AGE=120 TRACE_EVERY=5 \
  TRACE_MODES="${3:-base,probe,probe+split,probe+spread,probe+nopipes}" \
  telperion-heavy 8 --threads 4 -- "$B" cohort_trace --ignored --nocapture > "$R/trace-$2${4:-}.log" 2>&1
echo "rc=$?"
