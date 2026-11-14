# Node identity from the engine's lineage

## Goal & Context

fn-216's design found (FLOWERS-DESIGN.md, by reading) that fn-198's conversion (`pipeline/space.rs`) never sets a node's identity, so every draw keyed by identity (short shoots today; flowers next) would be the same on every piece of wood of an engine tree. The conversion sets each node's identity from the engine's lineage key, stable across walks, ages and partitions (fn-209's canonical order).

## Requirements

- **R1:** A failing test first: two distinct engine nodes with equal identity today.
- **R2:** Identity from lineage; distinct per node; equal for the same lineage across ages (age walk) and across cuts.
- **R3:** Every passed species' converted tree changes only in identity; drawn output unchanged where nothing reads identity (hashes recorded).
- **R4:** npm test and the workspace gate green.

## Boundaries

Depends on fn-198. GPU leaf draws keyed by station number are fn-125's.
