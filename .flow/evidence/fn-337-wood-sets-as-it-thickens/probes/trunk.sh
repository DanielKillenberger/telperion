#!/usr/bin/env bash
# fn-337 R4: fn-332's trunk probe on the oak, one process per tree, through
# fn-330's peak.py. Usage: trunk.sh <out-dir under raw/> <age>...
# Run inside one claim: telperion-heavy 8 --threads 4 -- probes/trunk.sh before 40 80
set -euo pipefail
cd /home/daniel/Projects/telperion/.worktrees/fn-319
out=.flow/evidence/fn-337-wood-sets-as-it-thickens/raw/$1
shift
mkdir -p "$out"
for age in "$@"; do
  for seed in 1 7 42; do
    python3 .flow/evidence/fn-330-how-far-the-full-cycle-grows-an-oak/probes/peak.py \
      target/ci/examples/trunk oregon-white-oak "$age" "$seed" >"$out/oak-$age-$seed.log" 2>&1
  done
done
