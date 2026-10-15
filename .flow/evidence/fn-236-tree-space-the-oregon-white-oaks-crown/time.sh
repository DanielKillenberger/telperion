#!/bin/bash
# fn-236: the oak's warm resident build time (oak_time_probe_tests.rs), run
# from the worktree root:
#   .flow/evidence/fn-236-tree-space-the-oregon-white-oaks-crown/time.sh <value file> <age> <mem GB>
# Copies the probe into the renderer's generation module with a mod line
# for the run, and removes both after (the module file is restored to its
# exact bytes).
set -e
here=.flow/evidence/fn-236-tree-space-the-oregon-white-oaks-crown
dir=crates/telperion-render/src/generation
module=crates/telperion-render/src/generation.rs
saved=$(mktemp -p "$here/raw/time")
cp "$module" "$saved"
cp "$here/oak_time_probe_tests.rs" "$dir/oak_time_probe_tests.rs"
trap 'cp "$saved" "$module"; rm -f "$saved" "$dir/oak_time_probe_tests.rs"' EXIT
printf '#[cfg(test)]\nmod oak_time_probe_tests;\n' >>"$module"
OAK_VALUES="$1" OAK_AGE="$2" telperion-heavy "$3" --gpu 4 --threads 8 -- \
  cargo test -q --profile ci -p telperion-render --lib oak_time_probe -- --ignored --nocapture 2>&1 \
  | grep -E "^OAK|panicked|^error" || true
