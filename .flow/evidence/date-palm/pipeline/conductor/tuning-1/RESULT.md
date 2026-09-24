# Run result: date-palm at seed 1

outcome plus gap list; a listed gap is never readiness and this file mints nothing

## Outcome

| | |
| --- | --- |
| Stopped | interrupted during visual assessment |
| Bootstrap | true |
| Machine ready | false |
| Owner acceptance | pending |
| Known gap | fruit-clusters-pendent, captured by fn-111-the-palms-infructescence-a-hanging-date |
| Adoptions kept / rolled back | 2 / 12 |
| rounds | 10 of ? |
| evaluations | 95 of ? |
| images | 356 of ? |
| visual passes | 38 of ? |

## Current tree

Trial `13e1256bc5a0ee80eb4a8ff5efba81be6e532bf701e8989cab8beb695aacfb8e`, round 0, baseline, score telemetry 1.8278.

Stills:

- P-WHOLE seed 1: `/home/daniel/Projects/telperion/.worktrees/fn-80-the-gap-loops-first-live-run/.flow/evidence/fn-80-the-gap-loops-first-live-run/local/palm/13e1256bc5a0ee80eb4a8ff5efba81be6e532bf701e8989cab8beb695aacfb8e-18d84b527154af24-e69eb-f/13e1256bc5a0ee80eb4a8ff5efba81be6e532bf701e8989cab8beb695aacfb8e-P-WHOLE.png` (55137db354e0)
- P-WHOLE seed 1: `/home/daniel/Projects/telperion/.worktrees/fn-80-the-gap-loops-first-live-run/.flow/evidence/fn-80-the-gap-loops-first-live-run/local/palm/13e1256bc5a0ee80eb4a8ff5efba81be6e532bf701e8989cab8beb695aacfb8e-18d84b527154af24-e69eb-f/13e1256bc5a0ee80eb4a8ff5efba81be6e532bf701e8989cab8beb695aacfb8e-P-WHOLE-twin.png` (4f6658c64b96)

Overlay:

```json
{
  "canopy": {
    "leafBaseLength": 0.38,
    "leafBaseRadius": 0.4,
    "rachisLength": 6.2,
    "rosetteFronds": 42,
    "rosettePitch": 25.0,
    "rosettePitchSpread": 85.0,
    "size": 1.3
  },
  "element": {
    "length": 0.5072000000000001,
    "width": 0.05
  },
  "material": {
    "barkBlue": 0.13499999999999998,
    "barkGreen": 0.1975,
    "barkRed": 0.2875,
    "barkRoughness": 0.9275000000000001,
    "brightnessRangeHigh": 0.04,
    "brightnessRangeLow": -0.04,
    "crestRed": 0.0125,
    "crestStrength": 0.0375,
    "depthStrength": 0.0375,
    "fissureRed": 0.125,
    "fissureStrength": 0.075,
    "hueRangeHigh": 0.0275,
    "hueRangeLow": -0.0275,
    "leafBackBlue": 0.11000000000000001,
    "leafBackGreen": 0.16,
    "leafBackRed": 0.12,
    "leafFrontBlue": 0.11000000000000001,
    "leafFrontGreen": 0.16,
    "leafFrontRed": 0.12,
    "orientationRed": 0.01,
    "orientationStrength": 0.075,
    "plateFurrowWidth": 0.0375,
    "plateScale": 0.0375,
    "ridgeScale": 0.005,
    "weatheringRed": 0.0025,
    "weatheringStrength": 0.075
  },
  "radii": {
    "trunkRadius": 0.01
  },
  "skeleton": {
    "bias": {
      "gravitropism": 1.0,
      "supernatural": {
        "writheWavelength": 0.35
      }
    },
    "envelope": {
      "crownBase": 0.6,
      "height": 22.86,
      "spread": 0.09999999999999998
    },
    "habit": {
      "attractorWeight": 0.7,
      "crookedness": 6.0,
      "lateralsPerStation": 1,
      "riseSecondary": -0.5
    },
    "twigs": {
      "hang": 1.0,
      "maxDroop": 3.35,
      "pendulousLength": 2.25,
      "twig": {
        "diameter": 0.007
      }
    }
  }
}
```

## Gaps (11)

