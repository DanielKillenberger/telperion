"use client";

/* ------------------------------------------------------------------ *
 * THE THING THE OWNER STEERS FROM
 *
 * A canvas and a panel, and it ships before the generator is any good
 * on purpose: steering starts at the first ugly tree, not at the
 * hundredth commit. The owner is the only judge of the look, and this
 * is what they judge from.
 *
 * Everything visual belongs to the stage, and the stage is the Rust
 * renderer now. This component owns the params, the panel, and the
 * lifecycle of the stage on the canvas - nothing else. Seed and every
 * slider regenerate in place; there is no reload and no route change
 * anywhere in the loop.
 * ------------------------------------------------------------------ */

import { Fragment, useCallback, useEffect, useRef, useState } from "react";

import { PRESETS, presetById, type TreePreset } from "../src/browser/core";
import type { FrameStats, Submitted, TimingReport, View } from "../src/browser/render";

import { CANOPY_FROM_SLIDERS, familyJson, presetToParams } from "./family";
import {
  DEFAULT_PARAMS,
  SLIDERS,
  type GrowerParams,
  normalizeSeed,
  randomSeed,
  readSlider,
} from "./params";
import { createStage, type Stage } from "./rust-stage";
import "./grower-dev.css";

/** A build this cheap, in milliseconds, runs on every notch of a dial;
 *  a dearer one waits `BUILD_SETTLE_MS` after the last notch. About
 *  five frames. Branch generations down to fixed twig anatomy can
 *  take seconds, so the measured last build decides the schedule. */
const SUPERNATURAL = new Set(["torsion", "writheAmplitude", "writheWavelength", "spiralRate"]);
/** Canopy terms the generic controls leave alone: the ones a slider already
 *  carries, whose second control the next build would overwrite, and the
 *  instance budget, which is a resource limit rather than a trait.
 *  `family.test.ts` holds the slider half to what `toCanopyParams` overrides. */
const CANOPY_SKIPPED: ReadonlySet<string> = new Set([...CANOPY_FROM_SLIDERS, "maxInstances"]);
const BUILD_LIVE_MS = 80;
const BUILD_SETTLE_MS = 250;

/** A dial's value, at a precision that can tell its own steps apart -
 *  which is the STEP's business and not the value's. Keying it off the
 *  value put a single dial at two precisions on either side of 0.1. */
function format(value: number, step: number): string {
  const decimals = Math.max(0, Math.ceil(-Math.log10(step) - 1e-9));
  return value.toFixed(decimals);
}

/** Every numeric trait of one family object, as a control. The panel keeps
 *  no table of its own: a trait the core adds to a family object appears
 *  under the owner's hand without a line here, and none of them is a tag
 *  with a label instead of a control. `skip` names the terms a slider
 *  already owns, whose second control `toFamily` would overwrite on the
 *  next build. */
