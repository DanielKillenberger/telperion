#!/usr/bin/env bash
# fn-344: stills and measures of today's oak with one old-oak cause off
# (probes/oldoak.patch applied, space_oak built with it), inside one GPU job:
#   telperion-heavy 12 --gpu 2 --threads 4 -- bash probes/stills.sh "<trees>" "<variants>"
# trees: age/seed pairs such as "300/1001 120/1001"; variants: letters A..F.
# A runs first per tree and writes the census curve the others stop at
# (four times A's held phytomers at that year, or 8 GiB resident).
# Variants: A today; B dead wood not charged; C stem line pulled vertical
# as the leader is; D the leader lives past the run; E sleeping buds on the
# trunk line wake as limbs; F = B, C and E.
set -u
E=.flow/evidence/fn-344-the-old-oak-which-cause-carries-the
PEAK=.flow/evidence/fn-330-how-far-the-full-cycle-grows-an-oak/probes/peak.py
out="$E/raw/stills"
mkdir -p "$out"
unset TELPERION_RINGS TELPERION_POSTURE TELPERION_LIVING
for tree in $1; do
  age=${tree%/*}; seed=${tree#*/}
  curve="$E/raw/curve-$age-$seed.txt"
  for v in $2; do
    case $v in A) set="" ;; F) set="BCE" ;; *) set="$v" ;; esac
    log="$out/oak-$age-$seed-$v.log"
    if [ "$v" = A ]; then c=""; else c="$curve"; fi
    ABLATE="$set" ABLATE_CENSUS=1 PROBE_YEAR=1 ABLATE_CURVE="$c" \
      python3 "$PEAK" target/ci/examples/space_oak "$out" "$age" --seeds "$seed" --tag "$v" \
      --shots bare,whole,bole > "$log" 2>&1
    echo "oak $age $seed $v rc=$? $(date +%T)"
    if [ "$v" = A ]; then awk '/^CENSUS/{print $2, $3}' "$log" > "$curve"; fi
  done
done
