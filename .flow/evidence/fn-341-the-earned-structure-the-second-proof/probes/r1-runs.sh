#!/usr/bin/env bash
# fn-341 R1: the structure's measures on today's cycle, one log per preset
# and age, seeds 1001, 1007 and 1012. Run from the repository root with
# probes/structure.patch applied and the core test binary built
# (cargo test --profile ci -p telperion-core --lib --no-run):
#   bash .flow/evidence/fn-341-the-earned-structure-the-second-proof/probes/r1-runs.sh <test binary> <preset> <age>
set -u
B="$1"
R=.flow/evidence/fn-341-the-earned-structure-the-second-proof/raw
STRUCT_PRESET="$2" STRUCT_AGE="$3" STRUCT_SEEDS=1001,1007,1012 \
  telperion-heavy 8 --threads 4 -- "$B" structure_measures --ignored --nocapture > "$R/r1-$2-$3.log" 2>&1
echo "$2 $3 rc=$?"
