/* The field's types, one definition each, read by the main entry's CPU field
 * and by `telperion/field`'s device field alike. Types only: importing this
 * loads no module. */

/** An axis-aligned box in metres. */
export interface Bounds { min: [number, number, number]; max: [number, number, number] }
/** The grid a field is read to (fn-257): `n` cells a side, `cell` metres
 * each, `n - 2` of them spanning the box's longest axis, centred on the box
 * in x and z with one empty cell below its lowest point. Cell `i + n * (j +
 * n * k)` is centred at `origin + (i + 0.5, j + 0.5, k + 0.5) * cell`. */
export interface Grid { n: number; cell: number; origin: [number, number, number] }
/** One answer per cell, in the grid's cell order. `flags` bits: wood = 1,
 * foliage = 2, flowers = 4. `woodRadius` is the thickest wood sweep reaching
 * the cell, in metres, zero without wood. `leaves` estimates the leaf
 * stations in the cell before the crown cull. `limbs` is the owning limb
 * system, `NO_LIMB` (UINT32_MAX) where no foliage reaches. `flowers` counts
 * the florets whose seat lies in the cell, at the flowering window's middle:
 * over the grid they sum to the tree's florets, zero on a tree that flowers
 * nowhere. */
export interface FieldAnswer { flags: Uint8Array; woodRadius: Float32Array; leaves: Float32Array; limbs: Uint32Array; flowers: Float32Array }
/** A field read out: its grid and one answer per cell. */
export interface FieldGrid extends Grid { answer: FieldAnswer }
