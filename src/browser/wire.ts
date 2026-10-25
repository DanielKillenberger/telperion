/* Parameters as the cores read them: the one serializer every generator
 * entry hands its family through (fn-317). */
import type { Family, TreePreset } from "./presets";

/** `family`'s parameters as JSON, as the cores read them: a preset's own
 * id, name and note left off, and a string, which is a preset's id and not
 * parameters, refused by name. JSON would otherwise silently replace a
 * nonfinite value with null, so one is refused too. */
export function wire(family: Family): string {
  if (typeof family !== "object" || family === null) {
    throw TypeError("A tree is its parameters, not a preset id: pass presetById(id)");
  }
  const { id: _id, name: _name, note: _note, ...parameters } = family as Partial<TreePreset>;
  return JSON.stringify(parameters, finite);
}
/** A JSON replacer that refuses a nonfinite number. */
export function finite(_key: string, value: unknown): unknown {
  if (typeof value === "number" && !Number.isFinite(value)) throw Error("Tree parameters must be finite");
  return value;
}
