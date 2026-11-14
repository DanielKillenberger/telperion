#!/usr/bin/env bash
# fn-371 R3: the seconds again, one thread on one core, the base build
# (raw/bin-before) and this branch's (raw/target/ci) taken in turn, three
# times each, at 40 and 80 on seed 1001 and at 80 on seed 1. Run inside one
# telperion-heavy job.
set -u
F="$(cd "$(dirname "$0")" && pwd)"
S="$(git -C "$F" rev-parse --show-toplevel)/.flow/evidence/fn-352-the-scoreboard-a-fixed-panel-of-trees/scoreboard"
for rep in 1 2 3; do
  for t in 40:1001 80:1001 80:1; do
    for build in before:"$F/raw/bin-before" after:"$F/raw/target/ci"; do
      echo "$rep ${build%%:*} $t $(RAYON_NUM_THREADS=1 taskset -c 3 python3 "$S/peak.py" "${build#*:}/scoreboard-probe" cost ordinary "${t%%:*}" "${t##*:}" 2>&1 | tr '\n' ' ')"
    done
  done
done
echo "COST DONE"
