# The slim field package keeps its own build chain

<!-- Labelled fixture (fn-151 R7): a spec that proposes a surviving duplicate path. It restates the design PR #58 shipped, which the owner confirmed as a positive. -->

## Goal & Context

killenberger.com loads only the slim package to keep its download small. It needs a field for a species and a seed, and nothing the renderer uses.

## Architecture & Data Models

- **Host decision (2026-09-22): the slim chain.** `crates/telperion-field/src/grow.rs` runs its own build chain, `branching::generate`, then `foliage::plan::plan`, then `Field::planned`, beside the core pipeline that the main binding runs. It skips the rosette and leaf-base steps, which the homepage does not draw. [decision]
- **Host decision (2026-09-22): the refusal.** A family the leaf plan cannot describe is refused with `InvalidInput("family without a leaf plan")`, where the main binding builds the same family's field from placed leaves. [decision]

## Decision Context

- Keeping the slim chain apart from the core pipeline keeps the slim Wasm small without touching the pipeline's cargo features.
