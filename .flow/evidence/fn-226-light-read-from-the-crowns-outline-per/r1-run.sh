#!/usr/bin/env bash
# fn-226 R1: the lit oak's light work by pass (best of 3) and the measures
# with one light pass or reader off. Waits for a one-minute load under 6
# before each tree. Output: raw/r1/r1.jsonl.
set -u
cd "$(dirname "$0")/../../.."
export RAYON_NUM_THREADS=8
out=.flow/evidence/fn-226-light-read-from-the-crowns-outline-per/raw/r1
mkdir -p "$out"
wait_load() { while awk '{exit !($1 >= 6)}' /proc/loadavg; do sleep 20; done; }
for v in lit unlit flat norelay noallot nobalance nohazard nogirth; do
  reps=1; case $v in lit|unlit) reps=3;; esac
  for age in 40 80; do
    for seed in 1 7; do
      wait_load
      target/ci/examples/r1 $age $seed $v $reps >> "$out/r1.jsonl"
    done
  done
done
echo done >> "$out/r1.done"
