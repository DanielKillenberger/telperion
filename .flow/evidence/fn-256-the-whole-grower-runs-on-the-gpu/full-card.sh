#!/usr/bin/env bash
# fn-256 task 5: ten builds on a card two-thirds full (COUNCIL.md round 4).
# A ballast process holds what takes the card to two thirds, then the ten
# builds run beside it; the ballast is stopped by its PID.
#   full-card.sh <render test binary> <output file>
set -u
bin=$1
out=$2
used=$(nvidia-smi --query-gpu=memory.used --format=csv,noheader,nounits | head -1)
total=$(nvidia-smi --query-gpu=memory.total --format=csv,noheader,nounits | head -1)
want=$((total * 2 / 3 - used))
echo "card ${used} of ${total} MiB used; ballast ${want} MiB" >>"$out"
BALLAST_MIB=$want BALLAST_SECS=900 timeout 1000 "$bin" --ignored --exact --nocapture \
    generation::resident::full_card_tests::ballast >>"$out.ballast" 2>&1 &
ballast=$!
for _ in $(seq 1 120); do
    grep -q 'ballast holds' "$out.ballast" 2>/dev/null && break
    sleep 1
done
nvidia-smi --query-gpu=memory.used,memory.total --format=csv,noheader >>"$out"
timeout 900 "$bin" --ignored --exact --nocapture \
    generation::resident::full_card_tests::ten_builds_on_a_full_card >>"$out" 2>&1
echo "rc=$?" >>"$out"
kill "$ballast"
wait "$ballast"
nvidia-smi --query-gpu=memory.used,memory.total --format=csv,noheader >>"$out"