a new gap escalates: its cause in generator terms and the shape of its spec are the host's, never this run's

### 1. owner-fronds-long-arching — stalled in tuning

the crown must read as a palm's rosette of long arching pinnate fronds with the lowest fronds drooping; the fronds are far too thin and short today (leaflet size and frond length are the first dials)

- Latest route: tuning
- Attempts: 31 evaluated, 31 feasible
  - round 9 bundle@1 (20 dials moved)
  - round 9 bundle@2 (20 dials moved), adopted and rolled back: required cell owner-priority:owner-frond-colour: the fronds read blue-green to grey-green, as the sources and the photographs show on the P-WHOLE view at seed 1 went from pass to fail
  - round 10 bundle@0.25 (19 dials moved)
  - round 10 bundle@0.5 (21 dials moved)
  - round 10 bundle@1 (21 dials moved)
  - round 10 bundle@2 (21 dials moved), adopted and stood
- Reviewer's words:
  - Fronds are still short and held stiffly outward; the crown reads as a small starburst, not the reference's full drooping mop
  - Trunk shows sparse isolated pegs, not a dense leaf-base lattice
  - All renders still have fronds that are too short and thin, and none has the long, heavily drooping mop-like crown. Every trunk shows sparse pegs, not a dense diamond lattice of leaf bases.
  - The upper trunk bends slightly into the crown
  - Fronds are short and sparse, with little droop in the lower fronds
  - Trunk has a visible kink and wave below the crown
  - Fronds are thin and wispy, and the crown is sparse
  - Trunk has a pronounced S-bend below the crown
  - Fronds are thin and wispy; the crown lacks mass
  - The trunk has a strong S-bend that breaks the never-wavy rule; renders 1 and 4 do not
  - Crown is compact and short, with no long drooping lower fronds hanging well below the crown the way they do in the reference
  - Trunk carries sparse separate pegs instead of a dense lattice of diamond-shaped leaf-base stubs, and it is too slender
- Check: pending: the invoker answers reachable (dials not yet tried), covered (an open spec) or new

### 2. owner-trunk-straight-constant — stalled in tuning

the trunk must be straight or gently leaning, never wavy, of near-constant diameter

- Latest route: tuning
- Attempts: 38 evaluated, 38 feasible
  - round 9 bundle@1 (20 dials moved)
  - round 9 bundle@2 (20 dials moved), adopted and rolled back: required cell owner-priority:owner-frond-colour: the fronds read blue-green to grey-green, as the sources and the photographs show on the P-WHOLE view at seed 1 went from pass to fail
  - round 10 bundle@0.25 (19 dials moved)
  - round 10 bundle@0.5 (21 dials moved)
  - round 10 bundle@1 (21 dials moved)
  - round 10 bundle@2 (21 dials moved), adopted and stood
- Reviewer's words:
  - Fronds are still short and held stiffly outward; the crown reads as a small starburst, not the reference's full drooping mop
  - Trunk shows sparse isolated pegs, not a dense leaf-base lattice
  - All renders still have fronds that are too short and thin, and none has the long, heavily drooping mop-like crown. Every trunk shows sparse pegs, not a dense diamond lattice of leaf bases.
  - The upper trunk bends slightly into the crown
  - Fronds are short and sparse, with little droop in the lower fronds
  - Trunk has a visible kink and wave below the crown
  - Fronds are thin and wispy, and the crown is sparse
  - Trunk has a pronounced S-bend below the crown
  - Fronds are thin and wispy; the crown lacks mass
  - The trunk has a strong S-bend that breaks the never-wavy rule; renders 1 and 4 do not
  - Crown is compact and short, with no long drooping lower fronds hanging well below the crown the way they do in the reference
  - Trunk carries sparse separate pegs instead of a dense lattice of diamond-shaped leaf-base stubs, and it is too slender
- Check: pending: the invoker answers reachable (dials not yet tried), covered (an open spec) or new

### 3. owner-trunk-bare — handed off

nothing grows on the trunk

- Latest route: insufficient_evidence
- Attempts: 0 evaluated, 0 feasible
- Check: pending: the invoker answers reachable (dials not yet tried), covered (an open spec) or new

### 4. owner-frond-colour — handed off

