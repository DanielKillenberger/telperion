/* The fn-257 early-test probe page, served by the dev server at
 * /probe.html and opened by the owner in Chrome. It runs once on load and
 * shows a table and a JSON block to copy. Never part of the library or the
 * harness. */
import { navigatorGpu } from "./gpu";
import { acquire, chain, CHAIN_DISPATCHES, compileRound, type ChainRun, type Round } from "./measure";

/* 0.1.4's oak build: the spec's stop when device plus cold compile exceed it. */
const OAK_MS = 273;
const VISITED = "telperion.probe.visited";
const CHAIN_REPEATS = 5;

type Row = readonly [string, string];

function firstVisit(): boolean | "unknown" {
  try {
    const seen = window.localStorage.getItem(VISITED) !== null;
    window.localStorage.setItem(VISITED, new Date().toISOString());
    return !seen;
  } catch {
    return "unknown";
  }
}

function adapterInfo(info: Readonly<Record<string, unknown>>): Record<string, string | number | boolean> {
  const out: Record<string, string | number | boolean> = {};
  for (const key in info) {
    const value = info[key];
    if (typeof value === "string" || typeof value === "number" || typeof value === "boolean") out[key] = value;
  }
  return out;
}

const ms = (value: number): string => value.toFixed(1);

function median(runs: readonly ChainRun[], key: keyof ChainRun): number {
  const sorted = runs.map(r => r[key]).sort((a, b) => a - b);
  return sorted[Math.floor(sorted.length / 2)];
}

function roundRows(label: string, round: Round): Row[] {
  const rows: Row[] = [];
  for (const kit of round.kits) {
    for (const p of kit.pipelines) {
      rows.push([`${label}: ${kit.kit}.${p.entry}`, p.error ? `${ms(p.ms)} (failed: ${p.error})` : ms(p.ms)]);
    }
    rows.push([`${label}: ${kit.kit} module, ${kit.layout} layout`, ms(kit.ms)]);
  }
  rows.push([`${label}: total, ${round.pipelines} pipelines (${round.failed} failed)`, ms(round.ms)]);
  return rows;
}

function show(rows: readonly Row[], report: unknown, status: string): void {
  const body = document.querySelector("#rows");
  const json = document.querySelector("#json");
  const note = document.querySelector("#status");
  if (!body || !json || !note) return;
  body.replaceChildren(
    ...rows.map(([name, value]) => {
      const tr = document.createElement("tr");
      for (const text of [name, value]) {
        const td = document.createElement("td");
        td.textContent = text;
        tr.append(td);
      }
      return tr;
    }),
  );
  json.textContent = JSON.stringify(report, null, 2);
  note.textContent = status;
}

async function probe(): Promise<void> {
  const report: Record<string, unknown> = { probe: "fn-257 early test", firstVisit: firstVisit(), userAgent: navigator.userAgent };
  const rows: Row[] = [["first visit on this origin", String(report.firstVisit)], ["browser", navigator.userAgent]];
  const gpu = navigatorGpu();
  if (!gpu) {
    show(rows, { ...report, error: "navigator.gpu is absent" }, "WebGPU is not available in this browser.");
    return;
  }
  const got = await acquire(gpu);
  const errors: string[] = [];
  got.device.addEventListener("uncapturederror", e => errors.push(e.error.message));
  void got.device.lost.then(l => errors.push(`device lost: ${l.reason} ${l.message}`));
  const info = adapterInfo(got.adapter.info);
  rows.push(["adapter info", JSON.stringify(info)]);
  rows.push(["requestAdapter ms", ms(got.adapterMs)], ["requestDevice ms", ms(got.deviceMs)]);
  Object.assign(report, { adapterInfo: info, requiredLimits: got.requiredLimits, requestAdapterMs: got.adapterMs, requestDeviceMs: got.deviceMs });
  show(rows, report, "Compiling the cycle kernels, cold...");

  const cold = await compileRound(got.device);
  rows.push(...roundRows("cold", cold));
  show(rows, { ...report, cold }, "Compiling the cycle kernels again, cached...");
  const cached = await compileRound(got.device);
  rows.push(...roundRows("cached", cached));
  const deviceAndCold = got.adapterMs + got.deviceMs + cold.ms;
  rows.push([`adapter + device + cold compile (stop past ${OAK_MS} ms)`, ms(deviceAndCold)]);
  Object.assign(report, { cold, cached, deviceAndColdMs: deviceAndCold, oakMs: OAK_MS, exceedsOak: deviceAndCold > OAK_MS });
  show(rows, report, `Running the empty chain of ${CHAIN_DISPATCHES} indirect dispatches...`);

  const runner = await chain(got.device);
  const first = await runner.run();
  const repeats: ChainRun[] = [];
  for (let i = 0; i < CHAIN_REPEATS; i++) repeats.push(await runner.run());
  rows.push([`chain kernel compile (step, noop)`, ms(runner.compile.ms)]);
  rows.push([`chain first: encode / submit to done / total (steps ${first.stepsDone}/${CHAIN_DISPATCHES})`,
    `${ms(first.encodeMs)} / ${ms(first.submitToDoneMs)} / ${ms(first.totalMs)}`]);
  rows.push([`chain median of ${CHAIN_REPEATS} more: encode / submit to done / total`,
    `${ms(median(repeats, "encodeMs"))} / ${ms(median(repeats, "submitToDoneMs"))} / ${ms(median(repeats, "totalMs"))}`]);
  Object.assign(report, { chain: { dispatches: CHAIN_DISPATCHES, compile: runner.compile, first, repeats }, errors });
  if (errors.length) rows.push(["device errors", errors.join(" | ")]);
  show(rows, report, "Done. Copy the JSON below.");
}

function wireCopy(): void {
  const button = document.querySelector("#copy");
  const json = document.querySelector("#json");
  button?.addEventListener("click", () => {
    void navigator.clipboard.writeText(json?.textContent ?? "").then(
      () => { if (button) button.textContent = "Copied"; },
      () => { if (button) button.textContent = "Copy failed; select the text"; },
    );
  });
}

wireCopy();
probe().catch((error: unknown) => {
  const note = document.querySelector("#status");
  if (note) note.textContent = `Probe failed: ${String(error)}`;
});
