/// <reference lib="webworker" />
/* The field probe's worker (fn-257 R3): grows each case through the package
 * entry, as the site's worker does, and times each growth read out to a
 * 36-cell grid, one call (on the CPU since fn-353). */
import { growField } from "../../field/index";
import { presetById } from "../../browser/presets";

export interface Case { species: string; seed: number }
export interface Timed { species: string; seed: number; ms: number; cells: number }
export type Message = { kind: "timed"; row: Timed } | { kind: "failed"; error: string } | { kind: "done" };

const post = (m: Message) => postMessage(m);

onmessage = async (event: MessageEvent<Case[]>) => {
  try {
    for (const c of event.data) {
      const t1 = performance.now();
      const preset = presetById(c.species);
      const tree = await growField({ ...preset, skeleton: { ...preset.skeleton, seed: c.seed } }, 36);
      post({ kind: "timed", row: { species: c.species, seed: c.seed, ms: performance.now() - t1, cells: tree.answer.flags.length } });
    }
    post({ kind: "done" });
  } catch (error) {
    post({ kind: "failed", error: String(error) });
  }
};
