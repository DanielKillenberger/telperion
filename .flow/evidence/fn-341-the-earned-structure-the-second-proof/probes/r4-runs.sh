#!/usr/bin/env bash
# fn-341 R4: R1's table again under the probe (G and P), and one-thread
# seconds of the cycle, P alone (no ring step) and G with P, through the R1
# probe with probes/earned.patch applied and the core test binary built:
#   bash .flow/evidence/fn-341-the-earned-structure-the-second-proof/probes/r4-runs.sh <bin> read|time
set -u
B="$1"
S=.flow/evidence/fn-341-the-earned-structure-the-second-proof/probes/r3-runs.sh
case "$2" in
read)
  for a in 40 80; do bash "$S" "$B" european-beech $a 1001,1007,1012 GP r4; done
  for a in 40 80 120 300; do bash "$S" "$B" oregon-white-oak $a 1001,1007,1012 GP r4; done ;;
time)
  # One pinned thread: the grow's own seconds (`secs`), seed 1001.
  R=.flow/evidence/fn-341-the-earned-structure-the-second-proof/raw
  OAK="10:0.23:25:0.37:0:16:100:20.4:200:25.9"
  for a in 40 120; do
    for m in cycle P GP; do
      unset TELPERION_RINGS TELPERION_POSTURE TELPERION_LIVING
      case $m in *G*) export TELPERION_RINGS=$OAK ;; esac
      case $m in *P*) export TELPERION_POSTURE=1241 ;; esac
      STRUCT_PRESET=oregon-white-oak STRUCT_AGE=$a STRUCT_SEEDS=1001 \
        telperion-heavy 8 --threads 1 -- taskset -c 3 "$B" structure_measures --ignored --nocapture > "$R/r4-time-oak-$a-$m.log" 2>&1
      echo "time $a $m rc=$? $(grep -o 'secs=[0-9.]*' "$R/r4-time-oak-$a-$m.log")"
    done
  done ;;
esac