the fronds read blue-green to grey-green, as the sources and the photographs show

- Latest route: insufficient_evidence
- Attempts: 0 evaluated, 0 feasible
- Check: pending: the invoker answers reachable (dials not yet tried), covered (an open spec) or new

### 5. crown-pinnate-arching-fronds — stalled in tuning

Crown composed of many long once-pinnate (feather) fronds radiating from a single apical point, each frond arching outward and downward so the outline of the crown reads as a rounded hemisphere to shuttlecock shape; frond count per crown appears high (roughly 30-60 visible), with the newest fronds held stiffly upward near the center and older ones progressively more recurved.

- Latest route: tuning
- Attempts: 30 evaluated, 30 feasible
  - round 9 bundle@1 (20 dials moved)
  - round 9 bundle@2 (20 dials moved), adopted and rolled back: required cell owner-priority:owner-frond-colour: the fronds read blue-green to grey-green, as the sources and the photographs show on the P-WHOLE view at seed 1 went from pass to fail
  - round 10 bundle@0.25 (19 dials moved)
  - round 10 bundle@0.5 (21 dials moved)
  - round 10 bundle@1 (21 dials moved)
  - round 10 bundle@2 (21 dials moved), adopted and stood
- Reviewer's words:
  - Fronds are still short and held stiffly outward; the crown reads as a small starburst, not the reference's full drooping mop
  - Trunk shows sparse isolated pegs, not a dense leaf-base lattice
  - All renders still have fronds that are too short and thin, and none has the long, heavily drooping mop-like crown. Every trunk shows sparse pegs, not a dense diamond lattice of leaf bases.
  - The upper trunk bends slightly into the crown
  - Fronds are short and sparse, with little droop in the lower fronds
  - Trunk has a visible kink and wave below the crown
  - Fronds are thin and wispy, and the crown is sparse
  - Trunk has a pronounced S-bend below the crown
  - Fronds are thin and wispy; the crown lacks mass
  - The trunk has a strong S-bend that breaks the never-wavy rule; renders 1 and 4 do not
  - Crown is compact and short, with no long drooping lower fronds hanging well below the crown the way they do in the reference
  - Trunk carries sparse separate pegs instead of a dense lattice of diamond-shaped leaf-base stubs, and it is too slender
- Check: pending: the invoker answers reachable (dials not yet tried), covered (an open spec) or new

### 6. leaflet-arrangement-stiff-narrow — handed off

Leaflets are narrow, stiff, linear-lanceolate and taper to sharp points; they are inserted along the rachis in groups and at differing angles so the frond has a plumose/V-sectioned look rather than a flat plane. Leaflet tips appear rigid and spreading, giving the frond margin a bristly silhouette.

- Latest route: insufficient_evidence
- Attempts: 13 evaluated, 13 feasible
  - round 3 bundle@1 (24 dials moved), adopted and rolled back: required cell owner-priority:trunk-colour-and-weathering: Trunk colour ranges from mid grey-brown to warm reddish-brown; stub faces are lighter and greyer than the surrounding fibre, some bleached to pale grey, consistent with weathered exposed surfaces. Lower trunk in reference-0 reads more uniformly grey-brown with a finer texture than the coarse stubs shown close up. on the P-WHOLE view at seed 1 went from pass to fail
  - round 3 bundle@2 (24 dials moved)
  - round 4 bundle@1 (20 dials moved), adopted and rolled back: required cell owner-priority:trunk-colour-and-weathering: Trunk colour ranges from mid grey-brown to warm reddish-brown; stub faces are lighter and greyer than the surrounding fibre, some bleached to pale grey, consistent with weathered exposed surfaces. Lower trunk in reference-0 reads more uniformly grey-brown with a finer texture than the coarse stubs shown close up. on the P-WHOLE view at seed 1 went from pass to fail; required cell owner-priority:trunk-colour-and-weathering: Trunk colour ranges from mid grey-brown to warm reddish-brown; stub faces are lighter and greyer than the surrounding fibre, some bleached to pale grey, consistent with weathered exposed surfaces. Lower trunk in reference-0 reads more uniformly grey-brown with a finer texture than the coarse stubs shown close up. on the P-WHOLE view at seed 42 went from pass to fail
  - round 4 bundle@2 (20 dials moved)
  - round 5 bundle@1 (23 dials moved), adopted and rolled back: required cell owner-priority:trunk-colour-and-weathering: Trunk colour ranges from mid grey-brown to warm reddish-brown; stub faces are lighter and greyer than the surrounding fibre, some bleached to pale grey, consistent with weathered exposed surfaces. Lower trunk in reference-0 reads more uniformly grey-brown with a finer texture than the coarse stubs shown close up. on the P-WHOLE view at seed 1 went from pass to fail; required cell owner-priority:trunk-colour-and-weathering: Trunk colour ranges from mid grey-brown to warm reddish-brown; stub faces are lighter and greyer than the surrounding fibre, some bleached to pale grey, consistent with weathered exposed surfaces. Lower trunk in reference-0 reads more uniformly grey-brown with a finer texture than the coarse stubs shown close up. on the P-WHOLE view at seed 42 went from pass to fail
  - round 5 bundle@2 (23 dials moved)
