import type { Family } from "../src/browser/presets";
import type { Growth, Submitted } from "../src/browser/render";
import { wire } from "../src/browser/wire";

/* ------------------------------------------------------------------ *
 * THE TREE GROWS IN (fn-270 R6; host rulings, 2026-10-14)
 *
 * Every change of the params builds the grown tree at once (owner,
 * 2026-10-14: growing from year 1 "shouldn't be the default mode when you
 * open the harness a new species"); "grow in" grows it from year 1,
 * a whole year a draw; "play the year" steps the grown tree
 * through its last year, budburst to full leaf, autumn, fall and winter,
 * ending at the whole age. Each year grows and is drawn on the CPU, on
 * the page's thread (fn-353): a drawn year stays up for the floor, which
 * lets the canvas paint it, before the next one grows, and a newer request
 * abandons a run between its draws.
 * ------------------------------------------------------------------ */

/** The least time a drawn year stays up, so a cheap tree does not flash
 *  past (host, 2026-10-14), and the canvas paints it before the next
 *  year's growth holds the thread (fn-353). */
export const FLOOR_MS = 150;

/** How many draws "play the year" takes through the last year: a month
 *  each, the host's "4 or more". */
export const DRAWS_A_YEAR = 12;

/** What a request asks of the stage: a tree built at once, grown in from
 *  year 1, or its last year played. */
export type Kind = "build" | "grow" | "year";

/** The ages a grow-in draws: every whole year from the first to the
 *  horizon. The seedling stands only from its first budburst, near the
 *  end of year 1, so the first draw is year 1's whole age. */
export function growInAges(horizon: number): number[] {
  return Array.from({ length: horizon }, (_, k) => k + 1);
}

/** The ages "play the year" draws: `DRAWS_A_YEAR` steps through the last
 *  year, the last at the whole age. */
export function yearAges(horizon: number): number[] {
  // A seedling has no wood before its first budburst: its year is its whole age.
  if (horizon === 1) return [1];
  return Array.from({ length: DRAWS_A_YEAR }, (_, k) => horizon - 1 + (k + 1) / DRAWS_A_YEAR);
}

export interface Pace {
  /** Whether the run is still the one the panel asked for. */
  current(): boolean;
  /** Told each tree as it goes up, with the age it was drawn at. */
  drawn(at: number, submitted: Submitted): void;
  wait(ms: number): Promise<void>;
}

/** Grows `growth` on to each age and draws it there in turn: each tree
 *  stays up for `FLOOR_MS` from its draw before the next year grows.
 *  Stops between draws once the run is not current, and resolves to the
 *  last tree put up, or null when it stopped first. */
export async function drawEach(growth: Growth, ages: number[], pace: Pace): Promise<Submitted | null> {
  let last: Submitted | null = null;
  for (const at of ages) {
    if (last !== null) await pace.wait(FLOOR_MS);
    if (!pace.current()) return null;
    growth.growTo(at);
    last = growth.draw(at);
    pace.drawn(at, last);
  }
  return last;
}

/** The age a label shows: whole years whole, within the year to the
 *  month. */
export function ageLabel(at: number, horizon: number): string {
  const shown = Number.isInteger(at) ? String(at) : at.toFixed(2);
  return `year ${shown} of ${horizon}`;
}

/** The growing tree the stage holds between requests, for "play the
 *  year" to go on from, and the parameters it grows, as their wire text:
 *  two families are the same tree when their text is. */
export interface Held {
  growth: Growth | null;
  family: string | null;
}

/** Lets the held growth go. */
export function release(held: Held): void {
  held.growth?.dispose();
  held.growth = null;
  held.family = null;
}

/** What a request draws, on whatever builds and grows trees. */
export interface Grower {
  setTree(family: Family): Submitted;
  beginGrowth(family: Family): Growth;
}

/** One request: the tree built at once, letting any growth go; or grown
 *  in from year 1; or its last year played, on the held growth when it
 *  is this tree's and grown whole, else on a new one. Any new growth
 *  abandons the one held. */
export async function draw(grower: Grower, held: Held, kind: Kind, family: Family, pace: Pace): Promise<Submitted | null> {
  if (kind === "build") {
    release(held);
    return grower.setTree(family);
  }
  const whole = held.growth !== null && held.growth.committed() === held.growth.horizon();
  const key = wire(family);
  if (kind === "grow" || held.family !== key || !whole) {
    release(held);
    held.growth = grower.beginGrowth(family);
    held.family = key;
  }
  const growth = held.growth!;
  const horizon = growth.horizon();
  return drawEach(growth, kind === "grow" ? growInAges(horizon) : yearAges(horizon), pace);
}

/** A request to the stage: these params, built, grown in or their year
 *  played. */
export interface Job<P> {
  params: P & { family: { age: number } };
  kind: Kind;
}

/** What the panel hears of a run: the year drawn (null once it ends),
 *  and a build's cost. */
export interface Hooks {
  growing(label: string | null): void;
  built(ms: number): void;
}

/** The stage, as a run asks it. */
export interface Stage extends Grower {
  waiting(): boolean;
  frame(): void;
}

/** The stage's run of one request (`latestBuild`'s): `encode` makes the
 *  params the family the renderer takes. A stage still waiting to be framed
 *  follows the tree as it grows. A run a newer request abandoned rejects,
 *  and `latestBuild` reports nothing of it. */
export function stageRun<P>(stage: Stage, held: Held, encode: (params: Job<P>["params"]) => Family, hooks: Hooks) {
  return async ({ params, kind }: Job<P>, current: () => boolean): Promise<Submitted & { buildMs: number }> => {
    const family = encode(params);
    const follow = stage.waiting();
    try {
      const started = performance.now();
      const submitted = await draw(stage, held, kind, family, {
        current,
        wait: (ms) => new Promise((resolve) => { setTimeout(resolve, ms); }),
        drawn: (at) => {
          if (!current()) return;
          hooks.growing(ageLabel(at, params.family.age));
          if (follow) stage.frame();
        },
      });
      if (submitted === null) throw new Error("abandoned for a newer request");
      const buildMs = performance.now() - started;
      if (kind === "build") hooks.built(buildMs);
      return { ...submitted, buildMs };
    } finally {
      if (current()) hooks.growing(null);
    }
  };
}
