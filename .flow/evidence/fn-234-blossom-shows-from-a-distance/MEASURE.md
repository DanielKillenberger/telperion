# fn-234 R1: where blossom hides at the whole-tree view

Measure only. Choosing R2's design is the host's job. Every number here comes from one run of the instrument at commit `fae9d10e` plus the instrument:

```
STILLS=<dir> cargo test --profile ci -p telperion-render --lib blossom_measure -- --ignored --nocapture --test-threads 1
```

It ran under the GPU lock in 15 s for all three trees. The log is `raw/m1/run.log` and the stills are in `raw/m1/` (ignored). The instrument is `crates/telperion-render/src/generation/fine/blossom_measure.rs`, with its fixtures in `blossom_trees.rs`.

## The trees

All three are fn-217 task 3's fixtures (`tests/bloom_form.rs`, `stills_at_whole_tree_distance`), restated. Each is a beech of age 40, seed 1:

- **cherry:** ages 12 and 14 flower at 1. The corymb has 6 florets with 16 mm petals, front colour 0.1/0.097/0.09, plus the eye and 20 stamens.
- **dense:** the cherry, with every outer age (10 to 14) flowering and 12 florets of 20 mm petals.
- **laurelin:** the horn racemes (24 florets, rachis 0.16, curl 0.6, gold 0.13/0.075/0.008) on the supernatural group at scale 3, with the gold rim.

The camera is `hero_pose` on the CPU mesh's bounds at 960 x 720, with the sun at azimuth 115 and elevation 40, as fn-217 used. Selection is `Level::Chosen` unless a row says otherwise.

## 1. Where the florets stand

Two depths are measured:
- the shader's own `depth_in_crown` (`common.wgsl:165`): 1 at the centre of the leaves' placement box, 0 on its ellipsoid;
- metres under the leaves' shell along the ray from the box centre. The shell is the 98th percentile of the leaves' reach in each of 24 x 12 direction bins.

| | cherry | dense | laurelin |
|---|---|---|---|
| Florets / leaves | 461,958 / 66,155 | 923,916 / 66,155 | 1,847,832 / 187,652 |
| Ellipsoid depth, median: florets / leaves | 0.506 / 0.472 | 0.506 / 0.472 | 0.506 / 0.468 |
| Metres under the shell, q25 / median / q75: florets | 0.29 / 0.65 / 1.14 | 0.29 / 0.65 / 1.14 | 0.88 / 1.93 / 3.40 |
| The same for leaves | 0.24 / 0.55 / 1.09 | 0.24 / 0.55 / 1.09 | 0.72 / 1.66 / 3.25 |
| Share within 0.25 m of the shell: florets / leaves | 20.7 % / 26.2 % | 20.7 % / 26.2 % | 7.1 % / 10.7 % |

The florets stand where the leaves stand, slightly deeper. Their depth distribution is the leaves' own.

**Checked against the code.** `pipeline/space.rs::convert` pushes a `Bloom` for every drawn node of an axis whose age row flowers (`share > 0`, around lines 385 and 475). That is every node along the shoot, not its tip. The device takes the same rule: `blooms.wgsl` `grown_bloom` reads every fine node's share (word 29). So the florets are seated along the flowering shoots inside the leaf mass, as the leaves are.

## 2. The share the hero camera loses to the leaves

These measurements use the organ mask (`set_organ_mask`): coverage with the leaves, then with the leaf instances cleared and the wood kept. Occluded is 1 minus the ratio. The leaf silhouette is the leaves drawn in the organ's slot, without the florets.

| | cherry | dense | laurelin |
|---|---|---|---|
| Florets' coverage, with leaves | 0.02156 | 0.02868 | 0.01260 |
| Without leaves | 0.04651 | 0.05410 | 0.01976 |
| **Occluded by leaves** | **53.6 %** | **47.0 %** | **36.3 %** |
| Leaf silhouette (frame share) | 0.06218 | 0.06219 | 0.02921 |
| Florets shown / leaf silhouette | 0.347 | 0.461 | 0.431 |

