"use client";

/* ------------------------------------------------------------------ *
 * THE DIALS, AS CONTROLS
 *
 * Every control the panel offers over a family: the generic numeric
 * rows a trait table renders itself as, and the named sliders. Split
 * out of the panel component because the panel owns the canvas, the
 * stage's lifecycle and the params, and a file that owns those has no
 * room left for a hundred lines of form.
 *
 * Nothing here knows what will draw the tree. It takes the params and
 * hands back the params changed, which is the whole contract.
 * ------------------------------------------------------------------ */

import { Fragment, type Dispatch, type SetStateAction } from "react";

import { CANOPY_FROM_SLIDERS } from "./family";
import { SLIDERS, type GrowerParams, readSlider } from "./params";

/** The supernatural terms, which sit under their own legend. */
const SUPERNATURAL = new Set(["torsion", "writheAmplitude", "writheWavelength", "spiralRate"]);

/** Canopy terms the generic controls leave alone: the ones a slider already
 *  carries, whose second control the next build would overwrite, and the
 *  instance budget, which is a resource limit rather than a trait.
 *  `family.test.ts` holds the slider half to what `toCanopyParams` overrides. */
const CANOPY_SKIPPED: ReadonlySet<string> = new Set([...CANOPY_FROM_SLIDERS, "maxInstances"]);

/** A dial's value, at a precision that can tell its own steps apart -
 *  which is the STEP's business and not the value's. Keying it off the
 *  value put a single dial at two precisions on either side of 0.1. */
function format(value: number, step: number): string {
  const decimals = Math.max(0, Math.ceil(-Math.log10(step) - 1e-9));
  return value.toFixed(decimals);
}

/** Every numeric trait of one row object, as a control. The panel keeps
 *  no table of its own: a trait the core adds to a family object - or a
 *  field the renderer adds to its scene row - appears under the owner's
 *  hand without a line here, and none of them is a tag with a label
 *  instead of a control. `skip` names the terms a slider already owns,
 *  whose second control `toFamily` would overwrite on the next build. */
export function Traits({ prefix, values, skip, onChange }: {
  prefix?: string;
  values: Record<string, unknown>;
  skip?: ReadonlySet<string>;
  onChange: (key: string, value: number) => void;
}) {
  return <>
    {Object.entries(values)
      .filter(([key, value]) => typeof value === "number" && skip?.has(key) !== true)
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

/** The two fieldsets the owner steers a family from: every botanical
 *  trait as a row or a slider, and the supernatural terms under their
 *  own legend with the switch that turns them on. */
export function Dials({ params, setParams }: {
  params: GrowerParams;
  setParams: Dispatch<SetStateAction<GrowerParams>>;
}) {
  return <>
    {[false, true].map(supernatural => (
      <fieldset key={String(supernatural)}>
        <legend>{supernatural ? "Supernatural" : "Botanical"}</legend>
        {supernatural ? <label className="gd-check">
          <input type="checkbox" checked={params.supernaturalEnabled}
            onChange={event => setParams(prev => ({ ...prev, supernaturalEnabled: event.target.checked }))} />
          enable supernatural effects
        </label> : <>
          <Traits values={params.family.skeleton.habit} onChange={(key, number) => setParams(prev => ({ ...prev, family: { ...prev.family,
            skeleton: { ...prev.family.skeleton, habit: { ...prev.family.skeleton.habit, [key]: number } } } }))} />
          <Traits prefix="foliage" values={params.family.element} onChange={(key, number) => setParams(prev => ({ ...prev, family: { ...prev.family,
            element: { ...prev.family.element, [key]: number } } }))} />
          <Traits prefix="leaf" values={params.family.canopy} skip={CANOPY_SKIPPED} onChange={(key, number) => setParams(prev => ({ ...prev, family: { ...prev.family,
            canopy: { ...prev.family.canopy, [key]: number } } }))} />
          <Traits prefix="material" values={params.family.material} onChange={(key, number) => setParams(prev => ({ ...prev, family: { ...prev.family,
            material: { ...prev.family.material, [key]: number } } }))} />
        </>}
        {SLIDERS.filter(spec => SUPERNATURAL.has(spec.key) === supernatural).map((spec) => (
          <Fragment key={spec.key}>
            {spec.group === undefined ? null : (
              <h3 className="gd-group">{spec.group}</h3>
            )}
            <div className="gd-slider">
              <label className="gd-label" htmlFor={`gd-${spec.key}`}>
                {spec.label}
              </label>
              <span className="gd-value">
                {format(params[spec.key], spec.step)}
                {spec.unit}
              </span>
              <input
                id={`gd-${spec.key}`}
                type="range"
                min={spec.min}
                max={spec.max}
                step={spec.step}
                value={params[spec.key]}
                onChange={(event) =>
                  setParams((prev) => ({
                    ...prev,
                    [spec.key]: readSlider(spec, event.target.value),
                  }))
                }
              />
            </div>
          </Fragment>
        ))}
      </fieldset>
    ))}
  </>;
}
