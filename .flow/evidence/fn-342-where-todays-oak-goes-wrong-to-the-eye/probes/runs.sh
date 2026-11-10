#!/usr/bin/env bash
# fn-342: the probe's runs, from the repository root with probes/turns.patch
# applied and the core test binary built
# (cargo test --profile ci -p telperion-core --lib --no-run):
#   bash probes/runs.sh <test binary> turns <age> [<neutral>]   # R1 joints, tagged
#   bash probes/runs.sh <test binary> years <age>               # R3 yearly record
# TURN_RMIN=<m> also walks every lateral of any PA that thick at its base.
# Neutral: wander, abortion, sag or straightening, set on the limb and bough PAs.
set -u
B="$1"; R=.flow/evidence/fn-342-where-todays-oak-goes-wrong-to-the-eye/raw
case "$2" in
turns)
  log="$R/t-$3${4:+-$4}${TURN_RMIN:+-r$TURN_RMIN}.log"
  STRUCT_AGE="$3" STRUCT_SEEDS=1001,1007 TURN_TAGS=1 TURN_NEUTRAL="${4:-}" \
    telperion-heavy 8 --threads 4 -- "$B" turns_measures --ignored --nocapture > "$log" 2>&1 ;;
years)
  log="$R/y-$3.log"
  STRUCT_AGE="$3" STRUCT_SEEDS=1001,1007 PROBE_YEAR=1 \
    telperion-heavy 8 --threads 4 -- "$B" turns_measures --ignored --nocapture > "$log" 2>&1 ;;
esac
echo "$2 $3 ${4:-} rc=$?"
