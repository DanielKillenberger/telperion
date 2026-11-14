# The scaffold test generator retires

## Goal & Context

Astra's release review (2026-10-16, ASTRA-RELEASE-REVIEW.md, "After 0.2.0" 1): `pipeline.rs:189` exposes the old scaffold generator only to tests, and lines 241-263 say production no longer reaches it; the old branching, colonisation, twig, radius and bias modules are 6,731 lines with their shared contracts and tests (an audit surface, not a deletion estimate). fn-198 R0 already called for retiring the old generator.

## Requirements

- **R1:** every test on the scaffold classified: it guards obsolete generation (deleted) or today's surface, foliage or serialisation (fed an engine specimen instead).
- **R2:** the obsolete modules deleted, keeping `clothe_leaf_bases`, validation, parameter types and shared helpers; entry and geometry coverage intact; the gate's time before and after.

## Boundaries

After the release (fn-237).
