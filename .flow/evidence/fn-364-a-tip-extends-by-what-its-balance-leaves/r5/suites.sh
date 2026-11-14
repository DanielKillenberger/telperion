#!/bin/bash
# fn-364 R3 and R4: the space crate once, the slow year test by name, the
# core suites that read a phytomer's length or pin a tree, the two guards.
R=/home/daniel/Projects/telperion/.worktrees/fn-364/.flow/evidence/fn-364-a-tip-extends-by-what-its-balance-leaves/raw
T=/home/daniel/.cache/claude-tmp/claude-1000/-home-daniel-Projects-telperion/d6d5cb56-440a-44f4-803a-6a595b8a3532/scratchpad/fn364/times.txt
mkdir -p $R
cd /home/daniel/Projects/telperion/.worktrees/fn-364-suites || exit 1
export TELPERION_HEAVY_RESERVE=8
h() { telperion-heavy 8 --threads 4 -- "$@"; }
run() { local name=$1; shift; echo "$name start $(date '+%F %T')" >> $T; h "$@" >> $R/$name.log 2>&1; echo "$name end rc=$? $(date '+%F %T')" >> $T; }
run guard-generation cargo test --profile ci -p telperion-core --test generation_limit_guard
run guard-compile cargo test --profile ci -p telperion-render --lib compile_size
run r3-every-year cargo test --profile ci -p telperion-space --lib -- --ignored --exact grow::cycle::paused::tests::every_year_is_the_horizons_prefix
run r4-space cargo test --profile ci -p telperion-space --no-fail-fast
run r4-core-lib cargo test --profile ci -p telperion-core --lib --no-fail-fast -- pipeline::cycle_cost_tests pipeline::space
run r4-core-catalogue cargo test --profile ci -p telperion-core --test catalogue_identity
echo "suites done $(date '+%F %T')" >> $T
