#!/usr/bin/env bash
# fn-351, OAK-SAPWOOD-BY-SIZE.md: the narrow tests that read the oak, to find
# the pins its new rows move. Never the gate.
#
#   telperion-heavy 4 --threads 4 -- bash job-c.sh
set -u
cd /home/daniel/Projects/telperion/.worktrees/oakwood || exit 1
keep='^test .*(FAILED|failed)|test result|panicked|left:|right:|^error|Running'
step() { echo "== $* $(date +%T)"; }
step space lib
cargo test --profile ci -p telperion-space --lib --no-fail-fast 2>&1 | grep -E "$keep" | tail -40
step space refusals chain blend_reach
cargo test --profile ci -p telperion-space --no-fail-fast --test refusals --test chain --test blend_reach 2>&1 | grep -E "$keep" | tail -30
step core identity and oak integration tests
cargo test --profile ci -p telperion-core --no-fail-fast --test catalogue_identity --test native_entry \
  --test space_stems --test walk_midpoint --test grown_total --test capability --test space_walks 2>&1 | grep -E "$keep" | tail -60
step core lib, the suites that name the oak
cargo test --profile ci -p telperion-core --lib --no-fail-fast -- suite::species suite::establish \
  suite::sag suite::codominance pipeline::space pipeline::cycle_cost family::tests 2>&1 | grep -E "$keep" | tail -60
step done