Occlusion by depth band, as quartiles of metres under the shell (outermost first):

| Band | cherry | dense | laurelin |
|---|---|---|---|
| Outermost quartile | 59.3 % | 54.5 % | 39.1 % |
| Second | 62.3 % | 57.9 % | 41.0 % |
| Third | 64.7 % | 60.5 % | 40.0 % |
| Innermost | 69.3 % | 66.0 % | 43.9 % |
| Camera's half of the crown | 52.1 % | 46.1 % | 29.1 % |
| Far half | 61.8 % | 56.9 % | 41.9 % |
| Camera's half, outermost quartile | 53.8 % | 48.7 % | 27.8 % |
| Sun's and camera's half, outermost quartile | 52.6 % | 48.4 % | 23.6 % |

- On the beech, half the coverage of even the outermost florets on the camera's side is hidden by leaves. Florets hang on stalks (droop 0.6) below the nodes whose leaves stand over them.
- With no leaves at all, the florets cover 4.7 % (cherry) and 5.4 % (dense) of the frame. The whole leaf silhouette covers 6.2 %.
- The tree fills a small part of the hero frame: the crown is a sparse 66k-leaf beech, and the frame is mostly ground and sky (viewed).

## 3. The petals' drawn brightness

The petal's drawn colour is unmixed per pixel from three stills at the same camera: the whole still (C), the same tree with the organ removed (B), and the mask (c). The formula is P = (C - (1 - c) B) / c, weighted by c, over pixels where c >= 0.5. Values are sRGB 0-255.

| | cherry | dense | laurelin |
|---|---|---|---|
| Material front colour, as a display value | (89, 88, 85) | (89, 88, 85) | (101, 77, 22) |
| Petal at the hero camera, c >= 0.5 | (82, 92, 75) | (84, 94, 78) | (105, 100, 75) |
| The same, c >= 0.9 | (83, 93, 76) | (85, 95, 78) | (104, 99, 78) |
| Outermost band / innermost band | (88, 98, 82) / (70, 79, 66) | (90, 100, 84) / (70, 80, 66) | (105, 101, 80) / (94, 89, 65) |
| Sun's and camera's half, outermost quartile | (95, 104, 85) | (97, 106, 87) | (117, 107, 77) |
| Close-up petal at 0.1 m (c >= 0.99) | (116, 124, 107) | (118, 125, 110) | (140, 114, 62) |
| Leaves at the hero camera (silhouette c >= 0.9) | (58, 66, 45) | (58, 66, 45) | (77, 85, 48) |

What the petals' colour depends on, at the hero camera (each change applied to the whole family, with B re-drawn under it):

| Change | cherry petal | laurelin petal |
|---|---|---|
| As drawn | (82, 92, 75) | (105, 100, 75) |
| `interiorDarkening` 0 | (84, 95, 79) | (109, 105, 82) |
| `crownShade` 0 | (84, 94, 77) | (107, 103, 79) |
| `lobeShade` 0 | (84, 95, 78) | (107, 103, 78) |
| Transmission (strength and diffuse) 0 | (77, 78, 72) | (99, 84, 72) |
| All four 0 | (82, 85, 81) | (108, 96, 87) |
| Every element at its finest level | (84, 94, 78) | (106, 101, 76) |
| Bloom colours x2 | (107, 113, 99) | (135, 121, 77) |
| Bloom colours x4 | (136, 140, 128) | (166, 148, 82) |
| Close-up at 0.1 m, x4 | (187, 188, 177) | (212, 186, 82) |

- **The petals render at about their own material value as a display colour, grey.** At distance a white petal is (82, 92, 75): 1.4 times the leaves' luminance and green-tinted.
- **Neither LOD nor the crown terms make them grey.** The finest levels move them by 2. Interior darkening, crown shade and own-mass shade each move them by 2 to 4.
- **The green tint is the leaf's transmission colour.** Without transmission, the cherry's petal goes from G-R +10 to +1.
- **Petals at distance are darker than the close-up.** The best-lit band (sun and camera side, outermost) is (95, 104, 85) against (116, 124, 107) close. The close-up camera picks the outermost floret toward the camera.

