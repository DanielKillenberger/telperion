# Bark and foliage appearance

## Conversation Evidence

> user: "those specs are excellent can you $flow-next-capture them?"

This approves the preceding seven-item roadmap, including this outcome. [paraphrase]

## Goal & Context

Add species-specific bark and foliage appearance through procedural materials, texture coordinates and leaf translucency, using the geometry and anatomy established by the species templates. [paraphrase]

## Architecture & Data Models

Keep one lean, engine-independent core with small boundaries between botanical state and requested representations. [strategy:The core and integration]

## Acceptance Criteria

- **R1:** Selected species have reference-based bark and foliage appearance, including suitable texture coordinates and translucency; no error surface beyond R2. [paraphrase]
- **R2:** Validate appearance at trunk, branch and leaf scales under controlled lighting. Seams, stretched coordinates, missing resources and unsupported effects are exposed and handled with a usable fallback or explicit limitation. [inferred]
- **R3:** Keep neutral geometry inspection available and report material-related GPU and memory costs; visual improvement must not conceal structural regressions. [inferred]

## Boundaries

Supernatural visual effects, seasonal transitions and lifecycle simulation are separate work. [inferred]

## Decision Context

Depends on Real-species profiles and procedural templates. Geometry and foliage anatomy precede appearance. [paraphrase]

## Requirement coverage

| Requirement | Task |
|---|---|
| R1–R3 | TBD during planning |
