/* The field probe page (fn-257 R3), served by the dev server at
 * /field-probe.html and opened by the owner in Chrome: each species the site
 * grows, at three seeds, twice (the first round on a cold shader cache, the
 * second warm), grown and read out in a worker as the site does it, beside
 * 0.1.4's time. Never part of the library or the harness. */
import type { Case, Message, Timed } from "./worker";

/* 0.1.4's end-to-end time per tree (TEST5.md, PROBE.md). */
const OLD: Record<string, number> = { "oregon-white-oak": 290, "silver-birch": 350, "norway-spruce": 240 };
const SPECIES = ["oregon-white-oak", "silver-birch", "norway-spruce", "date-palm"];
const SEEDS = [1, 1234, 5678];

const cases: Case[] = [];
for (let round = 0; round < 2; round++) for (const species of SPECIES) for (const seed of SEEDS) cases.push({ species, seed });

const rows: Timed[] = [];
const status = (text: string) => { const s = document.querySelector("#status"); if (s) s.textContent = text; };

function show(row: Timed, warm: boolean): void {
  const body = document.querySelector("#rows");
  if (!body) return;
  const tr = document.createElement("tr");
  const old = OLD[row.species];
  // The growth and its 36-cell read-out are one call; no query follows it.
  for (const text of [`${row.species}${warm ? "" : " (cold)"}`, String(row.seed), row.ms.toFixed(0), "-", row.ms.toFixed(0), old === undefined ? "-" : String(old)]) {
    const td = document.createElement("td");
    td.textContent = text;
    tr.append(td);
  }
  body.append(tr);
}

const worker = new Worker(new URL("./worker.ts", import.meta.url), { type: "module" });
let prepared = 0;
worker.onmessage = (event: MessageEvent<Message>) => {
  const m = event.data;
  if (m.kind === "prepared") { prepared = m.ms; status(`Device and programs ready in ${m.ms.toFixed(0)} ms; growing...`); }
  if (m.kind === "timed") { rows.push(m.row); show(m.row, rows.length > SPECIES.length * SEEDS.length); }
  if (m.kind === "failed") status(`Failed: ${m.error}`);
  if (m.kind === "done") {
    status("Done. The second round is warm.");
    const json = document.querySelector("#json");
    if (json) json.textContent = JSON.stringify({ probe: "fn-257 field", userAgent: navigator.userAgent, preparedMs: prepared, rows }, null, 2);
  }
};
worker.postMessage(cases);