**Checked against the code.**
- **Shader and organ block.** The organ is drawn by the leaf shader (`shaders/foliage.wgsl`) with the organ block from `scene/frame.rs::organ`.
  - The block zeroes the canopy normal, veins, mottle detail, margin and the leaf's mirrors.
  - It keeps the interior darkening (`leaf_front.w`), the crown shade overhead, the own-mass shade, the leaf transmission colour and strength (`u.transmission`, beech 0.22/0.5/0.08 at 0.5), the leaf hue and brightness variation, and the sheen.
- **Own-mass grid.** The own-mass grid of the organ is built from the florets' own placements (`select.rs:182`, `mass::grid` on the organ's instances), not the leaves'.
- **Crown ellipsoid.** It is the box of the leaf placements (`submit.rs::crown_of`, set at `lib.rs:214`; on the resident path `resident.crown`).
- **Tone curve.** `tone()` (`common.wgsl:210`) is a 0.6-scaled ACES fit. In sRGB, radiance 0.3 gives 141, 1.0 gives 214, 1.5 gives 229 and 3.0 gives 244.
  - A 0.1 petal facing the full sun (3.08 sun plus the sky) reaches about 0.35 radiance, about 150.
  - A white mass of about 230 needs about 1.5, which is a reflectance near 0.45 facing the sun.
- **Refuted.** The `bloom_front_red` doc (`material/params.rs`) says "past about 0.15 it clips to flat white under the hero sun". At 0.4 (x4) the close-up petal is (187, 188, 177) and the hero-camera petal is (136, 140, 128). Neither clips.

## 4. Organ coverage on both executors

| | cherry | dense | laurelin |
|---|---|---|---|
| `executor::apart` | ok | ok | ok |
| Florets: CPU / device (held + placed on the device) | 461,958 / 0 + 461,958 | 923,916 / 0 + 923,916 | 1,847,832 / 0 + 1,847,832 |
| Coverage: CPU (`mesh::build`, `submit_at`) | 0.02156 | 0.02868 | 0.01260 |
| Coverage: device (`prepare(.., Delivery::Resident)` through `prepare_fine`) | 0.02155 | 0.02867 | 0.01259 |

