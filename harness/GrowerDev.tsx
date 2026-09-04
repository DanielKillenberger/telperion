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

import { Fragment, useCallback, useEffect, useRef, useState } from "react";

import { PRESETS, type TreePreset } from "../src/presets";

import {
  DEFAULT_PARAMS,
  SLIDERS,
  type GrowerParams,
  normalizeSeed,
  randomSeed,
  readSlider,
} from "./params";
import {
  buildComparison,
  buildTree,
  presetToParams,
  type TreeStats,
} from "./skeleton-view";
import {
  createStage,
  describeSweep,
  SWEEP_RATIOS,
  type FrameStats,
  type Stage,
  type SweepResult,
} from "./stage";
import "./grower-dev.css";

/** A dial's value, at a precision that can tell its own steps apart -
 *  which is the STEP's business and not the value's. Keying it off the
 *  value put a single dial at two precisions on either side of 0.1:
 *  the trunk dial, whose step is 0.001, read `0.050` and then `0.15`
 *  on two adjacent notches, as if it had skipped a hundred of them. */
/** A build this cheap, in milliseconds, runs on every notch of a dial;
 *  a dearer one waits `BUILD_SETTLE_MS` after the last notch. About
 *  five frames: the default tree builds inside it and the deep ones -
 *  a second or two at eight twig orders - do not. */
const BUILD_LIVE_MS = 80;
const BUILD_SETTLE_MS = 250;

function format(value: number, step: number): string {
  const decimals = Math.max(0, Math.ceil(-Math.log10(step) - 1e-9));
  return value.toFixed(decimals);
}

/** What the scale figure is standing next to when the comparison is
 *  up: the taller of the trees on the ground. The stage frames the
 *  subject it can measure and uses this only to place the figure, so
 *  the tallest is the right answer - it is the one whose foot the
 *  figure has to look small beside. */
function tallestPresetHeight(): number {
  return Math.max(
    ...PRESETS.map((preset) => preset.skeleton.envelope.height),
  );
}

