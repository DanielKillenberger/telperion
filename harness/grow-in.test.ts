import { expect, it } from "vitest";
import type { Growth, Submitted } from "../src/browser/render";
import { DRAWS_A_YEAR, draw, FLOOR_MS, type Held, type Pace, yearAges } from "./grow-in";

/* fn-270 R6 (host rulings, 2026-10-14): a grow-in draws every whole year
   from 1, each up at least FLOOR_MS; "play the year" draws the last year
   on the tree already grown; a trait build lets the growth go; a newer
   request stops a run between its draws. */

function fakeGrowth(horizon: number, log: string[], grown: (age: number) => void, alone = new Set<number>()): Growth {
  let committed = 0;
  return {
    admission: () => (alone.has(committed) ? "makeRoom" : "beside"),
    committed: () => committed,
    horizon: () => horizon,
    growTo: async (age) => { log.push(`grow ${age}`); grown(age); committed = Math.max(committed, Math.ceil(age)); return committed; },
    draw: async (at) => { log.push(`draw ${at}`); return { foliageInstances: at } as unknown as Submitted; },
    dispose: () => { log.push("dispose"); },
  };
}

function setup(horizon = 3, growMs: (age: number) => number = () => 0, alone = new Set<number>()) {
  const log: string[] = [];
  let clock = 0;
  const grown = (age: number) => { clock += growMs(age); };
  const grower = {
    setTreeGpu: async (family: string) => { log.push(`build ${family}`); return {} as Submitted; },
    beginGrowth: async (family: string) => { log.push(`begin ${family}`); return fakeGrowth(horizon, log, grown, alone); },
  };
  const waits: number[] = [];
  const pace = (current = () => true): Pace => ({
    current,
    drawn: () => { clock += 40; },
    now: () => clock,
    wait: async (ms) => { waits.push(ms); clock += ms; },
    makingRoom: (on) => { log.push(`room ${on}`); },
  });
  const held: Held = { growth: null, family: null };
  return { log, grower, pace, held, waits };
}

it("grows in a whole year a draw, each up for the floor, and plays the year on the same tree", async () => {
  const { log, grower, pace, held, waits } = setup();
  await draw(grower, held, "grow", "oak", pace());
  expect(log.filter((l) => !l.startsWith("room"))).toEqual(["begin oak", "grow 1", "draw 1", "grow 2", "draw 2", "grow 3", "draw 3"]);
  expect(waits).toEqual([FLOOR_MS - 40, FLOOR_MS - 40]);
  log.length = 0;
  const last = await draw(grower, held, "year", "oak", pace());
  expect(log.filter((l) => l.startsWith("draw"))).toHaveLength(DRAWS_A_YEAR);
  expect(log).not.toContain("begin oak");
  expect(last).toEqual({ foliageInstances: 3 });
  log.length = 0;
  await draw(grower, held, "build", "oak", pace());
  expect(log).toEqual(["dispose", "build oak"]);
  expect(held.growth).toBeNull();
});

it("a run a newer request replaced stops between its draws, and a new grow-in lets the old state go", async () => {
  const { log, grower, pace, held } = setup();
  let draws = 0;
  const stopped = await draw(grower, held, "grow", "oak", { ...pace(() => draws < 2), drawn: () => { draws += 1; } });
  expect(stopped).toBeNull();
  expect(log.filter((l) => !l.startsWith("room"))).toEqual(["begin oak", "grow 1", "draw 1", "grow 2", "draw 2"]);
  log.length = 0;
  await draw(grower, held, "year", "spruce", pace());
  expect(log.slice(0, 2)).toEqual(["dispose", "begin spruce"]);
});

it("the floor runs from a tree's draw, however long the year before it took to grow", async () => {
  const { grower, pace, held, waits } = setup(2, (age) => (age === 1 ? 200 : 10));
  await draw(grower, held, "grow", "oak", pace());
  // Year 1 drawn at 200 (then 40 in the panel), year 2 grown by 250: 150 from 200 waits 100.
  expect(waits).toEqual([FLOOR_MS - 50]);
});

it("a draw that fits only alone says so while the drawn tree is let go, as a build does", async () => {
  const { log, grower, pace, held } = setup(3, () => 0, new Set([2]));
  await draw(grower, held, "grow", "oak", pace());
  expect(log).toEqual([
    "begin oak", "grow 1", "room false", "draw 1", "room false", "grow 2", "room false", "draw 2", "room false",
    "grow 3", "room true", "draw 3", "room false",
  ]);
});

it("a one-year tree plays its year as its whole age, before which there is no wood", () => {
  expect(yearAges(1)).toEqual([1]);
  expect(yearAges(2)).toHaveLength(DRAWS_A_YEAR);
});
