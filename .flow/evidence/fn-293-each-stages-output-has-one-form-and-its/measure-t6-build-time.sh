#!/bin/bash
# fn-293 task 6: the device build, paired warm A/B. The render test binary
# before the change (base, 2f437cbd4 plus the digest's bounds switch) and
# after it, run alternately, 16 pairs, the order flipped every pair. Each
# run times the spruce's and the oak's build cold, then warm (`build_time`:
# the second build in the process) and prints each build's device peak.
# The GPU's and CPU's load are sampled before every pair.
S=/tmp/claude-1000/-home-daniel-Projects-telperion/dd1a391e-8087-41d5-8d26-b3ea78d9cb37/scratchpad
OUT=/home/daniel/Projects/telperion/.worktrees/f4-293/.flow/evidence/fn-293-each-stages-output-has-one-form-and-its/raw/t6-build-time.txt
cd /home/daniel/Projects/telperion/.worktrees/f4-293/crates/telperion-render || exit 1
run() {
  env BUILD_PRESETS=norway-spruce,oregon-white-oak "$S/t6bin/render-$1" \
    generation::build_time_tests::build_time --exact --ignored --nocapture 2>&1 \
    | grep -E "^\| (norway|oregon)|^census" | sed "s/^/$1 /" \
    >> /home/daniel/Projects/telperion/.worktrees/f4-293/.flow/evidence/fn-293-each-stages-output-has-one-form-and-its/raw/t6-build-time.txt
}
for i in $(seq 1 16); do
  echo "load $i gpu $(nvidia-smi --query-gpu=utilization.gpu,memory.used --format=csv,noheader) cpu $(cut -d' ' -f1 /proc/loadavg)" >> "$OUT"
  if [ $((i % 2)) -eq 1 ]; then run base; run after; else run after; run base; fi
done
echo done
