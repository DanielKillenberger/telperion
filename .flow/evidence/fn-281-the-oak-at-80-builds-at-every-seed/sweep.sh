#!/bin/bash
# fn-281 R1: the "unsolved radii" sweep, one shard a preset and an age over
# seeds 1-50, each its own capped heavy job, run from the worktree root
# after `cargo build --profile ci -p telperion-core --example radii_sweep`:
#   .flow/evidence/fn-281-the-oak-at-80-builds-at-every-seed/sweep.sh [shard...]
# A shard is <preset>:<age|own>:<mem GB>. Lines land in raw/sweep/<preset>-<age>.log;
# `grep -h REFUSED raw/sweep/*.log` lists the refusals.
set -o pipefail
dir=.flow/evidence/fn-281-the-oak-at-80-builds-at-every-seed/raw/sweep
mkdir -p "$dir"
shards=("$@")
if [ ${#shards[@]} -eq 0 ]; then
  shards=(oregon-white-oak:80:6 oregon-white-oak:60:5 oregon-white-oak:70:5
    oregon-white-oak:90:6 oregon-white-oak:100:6 oregon-white-oak:own:5)
  for p in ordinary norway-spruce european-beech silver-birch date-palm telperion laurelin; do
    shards+=("$p:own:5" "$p:80:6")
  done
fi
for s in "${shards[@]}"; do
  IFS=: read -r preset age mem <<<"$s"
  log="$dir/$preset-$age.log"
  telperion-heavy "$mem" --threads 4 -- target/ci/examples/radii_sweep "$preset" "$age" 1-50 \
    >>"$log" 2>&1
  rc=$?
  echo "$s rc=$rc $(grep -c ': ok' "$log") ok $(grep -c REFUSED "$log") refused $(grep -h '^peak' "$log" | tail -1)"
done
