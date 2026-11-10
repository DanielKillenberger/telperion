#!/usr/bin/env bash
# fn-340 R1c (c): the law open-loop on the cycle's own tree and light
# (fn-339's replay, `cohort_replay`), the oak at 40, 80 and 120 on one seed,
# its by-decade lines read by probes/r1c-replay.py. Run from the repository
# root with probes/coupled2.patch applied and the core test binary built:
#   bash .flow/evidence/fn-340-the-cohort-law-carried-forward-the/probes/r1c-replay.sh <test binary> <seed>
set -u
R=.flow/evidence/fn-340-the-cohort-law-carried-forward-the/raw
for age in 40 80 120; do
  REPLAY_PRESET=oregon-white-oak REPLAY_SEEDS="$2" REPLAY_AGE=$age \
    telperion-heavy 8 --threads 4 -- "$1" cohort_replay --ignored --nocapture > "$R/replay-$2-$age.log" 2>&1
  echo "age $age rc=$?"
done
