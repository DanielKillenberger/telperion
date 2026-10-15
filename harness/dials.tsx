"use client";

/* ------------------------------------------------------------------ *
 * THE DIALS, AS CONTROLS
 *
 * One control per catalogue row the build reads, grouped by where the
 * row sits in the family, each with its meaning and, where it has one,
 * the condition under which it does nothing. Split out of the panel
 * component because the panel owns the canvas, the stage's lifecycle
 * and the params.
 *
 * Nothing here knows what will draw the tree. It takes the params and
 * hands back the params changed, which is the whole contract.
 * ------------------------------------------------------------------ */

import type { Dispatch, SetStateAction } from "react";

import type { Parameter } from "../src/browser/parameters.generated";
import type { GrowerParams } from "./params";
import { admit, groupOf, labelOf, readRow, shownRows, slider, writeRow, type Value } from "./rows";

/** Rows another control owns: the seed box and the age dial. */
const OWNED: ReadonlySet<string> = new Set(["/skeleton/seed", "/age"]);

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

/** An exact number box, and a slider where one resolves the value. */
function Numeric({ id, value, whole, range, onChange }: {
  id: string;
  value: number | undefined;
  whole: boolean;
  range: { ends: [number, number]; notch: number } | null;
  onChange: (raw: number, text: string) => void;
}) {
  return <>
    <input id={id} type="number" value={value ?? ""} placeholder="unset" step={whole ? 1 : "any"}
      onChange={event => onChange(event.target.valueAsNumber, event.target.value)} />
    {range !== null && value !== undefined && <input type="range" aria-label={id}
      min={range.ends[0]} max={range.ends[1]} step={range.notch} value={value}
      onChange={event => onChange(Number.parseFloat(event.target.value), event.target.value)} />}
  </>;
}

/** Where a row does nothing, said beside its control. */
function Dormant({ applies }: { applies: readonly string[] }) {
  return <>{applies.filter(a => a !== "").map(a => <p className="gd-dormant" key={a}>dormant: {a}</p>)}</>;
}

/** One catalogue row: its control, its meaning and its dormancy. */
function Row({ parameter: p, value, onChange }: {
  parameter: Parameter;
  value: Value;
  onChange: (value: Value) => void;
}) {
  const id = `gd-${p.path}`;
  const range = typeof value === "number" ? slider(p, value) : null;
  return <div className="gd-dial">
    <label className="gd-label" htmlFor={id}>
      {labelOf(p.path)}
      {typeof value === "number" && <span className="gd-value">
        {range === null ? value : format(value, range.notch)} {p.unit}
      </span>}
    </label>
    {p.kind === "switch"
      ? <input id={id} type="checkbox" checked={value === true} onChange={event => onChange(event.target.checked)} />
      : <Numeric id={id} value={typeof value === "number" ? value : undefined} whole={p.kind === "count"} range={range}
          onChange={(raw, text) => {
            if (p.optional && text === "") return onChange(undefined);
            const admitted = admit(p, raw);
            if (admitted !== null) onChange(admitted);
          }} />}
    <p className="gd-meaning">{p.meaning}</p>
    <Dormant applies={[p.applies]} />
  </div>;
}

/** The oldest age the age dial reaches, in years (fn-198 host decision 5:
 *  the engine's species open at 40 with the dial to 80). The engine grows
 *  at least one year. A preset older than this keeps its own age on the
 *  dial, as a row's slider takes its value in. */
export const AGE_MAX = 80;

/** The age the tree is drawn at, in whole years: the viewer's control, not
 *  a tuning dial (the `/age` row's catalogue entry), so it stands beside
 *  the seed. Every age rebuilds through the same engine path. */
export function AgeDial({ age, onChange }: { age: number; onChange: (age: number) => void }) {
  const set = (raw: number): void => {
    if (Number.isFinite(raw)) onChange(Math.max(1, Math.round(raw)));
  };
  return <div className="gd-row">
    <label className="gd-label" htmlFor="gd-age">age <span className="gd-value">{age} years</span></label>
    <input id="gd-age" type="range" min={1} max={Math.max(AGE_MAX, age)} step={1} value={age}
      onChange={event => set(event.target.valueAsNumber)} />
  </div>;
}

/** The rows the build reads, by group. */
export function Dials({ params, setParams }: {
  params: GrowerParams;
  setParams: Dispatch<SetStateAction<GrowerParams>>;
}) {
  const groups = new Map<string, Parameter[]>();
  for (const p of shownRows()) {
    if (!OWNED.has(p.path)) groups.set(groupOf(p.path), [...groups.get(groupOf(p.path)) ?? [], p]);
  }
  return <>
    {[...groups].map(([group, rows]) => (
      <details className="gd-dials" key={group}>
        <summary>{group}</summary>
        {rows.map(p => (
          <Row key={p.path} parameter={p} value={readRow(params.family, p.path)}
            onChange={value => setParams(prev => ({ ...prev, family: writeRow(prev.family, p.path, value) }))} />
        ))}
      </details>
    ))}
  </>;
}
