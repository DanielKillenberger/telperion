import { pathToFileURL } from 'node:url';
import { readFile, writeFile, mkdir } from 'node:fs/promises';
const { chromium } = await import(process.env.PLAYWRIGHT_MODULE ? pathToFileURL(process.env.PLAYWRIGHT_MODULE).href : 'playwright');
const url = process.env.BROWSER_URL ?? 'http://127.0.0.1:5184';
const out = process.env.BROWSER_EVIDENCE ?? '/tmp/fn8-browser';
await mkdir(out, { recursive: true });
const browser = await chromium.launch({ executablePath: process.env.CHROMIUM_EXECUTABLE, headless: true,
  args: ['--no-sandbox'] });
try {
  const page = await browser.newPage({ viewport: { width: 1600, height: 1000 }, deviceScaleFactor: 1 });
  page.setDefaultTimeout(120000);
  await page.route(url + '/', r => r.fulfill({ contentType: 'text/html', body: '<!doctype html><html><body style="margin:0;width:100vw;height:100vh;overflow:hidden"><canvas style="display:block;width:100%;height:100%"></canvas></body></html>' }));
  await page.goto(url + '/');
  const result = await page.evaluate(async () => {
    const check = (ok, message) => { if (!ok) throw Error(message); };
    const THREE = await import('/node_modules/.vite/deps/three.js');
    const { createStage } = await import('/harness/stage.ts');
    const stage = createStage(document.querySelector('canvas'));
    let subject, disposed = false;
    stage.setTree(clay => {
      subject = new THREE.Mesh(new THREE.BoxGeometry(), clay.surface);
      subject.geometry.addEventListener('dispose', () => { disposed = true; });
      return subject;
    });
    let rejected = false;
    try { stage.setTree(() => { throw Error('injected build failure'); }); } catch { rejected = true; }
    check(rejected && !disposed && subject.parent !== null, 'failed replacement must preserve prior scene and geometry');
    stage.dispose(); check(disposed, 'previous scene eventually disposed');
    return { transactionalReplacement: true };
  });
  await writeFile(out + '/transaction.json', JSON.stringify(result, null, 2));
  const bindings = await page.evaluate(async () => {
    const check = (ok, message) => { if (!ok) throw Error(message); };
    const rejects = async (action, message) => { let failed = false; try { await action(); } catch { failed = true; } check(failed, message); };
    const { TreeEngine, ORDINARY } = await import('/src/browser/core.ts');
    await rejects(() => TreeEngine.create(new Response('', { status: 503 })), 'load failure');
    await rejects(() => TreeEngine.create(new Uint8Array([0, 1, 2])), 'malformed module');
    const engine = await TreeEngine.create();
    const family = structuredClone(ORDINARY);
    family.skeleton.seed = 42; family.skeleton.attractors = 500; family.skeleton.step = 0.02;
    const meshFree = engine.build(family, { structure: true, field: true });
    check(!meshFree.surface && !meshFree.foliage && !meshFree.diagnostics.stages.surface && meshFree.diagnostics.timings.surfaceMs === 0, 'mesh-free must not invoke surface');
    check(meshFree.diagnostics.nodes === 13616 && meshFree.diagnostics.instances === 59810, 'matched ordinary growth/foliage');
    check(meshFree.diagnostics.complete, 'complete diagnostics');
    const cells = new Float64Array([0,0,0,0, 1e5,1e5,1e5,0.5]);
    const hits = meshFree.field.query(cells);
    check(hits[0] === 1 && hits[1] === 0, 'root wood / empty region');
    check(meshFree.field.query(new Float64Array([0,12,0,20]))[0] === 3, 'field-only includes both materials without render output');
    check(meshFree.field.query(new Float64Array()).length === 0, 'empty query');
    for (const bad of [new Float64Array(3), new Float64Array([NaN,0,0,1]), new Float64Array([0,0,0,-1]), new Float32Array(4)])
      await rejects(() => meshFree.field.query(bad), 'invalid query must reject');
    check(meshFree.field.query(cells)[0] === 1, 'queries recover after errors');
    const saved = meshFree.structure.values.slice();
    engine.release(); engine.release();
    check(saved.every((v,i) => v === meshFree.structure.values[i]), 'owned copies survive release');
    await rejects(() => meshFree.field.query(cells), 'released field handle');
    const structureOnly = engine.build(family, { structure: true });
    check(!structureOnly.surface && !structureOnly.foliage && !structureOnly.field && structureOnly.diagnostics.leavesPlaced === 0, 'optional outputs skipped');
    check(saved.every((v,i) => v === structureOnly.structure.values[i]), 'repeat deterministic');
    const field = engine.build(family, { field: true }).field;
    engine.build(family, {});
    await rejects(() => field.query(cells), 'rebuild stales field handle');
    for (const bad of [NaN, Infinity, -1]) {
      const p = structuredClone(family); p.skeleton.envelope.height = bad;
      await rejects(() => engine.build(p, { surface: true }), 'bad height');
    }
    const partial = structuredClone(family); partial.skeleton.growth.maxNodes = 20;
    check(!engine.build(partial, {}).diagnostics.complete, 'caps explicitly partial');
    const empty = structuredClone(family); empty.skeleton.attractors = 0;
    const result = engine.build(empty, { surface: true, foliage: true });
    check(result.surface.positions.length === 0 && result.surface.bounds === null && result.foliage.matrices.length === 0, 'valid empty outputs');
    engine.dispose(); engine.dispose();
    await rejects(() => engine.build(family, {}), 'disposed engine');
    // Exercise the native boundary directly, including malformed JSON and allocated byte validation.
    const bytes = await (await fetch('/src/browser/telperion.wasm')).arrayBuffer();
    const { instance } = await WebAssembly.instantiate(bytes, { env: { now: () => performance.now() } });
    const e = instance.exports;
    function raw(value) {
      const bytes = new TextEncoder().encode(value); check(e.request_alloc(bytes.length) === 0, 'allocate request');
      new Uint8Array(e.memory.buffer, e.request_ptr(), bytes.length).set(bytes); return e.build();
    }
    for (const value of ['{', 'null', '{}', '{"family":{},"outputs":{"surface":1}}', '{"family":{"skeleton":{"seed":-1}},"outputs":{}}', '{"family":{"unknown":1},"outputs":{}}', '{"family":{"skeleton":{"envelope":{"height":1e999}}},"outputs":{}}'])
      check(raw(value) === 1 && e.buffer_len(0) === 0, 'malformed request clears stale buffers');
    check(e.request_alloc(65537) === 1 && e.build() === 1, 'oversized request leaves no prior request');
    check(raw('{"family":{"skeleton":{"attractors":0}},"outputs":{"surface":true}}') === 0 && e.buffer_len(0) === 0, 'native empty recovery');
    check(raw(JSON.stringify({ family, outputs: { field: true } })) === 0, 'native field-only build');
    check([0,1,2,3,4,5].every(slot => e.buffer_len(slot) === 0), 'field-only allocates no render buffers');
    check(e.buffer_len(999) === 0 && e.buffer_ptr(999) === 0, 'unknown buffer slot');
    check(e.query_alloc(0) === 0 && e.query(0) === 1, 'no stale query accepted');
    e.release(); e.release();
    return { meshFree: true, ownership: true, malformed: true, empty: true, staleFields: true, deterministic: true, partialDiagnostics: true };
  });
  await writeFile(out + '/bindings.json', JSON.stringify(bindings, null, 2));
  await page.unroute(url + '/');
  let failedLoad = false;
  await page.route('**/telperion.wasm', route => {
    if (!failedLoad) { failedLoad = true; return route.abort('failed'); }
    return route.continue();
  });
  await page.goto(url + '/');
  await page.getByRole('alert').waitFor();
  await page.getByRole('button', { name: 'retry build' }).click();
  await page.getByRole('alert').waitFor({ state: 'detached' });
  const stats = page.locator('.gd-note').filter({ hasText: /tris, .* verts, .* nodes/ });
  await stats.waitFor();
  const before = await stats.textContent();
  await page.evaluate(async () => {
    const { initializeTreeCore } = await import('/src/browser/core.ts');
    const engine = await initializeTreeCore();
    const build = engine.build.bind(engine);
    engine.build = (...args) => { engine.build = build; throw Error('injected native build failure'); };
  });
  const seed = page.getByRole('textbox');
  await seed.fill('17');
  await page.getByRole('alert').waitFor();
  if (await stats.textContent() !== before) throw Error('failed UI build replaced prior diagnostics');
  await page.getByRole('button', { name: 'retry build' }).click();
  await page.getByRole('alert').waitFor({ state: 'detached' });
  if (await stats.textContent() === before) throw Error('UI retry did not build changed specimen');
  await page.screenshot({ path: out + '/viewer.png' });
  await writeFile(out + '/viewer.json', JSON.stringify({ loadFailureRetry: failedLoad, buildFailureRetry: true, previousDiagnosticsPreserved: true }, null, 2));
  console.log({ ...result, ...bindings, loadFailureRetry: failedLoad, buildFailureRetry: true });
} finally { await browser.close(); }
