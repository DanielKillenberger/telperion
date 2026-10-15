"use client";

/* ------------------------------------------------------------------ *
 * THE DIALS, AS CONTROLS
 *
 * One control per catalogue row the build reads and per engine setting
 * the species states, grouped by what it does to the tree (`panel.ts`),
 * each under a plain label with its unit, its meaning and, where it has
 * one, the condition under which it does nothing. A control that
 * differs from the preset says so, and goes back with its own reset, its
 * group's, or the tree's. Split out of the panel component because the
 * panel owns the canvas, the stage's lifecycle and the params.
 *
 * Nothing here knows what will draw the tree. It takes the params and
 * hands back the params changed, which is the whole contract.
 * ------------------------------------------------------------------ */

import { useState, type Dispatch, type SetStateAction } from "react";

import type { Parameter } from "../src/browser/parameters.generated";
import { Changed } from "./changed";
import { changed, controls, matches, reset, resetAll, type Control } from "./panel";
import type { GrowerParams } from "./params";
import { admit, readRow, slider, writeRow, type Value } from "./rows";
import { SpaceDial } from "./space-dials";

type SetParams = Dispatch<SetStateAction<GrowerParams>>;

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

/** One catalogue row: its control, its meaning and its dormancy. */
function Row({ parameter: p, value, isChanged, onChange, onReset }: {
  parameter: Parameter;
  value: Value;
  isChanged: boolean;
  onChange: (value: Value) => void;
  onReset: () => void;
}) {
  const id = `gd-${p.path}`;
  const range = typeof value === "number" ? slider(p, value) : null;
  return <div className={isChanged ? "gd-dial gd-changed" : "gd-dial"}>
    <label className="gd-label" htmlFor={id}>
      {p.label}
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
    {isChanged && <Changed label={p.label} onReset={onReset} />}
    <p className="gd-meaning">{p.meaning}</p>
    {p.applies !== "" && <p className="gd-dormant">dormant: {p.applies}</p>}
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

/** One control, as its kind draws it. */
function Dial({ c, params, setParams }: { c: Control; params: GrowerParams; setParams: SetParams }) {
  const isChanged = changed(c, params.family, params.preset);
  const onReset = (): void => setParams(prev => ({ ...prev, family: reset(c, prev.family, prev.preset) }));
  if (c.kind === "space") {
    return <SpaceDial s={c.setting} params={params} setParams={setParams} isChanged={isChanged} onReset={onReset} />;
  }
  const path = c.row.path;
  return <Row parameter={c.row} value={readRow(params.family, path)} isChanged={isChanged} onReset={onReset}
    onChange={value => setParams(prev => ({ ...prev, family: writeRow(prev.family, path, value) }))} />;
}

/** The groups a closed panel opens on: the most telling. */
const OPEN: ReadonlySet<string> = new Set(["size", "form"]);

/** Every control, by what it does, with a filter and the tree's reset. */
export function Dials({ params, setParams }: { params: GrowerParams; setParams: SetParams }) {
  const [filter, setFilter] = useState("");
  const groups = controls(params.preset);
  const anyChanged = groups.some(g => g.controls.some(c => changed(c, params.family, params.preset)));
  const supernatural = params.family.supernatural.enabled;
  return <>
    <div className="gd-row">
      <input className="gd-filter" type="search" placeholder="find a control" aria-label="find a control"
        value={filter} onChange={event => setFilter(event.target.value)} />
      <button className="gd-button" type="button" disabled={!anyChanged}
        onClick={() => setParams(prev => ({ ...prev, family: structuredClone(prev.preset) }))}>
        reset tree
      </button>
    </div>
    {groups.map(({ group, title, controls: all }) => {
      const shown = all.filter(c => matches(c, filter));
      if (shown.length === 0) return null;
      const changes = all.filter(c => changed(c, params.family, params.preset));
      const open = filter !== "" || OPEN.has(group);
      return <details className="gd-dials" key={`${group}-${open}`} open={open}>
        <summary>
          {title}
          {group === "supernatural" && !supernatural && <span className="gd-dormant"> dormant: off for this tree</span>}
          {changes.length > 0 && <span className="gd-value"> · {changes.length} changed</span>}
        </summary>
        {changes.length > 0 && <button className="gd-reset" type="button"
          onClick={() => setParams(prev => ({ ...prev, family: resetAll(changes, prev.family, prev.preset) }))}>
          reset {title}
        </button>}
        {shown.map(c => <Dial key={c.id} c={c} params={params} setParams={setParams} />)}
      </details>;
    })}
  </>;
}
