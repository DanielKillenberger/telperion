# Tree space: branches sag under their load and rest on the ground

## Goal & Context

The spruce (fn-194, round 1) cannot droop its lower crown: a branch's angles are fixed at birth, so old, loaded branches hold the pose of their youth (MODEL-SPRUCE.md, F3). Its lowest branches also cannot reach the ground, because wood below ground is refused; holding them up by values left the 10-year spruce a bare pole (MODEL-SPRUCE.md, F6). Both are generator gaps the spruce depends on, and the open-grown beech's lower limbs and the oak's sweeping limbs need the first (host, 2026-10-04).

## Design (host, 2026-10-04)

- **Sag is a beam, computed after growth.** In the geometry pass, after presences, each node turns downward by a curvature proportional to the bending moment of the wood and foliage it carries (presence-weighted mass times its horizontal lever arm) over its section's stiffness (radius to the fourth). The setting is `form.sag` per physiological age, 0 neutral (no change from today). Growth does not read it back.
- **The tip keeps its own tropism.** Load falls to nothing at the tip, so the base droops while the tip still rises: the spruce's swept-down, upturned branch, by construction and not by a second rule.
- **The ground supports.** Wood that bends down onto the ground rests on it and runs along it; it is never placed below. An axis whose base is below ground is still an error, and so is a tree whose trunk reaches the ground. Resting is a support, not a fallback: open-grown spruces lay their skirts on the ground and layer there.
- Continuity: mass is presence-weighted, so `form.sag` and every other setting still change the tree by degree; the walk bound of 30 holds.

## Requirements

- **R1:** `form.sag` as above, walk-tested, neutral at 0 (every existing tree byte-identical at 0).
- **R2:** A test that a loaded horizontal branch droops at the base more than at the tip, and that its tip direction still follows its tropism.
- **R3:** Ground support: a sagging branch that reaches the ground rests at ground height; a test that no vertex lies below ground and that the refusal for a base below ground still fires.
- **R4:** Oracle (phase A) and walks (phase B) green; beech byte-identical (sag 0).
- **R5:** One still strip of the spruce walking `form.sag` from 0 up, viewed by the host.

## Boundaries

Geometry only: no feedback of sag into growth, light or carbon (phase E). Dormant-bud release (F4) is not this spec.
