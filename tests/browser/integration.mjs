import { pathToFileURL } from 'node:url';
import { readFile, writeFile, mkdir } from 'node:fs/promises';
// Shared anatomy for small binding, renderer and UI fixtures. Binding-specific
// resource limits and internode spacing remain explicit at their call site.
function compactSpeciesFixture(family) {
  const fixture = structuredClone(family);
  fixture.skeleton.envelope.height = 4;
  fixture.skeleton.attractors = 40;
  const habit = fixture.skeleton.habit;
  if (habit.kind === 'tiered') {
    habit.tiers = 3;
    habit.branchesPerTier = 3;
    habit.secondarySpacing = 0.4;
  }
  if (habit.kind === 'spreading') { habit.scaffoldLimbs = 3; habit.subdivisions = 2; }
  return fixture;
}
const { chromium } = await import(process.env.PLAYWRIGHT_MODULE ? pathToFileURL(process.env.PLAYWRIGHT_MODULE).href : 'playwright');
const url = process.env.BROWSER_URL ?? 'http://127.0.0.1:5184';
const out = process.env.BROWSER_EVIDENCE ?? '.flow/tmp/fn98-browser';
await mkdir(out, { recursive: true });
const browser = await chromium.launch({ executablePath: process.env.CHROMIUM_EXECUTABLE, headless: true,
  args: ['--no-sandbox', ...(process.env.BINDINGS_ONLY === '1' ? ['--disable-gpu'] : [])] });
