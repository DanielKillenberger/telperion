import type { TreePreset } from "./preset";
import { LAURELIN, TELPERION } from "./two-trees";

/* ------------------------------------------------------------------ *
 * THE PRESET REGISTRY
 *
 * A list and a lookup, and nothing that generates anything. A consumer
 * takes a `TreePreset` from here and hands its four members to
 * `growSkeleton`, `solveRadii`, `buildSurface` and `buildCanopy` - the
 * same calls it would make with parameters of its own.
 *
 * Telperion first, because he is the elder.
 * ------------------------------------------------------------------ */

export type { PresetSkeleton, TreePreset } from "./preset";
export { LAURELIN, TELPERION } from "./two-trees";

/** Every named tree the library ships, in the order a panel should
 *  offer them. */
export const PRESETS: readonly TreePreset[] = [TELPERION, LAURELIN];

/** The preset with this id, or `undefined`. Total on purpose: an id
 *  can arrive from a panel, a query string or a saved session, and
 *  none of those is a guarantee. */
export function getPreset(id: string): TreePreset | undefined {
  return PRESETS.find((preset) => preset.id === id);
}
