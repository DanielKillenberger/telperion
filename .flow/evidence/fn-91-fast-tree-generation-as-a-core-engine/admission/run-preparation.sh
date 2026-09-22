#!/usr/bin/env bash
set -euo pipefail
repo=$(pwd -P)
evidence="$repo/.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/admission"
scratch=$(mktemp -d /tmp/telperion-admission-reproduce-XXXXXX)
git archive 852fdbd1 | tar -x -C "$scratch"
cp "$evidence/preparation.rs" "$scratch/crates/telperion-core/examples/preparation.rs"
for mode in control candidate; do
    if [[ "$mode" == candidate ]]; then
        cp "$repo/crates/telperion-core/src/surface.rs" "$scratch/crates/telperion-core/src/surface.rs"
        cp "$repo/crates/telperion-core/src/surface/prepared.rs" "$scratch/crates/telperion-core/src/surface/prepared.rs"
    fi
    CARGO_TARGET_DIR="$repo/target" timeout 600s cargo build --manifest-path "$scratch/Cargo.toml" --release -p telperion-core --example preparation > "$evidence/$mode-build.log" 2>&1
    cp "$repo/target/release/examples/preparation" "/tmp/telperion-fn91-tools/preparation-admission-$mode"
    sha256sum "/tmp/telperion-fn91-tools/preparation-admission-$mode" > "$evidence/$mode-binary.sha256"
    for species in oregon-white-oak norway-spruce; do
        for seed in 1 7; do
            "/tmp/telperion-fn91-tools/preparation-admission-$mode" "$species" "$seed"
        done
    done > "$evidence/$mode.jsonl"
done
