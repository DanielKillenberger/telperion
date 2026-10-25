/* The shipped presets as plain data (`telperion/presets`, fn-317 R3): a
 * preset is a reusable parameter set, and every generator entry takes the
 * parameters, never a preset's id. This module holds no Wasm and no engine,
 * so a page that only needs a preset's values (its bloom colours, its seed)
 * loads none. */
import { CATALOGUE, type Family } from "./presets.generated";
export type { Family } from "./presets.generated";

export interface TreePreset extends Family { id: string; name: string; note: string }
/** Explicit identity lookup; unknown identities never fall back to Ordinary. */
export function presetById(id: string): TreePreset {
  const entry = CATALOGUE.find(preset => preset.id === id);
  if (!entry) throw Error(`Unknown tree preset: ${id}`);
  return { ...structuredClone(entry.family), id: entry.id, name: entry.name, note: entry.note };
}
/** Plain parameter baseline, usable directly at the native request boundary. */
export const ORDINARY: Family = (({ id: _id, name: _name, note: _note, ...family }) => family)(presetById("ordinary"));
export const OREGON_WHITE_OAK = presetById("oregon-white-oak");
export const NORWAY_SPRUCE = presetById("norway-spruce");
export const SILVER_BIRCH = presetById("silver-birch");
export const TELPERION = presetById("telperion");
export const LAURELIN = presetById("laurelin");
export const TWO_TREES = [TELPERION, LAURELIN];
export const PRESETS = CATALOGUE.map(entry => presetById(entry.id));
