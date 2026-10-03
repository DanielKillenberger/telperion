import { chromium } from 'playwright';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync, readdirSync, mkdirSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';

/* Runs Véronique Letort's GreenLab structure simulators unchanged in headless
 * Chromium and records what they grow, as the oracle the tree space's engine
 * is tested against (fn-191, R3). The simulators have no licence: they are
 * cloned outside the repository, pinned by commit, run and compared against,
 * never copied.
 *
 *   git clone https://github.com/VeroniqueLC/PlantStructureFactoryStochastic <dir>/alternate
 *   git clone https://github.com/VeroniqueLC/PlantStructureFactoryStochastic_Acer <dir>/opposite
 *   node scripts/greenlab-oracle.mjs fixtures <dir> crates/telperion-space/tests/fixtures/greenlab-oracle.json
 *   node scripts/greenlab-oracle.mjs sheet <dir> <fixture> <our svg dir> <out dir>
 *
 * The alternate simulator bears one bud per node, the opposite one (the Acer
 * variant) two, drawn independently. Both draw each growth unit's phytomer
 * count, uniform or Poisson, and each bud's physiological age (PA).
 */
const PINS = {
  alternate: '9ac7a9fd74ae6dcd5fd6424f39b620515bc71aec',
  opposite: '7f30aca',
};
const SEEDS = { from: 1, count: 4000 };

const pa = (macro, terminal, nmin, nmax, lambda, p = {}) => ({ macro, terminal, nmin, nmax, lambda, p });
const ALTERNATE = [pa(10, 0, 1, 2, 1.5, { 2: 0.7 }), pa(4, 3, 2, 4, 3, { 3: 0.5 }), pa(2, 0, 1, 2, 1.5)];
const OPPOSITE = [pa(10, 0, 1, 2, 1.5, { 2: 0.45 }), pa(4, 3, 2, 4, 3, { 3: 0.35 }), pa(2, 0, 1, 2, 1.5)];
const SETS = [
  // The simulators' own defaults, under both count laws.
  { name: 'alternate-uniform', variant: 'alternate', maxCA: 10, poisson: false, pa: ALTERNATE },
  { name: 'alternate-poisson', variant: 'alternate', maxCA: 10, poisson: true, pa: ALTERNATE },
  { name: 'opposite-uniform', variant: 'opposite', maxCA: 10, poisson: false, pa: OPPOSITE },
  { name: 'opposite-poisson', variant: 'opposite', maxCA: 10, poisson: true, pa: OPPOSITE },
  // Four PAs, a perpetual PA 2 (its apex returns to itself) and a PA 3 that ages into PA 4.
  {
    name: 'alternate-four', variant: 'alternate', maxCA: 12, poisson: false,
    pa: [pa(12, 0, 1, 3, 2, { 2: 0.4, 3: 0.3, 4: 0.1 }), pa(3, 2, 1, 3, 2, { 3: 0.3, 4: 0.3 }),
      pa(2, 4, 2, 3, 2.5, { 4: 0.4 }), pa(2, 0, 1, 2, 1.5)],
  },
  // Deterministic: every count fixed, every bud certain.
  { name: 'alternate-fixed', variant: 'alternate', maxCA: 10, poisson: false, deterministic: true,
    pa: [pa(10, 0, 2, 2, 2, { 2: 1 }), pa(3, 3, 2, 2, 2, { 3: 1 }), pa(2, 0, 1, 1, 1)] },
  { name: 'alternate-fixed-loop', variant: 'alternate', maxCA: 9, poisson: false, deterministic: true,
    pa: [pa(2, 1, 1, 1, 1, { 2: 1 }), pa(2, 3, 2, 2, 2), pa(1, 3, 1, 1, 1)] },
  { name: 'opposite-fixed', variant: 'opposite', maxCA: 8, poisson: false, deterministic: true,
    pa: [pa(8, 0, 1, 1, 1, { 2: 1 }), pa(2, 3, 2, 2, 2, { 3: 1 }), pa(2, 0, 1, 1, 1)] },
];

