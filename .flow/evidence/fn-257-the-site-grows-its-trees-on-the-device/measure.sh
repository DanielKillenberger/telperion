#!/bin/bash
# fn-257 task 7: the release's measurements, warm, on the RTX 3080, one GPU
# job at a time: the harness's build per preset at seeds 1, 7 and 42
# (`build_time`), and the device field per preset (`the_device_field_...`,
# seeds 1, 1234 and 5678 for the site's species and 1 for the rest, every
# grid held to R2's tolerances). Run from the worktree root after
# `cargo test --profile ci -p telperion-render --lib --no-run`; output in
# raw/measure/.
set -u
here=.flow/evidence/fn-257-the-site-grows-its-trees-on-the-device
bin=target/ci/deps/$(ls -t target/ci/deps | grep '^telperion_render-' | grep -v '\.d$' | head -1)
mkdir -p "$here/raw/measure"
for seed in 1 7 42; do
  telperion-heavy 10 --gpu 6 --threads 4 -- env BUILD_SEED=$seed "$bin" build_time --ignored --nocapture \
    > "$here/raw/measure/build-$seed.txt" 2>&1
done
telperion-heavy 10 --gpu 6 --threads 4 -- env FIELD_PRESETS=oregon-white-oak,silver-birch,norway-spruce,date-palm FIELD_SEEDS=1,1234,5678 \
  "$bin" the_device_field --ignored --nocapture > "$here/raw/measure/field-site.txt" 2>&1
telperion-heavy 10 --gpu 6 --threads 4 -- env FIELD_PRESETS=ordinary,european-beech,telperion,laurelin FIELD_SEEDS=1 \
  "$bin" the_device_field --ignored --nocapture > "$here/raw/measure/field-rest.txt" 2>&1
echo done
