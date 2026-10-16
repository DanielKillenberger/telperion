/// <reference lib="webworker" />
/* The field probe's worker (fn-257 R3): grows each case on the worker's
 * device through the package entry, as the site's worker does, and times the
 * device's start, each growth and each query of a 36-cell grid. */
import { growField, prepareField } from "../../field/index";
import { grid } from "../../field/voxelize";

export interface Case { species: string; seed: number }
export interface Timed { species: string; seed: number; growMs: number; queryMs: number; cells: number }
export type Message = { kind: "prepared"; ms: number } | { kind: "timed"; row: Timed } | { kind: "failed"; error: string } | { kind: "done" };

const post = (m: Message) => postMessage(m);

onmessage = async (event: MessageEvent<Case[]>) => {
  try {
    const t0 = performance.now();
    await prepareField();
    post({ kind: "prepared", ms: performance.now() - t0 });
    for (const c of event.data) {
      const t1 = performance.now();
      const tree = await growField(c.species, c.seed);
      const t2 = performance.now();
      const g = grid(tree.bounds, 36);
      const answer = await tree.query(g.cells);
      const t3 = performance.now();
      tree.release();
      post({ kind: "timed", row: { species: c.species, seed: c.seed, growMs: t2 - t1, queryMs: t3 - t2, cells: answer.flags.length } });
    }
    post({ kind: "done" });
  } catch (error) {
    post({ kind: "failed", error: String(error) });
  }
};