function checkout(dir, variant) {
  const head = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: dir, encoding: 'utf8' }).trim();
  if (!head.startsWith(PINS[variant])) throw new Error(`${dir} is at ${head}, the pin is ${PINS[variant]}`);
  const html = join(dir, 'index.html');
  const sha256 = createHash('sha256').update(readFileSync(html)).digest('hex');
  return { url: pathToFileURL(resolve(html)).href, commit: head, sha256 };
}

/* In the page: the simulator's own state, generator and tree, untouched. */
function configure(set) {
  state.numPA = set.pa.length;
  state.maxCA = set.maxCA;
  state.poisson = set.poisson;
  state.pa = set.pa.map(p => ({ ...p, p: { ...p.p } }));
}

function runSeeds({ from, count }) {
  const tables = [];
  let truncated = 0;
  const started = performance.now();
  for (let seed = from; seed < from + count; seed++) {
    rng = makeRNG(seed);
    nodeCount = 0;
    plantTree = buildAxis(1, 0, state.maxCA);
    if (nodeCount > MAX_NODES) truncated++;
    const table = Array.from({ length: state.numPA }, () => new Array(state.maxCA).fill(0));
    const walk = axis => {
      if (!axis) return;
      for (const ph of axis.phytomers) {
        table[axis.pa - 1][ph.ca - 1]++;
        if (ph.axillary) walk(ph.axillary);
        for (const kid of ph.kids ?? []) walk(kid.axis);
      }
      walk(axis.continuation);
    };
    walk(plantTree);
    tables.push(table);
  }
  return { tables, truncated, ms: (performance.now() - started) / count };
}

/* The tree as a canonical string: an axis is its PA, its growth units in
 * order, each the sorted strings of its phytomers' laterals, then its
 * continuation; an axis that grew nothing is empty. */
function signature(seed) {
  rng = makeRNG(seed);
  nodeCount = 0;
  const tree = buildAxis(1, 0, state.maxCA);
  const axis = a => {
    if (!a) return '';
    const units = new Map();
    for (const ph of a.phytomers) {
      const kids = ph.axillary ? [ph.axillary] : (ph.kids ?? []).map(k => k.axis);
      const word = `p[${kids.map(axis).filter(s => s).sort().join(';')}]`;
      units.set(ph.ca, [...(units.get(ph.ca) ?? []), word]);
    }
    const next = axis(a.continuation);
    if (units.size === 0 && !next) return '';
    const body = [...units.keys()].sort((x, y) => x - y).map(ca => `(${units.get(ca).sort().join(',')})`);
    return `A${a.pa}${body.join('')}${next ? `>${next}` : ''}`;
  };
  return axis(tree);
}

export function fnv1a(text) {
  let hash = 0xcbf29ce484222325n;
  for (const byte of Buffer.from(text, 'utf8')) {
    hash = ((hash ^ BigInt(byte)) * 0x100000001b3n) & 0xffffffffffffffffn;
  }
  return hash.toString(16).padStart(16, '0');
}

function moments(values) {
  const n = values.length;
  const mean = values.reduce((s, v) => s + v, 0) / n;
  const central = k => values.reduce((s, v) => s + (v - mean) ** k, 0) / n;
  return { mean, var: central(2), m4: central(4) };
}

function summarise(tables, pas, cycles) {
  const cells = Array.from({ length: pas }, (_, p) =>
    Array.from({ length: cycles }, (_, c) => moments(tables.map(t => t[p][c]))));
  const totals = Array.from({ length: pas }, (_, p) =>
    moments(tables.map(t => t[p].reduce((s, v) => s + v, 0))));
  return { cells, totals };
}

