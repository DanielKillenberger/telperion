#!/usr/bin/env bash
# fn-339 R4: the coupled probe's reads, timings and peaks, serially, each
# through telperion-heavy. Run from the repository root with
# probes/coupled.patch applied and the core test binary built:
#   bash .flow/evidence/fn-339-the-cohort-law-the-first-proof/probes/r4-runs.sh <test binary> <part>
# Parts: old (reads at 80 and 120), time (one-thread batches), peak.
set -u
B="$1"
E=.flow/evidence/fn-339-the-cohort-law-the-first-proof
R="$E/raw"
PEAK=.flow/evidence/fn-330-how-far-the-full-cycle-grows-an-oak/probes/peak.py
S=1001,1002,1003,1004,1005,1006,1007,1008,1009,1010,1011,1012
case "$2" in
old)
  for age in 80 120; do
    COUPLED_PRESET=oregon-white-oak COUPLED_SEEDS=1001,1007,1012 COUPLED_AGE=$age \
      telperion-heavy 10 --threads 4 -- "$B" cohort_coupled --ignored --nocapture > "$R/r4-read-oak$age.log" 2>&1
  done ;;
time)
  # One pinned thread, five alternating batches (fn-327's habit).
  for p in oregon-white-oak european-beech norway-spruce; do
    COUPLED_PRESET=$p COUPLED_SEEDS=$S COUPLED_MODE=time COUPLED_BATCHES=5 \
      telperion-heavy 8 --threads 1 -- taskset -c 3 "$B" cohort_coupled --ignored --nocapture > "$R/r4-time-$p-40.log" 2>&1
  done
  for age in 80 120; do
    COUPLED_PRESET=oregon-white-oak COUPLED_SEEDS=1001,1007,1012 COUPLED_AGE=$age COUPLED_MODE=time COUPLED_BATCHES=5 \
      telperion-heavy 10 --threads 1 -- taskset -c 3 "$B" cohort_coupled --ignored --nocapture > "$R/r4-time-oregon-white-oak-$age.log" 2>&1
  done ;;
peak)
  # One process per tree and mode, seed 1001, one thread (fn-330's peak.py).
  for spec in oregon-white-oak:0 european-beech:0 norway-spruce:0 oregon-white-oak:80 oregon-white-oak:120; do
    p=${spec%%:*}; age=${spec##*:}
    for mode in base probe; do
      COUPLED_PRESET=$p COUPLED_SEEDS=1001 COUPLED_AGE=$age COUPLED_MODE=$mode \
        telperion-heavy 10 --threads 1 -- python3 "$PEAK" "$B" cohort_coupled --ignored --nocapture > "$R/r4-peak-$p-$age-$mode.log" 2>&1
    done
  done ;;
esac
