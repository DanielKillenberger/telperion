# Flat bark calibration

The evidence-only ignored Rust test uses the production Renderer and wood shader. Its flat 0.4 m × 0.4 m patch has 128 strips across, maps its horizontal coordinate to arc angle x / 0.4 m, and supplies the stated 0.4 m material radius to the renderer's private buffer. It adds no production API and invokes no tree generator. The seed 7 and complete shipped material rows are recorded for both presets; geometry seed does not affect this synthetic fixture.

The square-on perspective camera frames exactly 0.4 m at the patch plane. The full 400 × 400 still is the structure score's crop, so its physical width is also 0.4 m. The sky/sun scene uses sun azimuth 45° and elevation 55°; every other row is the shipped default. The complete scene is recorded in calibration.json. The first capture had the default azimuth 135°, which backlit the +Z patch; the host approved the corrected azimuth before this final baseline. PNG outputs are local evidence only.

Run from this worktree root:

```bash
CARGO_TARGET_DIR=/home/daniel/Projects/telperion/target cargo test -p telperion-render --lib flat_patch_coordinates_have_the_stated_physical_scale
BARK_CAPTURE_DIR="$PWD/.flow/evidence/fn-42-bark-depth-level-and-a-fair-capture/baseline" CARGO_TARGET_DIR=/home/daniel/Projects/telperion/target cargo test -p telperion-render --lib capture_calibrated_bark -- --ignored --nocapture
```

Capture fails if a hardware GPU is unavailable. The coordinate test checks the flat geometry and physical mapping. The capture checks dimensions and deterministic redraw, and measures the depth-off pixel difference on the same uploaded patch. Timing is not measured because the GPU is contended.

Original fn-32 reference pixels were not present in the root .refs directory (only fn9 quga788B.jpg and piab977.jpg). The host also checked sibling worktree caches. The owner photographs have no recoverable source URL. No reference physical scale or score has been invented: calibration.json records these as unavailable. R1 reference comparison and R3 colour fitting remain blocked; these captures cannot count as accepting evidence for either.

Verified: coordinate test passed (1 test); explicit hardware capture passed (1 test, 1.83 seconds after lighting correction). Both images are 400 × 400. Source commit, fixture source hashes and PNG hashes are in baseline/provenance.json. Complete material rows are recorded as Rust Debug field/value strings because MaterialParams does not implement Serialize.
