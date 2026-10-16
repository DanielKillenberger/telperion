#!/bin/bash
# fn-286: every relay's drawn turn, a preset at its own age, seeds 1-10, from
# a value file with rows replaced or added; run from the worktree root:
#   .flow/evidence/fn-286-a-relay-runs-back-onto-the-line-it/turns.sh <tag> <value file> [<path>=<value>...]
# Lines in raw/turns-<tag>.jsonl; prints the seeds' sums and medians.
set -u
here=.flow/evidence/fn-286-a-relay-runs-back-onto-the-line-it
tag=$1 file=$2
shift 2
example=crates/telperion-core/examples/relay_turns.rs
mkdir -p "$here/raw"
cp "$here/relay_turns.rs" "$example"
trap 'rm -f "$example"' EXIT
telperion-heavy 4 --threads 8 -- env CARGO_BUILD_JOBS=8 cargo build -q --profile ci -p telperion-core --example relay_turns \
  2>"$here/raw/turns-build.log" || { echo "build failed"; tail -20 "$here/raw/turns-build.log"; exit 1; }
rm -f "$example"
telperion-heavy 8 --threads 8 -- env RAYON_NUM_THREADS=8 target/ci/examples/relay_turns "$file" "$(seq -s, 1 10)" "$@" \
  >"$here/raw/turns-$tag.jsonl"
python3 - "$here/raw/turns-$tag.jsonl" "$tag" <<'EOF'
import json, sys, statistics as st
rows = [json.loads(l) for l in open(sys.argv[1])]
ok = [r for r in rows if 'error' not in r]
med = lambda k: st.median(r[k] for r in ok)
print(f"{sys.argv[2]}: relays {sum(r['relays'] for r in ok)} joint40 {sum(r['joint40'] for r in ok)} "
      f"chord40 {sum(r['chord40'] for r in ok)} joint50 {med('joint50'):.1f} joint90 {med('joint90'):.1f} "
      f"chord50 {med('chord50'):.1f} chord90 {med('chord90'):.1f} errors {len(rows) - len(ok)}")
EOF