export function GrowerDev() {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const stageRef = useRef<Stage | null>(null);

  const [params, setParams] = useState<GrowerParams>(DEFAULT_PARAMS);
  // The seed box is free text so a half-typed number is not thrown
  // away mid-keystroke; `params.seed` only moves when it parses.
  const [seedText, setSeedText] = useState(String(DEFAULT_PARAMS.seed));
  const [lightingCheck, setLightingCheck] = useState(false);
  /* The spec's acceptance test: both presets on the ground together,
     built from the library's own objects rather than from the dials,
     so what stands there is what the preset file says. The dials keep
     their own tree and get it back the moment this goes off. */
  const [compare, setCompare] = useState(false);
  // What the last build cost. The surface is allowed to cost more than
  // the tube viewer did; it is not allowed to cost it silently, so the
  // panel says the number every time a dial moves.
  const [stats, setStats] = useState<TreeStats | null>(null);
  /* What the last build cost, for deciding whether the next one may
     run on the dial's every notch. Measured at eight twig orders a
     build is one second on Telperion and two on Laurelin, and a build
     that runs on every input event holds the slider still for that
     long per notch; the default tree builds in a tenth of that and
     wants no delay at all. A ref, not state: the number steers the
     effect and must not re-run it. */
  const lastBuildMs = useRef(0);
  /* The flag under suspicion. It is a renderer CONSTRUCTION flag, so
     turning it over is not a setter: it builds a new renderer, and a
     WebGL canvas hands out one context for its whole life, so the
     canvas goes with it. Hence the key on the element below - React
     puts a fresh canvas in the DOM and the stage effect builds the
     stage on it. Expensive, and a dev harness measuring itself can
     afford it once per click. */
  const [logDepth, setLogDepth] = useState(true);
  /* What the renderer is told to draw at, or null for the harness
     default of min(dpr, 2). Not a GrowerParams member and deliberately
     not: it is a property of the picture's resolution, not of the tree,
     and a preset that carried one would be authoring a frame rate. */
  const [pixelRatio, setPixelRatio] = useState<number | null>(null);
  // What the renderer did on the last frame, polled - it changes when
  // the camera moves, which no React state does.
  const [frameStats, setFrameStats] = useState<FrameStats | null>(null);
  const [sweep, setSweep] = useState<SweepResult | null>(null);
  const [sweeping, setSweeping] = useState(false);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (canvas === null) return;
    const stage = createStage(canvas, { logarithmicDepthBuffer: logDepth });
    stageRef.current = stage;
    return () => {
      stage.dispose();
      stageRef.current = null;
    };
  }, [logDepth]);

  useEffect(() => {
    const build = (): void => {
      // `setTree` calls its builder synchronously, so the stats are in
      // hand by the time it returns.
      const built: { stats: TreeStats | null } = { stats: null };
      stageRef.current?.setTree((clay) => {
        if (compare) {
          const result = buildComparison(PRESETS, clay);
          built.stats = result.stats;
          return result.group;
        }
        const result = buildTree(params, clay);
        built.stats = result.stats;
        return result.tree;
      });
      setStats(built.stats);
      lastBuildMs.current = built.stats?.buildMs ?? 0;
      stageRef.current?.frameIfWaiting(
        compare ? tallestPresetHeight() : params.height,
      );
    };

    /* A cheap tree rebuilds on every notch; an expensive one waits for
       the dial to rest. The threshold is the build cost itself, which
       is the one number that says whether rebuilding per notch would
       hold the slider still: under it the drag reads as live, over it
       the build runs once the events stop arriving. A tree at depth
       is still built at depth - this defers, it does not coarsen. */
    if (lastBuildMs.current <= BUILD_LIVE_MS) {
      build();
    } else {
      const handle = window.setTimeout(build, BUILD_SETTLE_MS);
      return () => {
        window.clearTimeout(handle);
      };
    }

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
    /* `logDepth` is in the deps of this effect and of every other one
       that configures the stage, because turning it over replaces the
       stage: a fresh one has no subject, no lighting mode and no pinned
       resolution until it is told again. The stage effect is declared
       above this one, so on the commit where the flag changes React
       runs it first and this rebuilds onto the stage that now exists.
       Every read of "what is on the stage" is a call site of that
       replacement, and the ones outside the effect that builds it are
       the ones that get missed. */
  }, [params, compare, logDepth]);

  useEffect(() => {
    stageRef.current?.setLightingCheck(lightingCheck);
  }, [lightingCheck, logDepth]);

  useEffect(() => {
    stageRef.current?.setPixelRatio(pixelRatio);
  }, [pixelRatio, logDepth]);

  /* The renderer's own numbers, four times a second. They belong to
     frames rather than to renders the panel asked for - the draw count
     moves when the camera moves and the pixel ratio moves when the
     display does - so nothing but a poll sees them. */
  useEffect(() => {
    const read = (): void => {
      setFrameStats(stageRef.current?.stats() ?? null);
    };
    read();
    const handle = window.setInterval(read, 250);
    return () => {
      window.clearInterval(handle);
    };
  }, [logDepth]);

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

  /* Loads a named tree onto the dials. Every dial, exactly: the panel
     reaches all of the preset's terms, so this puts the owner ON the
     authored tree rather than near it, with the whole parameter set
     under their hands to argue with. Leaves the comparison, because
     picking a tree to steer is a request to look at that one. */
  const loadPreset = useCallback((preset: TreePreset) => {
    const loaded = presetToParams(preset);
    setSeedText(String(loaded.seed));
    setParams(loaded);
    setCompare(false);
    stageRef.current?.frameNext();
  }, []);

  const toggleCompare = useCallback(() => {
    setCompare((prev) => !prev);
    // The subject is about to change size by a factor of three, so the
    // camera has to move; `frameNext` frames whatever the effect
    // builds rather than the tree standing there now.
    stageRef.current?.frameNext();
  }, []);

  const reset = useCallback(() => {
    setSeedText(String(DEFAULT_PARAMS.seed));
    setCompare(false);
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

  /* The measurement. It drives the applied pixel ratio through the
     four points itself and restores whatever was pinned before, so the
     only thing the owner has to bring is vsync off - which is a browser
     flag and cannot be asserted from in here. */
  const runSweep = useCallback(async () => {
    const stage = stageRef.current;
    if (stage === null) return;
    setSweeping(true);
    setSweep(null);
    try {
      setSweep(await stage.sweep());
    } finally {
      setSweeping(false);
    }
  }, []);

  const seedValid = normalizeSeed(seedText) !== null;

  return (
    <div className="gd">
      {/* Keyed on the depth-buffer flag: a WebGL canvas hands out one
          context for its whole life, so a renderer built the other way
          needs a canvas of its own. React replacing the element is what
          makes the flag togglable at all. */}
      <canvas
        className="gd-canvas"
        key={logDepth ? "log-depth" : "linear-depth"}
        ref={canvasRef}
      />

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

        <div className="gd-row">
          {PRESETS.map((preset) => (
            <button
              className="gd-button"
              type="button"
              key={preset.id}
              title={preset.note}
              onClick={() => loadPreset(preset)}
            >
              {preset.name.toLowerCase()}
            </button>
          ))}
          <button
            className={compare ? "gd-button gd-button-on" : "gd-button"}
            type="button"
            aria-pressed={compare}
            onClick={toggleCompare}
          >
            both
          </button>
        </div>

        {SLIDERS.map((spec) => (
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
            onClick={() =>
              stageRef.current?.frame(
                // The subject's height, not a dial's: while the
                // comparison is up the dials are not what is standing
                // there, and `height` is what places the scale figure.
                compare ? tallestPresetHeight() : params.height,
              )
            }
          >
            reframe
          </button>
          <button className="gd-button" type="button" onClick={reset}>
            reset
          </button>
        </div>

        <div className="gd-row">
          <label className="gd-check">
            <input
              type="checkbox"
              checked={logDepth}
              onChange={(event) => setLogDepth(event.target.checked)}
            />
            log depth
          </label>
          <button
            className="gd-button"
            type="button"
            disabled={sweeping}
            onClick={() => {
              void runSweep();
            }}
          >
            {sweeping ? "sweeping..." : "sweep"}
          </button>
        </div>

        {/* The applied pixel ratio, by hand. `auto` is what the harness
            has always drawn at - min(dpr, 2) - and the other four are
            the sweep's own points, so the owner can sit at one of them
            and look at the tree rather than only read a number. */}
        <div className="gd-row">
          <button
            className={pixelRatio === null ? "gd-button gd-button-on" : "gd-button"}
            type="button"
            aria-pressed={pixelRatio === null}
            onClick={() => setPixelRatio(null)}
          >
            dpr auto
          </button>
          {SWEEP_RATIOS.map((ratio) => (
            <button
              className={
                pixelRatio === ratio ? "gd-button gd-button-on" : "gd-button"
              }
              type="button"
              key={ratio}
              aria-pressed={pixelRatio === ratio}
              onClick={() => setPixelRatio(ratio)}
            >
              {ratio.toFixed(2)}
            </button>
          ))}
        </div>

        <p className="gd-note">
          {stats === null
            ? "building..."
            : `${stats.triangles.toLocaleString()} tris, ${stats.vertices.toLocaleString()} verts, ${stats.nodes.toLocaleString()} nodes, ${stats.drawCalls.toLocaleString()} draws, ${stats.instances.toLocaleString()} leaves, ${stats.buildMs.toFixed(1)} ms`}
        </p>

        {/* The ceiling, in words. A capped tree is the ceiling's shape
            and not the envelope's, and a node count alone cannot say
            which it was - so the stop is never silent. */}
        {stats?.capped ? (
          <p className="gd-note gd-warn">
            node ceiling reached: growth was stopped, not finished. this
            tree is the ceiling&apos;s shape, not the envelope&apos;s - raise
            the step or lower the orders.
          </p>
        ) : null}

        {/* The renderer's half, which the build cannot know: what the
            scene actually cost last frame, at what resolution, and how
            the two suspect settings are standing. `dpr` is reported
            raw and applied because the gap between them is the largest
            single fill term in this room. */}
        <p className="gd-note">
          {frameStats === null
            ? "no renderer yet"
            : `${frameStats.drawCalls.toLocaleString()} scene draws, ${frameStats.triangles.toLocaleString()} tris drawn, dpr ${frameStats.pixelRatioRaw.toFixed(2)} raw / ${frameStats.pixelRatioApplied.toFixed(2)} applied, log depth ${frameStats.logarithmicDepthBuffer ? "on" : "off"}, gpu timer ${frameStats.gpuTimer ? "available" : "unavailable"}`}
        </p>

        <p className="gd-note gd-warn">
          {sweeping
            ? "sweeping 1.00 / 0.70 / 0.50 / 0.25 - do not resize the window"
            : sweep === null
              ? "no sweep yet. run it with vsync off: google-chrome --disable-gpu-vsync --disable-frame-rate-limit"
              : describeSweep(sweep)}
        </p>

        {compare ? (
          <p className="gd-note gd-warn">
            both presets, as authored in the library. the dials below are
            not what is standing there - pick telperion or laurelin to
            put one of them under the dials.
          </p>
        ) : null}

        <p className="gd-note gd-warn">
          one swept surface, lobed section winding along its own length,
          flared into the ground. spiral bends the centreline; surface
          twist winds the skin. foliage grows on the young wood at the
          end of every shoot and is culled to a shell, so what you are
          looking at is the outside of the canopy and not its filling.
        </p>
      </aside>
    </div>
  );
}
