#!/usr/bin/env bash
# fn-340 R1b: the one labelled diagnostic, never a correction. The probe with
# its cohorts' upkeep scaled each year by the cycle-over-probe ratio R1's
# trace measured, on one seed to 120. Run from the repository root with
# probes/coupled2.patch applied and the core test binary built:
#   bash .flow/evidence/fn-340-the-cohort-law-carried-forward-the/probes/r1b-diag.sh <test binary> <seed>
set -u
E=.flow/evidence/fn-340-the-cohort-law-carried-forward-the
DIAG_UPKEEP=$(python3 -I "$E/probes/diag-ratios.py" "$E/raw/trace-$2-c.log" "$2")
export DIAG_UPKEEP
bash "$E/probes/r1-trace.sh" "$1" "$2" probe+diag -d
