#!/usr/bin/env bash
# fn-343: save the probe in the crates as probes/<name>.patch (default stops), then revert the crates: bash probes/save.sh [name]
set -eu
P=.flow/evidence/fn-343-a-line-that-changes-hands-carries-on/probes/${1:-stops}.patch
git diff -- crates > "$P"
for f in $(git ls-files --others --exclude-standard crates); do
  git diff --no-index -- /dev/null "$f" >> "$P" || true
done
git apply -R --check "$P"
git apply -R "$P"
git status --short crates
echo saved
