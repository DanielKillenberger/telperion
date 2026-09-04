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
import { buildTree, type TreeStats } from "./skeleton-view";
import { createStage, type Stage } from "./stage";
import "./grower-dev.css";

/** A dial's value, at a precision that can tell its own steps apart -
 *  which is the STEP's business and not the value's. Keying it off the
 *  value put a single dial at two precisions on either side of 0.1:
 *  the trunk dial, whose step is 0.001, read `0.050` and then `0.15`
 *  on two adjacent notches, as if it had skipped a hundred of them. */
function format(value: number, step: number): string {
  const decimals = Math.max(0, Math.ceil(-Math.log10(step) - 1e-9));
  return value.toFixed(decimals);
}

export function GrowerDev() {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const stageRef = useRef<Stage | null>(null);

  const [params, setParams] = useState<GrowerParams>(DEFAULT_PARAMS);
  // The seed box is free text so a half-typed number is not thrown
  // away mid-keystroke; `params.seed` only moves when it parses.
  const [seedText, setSeedText] = useState(String(DEFAULT_PARAMS.seed));
  const [lightingCheck, setLightingCheck] = useState(false);
  // What the last build cost. The surface is allowed to cost more than
  // the tube viewer did; it is not allowed to cost it silently, so the
  // panel says the number every time a dial moves.
  const [stats, setStats] = useState<TreeStats | null>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (canvas === null) return;
    const stage = createStage(canvas);
    stageRef.current = stage;
    return () => {
      stage.dispose();
      stageRef.current = null;
    };
  }, []);

  useEffect(() => {
    // `setTree` calls its builder synchronously, so the stats are in
    // hand by the time it returns.
    let built: TreeStats | null = null;
    stageRef.current?.setTree((clay) => {
      const result = buildTree(params, clay);
      built = result.stats;
      return result.tree;
    });
    setStats(built);

    /* Frame once, off the first real tree, and then never again on the
       camera's own initiative. A camera that re-frames whenever the
       tree changes size follows the height dial around the scene while
       it is being dragged, which reads as the room moving rather than
       the tree growing - the owner's word for it was disorienting. So
       the camera moves when it is asked to and at no other time:
       reframe, reset, or the mouse.

       "Once" belongs to the stage and is latched there. This component
       outlives its own stage - React's development double-mount builds
       one, disposes it and builds a second - so a latch held here
       would be spent on a stage that no longer exists and would leave
       the live one unframed. */
    stageRef.current?.frameIfWaiting(params.height);
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
    // A fresh object rather than DEFAULT_PARAMS itself: reset from an
    // already-default state still has to rebuild and re-frame, and
    // React elides a state write that is the same reference.
    setParams({ ...DEFAULT_PARAMS });
    /* Not `frame()`. `setParams` is asynchronous and the tree standing
       on the stage right now is the one being replaced - framing it
       measures the outgoing subject, which at 400 m parks the camera
       758 m out looking at y=200 and then installs a 24 m tree at the
       origin. `frameNext` frames the tree this click actually builds,
       which is what makes reset at least as good as reframe at
       recovering a lost view rather than strictly worse. */
    stageRef.current?.frameNext();
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

        <p className="gd-note">
          {stats === null
            ? "building..."
            : `${stats.triangles.toLocaleString()} tris, ${stats.vertices.toLocaleString()} verts, ${stats.nodes.toLocaleString()} nodes, ${stats.buildMs.toFixed(1)} ms`}
        </p>

        <p className="gd-note gd-warn">
          one swept surface, lobed section winding along its own length,
          flared into the ground. spiral bends the centreline; surface
          twist winds the skin. no foliage yet - fn-11.6 emits the
          frames, the consumer places what goes on them.
        </p>
      </aside>
    </div>
  );
}