- Reviewer's words:
  - The fronds are short and radiate stiffly like a star; they do not arch, and the lowest ones do not hang down as they do in the reference.
  - The trunk is too thin and pole-like compared with the reference's thick trunk.
  - In every render the fronds are still far too short and radiate stiffly; none has the reference's long arching fronds with a heavy drooping lower tier or its dense hemispherical crown. Every trunk is also too thin next to the reference's massive trunk.
  - The upper trunk curves noticeably below the crown rather than holding one lean.
  - Pegs stick out along the trunk, and the fronds are too short, with no drooping lower tier.
  - Crown is small and short-fronded for the trunk height; the fronds do not arch and hang down the way the reference's do
  - Trunk is a thin pole with regular peg-like stubs sticking out, not the reference's thick, scaly trunk
  - None of the renders has fronds long and heavy enough to make the full, drooping shuttlecock crown seen in the reference. All of them still have peg-like stubs sticking out of the trunk and a trunk that is too thin.
  - Trunk has a visible kink just below the crown
  - Fronds are still thin and short, and the crown is sparse compared with the reference
  - The crown is still small next to the very tall, slender trunk, and the fronds are too short to arch and droop the way the reference fronds do.
  - Stub-like knobs stick out all along the trunk, when the reference trunk has a textured, even surface.
- Check: pending: the invoker answers reachable (dials not yet tried), covered (an open spec) or new

### 7. trunk-leaf-base-diamond-pattern — handed off

Trunk surface is covered in persistent cut or broken leaf-base stubs arranged in a spiral, tessellating into a diamond/rhomboid lattice; each stub is a blunt wedge projecting several centimetres from the trunk with a flat, angular outer face. The pattern is coarse and irregular in reference-1/2 and reads as a finer knobbly texture at distance in reference-0.