function Traits({ prefix, values, skip, onChange }: {
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

/** The timing session as the panel says it, and the old sweep's honesty
 *  rule kept word for word: WITHOUT A VALID VERDICT THERE IS NO
 *  MILLISECOND FIGURE IN THIS STRING. An invalid session carries no
 *  number that could be read as a pass, so there is none to print. */
function describeTiming(report: TimingReport): string {
  const who = `${report.adapter} (${report.backend})`;
  if (report.verdict !== "valid" || report.p50_ms === undefined || report.p95_ms === undefined) {
    return `${report.verdict}: ${report.reason ?? "no reason given"} - no frame time reported, on ${who}`;
  }
  return `vegetation p50 ${report.p50_ms.toFixed(3)} ms, p95 ${report.p95_ms.toFixed(3)} ms over ${report.samples} frames on ${who} - valid`;
}

export function GrowerDev() {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const stageRef = useRef<Stage | null>(null);

  const [ready, setReady] = useState(false);
  const [buildError, setBuildError] = useState<string | null>(null);
  const [loadAttempt, setLoadAttempt] = useState(0);

  /* One device per canvas, for the canvas's whole life. React's
     development double-mount builds a stage and disposes it before the
     live one, so the cleanup disposes whatever this mount created -
     including a stage that arrives after the mount is already gone. */
  useEffect(() => {
    const canvas = canvasRef.current;
    if (canvas === null) return;
    let cancelled = false;
    createStage(canvas, { onError: (message) => setBuildError(message) }).then(
      (stage) => {
        if (cancelled) { stage.dispose(); return; }
        stageRef.current = stage;
        setReady(true);
        setBuildError(null);
      },
      (error: unknown) => { if (!cancelled) setBuildError(String(error)); },
    );
    return () => {
      cancelled = true;
      stageRef.current?.dispose();
      stageRef.current = null;
      setReady(false);
    };
  }, [loadAttempt]);

  const [initialLink] = useState(() => {
    try {
      const query = new URLSearchParams(window.location.search);
      const id = query.get("species");
      const initial = id ? presetToParams(presetById(id)) : DEFAULT_PARAMS;
      const seed = query.has("seed") ? normalizeSeed(query.get("seed")!) : null;
      return { params: seed === null ? initial : { ...initial, seed }, error: null };
    } catch (error) {
      return { params: DEFAULT_PARAMS, error: String(error) };
    }
  });
  const [params, setParams] = useState<GrowerParams>(initialLink.params);
  const [linkError, setLinkError] = useState(initialLink.error);
  // The seed box is free text so a half-typed number is not thrown
  // away mid-keystroke; `params.seed` only moves when it parses.
  const [seedText, setSeedText] = useState(String(params.seed));
  /* Which view of the same tree is drawn. A view is a way of looking at
     a tree rather than a parameter of it, so it lives beside the dials
     rather than among the ones a preset would have to state. */
  const [view, setView] = useState<View>("whole");
  // What the last build cost and what went up. The surface is allowed
  // to cost more than the tube viewer did; it is not allowed to cost it
  // silently, so the panel says the numbers every time a dial moves.
  const [stats, setStats] = useState<(Submitted & { buildMs: number }) | null>(null);
  /* The last full build selects immediate or settle-after-drag updates.
     More branch generations and finer geometric internodes can cost
     seconds. A ref lets the measured cost steer the next effect
     without triggering another build itself. */
  const lastBuildMs = useRef(0);
  // What the renderer did on the last frame, polled - it changes when
  // the camera moves, which no React state does.
  const [frameStats, setFrameStats] = useState<FrameStats | null>(null);
  const [timing, setTiming] = useState<TimingReport | null>(null);
  const [measuring, setMeasuring] = useState(false);

  useEffect(() => {
    if (!ready) return;
    const build = (): void => {
      try {
        const started = performance.now();
        const submitted = stageRef.current?.setTree(familyJson(params));
        if (submitted === undefined) return;
        const buildMs = performance.now() - started;
        setBuildError(null);
        setStats({ ...submitted, buildMs });
        lastBuildMs.current = buildMs;
        stageRef.current?.frameIfWaiting();
      } catch (error) {
        /* The generator's or the renderer's own words. Nothing was
           replaced, so the tree already on the canvas stays where it is
           beside the message. */
        setBuildError(String(error));
      }
    };

    /* A cheap tree rebuilds on every notch; an expensive one waits for
       the dial to rest. The threshold is the build cost itself, which
       is the one number that says whether rebuilding per notch would
       hold the slider still. A tree at depth is still built at depth -
       this defers, it does not coarsen. */
    if (lastBuildMs.current <= BUILD_LIVE_MS) {
      build();
      return;
    }
    const handle = window.setTimeout(build, BUILD_SETTLE_MS);
    return () => { window.clearTimeout(handle); };

    /* Frame once, off the first real tree, and then never again on the
       camera's own initiative. A camera that re-frames whenever the
       tree changes size follows the height dial around the room while
       it is being dragged, which reads as the room moving rather than
       the tree growing. "Once" belongs to the stage and is latched
       there: this component outlives its own stage. */
  }, [params, ready]);

  useEffect(() => {
    if (!ready) return;
    stageRef.current?.setView(view);
  }, [view, ready]);

  /* The renderer's own numbers, four times a second. They belong to
     frames rather than to trees the panel asked for - the drawn
     triangle count moves when the view does - so nothing but a poll
     sees them. */
  useEffect(() => {
    const read = (): void => { setFrameStats(stageRef.current?.stats() ?? null); };
    read();
    const handle = window.setInterval(read, 250);
    return () => { window.clearInterval(handle); };
  }, [ready]);

  const commitSeed = useCallback((raw: string) => {
    setSeedText(raw);
    const seed = normalizeSeed(raw);
    if (seed !== null) { setParams((prev) => ({ ...prev, seed })); stageRef.current?.frameNext(); }
  }, []);

  const reroll = useCallback(() => {
    const seed = randomSeed();
    setSeedText(String(seed));
    setParams((prev) => ({ ...prev, seed }));
    stageRef.current?.frameNext();
  }, []);

  /* Loads a named tree onto the dials. Every dial, exactly: the panel
     reaches all of the preset's terms, so this puts the owner ON the
     authored tree rather than near it, with the whole parameter set
     under their hands to argue with. */
  const loadPreset = useCallback((preset: TreePreset) => {
    setParams((prev) => ({ ...presetToParams(preset), seed: prev.seed }));
    stageRef.current?.frameNext();
  }, []);

  const reset = useCallback(() => {
    setSeedText(String(DEFAULT_PARAMS.seed));
    // A fresh object rather than DEFAULT_PARAMS itself: reset from an
    // already-default state still has to rebuild and re-frame, and
    // React elides a state write that is the same reference.
    setParams({ ...DEFAULT_PARAMS });
    /* Not `frame()`. `setParams` is asynchronous and the tree on the
       canvas right now is the one being replaced; `frameNext` frames
       the tree this click actually builds. */
    stageRef.current?.frameNext();
  }, []);

  const runTiming = useCallback(async () => {
    const stage = stageRef.current;
    if (stage === null) return;
    setMeasuring(true);
    setTiming(null);
    try {
      setTiming(await stage.timing());
    } catch (error) {
      setBuildError(String(error));
    } finally {
      setMeasuring(false);
    }
  }, []);

  const seedValid = normalizeSeed(seedText) !== null;

  return (
    <div className="gd">
      <canvas className="gd-canvas" ref={canvasRef} />

      <aside className="gd-panel">
        <h1 className="gd-title">grower</h1>
        {linkError && <div role="alert" className="gd-note gd-warn">
          {linkError}. Showing the default tree; choose a preset below.
          <button className="gd-button" onClick={() => setLinkError(null)}>dismiss link error</button>
        </div>}
        {buildError && <div role="alert" className="gd-note gd-warn">
          {buildError}
          <button className="gd-button" onClick={() => setLoadAttempt(n => n + 1)}>retry build</button>
        </div>}
        {!ready && !buildError && <p role="status" className="gd-note">Starting the renderer…</p>}
        <p className="gd-note">
          clay. flat grey, neutral sky, no bloom. this is the judging mode.
          drag to walk round the tree, wheel to come in.
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
        </div>

        <div className="gd-row">
          <label htmlFor="gd-view">view</label>
          <select id="gd-view" value={view} onChange={event => setView(event.target.value as View)}>
            <option value="whole">whole tree</option>
            <option value="bare">bare branches</option>
            <option value="leaf">single leaf</option>
          </select>
        </div>
        {view === "leaf" && <p className="gd-note">One placed foliage unit with its connector, at generated scale. Empty foliage leaves an empty canvas.</p>}
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

        <div className="gd-row">
          <button className="gd-button" type="button" onClick={() => stageRef.current?.frame()}>
            reframe
          </button>
          <button className="gd-button" type="button" onClick={reset}>
            reset
          </button>
          <button
            className="gd-button"
            type="button"
            disabled={measuring || !ready}
            onClick={() => { void runTiming(); }}
          >
            {measuring ? "timing..." : "gpu timing"}
          </button>
        </div>

        <p className="gd-note">
          {stats === null
            ? "building..."
            : `${stats.woodTriangles.toLocaleString()} wood tris, ${stats.woodVertices.toLocaleString()} wood verts, ${stats.foliageInstances.toLocaleString()} foliage instances, ${stats.buildMs.toFixed(1)} ms`}
        </p>

        {/* The renderer's half, which the build cannot know: what the
            scene actually cost on the last frame, at the view standing
            now. */}
        <p className="gd-note">
          {frameStats === null
            ? "no renderer yet"
            : `${frameStats.drawCalls.toLocaleString()} scene draws, ${frameStats.triangles.toLocaleString()} tris drawn, ${frameStats.instances.toLocaleString()} instances drawn`}
        </p>

        <p className="gd-note gd-warn">
          {measuring
            ? "timing - do not resize the window"
            : timing === null
              ? "no timing yet. chrome quantizes webgpu timestamps to 100 us unless its developer features are on."
              : describeTiming(timing)}
        </p>

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
