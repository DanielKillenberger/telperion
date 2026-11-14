/* ------------------------------------------------------------------ *
 * THE PANEL, BY WHAT A CONTROL DOES
 *
 * Every control is a catalogue row or an engine setting, each in the
 * group the catalogue gives it (`GROUPS`, most telling first). A control
 * knows whether it differs from the preset the dials started from and
 * how to go back to it; a filter finds one by its words.
 * ------------------------------------------------------------------ */

import type { Family } from "../src/browser/core";
import {
  GROUPS,
  SPACE_SETTINGS,
  type Group,
  type Parameter,
  type SpaceSetting,
} from "../src/browser/parameters.generated";
import { readRow, shownRows, writeRow } from "./rows";
import { places, readSetting, stated, writeSetting } from "./space";

/** One control: a catalogue row, or an engine setting over every age. */
export type Control =
  | { kind: "row"; id: string; group: Group; row: Parameter }
  | { kind: "space"; id: string; group: Group; setting: SpaceSetting };

/** Rows another control owns: the seed box and the age dial. */
const OWNED: ReadonlySet<string> = new Set(["/skeleton/seed", "/age"]);

/** Every control the preset's tree offers, in its group, groups in the
 *  panel's order. */
export function controls(preset: Family): { group: Group; title: string; controls: Control[] }[] {
  const all: Control[] = [
    ...shownRows().filter(p => !OWNED.has(p.path))
      .map(row => ({ kind: "row" as const, id: row.path, group: row.group, row })),
    ...stated(preset, SPACE_SETTINGS)
      .map(setting => ({ kind: "space" as const, id: `/space/${setting.name}`, group: setting.group, setting })),
  ];
  return GROUPS.map(g => ({ group: g.id, title: g.title, controls: all.filter(c => c.group === g.id) }));
}

/** The words a filter searches: label, unit, meaning, path or name. */
function words(c: Control): string {
  const parts = c.kind === "row"
    ? [c.row.label, c.row.unit, c.row.meaning, c.row.path]
    : [c.setting.label, c.setting.unit, c.setting.meaning, c.setting.name];
  return parts.join(" ").toLowerCase();
}

/** Whether every word of `filter` is among the control's. */
export function matches(c: Control, filter: string): boolean {
  const text = words(c);
  return filter.toLowerCase().split(/\s+/).filter(w => w !== "").every(w => text.includes(w));
}

/** Where an engine setting sits, in either family. */
function everywhere(family: Family, preset: Family, s: SpaceSetting) {
  const seen = new Map(places(preset, s).map(at => [`${at.age}/${at.zone ?? ""}`, at]));
  for (const at of places(family, s)) seen.set(`${at.age}/${at.zone ?? ""}`, at);
  return [...seen.values()];
}

/** Whether the control differs from the preset. */
export function changed(c: Control, family: Family, preset: Family): boolean {
  if (c.kind === "row") return readRow(family, c.row.path) !== readRow(preset, c.row.path);
  const s = c.setting;
  return everywhere(family, preset, s).some(at => readSetting(family, s, at) !== readSetting(preset, s, at));
}

/** The family with the control back at the preset's value. */
export function reset(c: Control, family: Family, preset: Family): Family {
  if (c.kind === "row") return writeRow(family, c.row.path, readRow(preset, c.row.path));
  const s = c.setting;
  return everywhere(family, preset, s).reduce(
    (f, at) => (readSetting(f, s, at) === readSetting(preset, s, at)
      ? f
      : writeSetting(f, s, at, readSetting(preset, s, at))),
    family,
  );
}

/** The family with every one of `cs` back at the preset's value. */
export function resetAll(cs: readonly Control[], family: Family, preset: Family): Family {
  return cs.reduce((f, c) => reset(c, f, preset), family);
}
