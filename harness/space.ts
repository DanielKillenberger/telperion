/* ------------------------------------------------------------------ *
 * THE ENGINE'S SETTINGS, AS CONTROLS
 *
 * A species states its engine settings age by age under `/space/ages`.
 * One control moves a setting at every age the species states, on the
 * setting's own scale (`SPACE_SETTINGS`, generated from the engine's
 * table): a magnitude is multiplied, a share or an angle offset, each
 * age held inside the setting's bounds. The rows go back through the
 * path a preset's rows take; a value the engine refuses comes back by
 * name, never clamped here past the bounds the table states.
 * ------------------------------------------------------------------ */

import type { Family } from "../src/browser/core";
import type { SpaceRows, SpaceSetting } from "../src/browser/parameters.generated";

/** Where one setting sits: an age, and a zone where it is a zone's. */
export interface Place { age: string; zone?: string }

type Rows = { [key: string]: SpaceRows | number | boolean | "never" };

function ages(family: Family): Rows {
  return ((family.space as Rows | undefined)?.ages ?? {}) as Rows;
}

/** Every place the family states `s` at: each stated age, and each of
 *  its zones for a zone's setting. */
export function places(family: Family, s: SpaceSetting): Place[] {
  return Object.entries(ages(family)).flatMap(([age, rows]) => {
    if (!s.zone) return [{ age }];
    const zones = ((rows as Rows).zones ?? {}) as Rows;
    return Object.keys(zones).map(zone => ({ age, zone }));
  });
}

function path(s: SpaceSetting, at: Place): string[] {
  const head = at.zone === undefined ? ["ages", at.age] : ["ages", at.age, "zones", at.zone];
  return [...head, ...s.key.split("/")];
}

/** The setting's value at a place: as stated, or its neutral; `never` is
 *  an unbounded span. */
export function readSetting(family: Family, s: SpaceSetting, at: Place): number {
  let node: unknown = family.space;
  for (const key of path(s, at)) node = (node as Rows | undefined)?.[key];
  if (node === "never") return Infinity;
  return typeof node === "number" ? node : s.neutral;
}

/** A copy of the family with the setting at `at` set to `value`. */
export function writeSetting(family: Family, s: SpaceSetting, at: Place, value: number): Family {
  const copy = structuredClone(family);
  const keys = path(s, at);
  const last = keys.pop()!;
  let node = (copy.space ??= {}) as Rows;
  for (const key of keys) node = (node[key] ??= {}) as Rows;
  node[last] = value === Infinity ? "never" : value;
  return copy;
}

/** Whether the setting admits `value` on its own; its relation to other
 *  settings is the engine's to judge. */
export function admits(s: SpaceSetting, value: number): boolean {
  if (Number.isNaN(value) || value > s.high) return false;
  if (value === Infinity) return s.never;
  return s.lowOpen ? value > s.low : value >= s.low;
}

/** `value` moved by `by` on the setting's scale: a factor on a log
 *  scale, an offset otherwise (on the share 1 / (1 + value) a `kept`
 *  setting keeps), held to its bounds. Unchanged where the bounds cannot
 *  hold the move (an open end, or `never` where the wire refuses it). */
export function moved(s: SpaceSetting, value: number, by: number): number {
  let next: number;
  if (s.scale === "log") next = (value + s.floor) * by - s.floor;
  else if (s.scale === "kept") {
    const kept = Math.min(1, Math.max(0, 1 / (1 + value) + by));
    next = kept === 0 ? Infinity : 1 / kept - 1;
  } else next = value + by;
  const held = Number.isFinite(next) ? Math.min(s.high, Math.max(s.low, next)) : next;
  return admits(s, held) ? held : value;
}

/** The family the preset states, with the setting moved by `by` at every
 *  place the preset states it. */
export function moveAll(preset: Family, current: Family, s: SpaceSetting, by: number): Family {
  return places(preset, s).reduce((family, at) => {
    const from = readSetting(preset, s, at);
    const to = moved(s, from, by);
    return to === readSetting(family, s, at) ? family : writeSetting(family, s, at, to);
  }, current);
}

/** The whole-setting control's range and its neutral: a factor of a
 *  quarter to four on a log scale, an offset of half the span (a whole
 *  share on a `kept` one) otherwise. */
export function range(s: SpaceSetting): { min: number; max: number; neutral: number; step: number } {
  if (s.scale === "log") return { min: 0.25, max: 4, neutral: 1, step: 0.01 };
  if (s.scale === "kept") return { min: -1, max: 1, neutral: 0, step: 0.01 };
  const half = (s.high - s.low) / 2;
  return { min: -half, max: half, neutral: 0, step: half / 100 };
}

/** The settings a species states somewhere, in the table's order. */
export function stated(family: Family, settings: readonly SpaceSetting[]): SpaceSetting[] {
  return settings.filter(s => places(family, s).length > 0);
}