- Latest route: insufficient_evidence
- Attempts: 6 evaluated, 6 feasible
  - round 1 bundle@2 (20 dials moved), adopted and rolled back: uncalibrated side-effect question: the closing review reports a defect the previous one did not (Some("new_defect") at Some(0.67) against threshold 0.5); it says ["Fronds are straight radiating spokes. They do not arch outward and down, so the crown outline reads as a round starburst instead of the drooping dome in reference-0 (render-0, render-1).","Leaflets are short, small and sparse along each rachis. Each frond reads as a thin spray instead of the dense, long-leafleted plumose frond in reference-0 and reference-1, so the owner's 'too thin and short' concern is still visible.","The lowest fronds hang straight down in a tight column against the upper trunk. They do not arch out and droop around the crown (render-0, render-2).","The trunk is a smooth cylinder with sparse, widely spaced round-capped cylindrical pegs. There is no tessellating spiral lattice of blunt wedge stubs with flat angular faces (render-2, render-3 vs reference-1, reference-2).","No fibrous matting fills the space between stubs. The trunk surface between the pegs is bare and smooth (render-2, render-3 vs reference-1, reference-2).","The trunk is a uniform slate blue-grey. It is not the grey-brown to reddish-brown of the references, and the stub faces are not weathered paler.","There is no brown-to-grey collar of dead fronds under the living crown. Every hanging frond is the same dark green as the live ones.","In the P-WHOLE views the fronds read near-black dark green, not blue-green to grey-green. Only the closer render-2 shows sage/grey-green leaflet material."]
  - round 2 bundle@4 (18 dials moved), adopted and rolled back: uncalibrated side-effect question: the closing review reports a defect the previous one did not (Some("new_defect") at Some(0.91) against threshold 0.5); it says ["On seeds 1 and 42, the fronds are straight, stiff rays. The crown reads as a spiky starburst or ball, not the rosette of long arching fronds seen in reference-0. The lowest fronds angle down only about 30-45° instead of hanging steeply. Frond length is small next to the very tall trunk.","Frond shape fails the owner's first priority. The fronds are not arching, the lowest ones do not droop far enough, and they are short relative to the trunk.","The trunk is a smooth shaft. The leaf bases appear as sparse, round-capped cylindrical pegs set in a single file along the trunk (render-0, render-1, render-2, render-3). There is no dense spiral diamond lattice of blunt wedge-shaped stubs with flat outer faces, as in reference-1 and reference-2.","There is no fibrous matting between the leaf bases. The trunk surface is smooth and untextured in the P-BASE close view (render-3).","The collar under the crown hangs against the upper trunk, but it is grey-green like the living foliage. It is not brown to grey, and there is no abrupt change from living to dead fronds, so it does not read as dead fronds."]
  - round 6 bundle@2 (22 dials moved), adopted and rolled back: required cell owner-priority:owner-frond-colour: the fronds read blue-green to grey-green, as the sources and the photographs show on the P-WHOLE view at seed 1 went from pass to fail
  - round 8 bundle@2 (20 dials moved), adopted and rolled back: required cell owner-priority:owner-frond-colour: the fronds read blue-green to grey-green, as the sources and the photographs show on the P-WHOLE view at seed 1 went from pass to fail
  - round 9 bundle@2 (20 dials moved), adopted and rolled back: required cell owner-priority:owner-frond-colour: the fronds read blue-green to grey-green, as the sources and the photographs show on the P-WHOLE view at seed 1 went from pass to fail
  - round 10 bundle@2 (21 dials moved), adopted and stood
- Reviewer's words:
  - Fronds are still short and held stiffly outward; the crown reads as a small starburst, not the reference's full drooping mop
  - Trunk shows sparse isolated pegs, not a dense leaf-base lattice
  - All renders still have fronds that are too short and thin, and none has the long, heavily drooping mop-like crown. Every trunk shows sparse pegs, not a dense diamond lattice of leaf bases.
  - Crown is compact and short, with no long drooping lower fronds hanging well below the crown the way they do in the reference
  - Trunk carries sparse separate pegs instead of a dense lattice of diamond-shaped leaf-base stubs, and it is too slender
  - All renders lack the reference's long drooping lower fronds and skirt of older fronds. Their trunks are also too thin, with separate pegs instead of the dense diamond-shaped leaf-base texture.
  - The fronds are still too short and stiff, and the lowest ones don't hang down into a skirt the way they do in the reference
  - The trunk shows scattered small pegs instead of a coarse diamond pattern of leaf-base stubs
  - Every render is missing long, heavy fronds with drooping lower fronds and a hanging skirt of old leaves, a dense crown of many more fronds, and a trunk covered in a diamond pattern of leaf bases instead of sparse pegs.
  - The fronds are still shorter than the reference's, and too few of the lower fronds droop; the crown reads as a starburst instead of a rounded shuttlecock.
  - The trunk has sparse peg-like stubs instead of a dense lattice of leaf-base stubs.
  - Every render is missing long fronds that droop well below the crown, a dense crown of roughly 30 to 60 fronds shaped like a hemisphere, and a trunk covered in a coarse diamond lattice of leaf-base stubs instead of sparse pegs.
- Check: pending: the invoker answers reachable (dials not yet tried), covered (an open spec) or new

### 8. solitary-columnar-stem — handed off

A single unbranched columnar stem of roughly constant diameter carries the crown; no branching is visible anywhere along the stem in any reference.

- Latest route: insufficient_evidence
- Attempts: 4 evaluated, 4 feasible
  - round 10 bundle@0.25 (19 dials moved)
  - round 10 bundle@0.5 (21 dials moved)
  - round 10 bundle@1 (21 dials moved)
  - round 10 bundle@2 (21 dials moved), adopted and stood
