import { pathToFileURL } from 'node:url';
import { writeFile, mkdir } from 'node:fs/promises';

/* ------------------------------------------------------------------ *
 * THE BINDING, IN A BROWSER
 *
 * The Wasm core as a page reaches it: loading, owned copies, the field's
 * grid, independent outputs, the errors it must
 * refuse, and the raw C ABI underneath. None of it draws anything, so
 * none of it needs a GPU - this runs in a plain headless Chromium and
 * has nothing to say about the renderer. The picture is the render
 * suite's business, on the hardware adapter, beside this one.
 * ------------------------------------------------------------------ */

// Shared shrink for small binding and UI fixtures: a short envelope and a
// handful of attractors, and nothing per species - one flat habit row builds
// every tree, so there is no variant knob left to turn down. Binding-specific
// resource limits and internode spacing remain explicit at their call site.
function compactSpeciesFixture(family) {
  const fixture = structuredClone(family);
  fixture.skeleton.envelope.height = 4;
  fixture.skeleton.attractors = 40;
  return fixture;
}
const { chromium } = await import(process.env.PLAYWRIGHT_MODULE ? pathToFileURL(process.env.PLAYWRIGHT_MODULE).href : 'playwright');
const url = process.env.BROWSER_URL ?? 'http://127.0.0.1:5184';
const out = process.env.BROWSER_EVIDENCE ?? '.flow/tmp/bindings';
await mkdir(out, { recursive: true });
const browser = await chromium.launch({ executablePath: process.env.CHROMIUM_EXECUTABLE, headless: true, args: ['--no-sandbox'] });
try {
  const page = await browser.newPage({ viewport: { width: 800, height: 600 }, deviceScaleFactor: 1 });
  await page.addInitScript({ content: `window.compactSpeciesFixture = ${compactSpeciesFixture.toString()};` });
  page.setDefaultTimeout(120000);
  page.on('crash', () => console.error('Browser crashed'));
  browser.on('disconnected', () => console.log('Browser disconnected'));
  await page.route(url + '/', r => r.fulfill({ contentType: 'text/html', body: '<!doctype html><html><body></body></html>' }));
  await page.goto(url + '/');
  const bindings = await page.evaluate(async () => {
    const check = (ok, message) => { if (!ok) throw Error(message); };
    // A `fragment` holds the refusal to the core's own words, so a probe that
    // expects one field's complaint cannot pass on another field's.
    const rejects = async (action, message, fragment) => { let failed = false; try { await action(); } catch (error) { failed = fragment === undefined || String(error).includes(fragment); } check(failed, message); };
    const { TreeEngine, ORDINARY, PRESETS, presetById } = await import('/src/browser/core.ts');
    // The one decoder in the browser source, so this probe reads a leaf the
    // way a consumer does rather than keeping a second copy of the layout.
    const { leafWords, leafTransform } = await import('/src/browser/leaf.ts');
    await rejects(() => TreeEngine.create(new Response('', { status: 503 })), 'load failure');
    await rejects(() => TreeEngine.create(new Uint8Array([0, 1, 2])), 'malformed module');
    const engine = await TreeEngine.create();
    const family = structuredClone(ORDINARY);
    family.skeleton.seed = 42; family.skeleton.attractors = 500; family.skeleton.step = 0.02;
    const meshFree = engine.build(family, { structure: true, field: { n: 24 } });
    check(!meshFree.surface && !meshFree.foliage && !meshFree.diagnostics.stages.surface && meshFree.diagnostics.timings.surfaceMs === 0, 'mesh-free must not invoke surface');
    // FN-9 natural defaults and retained clipped laterals intentionally changed
    // historical 13616/59810 counts. Assert topology/cardinality and exact replay below.
    check(meshFree.diagnostics.nodes > meshFree.diagnostics.crossover && meshFree.diagnostics.leavesPlanned > 0 && meshFree.diagnostics.instances === 0, 'ordinary has local branches and a leaf plan');
    check(meshFree.diagnostics.stages.plan && !meshFree.diagnostics.stages.placement && !meshFree.diagnostics.stages.cull && meshFree.diagnostics.timings.foliageMs === 0, 'field-only reads the plan and places no leaf');
    check(meshFree.structure.values.length === meshFree.diagnostics.nodes * 6 && meshFree.structure.topology.length === meshFree.diagnostics.nodes * 3, 'packed node cardinality');
    check(meshFree.diagnostics.complete, 'complete diagnostics');
    // The field is the grid the request named, one answer a cell in its order.
    const grid = meshFree.field, cells = grid.n ** 3;
    check(grid.n === 24 && grid.cell > 0 && grid.origin.every(Number.isFinite), 'the field is the requested grid');
    const named = meshFree.diagnostics.field;
    check(named.n === grid.n && named.cell === grid.cell && named.origin.every((v, a) => v === grid.origin[a]), 'the diagnostics name the grid');
    check(Object.values(grid.answer).every(a => a.length === cells), 'one answer a cell');
    const at = p => { const [i, j, k] = [0, 1, 2].map(a => Math.floor((p[a] - grid.origin[a]) / grid.cell)); return i + grid.n * (j + grid.n * k); };
    const root = at([0, 0, 0]), { flags, woodRadius, leaves, limbs } = grid.answer;
    check(flags[root] & 1 && woodRadius[root] > 0, 'the trunk cell holds wood and carries its radius');
    check(flags.some(f => f === 3) && limbs.some(l => l !== 0xffffffff), 'field-only includes both materials and a limb without render output');
    check(flags.every((f, i) => (f & 1) === 1 || woodRadius[i] === 0) && flags.every((f, i) => (f & 2) === 2 || (leaves[i] === 0 && limbs[i] === 0xffffffff)), 'an empty cell has no radius, no count and no limb');
    const counted = leaves.reduce((a, b) => a + b, 0);
    check(Math.abs(counted - meshFree.diagnostics.leavesPlanned) <= meshFree.diagnostics.leavesPlanned * 0.01, 'the grid counts the plan');
    // A malformed or out-of-range request is refused by name, before any work.
    for (const [bad, named] of [[true, 'field request'], [{}, 'field request'], [{ n: 24, x: 1 }, 'field request'], [{ n: 24.5 }, 'field request'], [{ n: 24, limbOrder: -1 }, 'field request'], [{ n: 24, limbOrder: 1.5 }, 'field request'], [{ n: 2 }, 'field grid side'], [{ n: 257 }, 'field grid side']])
      await rejects(() => engine.build(family, { field: bad }), 'a malformed field request is refused', named);
    const saved = meshFree.structure.values.slice(), savedFlags = flags.slice();
    engine.release(); engine.release();
    check(saved.every((v,i) => v === meshFree.structure.values[i]), 'owned copies survive release');
    check(savedFlags.every((v,i) => v === meshFree.field.answer.flags[i]), 'the field is an owned copy');
    const structureOnly = engine.build(family, { structure: true });
    check(!structureOnly.surface && !structureOnly.foliage && !structureOnly.field && structureOnly.diagnostics.leavesPlaced === 0, 'optional outputs skipped');
    check(saved.every((v,i) => v === structureOnly.structure.values[i]), 'repeat deterministic');
    // The limb order: left out and the family's own order answer alike, byte
    // for byte, and a deeper order never parts the crown into fewer systems.
    const answers = field => { const q = engine.build(family, { field }).field.answer; return [q.flags, q.woodRadius, q.leaves, q.limbs]; };
    const byDefault = answers({ n: 8 }), byOrder = answers({ n: 8, limbOrder: family.canopy.clumpSystemOrder });
    check(byDefault.every((a, n) => a.length === byOrder[n].length && a.every((v, i) => v === byOrder[n][i])), 'the request without a limb order is the family order');
    const systems = limbs => new Set(limbs.filter(l => l !== 0xffffffff)).size;
    check(systems(answers({ n: 8, limbOrder: family.canopy.clumpSystemOrder + 3 })[3]) >= systems(byDefault[3]), 'a deeper limb order parts no coarser');
    const field = engine.build(family, { field: { n: 8 } }).field, before = field.answer.flags.slice();
    engine.build(family, {});
    check(before.every((v, i) => v === field.answer.flags[i]), 'a rebuild leaves an earlier field as it was');
    for (const bad of [NaN, Infinity, -1]) {
      const p = structuredClone(family); p.skeleton.envelope.height = bad;
      await rejects(() => engine.build(p, { surface: true }), 'bad height');
    }
    const partial = structuredClone(family); partial.skeleton.growth.maxNodes = 20;
    check(!engine.build(partial, {}).diagnostics.complete, 'caps explicitly partial');
    /* A tree with nowhere to grow is the node ceiling at zero now, not an
       empty attractor set: one builder grows every axis from the trait row,
       so a family with no attractors and no pull for them is still a tree,
       and a positive pull with none to pull toward is refused. */
    const empty = structuredClone(family); empty.skeleton.growth.maxNodes = 0;
    const result = engine.build(empty, { surface: true, foliage: true });
    check(result.surface.positions.length === 0 && result.surface.bounds === null && result.foliage.leaves.length === 0, 'valid empty outputs');
    check(PRESETS.length === 6 && new Set(PRESETS.map(p => p.id)).size === 6, 'complete identity catalogue');
    await rejects(() => presetById('european-beech'), 'the beech in work is not listed');
    await rejects(() => engine.build('european-beech', {}), 'the beech in work is not built by name');
    await rejects(() => presetById('missing'), 'unknown browser identity');
    await rejects(() => engine.build('missing', {}), 'unknown native identity');
    for (const [id, unit] of [['oregon-white-oak', 'leaf'], ['norway-spruce', 'needle'], ['silver-birch', 'leaf']]) {
      const specimen = window.compactSpeciesFixture(presetById(id));
      // Small valid fixtures retain the authored habit and element rows.
      // The birch stands on two stems, so each carries half the base through
      // the pipe model and its wood is thinner all the way out; at 4 m the
      // twig layer finds nothing long enough to clothe. Eight metres is the
      // smallest fixture that keeps a leaf on it.
      if (id === 'silver-birch') specimen.skeleton.envelope.height = 8;
      specimen.skeleton.growth.maxNodes = 12000;
      // A ceiling, not a target: the beech's short shoots put five leaves on
      // every 2.5 cm of its limbs, about 30,000 on this fixture's wood.
      specimen.canopy.maxInstances = 40000;
      specimen.skeleton.twigs.twig.internodeLength = 0.04;
      const output = engine.build(specimen, { foliage: true, structure: true, field: { n: 24 } });
      const foliage = output.foliage, d = output.diagnostics;
      check(d.complete, id + ' small binding fixture completes without truncation');
      check(!output.surface && !d.stages.surface && d.timings.surfaceMs === 0, id + ' independent foliage');
      // Twelve bytes a leaf: three u32 words, and one reference box the whole
      // crown decodes against.
      check(d.instances > 0 && d.biologicalUnits === d.instances && foliage.leaves.length === d.instances * 3, id + ' one biological unit per packed leaf');
      check(foliage.reference && foliage.reference.min.every(Number.isFinite) && foliage.reference.extent.every(e => Number.isFinite(e) && e >= 0), id + ' foliage reference box');
      const a = foliage.anatomy;
      check(a.unit === unit && a.vertices[0] < a.vertices[1] && a.vertices[1] <= foliage.positions.length / 3, id + ' unit vertices');
      check(a.indices[0] < a.indices[1] && a.indices[1] <= foliage.indices.length && a.indices[0] % 3 === 0 && a.indices[1] % 3 === 0, id + ' unit triangles');
      check(a.sections.length > 1 && a.sections.every(([start, end]) => start >= a.vertices[0] && end <= a.vertices[1] && start < end), id + ' transverse sections');
      check(foliage.indices.every(i => i < foliage.positions.length / 3), id + ' indices in bounds');
      const b = foliage.bounds;
      // Include connectors and every transformed prototype vertex in render bounds.
      // The core takes its own bounds from these same decoded words, so the
      // tolerance stays the f32 prototype-vertex one: the position quantum the
      // reference box implies is extent/65535 per axis, under a millimetre on
      // any fixture here, and it is already inside the reported bounds.
      for (let i = 0; i < d.instances; i++) {
        const m = leafTransform(leafWords(foliage.leaves, i), foliage.reference);
        for (let v = 0; v < foliage.positions.length; v += 3) {
          for (let axis = 0; axis < 3; axis++) {
            const x = m[axis] * foliage.positions[v] + m[4 + axis] * foliage.positions[v + 1] + m[8 + axis] * foliage.positions[v + 2] + m[12 + axis];
            check(x >= b.min[axis] - 1e-5 && x <= b.max[axis] + 1e-5, id + ' transformed bounds');
          }
        }
      }
      const original = output.structure.values.slice();
      const savedLeaves = foliage.leaves.slice();
      const savedAnatomy = JSON.stringify(a);
      engine.release();
      check(foliage.leaves.every((v, i) => v === savedLeaves[i]) && JSON.stringify(a) === savedAnatomy, id + ' released owned foliage');
      check(output.field.answer.flags.length === 24 ** 3, id + ' released field kept');
      const repeat = engine.build(specimen, { foliage: true, structure: true });
      check(repeat.structure.values.length === original.length && repeat.structure.values.every((v, i) => v === original[i]) && repeat.foliage.leaves.every((v, i) => v === savedLeaves[i]), id + ' deterministic');
      specimen.skeleton.bias.supernatural = { enabled: false, writheAmplitude: 0.1, writheWavelength: 0.3, spiralRate: 2 };
      const natural = engine.build(specimen, { structure: true });
      check(natural.structure.values.length === original.length && natural.structure.values.every((v, i) => v === original[i]), id + ' disabled stored effects');
      // Each refusal is the core naming the field: the three retired tags are
      // unknown keys under a closed schema, and a trait outside its range is
      // the trait by name.
      for (const [named, mutate] of [
        ['unknown element trait', p => p.element.anatomy = 'missing'],
        ['unknown canopy trait', p => p.canopy.attachment = 'missing'],
        ['unknown habit trait', p => p.skeleton.habit = { kind: 'missing' }],
        ['crookedness', p => p.skeleton.habit.crookedness = 90],
        ['foliage connector length', p => p.element.connectorLength = -1],
        ['leaf card carries no lobes and no section roundness', p => { p.element.card = true; p.element.lobeCount = 1; }],
        ['/skeleton/bias/supernatural/enabled', p => p.skeleton.bias.supernatural.enabled = 1],
      ]) {
        const bad = structuredClone(specimen); mutate(bad);
        await rejects(() => engine.build(bad, { foliage: true }), id + ' refuses ' + named, named);
      }
      const woodOnly = engine.build(specimen, { surface: true });
      check(woodOnly.surface.positions.length > 0 && !woodOnly.foliage && !woodOnly.structure && !woodOnly.field && !woodOnly.diagnostics.stages.foliage && woodOnly.diagnostics.biologicalUnits === null, id + ' independent wood surface');
      const fieldOnly = engine.build(specimen, { field: { n: 24 } });
      check(!fieldOnly.foliage && fieldOnly.diagnostics.foliageAnatomy === null && fieldOnly.diagnostics.biologicalUnits === null && fieldOnly.diagnostics.leavesPlaced === 0, id + ' field-only places no leaf and reports no units');
      check(fieldOnly.diagnostics.stages.plan && !fieldOnly.diagnostics.stages.placement && fieldOnly.diagnostics.leavesPlanned >= d.instances, id + ' field-only plan counts at least the retained leaves');
      const both = engine.build(specimen, { foliage: true, field: { n: 24 } });
      check(both.diagnostics.stages.plan && both.diagnostics.stages.placement && both.diagnostics.biologicalUnits === d.biologicalUnits, id + ' foliage with field places for the render and plans for the field');
      const zero = structuredClone(specimen); zero.canopy.size = 0;
      const emptyFoliage = engine.build(zero, { foliage: true });
      check(emptyFoliage.foliage.leaves.length === 0 && emptyFoliage.foliage.bounds === null && emptyFoliage.diagnostics.biologicalUnits === 0, id + ' empty biological geometry');
      const limited = structuredClone(specimen); limited.canopy.maxInstances = 1;
      await rejects(() => engine.build(limited, { foliage: true }), id + ' foliage budget rejects instead of truncating');
    }
    await rejects(() => engine.build(empty, { field: { n: 8 } }), 'an empty tree has no field', 'field of an empty tree');
    engine.dispose(); engine.dispose();
    check(savedFlags.every((v,i) => v === meshFree.field.answer.flags[i]), 'the field survives disposal');
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
    check(raw('{"family":{"skeleton":{"growth":{"maxNodes":0}}},"outputs":{"surface":true}}') === 0 && e.buffer_len(0) === 0, 'native empty recovery');
    check(raw(JSON.stringify({ family, outputs: { field: { n: 8 } } })) === 0, 'native field-only build');
    check([8,19,20,25].every(slot => e.buffer_len(slot) === 8 ** 3), 'the native field is one answer a cell');
    check([0,1,2,3,4,5].every(slot => e.buffer_len(slot) === 0), 'field-only allocates no render buffers');
    check(['field_snapshot', 'query_alloc', 'query'].every(name => !(name in e)), 'no cube query and no snapshot at the native boundary');
    check(e.buffer_len(999) === 0 && e.buffer_ptr(999) === 0, 'unknown buffer slot');
    e.release(); e.release();
    return { fieldGrid: true, speciesAnatomy: true, identityCatalogue: true, independentOutputs: true, meshFree: true, ownership: true, malformed: true, empty: true, staleFields: true, deterministic: true, partialDiagnostics: true };
  });
  await writeFile(out + '/bindings.json', JSON.stringify(bindings, null, 2));
  console.log(bindings);
} finally { await browser.close(); }
