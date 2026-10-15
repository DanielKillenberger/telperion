"use client";

/* ------------------------------------------------------------------ *
 * ONE ENGINE SETTING, AS A CONTROL
 *
 * A slider moves the setting at every age the species states, from the
 * preset's values on the setting's own scale (`space.ts`); it opens to
 * one box per age, and per zone, for finer work.
 * ------------------------------------------------------------------ */

import { useEffect, useState, type Dispatch, type SetStateAction } from "react";

import type { SpaceSetting } from "../src/browser/parameters.generated";
import type { GrowerParams } from "./params";
import { admits, moveAll, places, range, readSetting, writeSetting, type Place } from "./space";

function where(at: Place): string {
  return at.zone === undefined ? `age ${at.age}` : `age ${at.age}, zone ${at.zone}`;
}

/** The move a whole-setting slider reads: a factor, or an offset. */
function describe(s: SpaceSetting, by: number): string {
  return s.scale === "log" ? `×${by.toFixed(2)}` : `${by >= 0 ? "+" : ""}${by.toPrecision(3)}`;
}

/** One box per place the species states the setting at. */
function PerAge({ s, params, setParams }: {
  s: SpaceSetting;
  params: GrowerParams;
  setParams: Dispatch<SetStateAction<GrowerParams>>;
}) {
  return <details className="gd-ages">
    <summary>by age</summary>
    {places(params.family, s).map(at => {
      const value = readSetting(params.family, s, at);
      const id = `gd-space-${s.name}-${at.age}-${at.zone ?? ""}`;
      return <label className="gd-row" key={id} htmlFor={id}>
        {where(at)}
        <input id={id} type="number" step="any" placeholder={value === Infinity ? "never" : ""}
          value={Number.isFinite(value) ? value : ""}
          onChange={event => {
            const v = event.target.value === "" && s.never ? Infinity : event.target.valueAsNumber;
            if (admits(s, v)) setParams(prev => ({ ...prev, family: writeSetting(prev.family, s, at, v) }));
          }} />
      </label>;
    })}
  </details>;
}

/** An engine setting: a slider over every stated age, and the ages
 *  beneath it. */
export function SpaceDial({ s, params, setParams }: {
  s: SpaceSetting;
  params: GrowerParams;
  setParams: Dispatch<SetStateAction<GrowerParams>>;
}) {
  const r = range(s);
  const [by, setBy] = useState(r.neutral);
  useEffect(() => { setBy(r.neutral); }, [params.preset, r.neutral]);
  const id = `gd-space-${s.name}`;
  const slide = (raw: number): void => {
    const next = s.scale === "log" ? 2 ** raw : raw;
    if (!Number.isFinite(next)) return;
    setBy(next);
    setParams(prev => ({ ...prev, family: moveAll(prev.preset, prev.family, s, next) }));
  };
  const position = s.scale === "log" ? Math.log2(by) : by;
  return <div className="gd-dial">
    <label className="gd-label" htmlFor={id}>
      {s.label}<span className="gd-value">{describe(s, by)} {s.unit}</span>
    </label>
    <input id={id} type="range" aria-label={s.label}
      min={s.scale === "log" ? Math.log2(r.min) : r.min}
      max={s.scale === "log" ? Math.log2(r.max) : r.max}
      step={s.scale === "log" ? 0.01 : r.step} value={position}
      onChange={event => slide(Number.parseFloat(event.target.value))} />
    <p className="gd-meaning">{s.meaning}</p>
    <PerAge s={s} params={params} setParams={setParams} />
  </div>;
}
