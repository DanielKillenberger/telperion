# A person for scale and a nicer world around the tree

## Goal & Context

Owner (2026-10-23), on the release film's cut 6: "video looks really good but [a friend]'s right the cylinder thing looks like a stupid unity demo. could we make the environment a bit nicer and remove that thing and add something else for scale?" The owner chose a person as the new scale reference, asked for "a concept screenshot first before rendering the whole video", and ruled that this belongs in a spec as part of the release.

The scene is the shipped renderer's: `crates/telperion-render/src/scene.rs`, `scene/room.rs` (the 1.8 m capsule figure, `FIGURE_HEIGHT`), `scene/row.rs` (`SceneRow`) and `shaders/sky.wgsl`. The film draws through the shipped renderer with no film-only drawing (fn-272 R2), so the change reaches the harness and the package's `createRenderer` too. It joins the release (fn-237), and the film (fn-272) re-renders after it.

A concept prototype is being made on branch `scene-concept` (host, 2026-10-23). It holds stills of the current look and two or three environment variants, on the oak at 40 and Telperion at 80, seed 7, at 1280×720.

## Requirements

- **R1 (concept, owner's choice first):** the owner picks one environment variant, and passes on the person, from the concept sheet, before any task is planned for real. The pick and the owner's words are recorded here verbatim.
- **R2 (the person):** a standing person about 1.75 m tall replaces the capsule.
  - Built procedurally in code, with no copied mesh or asset.
  - Calm and low-detail, like an architectural scale figure: one matte colour from the scene palette, no face.
  - A small vertex budget, stated and tested.
  - The placement and `FIGURE_HEIGHT`'s role for the camera are unchanged.
- **R3 (the environment):** the chosen variant.
  - **Sky:** a gradient with a glow around the sun.
  - **Ground:** tonal variation from deterministic shader noise, with no textures.
  - **Horizon:** haze where the ground meets the sky, plus distance haze on far ground only. The tree is not hazed at the film's distances.
  - **Parameters:** every colour and strength is a `SceneRow` parameter, never a literal. The clay room keeps its neutral light.
- **R4 (cost):**
  - **Frame time:** measured in the harness against the base on the owner's RTX 3080, within noise or stated.
  - **Wasm size:** the render module's byte change is stated and stays inside `scripts/artifact-budgets.json`.
  - **Determinism:** the noise is deterministic, so the same frame renders the same bytes.
- **R5 (proof):**
  - The existing scene, shadow and renderer tests are green, updated only where the figure or the sky changed by design.
  - New tests cover the figure's height, its vertex budget and its placement out of the canopy.
  - The gate (`scripts/gate.py` as declared) and `npm test` are green, with a codex review per task, each dispatched inside `telperion-heavy` per AGENTS.md.
  - Final stills at the concept's poses go to the owner, then the film's cut 7 is rendered.

## Boundaries

- No change to growth, the trees or the camera path.
- No new external asset and no new runtime dependency.
