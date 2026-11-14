#!/usr/bin/env bash
# fn-343 R1: the joint walk on all wood 1 cm or more, tagged, with each
# take-over's bearer stop cause (probes/stops.patch applied, binary built):
#   bash probes/runs.sh <test binary> <age>
set -u
B="$1"; R=.flow/evidence/fn-343-a-line-that-changes-hands-carries-on/raw
log="$R/t-$2-r0.01-fork.log"
STRUCT_AGE="$2" STRUCT_SEEDS=1001,1007 TURN_TAGS=1 TURN_RMIN=0.01 \
  telperion-heavy 8 --threads 4 -- "$B" turns_measures --ignored --nocapture > "$log" 2>&1
echo "turns $2 rc=$?"
