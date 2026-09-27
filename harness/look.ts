/* ------------------------------------------------------------------ *
 * A LOOK: A TREE SOMEBODY WANTS THE OWNER TO SEE
 *
 * A named file under `harness/looks/`, served by the dev server and
 * never committed. `species <id> --look` writes a run's kept tree there
 * with the families core makes of it; a host drops a plain overlay (the
 * JSON `headless --family` takes) there to show a candidate without a
 * run. Either way the overlay is laid over its preset here and lands on
 * the dials as a preset does. A runner's look also carries core's own
 * family, and the harness refuses to draw a tree that differs from it.
 * ------------------------------------------------------------------ */

import { presetById, type Family } from "../src/browser/core";
import { PARAMETERS } from "../src/browser/parameters.generated";

import { presetToParams, toFamily } from "./family";
import type { GrowerParams } from "./params";

/** Where the dev server serves looks from. */
export const LOOKS = "/harness/looks";

/** Where a runner's look comes from, as the panel says it. */
export interface LookSource {
  run: string | null;
  revision: number | null;
  round: number | null;
  label: string | null;
  kept: number | null;
  key: string | null;
}

/** A look opened on the dials: the kept tree and the preset under it. */
export interface Opened {
  name: string;
  preset: string;
  /** Null for a plain overlay, which comes from no run. */
  source: LookSource | null;
  look: GrowerParams;
  shipped: GrowerParams;
}

type Node = Record<string, unknown>;

const isNode = (value: unknown): value is Node =>
  typeof value === "object" && value !== null && !Array.isArray(value);

/** The overlay laid over `family`, every row by its catalogue path. A key
 *  the catalogue has not, an object where a row is, or a value where a
 *  group is, is refused by its path. */
export function overlay(family: Family, overrides: unknown, at = ""): Family {
  if (!isNode(overrides)) throw Error(`the overlay at ${at || "/"} is not an object`);
  const copy = structuredClone(family) as unknown as Node;
  lay(copy, overrides, at);
  return copy as unknown as Family;
}

function lay(into: Node, over: Node, at: string): void {
  for (const [key, value] of Object.entries(over)) {
    const path = `${at}/${key}`;
    const row = PARAMETERS.some(p => p.path === path);
    const group = PARAMETERS.some(p => p.path.startsWith(`${path}/`));
    if (isNode(value) && group && isNode(into[key])) lay(into[key] as Node, value, path);
    else if (row && (typeof value === "number" || typeof value === "boolean" || value === null)) into[key] = value;
    else throw Error(`the overlay path ${path} is not a row of the family`);
  }
}

/** The first path at which two families differ, or null. */
export function difference(a: unknown, b: unknown, at = ""): string | null {
  if (isNode(a) && isNode(b)) {
    for (const key of new Set([...Object.keys(a), ...Object.keys(b)])) {
      const found = difference(a[key], b[key], `${at}/${key}`);
      if (found !== null) return found;
    }
    return null;
  }
  return Object.is(a, b) ? null : at || "/";
}

const field = <T>(value: unknown, is: (v: unknown) => v is T): T | null => (is(value) ? value : null);
const isNumber = (v: unknown): v is number => typeof v === "number";
const isString = (v: unknown): v is string => typeof v === "string";

function source(value: unknown): LookSource {
  const s = isNode(value) ? value : {};
  return {
    run: field(s.run, isString), revision: field(s.revision, isNumber), round: field(s.round, isNumber),
    label: field(s.label, isString), kept: field(s.kept, isNumber), key: field(s.key, isString),
  };
}

const onDials = (family: Family, preset: { id: string; name: string; note: string }): GrowerParams =>
  presetToParams({ ...family, id: preset.id, name: preset.name, note: preset.note });

/** A look's file, opened: a runner's look over the preset family it
 *  carries, checked against core's family; a plain overlay over the
 *  shipped preset `species` names. Every refusal names the look. */
export function open(name: string, body: unknown, species: string | null): Opened {
  const refuse = (why: string): never => { throw Error(`look ${name}: ${why}`); };
  if (!isNode(body)) return refuse("the file is not a JSON object");
  try {
    if (body.schema !== "harness-look") {
      if (species === null) return refuse("a plain overlay needs ?species=<preset> as its base");
      const preset = presetById(species);
      const base = onDials(preset, preset);
      return { name, preset: species, source: null, look: onDials(overlay(base.family, body), preset), shipped: base };
    }
    const preset = field(body.preset, isString) ?? refuse("names no preset");
    if (species !== null && species !== preset) refuse(`is laid over ${preset}, not ${species}`);
    if (!isNode(body.preset_family) || !isNode(body.family)) return refuse("carries no family");
    const named = { id: preset, name: preset, note: "" };
    const shipped = onDials(body.preset_family as unknown as Family, named);
    const look = onDials(overlay(shipped.family, body.overrides), named);
    const differs = difference(toFamily(look), body.family);
    if (differs !== null) refuse(`the harness's family differs from core's at ${differs}`);
    return { name, preset, source: source(body.source), look, shipped };
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    throw Error(message.startsWith(`look ${name}: `) ? message : `look ${name}: ${message}`);
  }
}

/** Fetches and opens the named look from the dev server. */
export async function fetchLook(name: string, species: string | null, get: typeof fetch = fetch): Promise<Opened> {
  if (!/^[a-z0-9][a-z0-9-]*$/.test(name)) throw Error(`look ${name}: a look is named in lowercase letters, digits and dashes`);
  const url = `${LOOKS}/${name}.json`;
  const response = await get(url);
  /* The dev server answers a file it lacks with the page itself. */
  const json = response.headers.get("content-type")?.includes("json") ?? false;
  if (!response.ok || !json) {
    throw Error(`look ${name}: the dev server has no ${url}; write it with \`species <id> --look\``);
  }
  let body: unknown;
  try { body = await response.json(); } catch (error) { throw Error(`look ${name}: ${url} is not JSON: ${String(error)}`); }
  return open(name, body, species);
}