- Reviewer's words:
  - Fronds are still short and held stiffly outward; the crown reads as a small starburst, not the reference's full drooping mop
  - Trunk shows sparse isolated pegs, not a dense leaf-base lattice
  - All renders still have fronds that are too short and thin, and none has the long, heavily drooping mop-like crown. Every trunk shows sparse pegs, not a dense diamond lattice of leaf bases.
  - The upper trunk bends slightly into the crown
  - Fronds are short and sparse, with little droop in the lower fronds
  - Trunk has a visible kink and wave below the crown
  - Fronds are thin and wispy, and the crown is sparse
  - Trunk has a pronounced S-bend below the crown
  - Fronds are thin and wispy; the crown lacks mass
  - The trunk has a strong S-bend that breaks the never-wavy rule; renders 1 and 4 do not
- Check: pending: the invoker answers reachable (dials not yet tried), covered (an open spec) or new

### 9. dead-frond-skirt — handed off

Beneath the living crown hangs a collar of dead and dying fronds, brown to grey, collapsed downward against the upper trunk and partly obscuring it; the transition from green living fronds to brown dead fronds is abrupt in reference-0.

- Latest route: insufficient_evidence
- Attempts: 0 evaluated, 0 feasible
- Check: pending: the invoker answers reachable (dials not yet tried), covered (an open spec) or new

### 10. trunk-colour-and-weathering — stalled in tuning

Trunk colour ranges from mid grey-brown to warm reddish-brown; stub faces are lighter and greyer than the surrounding fibre, some bleached to pale grey, consistent with weathered exposed surfaces. Lower trunk in reference-0 reads more uniformly grey-brown with a finer texture than the coarse stubs shown close up.

- Latest route: tuning
- Attempts: 33 evaluated, 33 feasible
  - round 8 bundle@2 (13 dials moved)
  - round 8 bundle@4 (13 dials moved)
  - round 8 bundle@8 (13 dials moved)
  - round 9 bundle@0.5 (12 dials moved)
  - round 9 bundle@1 (12 dials moved)
  - round 9 bundle@2 (12 dials moved)
- Reviewer's words:
  - No crown or fronds are visible. Only a trunk fragment appears in the bottom-right corner, with the tree almost entirely out of frame.
  - The trunk is a saturated brick red, well outside the references' grey-brown to warm reddish-brown range. It also forks into a V.
  - Its trunk hue moves to a saturated brick red, which is outside the references' colour range. The other renders stay within or near grey-brown to reddish-brown.
  - In every render, the tree is framed almost entirely out of view, so there is no visible crown or frond rosette to judge frond colour. Every visible trunk also forks into a V and lacks the stubbed, fibrous, weathered surface of the references.
  - The trunk forks into a V and is a smooth pinkish maroon, with no lighter grey stub faces or fibrous texture.
  - The trunk forks into a V and has a purplish-brown cast that neither the grey-brown nor the reddish-brown in the references shows.
  - The trunk is a saturated hot magenta-pink with no natural bark colour.
  - Only a sliver of trunk shows in the corner, with no crown and no stub or fibre texture.
  - In every render the tree is almost entirely out of frame, so there is no palm crown, and no trunk shows its full length. No render has the coarse, layered leaf-base stubs or the reddish-brown fibre mat between them seen in the references.
  - The trunk is a deep magenta-crimson with no natural bark colour.
  - The trunk is a dark plum-magenta, nothing like the references' grey-brown to reddish-brown fibre.
  - Only a sliver of trunk shows in the bottom-right corner. There is no crown of fronds and no visible leaf-base stubs or fibre texture.
- Check: pending: the invoker answers reachable (dials not yet tried), covered (an open spec) or new

### 11. basal-flare — handed off

Where the trunk meets the ground in reference-0 it widens into a broad rooted base merging into the soil mound; no clear detail of root boss or offshoots is resolvable at this scale.

- Latest route: insufficient_evidence
- Attempts: 0 evaluated, 0 feasible
- Check: pending: the invoker answers reachable (dials not yet tried), covered (an open spec) or new
