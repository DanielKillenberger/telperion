## Conversation Evidence

> user: "can you make a short motion design video on how the tree generator pipeline works now? really bring home the vision of telperion. Show the fidelity of the trees and the streamlined pipeline and the generator algo. Make sure it's clear that we're in continuous tree space etc."
> user (destination): a site hero plus a social cut, i.e. a 60–90 s 16:9 master for killenberger.com and a 15–20 s 9:16 cut for social from the same material.

## Goal & Context
<!-- scope: business -->

A short motion design piece that makes Telperion's vision felt. It is a runtime tree generator where every tree is a point in one continuous tree space, grown by one streamlined pipeline, at a fidelity that holds up close. It is for killenberger.com's hero and for social. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **What it shows** [paraphrase]:
  - **Fidelity:** hero shots of real presets, including Telperion and the date palm, from the whole tree down to leaf and bark.
  - **The generator algorithm:** space colonization growing the crown, botanical rules below the crossover, and the one bias field that carries supernatural character (STRATEGY.md).
  - **The streamlined pipeline:** one path through its stages (grow, plan, expand, cull, draw), each stage shown with the artifact it actually produces, and the parameter catalogue feeding it.
  - **Continuous tree space:** a tree moving smoothly from one preset to another, a parameter sweep, and seeds picking individual specimens.
- **Made from the real generator.** Every tree on screen is rendered by the generator from recorded presets, seeds and parameters. Motion graphics carry labels, diagrams and transitions around real renders, never stand-in trees. [inferred]
- **Honest about continuity.** Only blends that are measured smooth appear. The date palm is not blended with a branching tree until fn-148 fixes that path's jump. [inferred]
- **Budget.** AGENTS.md's capture rules hold: small before large, and at most one full-forest capture. [inferred]
- **Unknown.** The tools used to compose the motion design and the shot list are decided when planning. [unknown]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A 60–90 s 16:9 master and a 15–20 s 9:16 cut are delivered, both from the same material. [user]
- **R2:** Every tree on screen is a generator render, and each shot's preset, seed and parameters are recorded so it can be re-rendered. Errors: a shot that cannot be traced to a render is removed. [inferred]
- **R3:** The pipeline sequence shows its stages in order as one path, each with the real artifact at that stage. [paraphrase]
- **R4:** Continuous tree space is shown by at least one smooth morph between two presets and one live parameter sweep. Every blend shown is one measured smooth. [paraphrase]
- **R5:** Fidelity is shown by close-ups of at least three species, Telperion and the date palm among them. [paraphrase]
- **R6:** The generator algorithm is shown: space colonization growing a crown, and the bias field shaping a supernatural tree. [paraphrase]
- **R7:** The owner watches both cuts and approves them; the owner's eye is the acceptance. [inferred]
- **R8:** The masters are kept under the repository's ignored demo-video directory, as the evidence-retention policy says. Each is delivered at a stated path, with its shot list and render records committed. [inferred]

## Boundaries
<!-- scope: business -->

- Not the website integration. No new generator features; the video shows what the generator does today.

## Strategy Alignment

- Serves "Our approach": one continuous tree space and one pipeline, judged by visual evidence. [strategy:Our approach]
