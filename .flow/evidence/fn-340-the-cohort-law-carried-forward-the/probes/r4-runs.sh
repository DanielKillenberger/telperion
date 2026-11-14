#!/usr/bin/env bash
# fn-340 R4: the corrected, carried probe's reads, timings, peaks, phases
# and stills, serially, each through telperion-heavy (fn-339's
# probes/r4-runs.sh and r4-stills.sh, with R4's seeds). Run from the
# repository root with probes/coupled2.patch applied, the core test binary
# and the space_oak example built:
#   bash .flow/evidence/fn-340-the-cohort-law-carried-forward-the/probes/r4-runs.sh <test binary> <part>
# Parts: read40, readold, time40, timeold, peak, phases, stills.
set -u
B="$1"
E=.flow/evidence/fn-340-the-cohort-law-carried-forward-the
R="$E/raw"
PEAK=.flow/evidence/fn-330-how-far-the-full-cycle-grows-an-oak/probes/peak.py
S12=1001,1002,1003,1004,1005,1006,1007,1008,1009,1010,1011,1012
S6=1001,1002,1003,1004,1005,1006
case "$2" in
read40)
  for p in oregon-white-oak norway-spruce european-beech; do
    COUPLED_PRESET=$p COUPLED_SEEDS=$S12 \
      telperion-heavy 8 --threads 4 -- "$B" cohort_coupled --ignored --nocapture > "$R/r4-read-$p-40.log" 2>&1
    echo "$p rc=$?"
  done ;;
readold)
  for age in 80 120; do
    COUPLED_PRESET=oregon-white-oak COUPLED_SEEDS=$S6 COUPLED_AGE=$age \
      telperion-heavy 8 --threads 4 -- "$B" cohort_coupled --ignored --nocapture > "$R/r4-read-oregon-white-oak-$age.log" 2>&1
    echo "$age rc=$?"
  done ;;
time40)
  # One pinned thread, five alternating batches (fn-327's habit).
  for p in oregon-white-oak european-beech norway-spruce; do
    COUPLED_PRESET=$p COUPLED_SEEDS=$S12 COUPLED_MODE=time COUPLED_BATCHES=5 \
      telperion-heavy 8 --threads 1 -- taskset -c 3 "$B" cohort_coupled --ignored --nocapture > "$R/r4-time-$p-40.log" 2>&1
    echo "$p rc=$?"
  done ;;
timeold)
  for age in 80 120; do
    COUPLED_PRESET=oregon-white-oak COUPLED_SEEDS=$S6 COUPLED_AGE=$age COUPLED_MODE=time COUPLED_BATCHES=5 \
      telperion-heavy 8 --threads 1 -- taskset -c 3 "$B" cohort_coupled --ignored --nocapture > "$R/r4-time-oregon-white-oak-$age.log" 2>&1
    echo "$age rc=$?"
  done ;;
peak)
  # One process per tree and mode, seed 1001, one thread (fn-330's peak.py).
  for spec in oregon-white-oak:0 european-beech:0 norway-spruce:0 oregon-white-oak:80 oregon-white-oak:120; do
    p=${spec%%:*}; age=${spec##*:}
    for mode in base probe; do
      COUPLED_PRESET=$p COUPLED_SEEDS=1001 COUPLED_AGE=$age COUPLED_MODE=$mode \
        telperion-heavy 8 --threads 1 -- python3 "$PEAK" "$B" cohort_coupled --ignored --nocapture > "$R/r4-peak-$p-$age-$mode.log" 2>&1
    done
  done ;;
phases)
  # Where the seconds go, seed 1001, one thread: the cohort step is `Grown`.
  for spec in oregon-white-oak:0 norway-spruce:0 european-beech:0 oregon-white-oak:80 oregon-white-oak:120; do
    p=${spec%%:*}; age=${spec##*:}
    COUPLED_PRESET=$p COUPLED_SEEDS=1001 COUPLED_AGE=$age COUPLED_PHASES=1 \
      telperion-heavy 8 --threads 1 -- taskset -c 3 "$B" cohort_coupled --ignored --nocapture > "$R/r4-phases-$p-$age.log" 2>&1
    echo "$p $age rc=$?"
  done ;;
stills)
  # The oak's whole view, seeds 1001 and 1007, the cycle and the probe;
  # inside one GPU job: telperion-heavy 8 --gpu 2 --threads 4 -- bash <this> <bin> stills
  for mode in base probe; do
    out="$R/stills/$mode"
    mkdir -p "$out"
    if [ "$mode" = probe ]; then export TELPERION_COHORTS=0.5; else unset TELPERION_COHORTS; fi
    python3 "$PEAK" target/ci/examples/space_oak "$out" 40 80 120 --seeds 1001,1007 --shots whole > "$out/space_oak.log" 2>&1
    echo "$mode rc=$?"
  done ;;
esac
