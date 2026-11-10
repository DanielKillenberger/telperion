#!/usr/bin/env bash
# fn-344 second reading (not the spec's R2 rule; for the host): the variants
# stopped against four times the most phytomers A had held by that year
# (A's running peak), not A's count that year, which halves when A sheds its
# fork limbs (year 132 on 1001). The 8 GiB resident stop still holds. Needs
# the A runs of probes/stills.sh first. Stills and logs are tagged <v>p.
#   telperion-heavy 12 --gpu 2 --threads 4 -- bash probes/stills-peak.sh "<trees>" "<variants>"
set -u
E=.flow/evidence/fn-344-the-old-oak-which-cause-carries-the
PEAK=.flow/evidence/fn-330-how-far-the-full-cycle-grows-an-oak/probes/peak.py
out="$E/raw/stills"
unset TELPERION_RINGS TELPERION_POSTURE TELPERION_LIVING
for tree in $1; do
  age=${tree%/*}; seed=${tree#*/}
  curve="$E/raw/curve-$age-$seed-peak.txt"
  awk '{if ($2 > m) m = $2; print $1, m}' "$E/raw/curve-$age-$seed.txt" > "$curve"
  for v in $2; do
    case $v in F) set="BCE" ;; *) set="$v" ;; esac
    log="$out/oak-$age-$seed-${v}p.log"
    ABLATE="$set" ABLATE_CENSUS=1 PROBE_YEAR=1 ABLATE_CURVE="$curve" \
      python3 "$PEAK" target/ci/examples/space_oak "$out" "$age" --seeds "$seed" --tag "${v}p" \
      --shots bare,whole,bole > "$log" 2>&1
    echo "oak $age $seed ${v}p rc=$? $(date +%T)"
  done
done
