"use client";

/* ------------------------------------------------------------------ *
 * THE DIALS, AS CONTROLS
 *
 * One control per catalogue row the build reads, grouped by where the
 * row sits in the family, each with its meaning and, where it has one,
 * the condition under which it does nothing. The two named transforms
 * sit in the group whose rows they move. Split out of the panel
 * component because the panel owns the canvas, the stage's lifecycle
 * and the params.
 *
 * Nothing here knows what will draw the tree. It takes the params and
 * hands back the params changed, which is the whole contract.
 * ------------------------------------------------------------------ */

import type { Dispatch, SetStateAction } from "react";

import type { Parameter } from "../src/browser/parameters.generated";
import { ADAPTERS, type Adapter } from "./family";
import type { GrowerParams } from "./params";
import { admit, groupOf, labelOf, notch, readRow, shownRows, span, writeRow, type Value } from "./rows";

/** Rows another control owns: the seed box, and each adapter's row. */
const OWNED: ReadonlySet<string> = new Set([
  "/skeleton/seed", ...ADAPTERS.flatMap(a => (a.owns === undefined ? [] : [a.owns])),
]);

/** A dial's value, at a precision that can tell its own steps apart -
 *  which is the STEP's business and not the value's. */
function format(value: number, step: number): string {
  const decimals = Math.max(0, Math.ceil(-Math.log10(step) - 1e-9));
  return value.toFixed(decimals);
}

/** Every numeric member of one plain object, as a number box: the
 *  renderer's scene row, which is no catalogue row. */
export function Traits({ prefix, values, onChange }: {
  prefix?: string;
  values: Record<string, unknown>;
  onChange: (key: string, value: number) => void;
}) {
  return <>
    {Object.entries(values)
      .filter(([, value]) => typeof value === "number")
      .map(([key, value]) => (
        <label className="gd-row" key={key}>
          {prefix === undefined ? "" : `${prefix} `}
          {key.replace(/[A-Z]/g, letter => ` ${letter.toLowerCase()}`)}
          <input type="number" value={value as number} step="any" onChange={event => {
            const number = event.target.valueAsNumber;
            if (Number.isFinite(number)) onChange(key, number);
          }} />
        </label>
      ))}
  </>;
}

/** A slider over `[low, high]` and a box for the exact value. */
function Numeric({ id, value, ends, step, onChange }: {
  id: string;
  value: number | undefined;
  ends: [number, number] | null;
  step: number | "any";
  onChange: (raw: number, text: string) => void;
}) {
  return <>
    <input id={id} type="number" value={value ?? ""} placeholder="unset" step={step}
      onChange={event => onChange(event.target.valueAsNumber, event.target.value)} />
    {ends !== null && value !== undefined && <input type="range" aria-label={id}
      min={ends[0]} max={ends[1]} step={step} value={value}
      onChange={event => onChange(Number.parseFloat(event.target.value), event.target.value)} />}
  </>;
}

/** One catalogue row: its control, its meaning and its dormancy. */
function Row({ parameter: p, value, onChange }: {
  parameter: Parameter;
  value: Value;
  onChange: (value: Value) => void;
}) {
  const id = `gd-${p.path}`;
  const ends = typeof value === "number" ? span(p, value) : null;
  const step = ends === null ? (p.kind === "count" ? 1 : "any") : notch(p, ends);
  return <div className="gd-dial">
    <label className="gd-label" htmlFor={id}>
      {labelOf(p.path)}
      {typeof value === "number" && <span className="gd-value">
        {step === "any" ? value : format(value, step)} {p.unit}
      </span>}
    </label>
    {p.kind === "switch"
      ? <input id={id} type="checkbox" checked={value === true} onChange={event => onChange(event.target.checked)} />
      : <Numeric id={id} value={typeof value === "number" ? value : undefined} ends={ends} step={step}
          onChange={(raw, text) => {
            if (p.optional && text === "") return onChange(undefined);
            const admitted = admit(p, raw);
            if (admitted !== null) onChange(admitted);
          }} />}
    <p className="gd-meaning">{p.meaning}</p>
    {p.applies !== "" && <p className="gd-dormant">dormant: {p.applies}</p>}
  </div>;
}

/** A named transform, drawn as a row is. */
function AdapterDial({ adapter: a, value, onChange }: {
  adapter: Adapter;
  value: number;
  onChange: (value: number) => void;
}) {
  const id = `gd-${a.key}`;
  return <div className="gd-dial">
    <label className="gd-label" htmlFor={id}>
      {a.key}<span className="gd-value">{format(value, a.step)}</span>
    </label>
    <Numeric id={id} value={value} step={a.step}
      ends={[Math.min(a.min, value), Math.max(a.max, value)]}
      onChange={raw => { if (Number.isFinite(raw)) onChange(raw); }} />
    <p className="gd-meaning">{a.meaning}</p>
  </div>;
}

/** The rows the build reads, by group; growth-path rows too when the
 *  growth path is open. */
export function Dials({ params, setParams, growth }: {
  params: GrowerParams;
  setParams: Dispatch<SetStateAction<GrowerParams>>;
  growth: boolean;
}) {
  const groups = new Map<string, Parameter[]>();
  for (const p of shownRows(growth)) {
    if (!OWNED.has(p.path)) groups.set(groupOf(p.path), [...groups.get(groupOf(p.path)) ?? [], p]);
  }
  return <>
    {[...groups].map(([group, rows]) => (
      <details className="gd-dials" key={group}>
        <summary>{group}</summary>
        {ADAPTERS.filter(a => a.group === group).map(a => (
          <AdapterDial key={a.key} adapter={a} value={params[a.key]}
            onChange={value => setParams(prev => ({ ...prev, [a.key]: value }))} />
        ))}
        {rows.map(p => (
          <Row key={p.path} parameter={p} value={readRow(params.family, p.path)}
            onChange={value => setParams(prev => ({ ...prev, family: writeRow(prev.family, p.path, value) }))} />
        ))}
      </details>
    ))}
  </>;
}
