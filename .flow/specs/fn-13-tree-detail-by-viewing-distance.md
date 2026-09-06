# Tree detail by viewing distance

## Conversation Evidence

> user: "those specs are excellent can you $flow-next-capture them?"

This approves the preceding seven-item roadmap, including this outcome. [paraphrase]

## Goal & Context

Preserve recognizable silhouettes while reducing branch, surface and foliage detail with viewing distance, and measure hero-tree and forest rendering budgets. [paraphrase]

## Architecture & Data Models

Keep one lean, engine-independent core with small boundaries between botanical state and requested representations. [strategy:The core and integration]

## Acceptance Criteria

- **R1:** Provide distance-appropriate structural, surface and foliage representations of the same specimen, preserving its identity and silhouette; no error surface beyond R2. [paraphrase]
- **R2:** Validate transitions and near/far views against full-detail references across selected species. Camera movement, threshold crossings and unavailable detail must not produce invalid geometry or unexplained disappearance. [inferred]
- **R3:** Measure real GPU time, generation/update costs and memory on named hardware for hero trees and a thousand-tree forest. Assess the strategy's 2 ms hero and 4 ms vegetation targets and report misses without substituting CPU timings. [strategy:Surface and rendering at scale]

## Boundaries

This spec owns representation and rendering detail, not new botanical rules or a mandatory GPU generation backend. [inferred]

## Decision Context

Depends on Real-species profiles and procedural templates for representative fidelity checks; does not depend on legendary templates. [paraphrase]

## Requirement coverage

| Requirement | Task |
|---|---|
| R1–R3 | TBD during planning |
