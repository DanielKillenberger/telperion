# Parallax correction evidence

Implemented the host's design: descend away from the eye's tangent projection, then take two damped fixed-point corrections against the displaced production bark height and groove, retaining the original radius and footprint. This approximates the intersection; it is not full occlusion mapping. No geometry, material row or existing tolerance changed in this worker's patch.

The analytic GPU test extracts the current production `bark_parallax` function and supplies flat/ramp height stubs. Before the correction, its flat-floor assertion failed: +0.01 m instead of -0.01 m. Afterward flat depth is -0.01 m; zero strength and head-on view stay unchanged. For the linear ramp the intersection residual improves from 0.005 to 0.0028125 m. See parallax-red.log and parallax-green.log.

Verified commands:

```bash
CARGO_TARGET_DIR=/home/daniel/Projects/telperion/target cargo test -p telperion-render --lib flat_patch_coordinates_have_the_stated_physical_scale
CARGO_TARGET_DIR=/home/daniel/Projects/telperion/target cargo test -p telperion-render --test bark_parallax --test material_shaders -- --nocapture
```

The first command passed one test. The second passed the analytic GPU test and production WGSL validation (two tests). A first compile of the correction used WGSL's reserved identifier `target`; it was renamed to `corrected_below` without algorithm change before the green run. Broader continuity/filtering gates and visual captures belong to the host handoff, not these narrow checks.
