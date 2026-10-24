#!/usr/bin/env bash
# Non-test lines per crate (fn-293 R5): every .rs and .wgsl file under the
# crate's src/, less *_tests.rs, tests.rs, and anything under a tests/ or
# suite/ directory. Run from the checkout's root.
set -euo pipefail
printf '| crate | rs | wgsl |\n| --- | ---: | ---: |\n'
for c in crates/*/; do
  name=$(basename "$c")
  [ -d "$c/src" ] || continue
  count() {
    find "$c/src" -type f -name "$1" \
      ! -name '*_tests.rs' ! -name 'tests.rs' \
      ! -path '*/tests/*' ! -path '*/suite/*' -print0 |
      xargs -0 -r cat | wc -l
  }
  printf '| %s | %s | %s |\n' "$name" "$(count '*.rs')" "$(count '*.wgsl')"
done