try {
  const page = await browser.newPage({ viewport: { width: 1600, height: 1000 }, deviceScaleFactor: 1 });
  await page.addInitScript({ content: `window.compactSpeciesFixture = ${compactSpeciesFixture.toString()};` });
  // Software WebGL must finish each submitted frame before the next one queues.
  // Preserve full fixture geometry, animation/orbit and screenshot assertions;
  // this is synchronization, not a GPU performance measurement.
  await page.addInitScript(() => {
    const schedule = window.requestAnimationFrame.bind(window);
    window.requestAnimationFrame = callback => schedule(time => {
      callback(time);
      for (const canvas of document.querySelectorAll('canvas')) {
        canvas.getContext('webgl2')?.finish();
      }
    });
  });
  page.setDefaultTimeout(120000);
  page.on('crash', () => console.error('Browser renderer crashed'));
  browser.on('disconnected', () => console.log('Browser disconnected'));
  await page.route(url + '/', r => r.fulfill({ contentType: 'text/html', body: '<!doctype html><html><body style="margin:0;width:100vw;height:100vh;overflow:hidden"><canvas style="display:block;width:100%;height:100%"></canvas></body></html>' }));
  await page.goto(url + '/');
  const result = process.env.BINDINGS_ONLY === '1' ? { skipped: 'rendering (bindings-only)' } : await page.evaluate(async () => {
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
    const { TreeEngine, ORDINARY, PRESETS, presetById } = await import('/src/browser/core.ts');
    await rejects(() => TreeEngine.create(new Response('', { status: 503 })), 'load failure');
    await rejects(() => TreeEngine.create(new Uint8Array([0, 1, 2])), 'malformed module');
    const engine = await TreeEngine.create();
    const family = structuredClone(ORDINARY);
    family.skeleton.seed = 42; family.skeleton.attractors = 500; family.skeleton.step = 0.02;
    const meshFree = engine.build(family, { structure: true, field: true });
    check(!meshFree.surface && !meshFree.foliage && !meshFree.diagnostics.stages.surface && meshFree.diagnostics.timings.surfaceMs === 0, 'mesh-free must not invoke surface');
    // FN-9 natural defaults and retained clipped laterals intentionally changed
    // historical 13616/59810 counts. Assert topology/cardinality and exact replay below.
    check(meshFree.diagnostics.nodes > meshFree.diagnostics.crossover && meshFree.diagnostics.instances > 0, 'ordinary has local branches and retained foliage');
    check(meshFree.structure.values.length === meshFree.diagnostics.nodes * 6 && meshFree.structure.topology.length === meshFree.diagnostics.nodes * 3, 'packed node cardinality');
    check(meshFree.diagnostics.complete, 'complete diagnostics');
    const cells = new Float64Array([0,0,0,0, 1e5,1e5,1e5,0.5]);
    const hits = meshFree.field.query(cells);
    check(hits[0] === 1 && hits[1] === 0, 'root wood / empty region');
    check(meshFree.field.query(new Float64Array([0,12,0,20]))[0] === 3, 'field-only includes both materials without render output');
    check(meshFree.field.query(new Float64Array()).length === 0, 'empty query');
    for (const bad of [new Float64Array(3), new Float64Array([NaN,0,0,1]), new Float64Array([0,0,0,-1]), new Float32Array(4)])
      await rejects(() => meshFree.field.query(bad), 'invalid query must reject');
    check(meshFree.field.query(cells)[0] === 1, 'queries recover after errors');
    const { querySnapshot, boundaryCells, gridCells, snapshotArrays } = await import('/scripts/benchmarks/generation-inputs.mjs');
    const snapshot = meshFree.field.snapshot();
    check(snapshot.wood.length === meshFree.diagnostics.nodes * 8, 'snapshot wood count');
    check(snapshot.leaves.bounds.length / 6 - snapshot.leaves.nodeCount === meshFree.diagnostics.instances, 'snapshot foliage count');
    for (const packed of [cells, new Float64Array(), boundaryCells(snapshot), gridCells(snapshot.bounds, 8)]) {
      const reference = meshFree.field.query(packed), copied = querySnapshot(snapshot, packed);
      check(copied.length === reference.length && copied.every((v,i) => v === reference[i]), 'snapshot exact indexed flags');
    }
    for (const bad of [new Float64Array(3), new Float64Array([NaN,0,0,1]), new Float64Array([0,0,0,-1]), new Float64Array([0,0,0,Infinity]), new Float64Array([Number.MAX_VALUE,0,0,0]), new Float32Array(4)])
      await rejects(() => querySnapshot(snapshot, bad), 'snapshot invalid query');
    const snapshotSaved = snapshot.wood.slice();
    snapshot.wood[6] = 0;
    check(meshFree.field.query(cells)[0] === 1, 'snapshot mutation isolated');
    snapshot.wood.set(snapshotSaved);
    const saved = meshFree.structure.values.slice();
    engine.release(); engine.release();
    check(saved.every((v,i) => v === meshFree.structure.values[i]), 'owned copies survive release');
    await rejects(() => meshFree.field.query(cells), 'released field handle');
    await rejects(() => meshFree.field.snapshot(), 'released snapshot handle');
    check(querySnapshot(snapshot, cells).every((v,i) => v === hits[i]), 'snapshot survives release');
    const structureOnly = engine.build(family, { structure: true });
    check(!structureOnly.surface && !structureOnly.foliage && !structureOnly.field && structureOnly.diagnostics.leavesPlaced === 0, 'optional outputs skipped');
    check(saved.every((v,i) => v === structureOnly.structure.values[i]), 'repeat deterministic');
    const field = engine.build(family, { field: true }).field;
    engine.build(family, {});
    await rejects(() => field.query(cells), 'rebuild stales field handle');
    await rejects(() => field.snapshot(), 'rebuild stales snapshot handle');
    for (const bad of [NaN, Infinity, -1]) {
      const p = structuredClone(family); p.skeleton.envelope.height = bad;
      await rejects(() => engine.build(p, { surface: true }), 'bad height');
    }
    const partial = structuredClone(family); partial.skeleton.growth.maxNodes = 20;
    check(!engine.build(partial, {}).diagnostics.complete, 'caps explicitly partial');
    const empty = structuredClone(family); empty.skeleton.attractors = 0;
    const result = engine.build(empty, { surface: true, foliage: true });
    check(result.surface.positions.length === 0 && result.surface.bounds === null && result.foliage.matrices.length === 0, 'valid empty outputs');
    check(PRESETS.length === 5 && new Set(PRESETS.map(p => p.id)).size === 5, 'complete identity catalogue');
    await rejects(() => presetById('missing'), 'unknown browser identity');
    await rejects(() => engine.build('missing', {}), 'unknown native identity');
    for (const [id, unit] of [['oregon-white-oak', 'leaf'], ['norway-spruce', 'needle']]) {
      const specimen = window.compactSpeciesFixture(presetById(id));
      // Small valid fixtures retain the authored habit and element anatomy.
      specimen.skeleton.growth.maxNodes = 12000;
      specimen.canopy.maxInstances = 12000;
      specimen.skeleton.twigs.twig.internodeLength = 0.04;
      const output = engine.build(specimen, { foliage: true, structure: true, field: true });
      const foliage = output.foliage, d = output.diagnostics;
      check(d.complete, id + ' small binding fixture completes without truncation');
      check(!output.surface && !d.stages.surface && d.timings.surfaceMs === 0, id + ' independent foliage');
      check(d.instances > 0 && d.biologicalUnits === d.instances && foliage.matrices.length === d.instances * 16, id + ' one biological unit per matrix');
      const a = foliage.anatomy;
      check(a.unit === unit && a.vertices[0] < a.vertices[1] && a.vertices[1] <= foliage.positions.length / 3, id + ' unit vertices');
      check(a.indices[0] < a.indices[1] && a.indices[1] <= foliage.indices.length && a.indices[0] % 3 === 0 && a.indices[1] % 3 === 0, id + ' unit triangles');
      check(a.sections.length > 1 && a.sections.every(([start, end]) => start >= a.vertices[0] && end <= a.vertices[1] && start < end), id + ' transverse sections');
      check(foliage.indices.every(i => i < foliage.positions.length / 3), id + ' indices in bounds');
      const b = foliage.bounds;
      // Include connectors and every transformed prototype vertex in render bounds.
      for (let m = 0; m < foliage.matrices.length; m += 16) {
        for (let v = 0; v < foliage.positions.length; v += 3) {
          for (let axis = 0; axis < 3; axis++) {
            const x = foliage.matrices[m + axis] * foliage.positions[v] + foliage.matrices[m + 4 + axis] * foliage.positions[v + 1] + foliage.matrices[m + 8 + axis] * foliage.positions[v + 2] + foliage.matrices[m + 12 + axis];
            check(x >= b.min[axis] - 1e-5 && x <= b.max[axis] + 1e-5, id + ' transformed bounds');
          }
        }
      }
      const original = output.structure.values.slice();
      const savedMatrices = foliage.matrices.slice();
      const savedAnatomy = JSON.stringify(a);
      engine.release();
      check(foliage.matrices.every((v, i) => v === savedMatrices[i]) && JSON.stringify(a) === savedAnatomy, id + ' released owned foliage');
      await rejects(() => output.field.query(cells), id + ' released field');
      const repeat = engine.build(specimen, { foliage: true, structure: true });
      check(repeat.structure.values.length === original.length && repeat.structure.values.every((v, i) => v === original[i]) && repeat.foliage.matrices.every((v, i) => v === savedMatrices[i]), id + ' deterministic');
      specimen.skeleton.bias.supernatural = { enabled: false, writheAmplitude: 0.1, writheWavelength: 0.3, spiralRate: 2 };
      const natural = engine.build(specimen, { structure: true });
      check(natural.structure.values.length === original.length && natural.structure.values.every((v, i) => v === original[i]), id + ' disabled stored effects');
      for (const mutate of [p => p.element.anatomy = 'missing', p => p.canopy.attachment = 'missing', p => p.element.connectorLength = -1, p => p.element.card = true, p => p.skeleton.habit = { kind: 'missing' }, p => p.skeleton.bias.supernatural.enabled = 1, p => p.skeleton.twigs.twig.stationsPerInternode = 2]) {
        const bad = structuredClone(specimen); mutate(bad);
        await rejects(() => engine.build(bad, { foliage: true }), id + ' invalid anatomy control');
      }
      const woodOnly = engine.build(specimen, { surface: true });
      check(woodOnly.surface.positions.length > 0 && !woodOnly.foliage && !woodOnly.structure && !woodOnly.field && !woodOnly.diagnostics.stages.foliage && woodOnly.diagnostics.biologicalUnits === null, id + ' independent wood surface');
      const fieldOnly = engine.build(specimen, { field: true });
      check(!fieldOnly.foliage && fieldOnly.diagnostics.foliageAnatomy === null && fieldOnly.diagnostics.biologicalUnits === d.biologicalUnits, id + ' field-only unit counts without geometry');
      const zero = structuredClone(specimen); zero.canopy.size = 0;
      const emptyFoliage = engine.build(zero, { foliage: true });
      check(emptyFoliage.foliage.matrices.length === 0 && emptyFoliage.foliage.bounds === null && emptyFoliage.diagnostics.biologicalUnits === 0, id + ' empty biological geometry');
      const limited = structuredClone(specimen); limited.canopy.maxInstances = 1;
      await rejects(() => engine.build(limited, { foliage: true }), id + ' foliage budget rejects instead of truncating');
    }
    const zeroNodes = structuredClone(empty); zeroNodes.skeleton.growth.maxNodes = 0;
    const emptySnapshot = engine.build(zeroNodes, {field:true}).field.snapshot();
    check(Object.values(snapshotArrays(emptySnapshot)).every(a => a.length === 0), 'empty snapshot');
    check(querySnapshot(emptySnapshot,cells).every(v => v === 0), 'empty snapshot occupancy');
    engine.dispose(); engine.dispose();
    await rejects(() => meshFree.field.snapshot(), 'disposed snapshot handle');
    check(querySnapshot(snapshot,cells).every((v,i) => v === hits[i]), 'owned snapshot survives disposal');
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
    check(e.catalogue() === 0, 'native catalogue');
    const catalogue = JSON.parse(new TextDecoder().decode(new Uint8Array(e.memory.buffer, e.metadata_ptr(), e.metadata_len())));
    for (const entry of [...catalogue].reverse()) {
      check(e.preset(entry.abiId) === 0, 'stable ABI identity');
      const parameters = JSON.parse(new TextDecoder().decode(new Uint8Array(e.memory.buffer, e.metadata_ptr(), e.metadata_len())));
      check(JSON.stringify(parameters) === JSON.stringify(entry.family), 'catalogue order independent');
    }
    check(e.preset(999) === 1, 'unknown ABI identity');
    check(e.request_alloc(65537) === 1 && e.build() === 1, 'oversized request leaves no prior request');
    check(raw('{"family":{"skeleton":{"attractors":0}},"outputs":{"surface":true}}') === 0 && e.buffer_len(0) === 0, 'native empty recovery');
    check(raw(JSON.stringify({ family, outputs: { field: true } })) === 0, 'native field-only build');
    check([9,10,11,12,13].every(slot => e.buffer_len(slot) === 0), 'ordinary field has no snapshot copies');
    check([0,1,2,3,4,5].every(slot => e.buffer_len(slot) === 0), 'field-only allocates no render buffers');
    const revision = JSON.parse(new TextDecoder().decode(new Uint8Array(e.memory.buffer,e.metadata_ptr(),e.metadata_len()))).revision;
    check(e.field_snapshot(revision) === 0 && e.buffer_len(9) > 0, 'native explicit snapshot');
    e.field_snapshot_release(); e.field_snapshot_release();
    check([9,10,11,12,13].every(slot => e.buffer_len(slot) === 0), 'native snapshot staging freed');
    check(e.field_snapshot(revision - 1) === 1 && [9,10,11,12,13].every(slot => e.buffer_len(slot) === 0), 'stale snapshot returns no partial buffers');
    check(e.buffer_len(999) === 0 && e.buffer_ptr(999) === 0, 'unknown buffer slot');
    check(e.query_alloc(0) === 0 && e.query(0) === 1, 'no stale query accepted');
    e.release(); e.release();
    return { fieldSnapshots: true, snapshotOwnership: true, snapshotExactFlags: true, speciesAnatomy: true, identityCatalogue: true, independentOutputs: true, meshFree: true, ownership: true, malformed: true, empty: true, staleFields: true, deterministic: true, partialDiagnostics: true };
  });
  await writeFile(out + '/bindings.json', JSON.stringify(bindings, null, 2));
  if (process.env.BINDINGS_ONLY === '1') { console.log(bindings); process.exitCode = 0; } else {
  await page.setViewportSize({ width: 960, height: 720 });
  const captures = [];
  for (const id of ['oregon-white-oak', 'norway-spruce']) {
    for (const view of ['whole', 'bare', 'foliage-detail']) {
      await page.goto(url + '/');
      const capture = await page.evaluate(async ({ id, view }) => {
        const THREE = await import('/node_modules/.vite/deps/three.js');
        const { initializeTreeCore, presetById } = await import('/src/browser/core.ts');
        const { buildPreset, selectSpecimenView, countDraws } = await import('/harness/skeleton-view.ts');
        const { createStage, measureSubject } = await import('/harness/stage.ts');
        await initializeTreeCore();
        const preset = window.compactSpeciesFixture(presetById(id));
        preset.skeleton.seed = 42;
        const stage = createStage(document.querySelector('canvas'));
        let bounds, stats, draws;
        stage.setTree(clay => {
          const built = buildPreset(preset, clay, view !== 'bare');
          stats = built.stats;
          const tree = selectSpecimenView(built.tree, view);
          bounds = measureSubject(tree);
          draws = countDraws(tree);
          tree.traverse(node => {
            if (node instanceof THREE.InstancedMesh && (!Number.isFinite(node.boundingSphere.radius) || node.boundingSphere.radius <= 0)) throw Error('invalid instance sphere');
          });
          return tree;
        });
        if (bounds.isEmpty() || ![...bounds.min, ...bounds.max].every(Number.isFinite)) throw Error('invalid displayed bounds');
        stage.frame(4);
        await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)));
        const gl = document.querySelector('canvas').getContext('webgl2');
        const debug = gl.getExtension('WEBGL_debug_renderer_info');
        const backend = debug ? gl.getParameter(debug.UNMASKED_RENDERER_WEBGL) : gl.getParameter(gl.RENDERER);
        if (stage.stats().drawCalls < draws.drawCalls + (view === 'foliage-detail' ? 1 : 2)) throw Error('subject draws missing');
        window.captureStage = stage;
        return { id, view, preset, stats, draws, bounds: { min: bounds.min.toArray(), max: bounds.max.toArray() }, backend, rendered: stage.stats() };
      }, { id, view });
      await page.screenshot({ path: `${out}/${id}-${view}.png`, timeout: 240000 });
      await page.mouse.move(700, 350);
      await page.mouse.down();
      await page.mouse.move(850, 390, { steps: 8 });
      await page.mouse.up();
      await page.waitForTimeout(500);
      const rotated = await page.evaluate(() => window.captureStage.stats());
      if (rotated.drawCalls < capture.draws.drawCalls + (view === 'foliage-detail' ? 1 : 2) || rotated.triangles <= 0) throw Error(id + ' invisible after orbit');
      captures.push({ ...capture, rotated });
      console.log('captured', id, view, capture.backend, capture.stats.instances);
      await page.evaluate(() => window.captureStage.dispose());
    }
  }
  const emptyRendering = await page.evaluate(async () => {
    const { treeCore, presetById } = await import('/src/browser/core.ts');
    const { materializeTree } = await import('/src/browser/three.ts');
    const { createStage, measureSubject } = await import('/harness/stage.ts');
    const preset = window.compactSpeciesFixture(presetById('norway-spruce'));
    preset.canopy.size = 0;
    const output = treeCore().build(preset, { surface: true, foliage: true });
    treeCore().release();
    const stage = createStage(document.querySelector('canvas'));
    stage.setTree(clay => {
      const tree = materializeTree(output, clay);
      if (tree.getObjectByName('grower-canopy') || measureSubject(tree).isEmpty()) throw Error('empty foliage corrupts wood');
      return tree;
    });
    stage.frame(4);
    await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)));
    if (stage.stats().drawCalls < 3) throw Error('empty foliage interrupts rendering');
    stage.dispose();
    return true;
  });
  await writeFile(out + '/empty-rendering.json', JSON.stringify({ emptyRendering }));
  await writeFile(out + '/captures.json', JSON.stringify(captures, null, 2));
  await page.unroute(url + '/');
  // Apply the same documented small fixture before the first UI render.
  // Full-size Ordinary can exhaust the software GPU during the failure/retry gate.
  await page.route('**/src/browser/core.ts*', async route => {
    const response = await route.fetch();
    const source = await response.text();
    await route.fulfill({ response, body: source + `
      const originalBuild = TreeEngine.prototype.build;
      TreeEngine.prototype.build = function(family, outputs) {
        const fixture = window.compactSpeciesFixture(family);
        const output = originalBuild.call(this, fixture, outputs);
        window.viewerBuild = { seed: fixture.skeleton.seed, signature: JSON.stringify(Array.from(output.foliage?.matrices ?? output.surface?.positions ?? [])) };
        return output;
      };
    ` });
  });
  let failedLoad = false;
  await page.route('**/telperion.wasm', route => {
    if (!failedLoad) { failedLoad = true; return route.abort('failed'); }
    return route.continue();
  });
  await page.goto(url + '/');
  await page.getByRole('alert').waitFor();
  console.log('UI load failure surfaced');
  await page.getByRole('button', { name: 'retry build' }).click();
  await page.getByRole('alert').waitFor({ state: 'detached' });
  const stats = page.locator('.gd-note').filter({ hasText: /tris, .* verts, .* nodes/ });
  await stats.waitFor();
  const before = await stats.textContent();
  console.log('UI initial', before);
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
  for (const name of ['oregon white oak', 'norway spruce']) {
    await page.getByRole('button', { name, exact: true }).click();
    await page.waitForTimeout(1000);
    if (await seed.inputValue() !== '17') throw Error('species selection reset seed');
    if (await page.getByRole('checkbox', { name: 'enable supernatural effects' }).isChecked()) throw Error('natural species enables effects');
    if (await page.locator('fieldset').filter({ hasText: 'Botanical' }).count() !== 1 || await page.locator('fieldset').filter({ hasText: 'Supernatural' }).count() !== 1) throw Error('control groups missing');
    const first = await stats.textContent();
    const firstGeometry = await page.evaluate(() => window.viewerBuild.signature);
    console.log('UI selected', name, await stats.textContent());
    await seed.fill('18');
    await page.waitForFunction(before => [...document.querySelectorAll('.gd-note')].some(node => /tris, .* verts, .* nodes/.test(node.textContent) && node.textContent !== before), first);
    const changed = await page.evaluate(() => window.viewerBuild);
    if (changed.seed !== 18 || changed.signature === firstGeometry) throw Error('UI seed did not vary geometry');
    for (const view of ['bare', 'foliage-detail', 'whole']) {
      await page.getByLabel('view', { exact: true }).selectOption(view);
      await page.waitForTimeout(1000);
      if (await page.getByRole('alert').count()) throw Error('view build failed');
    }
    await seed.fill('17');
  }
  await page.screenshot({ path: out + '/viewer.png' });
  await page.goto(url + '/?species=unknown');
  const linkAlert = page.getByRole('alert');
  await linkAlert.waitFor();
  if (!(await linkAlert.textContent()).includes('Unknown tree preset: unknown')) throw Error('invalid species URL did not explain the identity error');
  await stats.waitFor();
  await page.getByRole('button', { name: 'dismiss link error' }).click();
  await linkAlert.waitFor({ state: 'detached' });
  await page.getByRole('button', { name: 'norway spruce', exact: true }).click();
  await page.waitForTimeout(1000);
  if (await page.getByRole('alert').count()) throw Error('species URL recovery failed');
  await page.screenshot({ path: out + '/viewer-link-recovery.png' });
  console.log('UI invalid species URL recovered');
  await writeFile(out + '/viewer.json', JSON.stringify({ loadFailureRetry: failedLoad, buildFailureRetry: true, previousDiagnosticsPreserved: true, speciesSelection: true, independentSeeds: true, seedGeometryChanges: true, allViews: true, emptyRendering }, null, 2));
  console.log({ ...result, ...bindings, loadFailureRetry: failedLoad, buildFailureRetry: true });
  }
} finally { await browser.close(); }
