#!/usr/bin/env bash
set -euo pipefail
# Run from the main repository. Creates a new scratch tree; removes nothing.
repo=$(pwd -P)
evidence="$repo/.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/wood-profile"
revision=b0c38c259bb733f1b85b137f4cf36916e4352724
scratch=${WOOD_SCRATCH:-$(mktemp -d /tmp/telperion-wood-profile-XXXXXX)}
if [[ ! -f "$scratch/Cargo.toml" ]]; then git archive "$revision" | tar -x -C "$scratch"; fi
cp "$evidence/wood_profile.rs" "$scratch/crates/telperion-core/examples/wood_profile.rs"
export CARGO_TARGET_DIR="${WOOD_TARGET:-$repo/target}"
cd "$scratch"
for mode in control instrumented; do
    if [[ "$mode" == instrumented ]]; then python3 "$evidence/instrument.py" "$scratch"; fi
    timeout 600s cargo build --release -p telperion-core --example wood_profile > "$evidence/$mode-build.log" 2>&1
    sha256sum "$CARGO_TARGET_DIR/release/examples/wood_profile" > "$evidence/$mode-binary.sha256"
    : > "$evidence/$mode.jsonl"
    for species in oregon-white-oak norway-spruce; do
        for seed in 1 7; do
            "$CARGO_TARGET_DIR/release/examples/wood_profile" "$species" "$seed" >> "$evidence/$mode.jsonl"
        done
    done
done
printf '%s\n' "$scratch" > "$evidence/scratch-path.txt"
