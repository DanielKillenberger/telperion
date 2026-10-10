#!/usr/bin/env bash
# argv: version(old|new) out.jsonl args... ; one probe under the GPU lock.
P=/tmp/claude-1000/-home-daniel-Projects-telperion/dd1a391e-8087-41d5-8d26-b3ea78d9cb37/scratchpad
V=$1; OUT=$2; shift 2
BIN=$P/fn258/target-$V/release/parity
export RAYON_NUM_THREADS=4 __GL_SHADER_DISK_CACHE_SIZE=4000000000
flock -w 1800 $P/gpu.lock bash -c '
  for i in $(seq 90); do u=$(nvidia-smi --query-gpu=utilization.gpu --format=csv,noheader,nounits); [ "$u" -lt 15 ] && break; sleep 2; done
  u0=$(nvidia-smi --query-gpu=utilization.gpu --format=csv,noheader,nounits)
  out=$(timeout 900 "$0" "$@" 2>>/tmp/claude-1000/-home-daniel-Projects-telperion/dd1a391e-8087-41d5-8d26-b3ea78d9cb37/scratchpad/fn258/err.log); rc=$?
  echo "{\"rc\":$rc,\"gpu_util_before\":$u0,\"run\":${out:-null}}"
' "$BIN" "$@" >> "$OUT"