- **Backend.** All three trees took the fine device path (backend `Gpu`, every floret placed on the device).
- **Coverage.** The two executors agree to 1e-5.
- **Stills.** The device's whole and mask stills match the CPU's by eye (viewed).
- **Bounds (seen in passing, not this spec's).** The prepared tree's bounds differ from `mesh::build`'s below ground and in z: cherry min.y -0.227 against -0.040, and laurelin -0.682 against -0.120. The camera here was taken from the CPU bounds for both.

## Stills (all viewed)

- **`raw/m1/sheet-cherry.png` and `sheet-dense.png`.** Each sheet shows the CPU whole still, the device whole still, the tree bare of florets, the close-up, the CPU mask, the device mask and the leaf silhouette.
  - At the whole-tree view the flowering crown and the bare crown are hard to tell apart: a grey-green speckle on a sparse crown.
  - Close up, the cherry's petals read pale grey-green, not white.
- **`raw/m1/sheet-laurelin.png`.** At the whole-tree view the racemes give the upper crown a tan speckle, not gold. Close up they are orange-gold horns.
- **Not done here.** The cherry photograph fn-217 compared against is in fn-217's ignored `raw/`, which this worktree does not have, so no side-by-side was made.

## What the measurement shows, for the host's choice

These are facts only. No candidate was built.

1. **Placement.** Florets stand along the flowering shoots at the leaves' own depth. Their distribution matches the leaves', with 20.7 % within 0.25 m of the shell on the beech. 47 to 54 % of their coverage is hidden on the beech and 36 % on Laurelin.
   - Even the outermost quartile on the camera's side loses 49 to 54 % on the beech.
   - With no leaves at all, the cherry-dense florets reach only 5.4 % of the frame against a 6.2 % leaf silhouette.
2. **Brightness.** A petal at the white value 0.1 draws at about sRGB 85 to 95 at distance and about 120 close. Under this tone curve, display white needs about 4 times the reflectance. The doc's clipping claim does not hold.
   - The leaf transmission colour tints the petals green.
   - The crown-shade terms and LOD are minor (2 to 4 units).
3. **Frame.** At `hero_pose` the age-40 beech's whole leaf silhouette is 6.2 % of the frame. So any blossom mass is bounded by a sparse crown that fills little of the frame.

## Host design for R2 (2026-10-09)

The measurement narrows the problem. Placement is not the main loss: with the leaves taken off, the florets cover 5.4 % of the frame against the crown outline's 6.2 %, and the leaves hide only about half of them. The loss is that a white petal draws grey-green, at sRGB (82, 92, 75). Two causes are measured: the petals take the leaves' transmission colour, and a "white" material value of 0.1 sits far below a white albedo. The design fixes those two first and measures again before touching placement.

1. **Petals transmit their own colour.** In the organ shader, use the petal's own material colour where the leaves' transmission colour is used today. Nothing else in the leaf lighting changes. No shipped preset flowers, so every shipped species stays byte-identical. Flowering test trees and Laurelin's racemes will change, as intended.
2. **Petal colour is an albedo, the same quantity as the leaf colour.** Fix the material doc comment, which wrongly says petals "clip to flat white past about 0.15". Set the flowering test trees, and later Laurelin and Telperion, to real albedos: a white petal about 0.8, a gold one about (0.85, 0.65, 0.1). This is a values change with no code switch. If the tone curve cannot show a white albedo as near-white under the scene's sun, stop and report. That would be a renderer question.
3. **Measure again,** on both executors, at the whole-tree view, beside the cherry photograph. That photograph is referenced in fn-217's evidence; copy it into this worktree's evidence. If the cherry-dense tree reads as a white blossom mass and Laurelin reads gold, R2 is done. If it does not, report the remaining occlusion and stop. A placement or phenology row (leaf presence at bloom) is then the host's next step. Do not build it now.

The device's tree reaching 0.23 m below ground is outside this spec. The host records it for fn-198's release checks.

## R2 as the host designed it, measured (2026-10-09)

**What was built:**
- **Steps 1 and 2.** The organ block passes the petal's front colour where the leaf passes its transmission colour (`scene/frame.rs::organ`).
  - The petal rows are documented as albedos: `material/params.rs`, `catalogue/reference.rs`, and the regenerated `docs/parameters.md` and `parameters.generated.ts`.
  - The test trees take albedos. The cherry is the material default white (0.82/0.8/0.74, back 0.72/0.72/0.64) in `blossom_trees.rs` and `tests/bloom_form.rs`. Laurelin's gold is 0.85/0.65/0.1, back 0.72/0.55/0.085.
- **Red/green test.** `tests/petal_white.rs` uses a beech of 20 flowering white under a saturated green leaf transmission, at the hero camera.
  - Red on the base: lit from behind the crown, the petals drew (126, 193, 139), green.
  - Green after the fix: (188, 190, 188) from behind and (215, 215, 212) under the hero sun.
- **Re-measure.** The run log is `raw/m2/run.log`, and the stills are in `raw/m2/` (all viewed). The sheet beside the photograph is `raw/m2/sheet-r2.png`.
- **Photograph.** `reference/prunus-avium-hohenwettersbach.jpg` is Wikimedia Commons "Prunus avium - Hohenwettersbach 01.jpg", by H. Zell, CC BY-SA 3.0, resized to 640 px. FLOWERS-DESIGN.md lists it as the whole-crown reference.

**Petals at the hero camera, sRGB (c >= 0.5):**

| | cherry | dense | laurelin |
|---|---|---|---|
| R1 (0.1 values, leaf transmission) | (82, 92, 75) | (84, 94, 78) | (105, 100, 75) |
| Albedos only | (163, 168, 160) | (169, 173, 165) | (185, 180, 105) |
| Albedos and own transmission | (168, 170, 168) | (173, 175, 174) | (190, 182, 106) |
| Outermost quartile, sun and camera side | (186, 187, 182) | (192, 192, 188) | (206, 197, 114) |
| Close-up at 0.1 m | (214, 215, 210) | (220, 220, 215) | (229, 221, 129) |

Coverage and occlusion do not change: the build moved no floret. The cherry is 0.02156 on the CPU and 0.02155 on the device. The dense tree is 0.02868 and 0.02867, and Laurelin 0.01260 and 0.01259. The leaves still hide 53.6, 47.0 and 36.3 % of the florets.

**Pale share of the crown's pixels** (luma >= 150, not green-dominated). The crown is the union of the florets' mask and the leaf silhouette:

| | photograph (non-sky) | cherry | dense |
|---|---|---|---|
| luma >= 130 | 0.464 | 0.260 | 0.326 |
| luma >= 150 | 0.346 | 0.186 | 0.251 |
| luma >= 170 | 0.256 | 0.122 | 0.183 |

On R1's stills, the white share by the stricter test (min channel >= 150, spread <= 45) was 0.015 for the cherry and 0.013 for the dense tree. It is now 0.162 and 0.228, against the photograph's 0.219. Laurelin's gold share (R >= 150, G >= 120, B at least 40 below G) went from 0.000 to 0.196.

**Reading the stills (viewed):**
- **The dense tree:**
  - It now reads as a white-flowering crown. White blossom is spread over the whole crown, on the CPU and the device alike.
  - It is a white-and-dark mottle, not the photograph's white mass. Dark green leaves stand between and in front of the blossom across the crown, and the lower crown is mostly leaf.
  - Pale pixels are about 70 % of the photograph's share. The photograph is a crop from inside and below the crown, so its share is an upper bound.
- **The cherry** reads the same, sparser.
- **Laurelin** reads as a pale straw-yellow crown, not gold. The tone curve brings a 0.85/0.65/0.1 petal to (229, 221, 129) close up, and the red and green channels nearly meet.

**Not a mass, so I stopped** (per the host's design). The remaining loss is occlusion, unchanged from R1:
- The leaves hide 47 % of the dense tree's florets and 54 % of the cherry's, and 49 to 54 % of even the outermost quartile on the camera's side.
- Leaves fill the lower crown.

Laurelin's colour is a second, separate finding. Gold albedos draw straw-yellow under this tone curve, which compresses red and green toward each other near white.

## Host decision after R2 (2026-10-09)

I viewed `raw/m2/sheet-r2.png`. White blossom now shows across the whole crown, on both executors. The remaining gap to the cherry photograph is not a gap in the blossom. *Prunus avium* blooms before or as its leaves come out, so the photograph is a crown of flowers on bare twigs. Our test tree flowers with every leaf present.

- **No phenology row in this spec.** A "leaf presence while flowering" row fits only species that bloom before leafing out. The release's flowering trees are Laurelin and Telperion, which bloom in full leaf. That row is captured as its own post-release spec.
- **What R3 now asks.** It judges against what the release needs: blossom that reads at the whole-tree view as a distinct white (or gold) across the crown, which this round meets. R3's acceptance is amended to match.
- **Gold.** Laurelin's straw yellow is a value question. A deeper gold albedo, roughly (0.85, 0.55, 0.05), is settled in fn-218 (Laurelin) on its own stills. This spec keeps the gold test tree at the documented value.

Finish with:
- R3's fixtures: CPU against device on flowers and coverage with the change on, and neutral rows byte-identical on both paths. Also compare renders of a tree without flowers against the base, to prove the shading change leaves it alone.
- R4 cost.
- The gate, npm test, impl-review and `flowctl done`.
