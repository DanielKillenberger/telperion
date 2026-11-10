#!/usr/bin/env bash
# fn-343: save the probe (fn-342's turns.patch plus the stop tags) as one patch, then revert the crates.
set -eu
P=.flow/evidence/fn-343-a-line-that-changes-hands-carries-on/probes/stops.patch
git diff -- crates > "$P"
for f in $(git ls-files --others --exclude-standard crates); do
  git diff --no-index -- /dev/null "$f" >> "$P" || true
done
git apply -R --check "$P"
git apply -R "$P"
git status --short crates
echo saved
