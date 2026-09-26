/* ------------------------------------------------------------------ *
 * THE CATALOGUE'S ROWS, AS CONTROLS
 *
 * Every control the panel offers over a family is one catalogue row,
 * read from the metadata the catalogue generates
 * (`src/browser/parameters.generated.ts`): its meaning, unit, bounds,
 * tuning window and where it lies dormant. Nothing here keeps a list
 * of its own; a row the catalogue adds is a control the next build
 * shows.
 * ------------------------------------------------------------------ */

import type { Family } from "../src/browser/core";
import { PARAMETERS, type Parameter } from "../src/browser/parameters.generated";

/** A row's value: a number, a switch, or unset where the row is optional. */
export type Value = number | boolean | undefined;

type Node = Record<string, unknown>;

function segments(path: string): string[] {
  return path.split("/").slice(1);
}

/** The value at a row's pointer. */
export function readRow(family: Family, path: string): Value {
  let at: unknown = family;
  for (const key of segments(path)) at = (at as Node | undefined)?.[key];
  return typeof at === "number" || typeof at === "boolean" ? at : undefined;
}

/** A copy of the family with the row at `path` set; unset deletes it. */
export function writeRow(family: Family, path: string, value: Value): Family {
  const copy = structuredClone(family);
  const keys = segments(path);
  const last = keys.pop()!;
  let at = copy as unknown as Node;
  for (const key of keys) at = at[key] as Node;
  if (value === undefined) delete at[last];
  else at[last] = value;
  return copy;
}

/** The rows a mature build reads. Growth-path rows join them when the
 *  growth path is open; a deprecated row no build reads never does. */
export function shownRows(growth: boolean): Parameter[] {
  return PARAMETERS.filter(p => p.reach === "mature" || (growth && p.reach === "growth"));
}

/** The group a row sits in: its pointer's parent, "/" for the family's own. */
export function groupOf(path: string): string {
  return path.slice(0, path.lastIndexOf("/")) || "/";
}

/** A row's label: its key, spaced. */
export function labelOf(path: string): string {
  return path.slice(path.lastIndexOf("/") + 1).replace(/[A-Z]/g, c => ` ${c.toLowerCase()}`);
}

/** The slider over a row: its ends and notch. The ends are the tuning
 *  window where the row offers a dial, its bounds otherwise, widened to take
 *  the value in so a preset is never clamped on arrival. The notch is one
 *  for a count, a round two-hundredth of the span otherwise. None where an
 *  end is unbounded, or where the notch is coarser than the value it would
 *  move (a window as wide as the validation bounds): the number box alone
 *  serves that row. */
export function slider(p: Parameter, value: number): { ends: [number, number]; notch: number } | null {
  const [low, high] = p.dial?.window ?? [p.low, p.high];
  if (!Number.isFinite(low) || !Number.isFinite(high)) return null;
  const ends: [number, number] = [Math.min(low, value), Math.max(high, value)];
  const notch = p.kind === "count" ? 1 : 10 ** Math.floor(Math.log10((ends[1] - ends[0]) / 200 || 1));
  const scale = Math.max(Math.abs(value), p.dial?.step ?? 0);
  return notch > scale ? null : { ends, notch };
}

/** A typed or dragged number, as the row admits it: rounded where it
 *  counts and held inside the bounds. Null where it is no number, or
 *  the low end a row refuses. */
export function admit(p: Parameter, raw: number): number | null {
  if (!Number.isFinite(raw)) return null;
  const whole = p.kind === "count" ? Math.round(raw) : raw;
  if (p.zero && whole === 0) return 0;
  if (p.lowOpen && whole <= p.low) return null;
  return Math.min(p.high, Math.max(p.low, whole));
}
