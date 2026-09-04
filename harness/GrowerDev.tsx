"use client";

/* ------------------------------------------------------------------ *
 * THE THING THE OWNER STEERS FROM
 *
 * A canvas and a panel, and it ships before the generator is any good
 * on purpose: steering starts at the first ugly tree, not at the
 * hundredth commit. The owner is the only judge of the look, and this
 * is what they judge from.
 *
 * Everything visual belongs to stage.ts. This component owns the
 * params, the panel, and the lifecycle of the stage on the canvas -
 * nothing else. Seed and every slider regenerate in place; there is no
 * reload and no route change anywhere in the loop.
 * ------------------------------------------------------------------ */

import { useCallback, useEffect, useRef, useState } from "react";

import {
  DEFAULT_PARAMS,
  SLIDERS,
  type GrowerParams,
  normalizeSeed,
  randomSeed,
  readSlider,
} from "./params";
import { buildTree } from "./skeleton-view";
import { createStage, type Stage } from "./stage";
import "./grower-dev.css";

export function GrowerDev() {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const stageRef = useRef<Stage | null>(null);

  const [params, setParams] = useState<GrowerParams>(DEFAULT_PARAMS);
  // The seed box is free text so a half-typed number is not thrown
  // away mid-keystroke; `params.seed` only moves when it parses.
  const [seedText, setSeedText] = useState(String(DEFAULT_PARAMS.seed));
  const [lightingCheck, setLightingCheck] = useState(false);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (canvas === null) return;
    const stage = createStage(canvas);
    stageRef.current = stage;
    stage.frame(DEFAULT_PARAMS.height);
    return () => {
      stage.dispose();
      stageRef.current = null;
    };
  }, []);

  useEffect(() => {
    stageRef.current?.setTree((clay) => buildTree(params, clay));
  }, [params]);

  useEffect(() => {
    stageRef.current?.setLightingCheck(lightingCheck);
  }, [lightingCheck]);

  const commitSeed = useCallback((raw: string) => {
    setSeedText(raw);
    const seed = normalizeSeed(raw);
    if (seed !== null) setParams((prev) => ({ ...prev, seed }));
  }, []);

  const reroll = useCallback(() => {
    const seed = randomSeed();
    setSeedText(String(seed));
    setParams((prev) => ({ ...prev, seed }));
  }, []);

  const reset = useCallback(() => {
    setSeedText(String(DEFAULT_PARAMS.seed));
    setParams(DEFAULT_PARAMS);
    stageRef.current?.frame(DEFAULT_PARAMS.height);
  }, []);

  const seedValid = normalizeSeed(seedText) !== null;

  return (
    <div className="gd">
      <canvas className="gd-canvas" ref={canvasRef} />

      <aside className="gd-panel">
        <h1 className="gd-title">grower</h1>
        <p className="gd-note">
          clay. flat grey, neutral sky, no bloom. this is the judging mode.
        </p>

        <div className="gd-row">
          <label className="gd-label" htmlFor="gd-seed">
            seed
          </label>
          <input
            id="gd-seed"
            className={seedValid ? "gd-seed" : "gd-seed gd-seed-bad"}
            type="text"
            inputMode="numeric"
            value={seedText}
            onChange={(event) => commitSeed(event.target.value)}
            aria-invalid={!seedValid}
          />
          <button className="gd-button" type="button" onClick={reroll}>
            reroll
          </button>
        </div>

        {SLIDERS.map((spec) => (
          <div className="gd-slider" key={spec.key}>
            <label className="gd-label" htmlFor={`gd-${spec.key}`}>
              {spec.label}
            </label>
            <span className="gd-value">
              {params[spec.key].toFixed(2)}
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
        ))}

        <div className="gd-row">
          <label className="gd-check">
            <input
              type="checkbox"
              checked={lightingCheck}
              onChange={(event) => setLightingCheck(event.target.checked)}
            />
            lighting check
          </label>
          <button
            className="gd-button"
            type="button"
            onClick={() => stageRef.current?.frame(params.height)}
          >
            reframe
          </button>
          <button className="gd-button" type="button" onClick={reset}>
            reset
          </button>
        </div>

        <p className="gd-note gd-warn">
          structure and thickness: branches are round tubes, a viewer for
          the radius solve. taper is its fork exponent - 2 conserves
          cross-sectional area. the real surface, non-circular and
          rotating with a root flare, lands in fn-11.5.
        </p>
      </aside>
    </div>
  );
}