async function fixtures(cloneDir, out) {
  const browser = await chromium.launch();
  const page = await browser.newPage();
  const sources = { alternate: checkout(join(cloneDir, 'alternate'), 'alternate'),
    opposite: checkout(join(cloneDir, 'opposite'), 'opposite') };
  const sets = [];
  for (const set of SETS) {
    await page.goto(sources[set.variant].url);
    await page.evaluate(configure, set);
    const seeds = set.deterministic ? { from: 1, count: 2 } : SEEDS;
    const run = await page.evaluate(runSeeds, seeds);
    if (run.truncated) throw new Error(`${set.name}: ${run.truncated} trees hit the simulator's node cap`);
    const record = { ...set, seeds, ms_per_structure: run.ms,
      oracle: summarise(run.tables, set.pa.length, set.maxCA) };
    if (set.deterministic) {
      const [first, second] = await Promise.all([1, 2].map(s => page.evaluate(signature, s)));
      if (first !== second) throw new Error(`${set.name}: a deterministic set differs between seeds`);
      record.counts = run.tables[0];
      record.signature = fnv1a(first);
      record.signature_length = first.length;
    }
    sets.push(record);
    console.log(`${set.name}: ${run.ms.toFixed(3)} ms per structure`);
  }
  await browser.close();
  const pins = Object.fromEntries(Object.entries(sources).map(([k, v]) => [k, { commit: v.commit, sha256: v.sha256 }]));
  writeFileSync(out, `${JSON.stringify({ sources: pins, sets }, null, 1)}\n`);
}

/* The side-by-side sheet: per set, the simulator's own drawing at two seeds
 * beside ours at two seeds, and the mean counts per PA from both. */
async function sheet(cloneDir, fixturePath, oursDir, outDir) {
  const fixture = JSON.parse(readFileSync(fixturePath, 'utf8'));
  const timing = JSON.parse(readFileSync(join(oursDir, 'timing.json'), 'utf8'));
  const browser = await chromium.launch();
  const page = await browser.newPage();
  const rows = [];
  for (const set of fixture.sets) {
    await page.goto(checkout(join(cloneDir, set.variant), set.variant).url);
    await page.evaluate(configure, set);
    const theirs = [];
    for (const seed of [1, 2]) {
      theirs.push(await page.evaluate(s => {
        document.getElementById('seed').value = s;
        generate();
        return document.getElementById('svgPlant').outerHTML;
      }, seed));
    }
    const ours = readdirSync(oursDir).filter(f => f.startsWith(`${set.name}.`) && f.endsWith('.svg')).sort()
      .slice(0, 2).map(f => readFileSync(join(oursDir, f), 'utf8'));
    const mine = timing[set.name];
    const means = set.oracle.totals.map((t, p) =>
      `PA${p + 1}: ${t.mean.toFixed(1)} / ${mine.totals[p].toFixed(1)}`).join('<br>');
    rows.push(`<tr><th>${set.name}<div class=n>${means}<br>oracle ${set.ms_per_structure.toFixed(3)} ms,
      ours ${(mine.us_per_structure / 1000).toFixed(3)} ms</div></th>
      ${[...theirs, ...ours].map(svg => `<td>${svg}</td>`).join('')}</tr>`);
  }
  const html = `<!doctype html><meta charset=utf-8><style>
    body{font:12px sans-serif;background:#fff;margin:8px} table{border-collapse:collapse}
    td,th{border:1px solid #ccc;padding:2px;vertical-align:top} th{width:150px;text-align:left}
    td svg{width:220px;height:220px;display:block} .n{font-weight:normal;margin-top:4px}</style>
    <table><tr><th>set (mean phytomers per PA: oracle / ours)</th><th>oracle seed 1</th><th>oracle seed 2</th>
    <th>ours seed 1</th><th>ours seed 2</th></tr>${rows.join('')}</table>`;
  mkdirSync(outDir, { recursive: true });
  writeFileSync(join(outDir, 'sheet.html'), html);
  await page.setContent(html);
  await page.screenshot({ path: join(outDir, 'sheet.png'), fullPage: true });
  await browser.close();
}

const [mode, ...args] = process.argv.slice(2);
if (mode === 'fixtures' && args.length === 2) await fixtures(...args);
else if (mode === 'sheet' && args.length === 4) await sheet(...args);
else {
  console.error('usage: greenlab-oracle.mjs fixtures <clones> <out.json> | sheet <clones> <fixture> <ours> <out>');
  process.exit(2);
}
