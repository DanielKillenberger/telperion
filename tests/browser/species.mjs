import { readFile, writeFile, mkdir, access } from 'node:fs/promises';
import { createHash, randomInt } from 'node:crypto';
import { spawn } from 'node:child_process';
import { tmpdir } from 'node:os';
import { resolve, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const args = process.argv.slice(2);
const option = name => { const i = args.indexOf(name); return i < 0 ? undefined : args[i + 1]; };
if (args.includes('--help')) {
  console.log(`Species QA (run from repository root; mature presets, no generation caps).
  --draw-seeds                 Record fresh seeds once, before generation/tuning
  --seeds FILE                 Default .flow/evidence/fn9/seeds.json
  --output DIR                Default new directory under OS temp
  --measure-only              Native 24-seed measurements per species
  --capture-only              Reuse measurements in --output; do not regenerate
  --case ID                    Capture only this case (partial, never protocol pass)
  --frustum-cull               Conservatively skip offscreen foliage in detail frames
  --batch-instances            Submit original foliage in bounded ordered draws
  --targets FILE               Prior capture receipts pin exterior endpoint/socket IDs
  --timeout-ms N               Per native/capture process limit (default 300000)
BROWSER_URL defaults to http://127.0.0.1:5184 (start Vite separately).
Build species_measure and Wasm first; install Playwright Chromium. Optional
PLAYWRIGHT_MODULE / CHROMIUM_EXECUTABLE overrides. Results checkpoint per case.
Whole and bare share full bounds; foliage-detail shows attached local foliage;
element isolates one unit; junction-detail keeps full wood without local clipping.
Exterior views select a real terminal twig and parent socket, at three angles;
peg views inspect its first attached unit from above/below, with unrestricted depth. JSON records cameras,
parameters, renderer, hashes, numeric/visual/missing/owner fields separately.
Exit 1 for failed/missing required evidence or unassessed visual results.
Human inspection goes in REPORT.md; this runner never awards visual approval.`);
  process.exit(0);
}
const known = new Set(['--draw-seeds', '--seeds', '--output', '--measure-only', '--capture-only', '--case', '--timeout-ms', '--worker', '--frustum-cull', '--targets', '--batch-instances']);
for (let i = 0; i < args.length; i++) {
  if (!known.has(args[i])) throw Error(`Unknown option ${args[i]}`);
  if (['--seeds', '--output', '--case', '--timeout-ms', '--worker', '--targets'].includes(args[i])) {
    if (!args[++i] || args[i].startsWith('--')) throw Error('Missing option value');
  }
}
const timeout = Number(option('--timeout-ms') ?? 300000);
if (!Number.isSafeInteger(timeout) || timeout < 1000) throw Error('Invalid timeout');
const sha = bytes => createHash('sha256').update(bytes).digest('hex');
const json = async path => JSON.parse(await readFile(path, 'utf8'));
const save = (path, value) => writeFile(path, JSON.stringify(value, null, 2) + '\n');
async function command(program, argv, limit = timeout) {
  return new Promise(resolveResult => {
    const child = spawn(program, argv, { stdio: ['ignore', 'pipe', 'pipe'], detached: process.platform !== 'win32' });
    let stdout = '', stderr = '', expired = false;
    child.stdout.on('data', x => { stdout += x; }); child.stderr.on('data', x => { stderr += x; });
    const timer = setTimeout(() => {
      expired = true;
      try { if (process.platform === 'win32') child.kill('SIGKILL'); else process.kill(-child.pid, 'SIGKILL'); } catch { /* already exited */ }
    }, limit);
    child.on('error', error => { clearTimeout(timer); resolveResult({ code: -1, error: String(error), stdout, stderr }); });
    child.on('close', code => { clearTimeout(timer); resolveResult({ code, expired, stdout, stderr }); });
  });
}
async function capture(job) {
  const { chromium } = await import(process.env.PLAYWRIGHT_MODULE ? pathToFileURL(process.env.PLAYWRIGHT_MODULE).href : 'playwright');
  const browser = await chromium.launch({ executablePath: process.env.CHROMIUM_EXECUTABLE, headless: true, args: ['--no-sandbox'] });
  try {
    const page = await browser.newPage({ viewport: { width: 960, height: 720 }, deviceScaleFactor: 1 });
    const url = process.env.BROWSER_URL ?? 'http://127.0.0.1:5184';
    await page.route(url + '/', r => r.fulfill({ contentType: 'text/html', body: '<!doctype html><canvas></canvas>' }));
    await page.goto(url + '/');
    await save(job.result, { ...job, browser: browser.version(), capture_status: 'started' });
    const result = await page.evaluate(async ({ preset: id, seed, view, frustumCull = false, fixedTarget = null, contactAngle = null, batchInstances = false }) => {
      const THREE = await import('/node_modules/.vite/deps/three.js');
      const { TreeEngine, presetById } = await import('/src/browser/core.ts');
      const { materializeTree, recomputeInstanceBounds } = await import('/src/browser/three.ts');
      const { selectSpecimenView } = await import('/harness/skeleton-view.ts');
      const { measureSubject } = await import('/harness/stage.ts');
      const engine = await TreeEngine.create();
      const preset = presetById(id); if (seed !== null) preset.skeleton.seed = seed;
      const output = engine.build(preset, { surface: true, foliage: true, structure: true });
      engine.release();
      const hashes = {};
      for (const [kind, values] of Object.entries({ wood: output.surface.positions, indices: output.surface.indices, foliage: output.foliage.positions, matrices: output.foliage.matrices })) {
        hashes[kind] = [...new Uint8Array(await crypto.subtle.digest('SHA-256', values))].map(x => x.toString(16).padStart(2, '0')).join('');
      }
      const canvas = document.querySelector('canvas');
      const renderer = new THREE.WebGLRenderer({ canvas, antialias: true, preserveDrawingBuffer: true });
      renderer.setPixelRatio(1); renderer.setSize(960, 720); renderer.toneMapping = THREE.NoToneMapping;
      const gl = renderer.getContext(), debug = gl.getExtension('WEBGL_debug_renderer_info');
      const backend = gl.getParameter(debug ? debug.UNMASKED_RENDERER_WEBGL : gl.RENDERER);
      const materials = { surface: new THREE.MeshStandardMaterial({ color: 0x9d968c, roughness: .92 }), element: new THREE.MeshStandardMaterial({ color: 0x9d968c, roughness: .92, side: THREE.DoubleSide, flatShading: true, alphaTest: .5 }) };
      let tree = materializeTree(output, materials);
      let bounds = measureSubject(tree);
      const fullBounds = { min: bounds.min.toArray(), max: bounds.max.toArray() };
      let direction = new THREE.Vector3(.62, .28, 1).normalize();
      const canopy = tree.getObjectByName('grower-canopy');
      if (view === 'bare' && canopy) canopy.visible = false;
      if (view === 'element') {
        tree = selectSpecimenView(tree, 'foliage-detail'); bounds = measureSubject(tree);
        direction = tree.userData.detailDirection ?? direction;
      }
      let selectedInstance = null;
      let selectedTwig = null;
      const exterior = view.startsWith('exterior-') || view.startsWith('peg-') || view.startsWith('socket-');
      if (exterior) {
        const { values, topology } = output.structure;
        const point = i => new THREE.Vector3(...values.subarray(i * 6, i * 6 + 3));
        const outward = new THREE.Vector3(.62, 0, 1).normalize();
        if (view.includes('-alt-')) outward.applyAxisAngle(new THREE.Vector3(0, 1, 0), Math.PI / 2);
        const children = new Uint32Array(topology.length / 3);
        for (let i = 1; i < children.length; i++) children[topology[i * 3]]++;
        let best = -Infinity;
        for (let i = 1; i < children.length; i++) {
          if (topology[i * 3 + 2] !== 2 || children[i]) continue;
          const tip = point(i);
          if (tip.y < fullBounds.max[1] * .25 || tip.y > fullBounds.max[1] * .8) continue;
          const score = tip.dot(outward);
          if (score > best) { best = score; selectedTwig = { node: i }; }
        }
        if (fixedTarget) selectedTwig = { node: fixedTarget.node };
        if (!selectedTwig) throw Error('No exterior terminal twig');
        const node = selectedTwig.node, parent = topology[node * 3], socket = topology[parent * 3];
        if (fixedTarget && (parent !== fixedTarget.parent || socket !== fixedTarget.socket)) throw Error('Retained target topology changed');
        const tip = point(node), base = point(parent), support = point(socket);
        Object.assign(selectedTwig, { parent, socket, tip: tip.toArray(), base: base.toArray(), support: support.toArray() });
        bounds = new THREE.Box3().setFromPoints([tip, base, support]);
        bounds.expandByScalar(id === 'norway-spruce' ? .035 : .12);
        // Camera looks outward through the selected exterior terminal.
        // Original connected wood and all foliage remain intact at unrestricted depth.
        const yaw = view.endsWith('left') ? -.65 : view.endsWith('right') ? .65 : 0;
        direction = outward.clone().negate().applyAxisAngle(new THREE.Vector3(0, 1, 0), yaw);
        direction.y = view.startsWith('peg-') && view.endsWith('upper') ? .65 : view.startsWith('peg-') && view.endsWith('lower') ? -.65 : .18;
        direction.normalize();
        if (view.startsWith('socket-')) {
          const root = view.startsWith('socket-root-');
          const joint = root ? support : base;
          const incoming = root ? point(topology[socket * 3]) : support;
          const outgoing = root ? base : tip;
          const tangent = joint.clone().sub(incoming).normalize();
          const branch = outgoing.clone().sub(joint).normalize();
          let radial = branch.clone().addScaledVector(tangent, -branch.dot(tangent));
          if (radial.lengthSq() < 1e-8) radial = outward.clone().addScaledVector(tangent, -outward.dot(tangent));
          radial.normalize();
          const around = tangent.clone().cross(radial).normalize();
          const angle = contactAngle ?? (view.endsWith('left') ? -.75 : view.endsWith('right') ? .75 : 0);
          direction = radial.clone().multiplyScalar(.6).addScaledVector(around, .8).addScaledVector(tangent, .15).normalize().applyAxisAngle(tangent, angle);
          bounds = new THREE.Box3(joint.clone().addScalar(-.010), joint.clone().addScalar(.010));
          selectedTwig.socketCamera = { jointNode: root ? socket : parent, incomingNode: root ? topology[socket * 3] : socket, departingNode: root ? parent : node, joint: joint.toArray(), tangent: tangent.toArray(), radial: radial.toArray(), angle, halfWidth: .010 };
        }
        if (view.startsWith('peg-')) {
          direction.x = -direction.x; direction.z = -direction.z;
          const axis = tip.clone().sub(base), length2 = axis.lengthSq();
          const matrix = new THREE.Matrix4(), location = new THREE.Vector3();
          let nearest = Infinity;
          for (let i = 0; i < canopy.count; i++) {
            canopy.getMatrixAt(i, matrix); location.setFromMatrixPosition(matrix);
            const fraction = location.clone().sub(base).dot(axis) / length2;
            const distance = location.distanceTo(base.clone().addScaledVector(axis, Math.max(0, Math.min(1, fraction))));
            const surfaceRadius = values[node * 6 + 4] * (1 - fraction) + values[node * 6 + 3] * fraction;
            const contactRadial = location.clone().sub(base.clone().addScaledVector(axis, fraction)).normalize();
            if (fraction > .1 && fraction < .5 && Math.abs(distance - surfaceRadius) < Math.max(.0002, surfaceRadius * .5) && contactRadial.dot(outward) > .5 && fraction < nearest) {
              nearest = fraction; selectedInstance = i;
              selectedTwig.attachment = { fraction, distance, surfaceRadius, origin: location.toArray() };
            }
          }
          if (selectedInstance === null) throw Error('No attached unit on exterior twig');
          canopy.getMatrixAt(selectedInstance, matrix); location.setFromMatrixPosition(matrix);
          // Intersect the actual rendered polygonal surface, including socket
          // sinking/swelling. The centreline residual alone cannot establish contact.
          const fraction = selectedTwig.attachment.fraction;
          const centreline = base.clone().addScaledVector(axis, fraction);
          const radial = location.clone().sub(centreline).normalize();
          const ray = new THREE.Raycaster(centreline, radial, 0, .03);
          tree.updateMatrixWorld(true);
          const wood = tree.getObjectByName('grower-trunk');
          if (!wood?.isMesh) throw Error('Missing rendered wood for contact check');
          const side = wood.material.side; wood.material.side = THREE.DoubleSide;
          const hit = ray.intersectObject(wood, false)[0]; wood.material.side = side;
          selectedTwig.attachment.renderedSurface = hit ? {
            distance: hit.distance, point: hit.point.toArray(), faceIndex: hit.faceIndex,
            originGap: location.distanceTo(centreline) - hit.distance,
          } : null;
          bounds = new THREE.Box3(location.clone().addScalar(-.015), location.clone().addScalar(.015));
          if (view.includes('-contact-') || view.includes('-clear-')) {
            const tangent = axis.clone().normalize();
            const around = tangent.clone().cross(radial).normalize();
            const angle = contactAngle ?? (view.endsWith('left') ? -.85 : view.endsWith('right') ? .85 : 0);
            direction = radial.clone().multiplyScalar(.45).addScaledVector(around, .8).addScaledVector(tangent, .25).normalize().applyAxisAngle(tangent, angle);
            bounds = new THREE.Box3(location.clone().addScalar(-.006), location.clone().addScalar(.006));
            if (view.includes('-clear-')) {
              // Tangential connector profile; all connected scene geometry and depth remain.
              direction = radial.clone().multiplyScalar(.22).addScaledVector(around, .95).addScaledVector(tangent, -.15).normalize().applyAxisAngle(tangent, angle);
              bounds = new THREE.Box3(location.clone().addScalar(-.003), location.clone().addScalar(.003));
            }
            selectedTwig.attachment.contactCamera = { radial: radial.toArray(), tangent: tangent.toArray(), angle, halfWidth: view.includes('-clear-') ? .003 : .006 };
          }
        }
      }
      if (view === 'branch-curtain') {
        const { values, topology } = output.structure;
        const point = i => new THREE.Vector3(...values.subarray(i * 6, i * 6 + 3));
        const outward = new THREE.Vector3(.62, 0, 1).normalize();
        let best = -Infinity;
        for (let i = 1; i < topology.length / 3; i++) {
          if (topology[i * 3 + 2] !== 0) continue;
          const parent = topology[i * 3], base = point(parent), tip = point(i);
          const delta = tip.clone().sub(base).normalize();
          if (delta.y > -.7 || base.y < fullBounds.max[1] * .35 || base.y > fullBounds.max[1] * .65) continue;
          const score = base.dot(outward);
          if (score > best) { best = score; selectedTwig = { node: i, parent, tip: tip.toArray(), base: base.toArray(), protocol: 'actual descending structural secondary, all connected geometry retained' }; }
        }
        if (!selectedTwig) throw Error('No descending structural branch');
        const centre = new THREE.Vector3(...selectedTwig.base).add(new THREE.Vector3(0, -.55, 0));
        bounds = new THREE.Box3(centre.clone().add(new THREE.Vector3(-.8,-.95,-.8)), centre.clone().add(new THREE.Vector3(.8,.65,.8)));
        direction = outward.clone(); direction.y = .12; direction.normalize();
      }
      if (view === 'foliage-detail' || view === 'junction-detail') {
        if (!canopy?.count) throw Error('No attached foliage to inspect');
        selectedInstance = Math.floor(canopy.count / 2);
        const matrix = new THREE.Matrix4(); canopy.getMatrixAt(selectedInstance, matrix);
        const centre = new THREE.Vector3().setFromMatrixPosition(matrix);
        const radius = id === 'norway-spruce' ? .10 : .28;
        // Retain neighbouring original matrices and original wood; camera clips
        // a local shoot, never scales needles/leaves or fabricates attachment.
        const retained = [];
        for (let i = 0; i < canopy.count; i++) {
          canopy.getMatrixAt(i, matrix);
          if (new THREE.Vector3().setFromMatrixPosition(matrix).distanceTo(centre) <= radius) retained.push(...matrix.elements);
        }
        canopy.instanceMatrix = new THREE.InstancedBufferAttribute(new Float32Array(retained), 16);
        canopy.count = retained.length / 16; recomputeInstanceBounds(canopy);
        bounds = new THREE.Box3(centre.clone().addScalar(-radius), centre.clone().addScalar(radius));
      }
      if (bounds.isEmpty() || ![...bounds.min, ...bounds.max].every(Number.isFinite)) throw Error('Missing/invalid bounds');
      const scene = new THREE.Scene(); scene.background = new THREE.Color(0xc6ced5);
      scene.add(new THREE.HemisphereLight(0xffffff, 0x6a6966, 3.1), tree);
      const centre = bounds.getCenter(new THREE.Vector3()), size = bounds.getSize(new THREE.Vector3());
      const distance = Math.max(size.y / 2 / Math.tan(38 * Math.PI / 360), Math.max(size.x, size.z) / 2 / (Math.tan(38 * Math.PI / 360) * 960 / 720)) * 1.3 + size.length() / 2;
      const detail = view === 'foliage-detail' || view === 'junction-detail' || view === 'element' || view === 'branch-curtain' || exterior;
      const camera = new THREE.PerspectiveCamera(38, 960 / 720, detail ? .0001 : .1, Math.max(4000, distance * 4));
      camera.position.copy(centre).addScaledVector(direction, distance); camera.lookAt(centre);
      if (view === 'foliage-detail') { camera.near = Math.max(.0001, distance - size.length() / 2); camera.far = distance + size.length() / 2; camera.updateProjectionMatrix(); }
      let frustumCulling = null;
      if (frustumCull && detail && canopy?.visible && view !== 'element') {
        camera.updateMatrixWorld(true);
        const frustum = new THREE.Frustum().setFromProjectionMatrix(new THREE.Matrix4().multiplyMatrices(camera.projectionMatrix, camera.matrixWorldInverse));
        canopy.geometry.computeBoundingSphere();
        const sphere = canopy.geometry.boundingSphere;
        const centre = sphere.center, source = canopy.instanceMatrix.array;
        const retained = [];
        for (let i = 0; i < canopy.count; i++) {
          const k = i * 16;
          const x = source[k]*centre.x + source[k+4]*centre.y + source[k+8]*centre.z + source[k+12];
          const y = source[k+1]*centre.x + source[k+5]*centre.y + source[k+9]*centre.z + source[k+13];
          const z = source[k+2]*centre.x + source[k+6]*centre.y + source[k+10]*centre.z + source[k+14];
          // Frobenius norm bounds even a sheared transform's maximum stretch.
          const radius = sphere.radius * Math.hypot(source[k],source[k+1],source[k+2],source[k+4],source[k+5],source[k+6],source[k+8],source[k+9],source[k+10]) + 1e-5;
          if (frustum.planes.every(p => p.normal.x*x + p.normal.y*y + p.normal.z*z + p.constant >= -radius)) retained.push(i);
        }
        const matrices = new Float32Array(retained.length * 16);
        retained.forEach((i,j) => matrices.set(source.subarray(i*16,i*16+16),j*16));
        frustumCulling = { originalInstances: canopy.count, submittedInstances: retained.length, method: 'conservative transformed prototype sphere; original order; no wood culling; unchanged camera/depth' };
        canopy.instanceMatrix = new THREE.InstancedBufferAttribute(matrices,16);
        canopy.count = retained.length; recomputeInstanceBounds(canopy);
      }
      let instanceBatches = null;
      if (batchInstances && canopy?.visible && canopy.count > 100000 && view !== 'element') {
        // Bound software-driver draw assembly without dropping or reordering
        // triangles. Every original matrix is submitted exactly once.
        const parent = canopy.parent, source = canopy.instanceMatrix.array;
        const batches = [];
        for (let first = 0; first < canopy.count; first += 100000) {
          const count = Math.min(100000, canopy.count - first);
          const mesh = new THREE.InstancedMesh(canopy.geometry, canopy.material, count);
          mesh.instanceMatrix = new THREE.InstancedBufferAttribute(source.subarray(first * 16, (first + count) * 16), 16);
          mesh.matrix.copy(canopy.matrix); mesh.matrixAutoUpdate = false;
          mesh.frustumCulled = false; mesh.renderOrder = 1 + batches.length;
          parent.add(mesh); batches.push({ first, count });
        }
        parent.remove(canopy);
        instanceBatches = { method: 'ordered original matrix slices; same prototype, material and depth; no omitted instances', batches };
      }
      // Exactly one frame avoids a software GPU animation queue starving capture.
      renderer.render(scene, camera); gl.finish();
      if (gl.isContextLost() || gl.getError() !== gl.NO_ERROR || !renderer.info.render.triangles) throw Error('Missing rendered geometry/context lost');
      const png = canvas.toDataURL('image/png').split(',')[1];
      return { png, preset, diagnostics: output.diagnostics, hashes, backend, userAgent: navigator.userAgent,
        camera: { position: camera.position.toArray(), target: centre.toArray(), near: camera.near, far: camera.far, fov: camera.fov, aspect: camera.aspect },
        fullBounds, displayedBounds: { min: bounds.min.toArray(), max: bounds.max.toArray() }, selectedInstance, selectedTwig,
        frustumCulling, instanceBatches, displayedInstances: canopy?.visible ? canopy.count : 0, rendered: { ...renderer.info.render },
        environment: { width: 960, height: 720, dpr: 1, antialias: true, neutral: true, frames: 1, ground: false, shadows: false } };
    }, job);
    const png = Buffer.from(result.png, 'base64'); delete result.png;
    await writeFile(job.png, png);
    await save(job.result, { ...job, ...result, browser: browser.version(), pngSha256: sha(png), capture_status: 'pass', visual_status: 'unassessed', owner_feedback: null });
  } finally { await browser.close(); }
}
if (option('--worker')) { await capture(await json(option('--worker'))); process.exit(0); }
const seedPath = resolve(option('--seeds') ?? '.flow/evidence/fn9/seeds.json');
const profiles = await json('.flow/evidence/fn9/profiles.json');
if (args.includes('--draw-seeds')) {
  const manifest = { drawn_at: new Date().toISOString(), calibration_commit: (await command('git', ['rev-parse', 'HEAD'])).stdout.trim(), method: 'OS cryptographic random u32; reject only fixed/duplicate seeds', fixed: profiles.protocol.fixed_seeds, fresh: {} };
  for (const profile of profiles.profiles) {
    const used = new Set(manifest.fixed), fresh = [];
    while (fresh.length < 12) { const seed = randomInt(0, 2 ** 32); if (!used.has(seed)) { used.add(seed); fresh.push(seed); } }
    manifest.fresh[profile.id] = fresh;
  }
  await writeFile(seedPath, JSON.stringify(manifest, null, 2) + '\n', { flag: 'wx' });
  console.log(seedPath); process.exit(0);
}
if (args.includes('--measure-only') && args.includes('--capture-only')) throw Error('Conflicting modes');
const seeds = await json(seedPath);
if (JSON.stringify(seeds.fixed) !== JSON.stringify(profiles.protocol.fixed_seeds)) throw Error('Fixed seed protocol mismatch');
for (const p of profiles.profiles) {
  const fresh = seeds.fresh[p.id];
  if (!Array.isArray(fresh) || fresh.length !== 12 || new Set([...seeds.fixed, ...fresh]).size !== 24 || fresh.some(s => !Number.isInteger(s) || s < 0 || s >= 2 ** 32)) throw Error(`Invalid fresh seeds for ${p.id}`);
}
const out = resolve(option('--output') ?? join(tmpdir(), `telperion-species-${Date.now()}`));
await mkdir(out, { recursive: true }); console.log(out);
if (!args.includes('--capture-only')) {
  for (const p of profiles.profiles) for (const seed of [...seeds.fixed, ...seeds.fresh[p.id]]) {
    let exists = false;
    try { await access(join(out, `${p.id}-${seed}.jsonl`)); exists = true; } catch { /* new output */ }
    if (exists) throw Error('Existing numeric evidence: use a new output directory or --capture-only');
  }
  await writeFile(join(out, 'measurement-lock.json'), JSON.stringify({ started: new Date().toISOString() }), { flag: 'wx' });
}
await save(join(out, 'seeds.json'), seeds);
const cases = profiles.profiles.flatMap(p => ['fixed', 'fresh'].flatMap(group => (group === 'fixed' ? seeds.fixed : seeds.fresh[p.id]).map((seed, index) => ({ id: `${p.id}-${seed}`, preset: p.id, seed, group, required: index < 3 }))));
for (const c of cases) {
  const path = join(out, `${c.id}.jsonl`);
  if (!args.includes('--capture-only')) {
    // Native output refuses overwrite; explicit replay uses a new output dir.
    const run = await command('target/release/examples/species_measure', ['--case', `${c.id}:${c.preset}:${c.preset}:${c.seed}`, '--output', path]);
    await save(join(out, `${c.id}-process.json`), run);
  }
  try {
    const events = (await readFile(path, 'utf8')).split('\n').slice(0, -1).map(JSON.parse);
    c.numeric = events.findLast(e => (e.event === 'completed' || e.event === 'failed') && e.case === `${c.id}:${c.preset}:${c.preset}:${c.seed}`) ?? { numeric_status: 'unassessed', reason: 'No completed case' };
  } catch (error) { c.numeric = { numeric_status: 'unassessed', reason: String(error) }; }
  // Retain the original width counterexample even after its numeric repair.
  c.required ||= c.id === 'norway-spruce-4250668600' || c.numeric.numeric_status !== 'pass';
  c.visual_status = 'unassessed'; c.owner_feedback = null;
  console.log(c.id, c.numeric.numeric_status ?? 'failed');
}
await save(join(out, 'numeric.json'), cases);
if (args.includes('--measure-only')) { process.exit(cases.every(c => c.numeric.numeric_status === 'pass') ? 0 : 1); }
const subjects = [...cases.filter(c => c.required), ...['ordinary', 'telperion', 'laurelin'].map(preset => ({ id: preset, preset, seed: null }))];
if (option('--case') && !subjects.some(c => c.id === option('--case'))) throw Error('Unknown or non-required capture case');
const sourceFiles = (await command('git', ['ls-files', 'src/browser', 'harness/stage.ts', 'harness/skeleton-view.ts', 'package-lock.json'])).stdout.trim().split('\n');
const sourceHashes = Object.fromEntries(await Promise.all(sourceFiles.map(async path => [path, sha(await readFile(path))])));
const provenance = { sourceHashes, sourceSha256: sha(JSON.stringify(sourceHashes)), commit: (await command('git', ['rev-parse', 'HEAD'])).stdout.trim(), wasmSha256: sha(await readFile('src/browser/telperion.wasm')), profilesSha256: sha(await readFile('.flow/evidence/fn9/profiles.json')), runnerSha256: sha(await readFile(fileURLToPath(import.meta.url))) };
await save(join(out, 'provenance.json'), provenance);
const retainedTargets = option('--targets') ? (await json(option('--targets'))).captures : [];
if (!Array.isArray(retainedTargets)) throw Error('Invalid target receipts');
const jobs = subjects.flatMap(c => ['whole', 'bare', 'foliage-detail', ...(c === subjects[0] || c.id === 'norway-spruce-1' ? ['element', 'junction-detail', 'exterior-front', 'exterior-left', 'exterior-right', 'exterior-alt-front', 'exterior-alt-left', 'exterior-alt-right', ...(c.id === 'norway-spruce-1' ? ['branch-curtain', 'peg-upper', 'peg-lower', 'peg-alt-upper', 'peg-alt-lower', 'peg-contact-front', 'peg-contact-left', 'peg-contact-right', 'peg-alt-contact-front', 'peg-alt-contact-left', 'peg-alt-contact-right', 'peg-clear-front', 'peg-clear-left', 'peg-clear-right', 'socket-root-front', 'socket-root-left', 'socket-root-right', 'socket-tip-front', 'socket-tip-left', 'socket-tip-right'] : [])] : [])].map(view => ({ id: c.id, preset: c.preset, seed: c.seed, view, batchInstances: args.includes('--batch-instances'), fixedTarget: retainedTargets.find(r => r.id === c.id && r.view === (view.startsWith('socket-') ? 'exterior-' + view.split('-').at(-1) : view.replace('-clear-', '-contact-')))?.selectedTwig ?? null, frustumCull: args.includes('--frustum-cull'), provenance, png: join(out, `${c.id}-${view}.png`), result: join(out, `${c.id}-${view}.json`), capture_status: 'pending', visual_status: 'unassessed', owner_feedback: null })));
const suffix = option('--case') ? `-${option('--case')}` : '';
const capturesPath = join(out, `captures${suffix}.json`);
await save(join(out, `capture-plan${suffix}.json`), jobs);
for (const job of jobs.filter(j => !option('--case') || j.id === option('--case'))) {
  try {
    // Reuse only matching successful receipt + PNG hash; preserve failures.
    const old = await json(job.result);
    if (old.id === job.id && old.view === job.view && !!old.batchInstances === job.batchInstances && JSON.stringify(old.fixedTarget ?? null) === JSON.stringify(job.fixedTarget) && !!old.frustumCull === job.frustumCull && old.capture_status === 'pass' && old.provenance?.sourceSha256 === provenance.sourceSha256 && old.provenance?.wasmSha256 === provenance.wasmSha256 && old.provenance?.runnerSha256 === provenance.runnerSha256 && old.preset.skeleton.seed === (job.seed ?? old.preset.skeleton.seed) && old.pngSha256 === sha(await readFile(job.png))) { Object.assign(job, old); await save(capturesPath, jobs); continue; }
      await save(job.result + `.previous-${Date.now()}`, old);
  } catch { /* capture not available */ }
  const path = join(out, `${job.id}-${job.view}-job.json`); await save(path, job);
  await save(job.result, { ...job, capture_status: 'pending' });
  const run = await command(process.execPath, [fileURLToPath(import.meta.url), '--worker', path], timeout);
  if (run.code === 0) Object.assign(job, await json(job.result));
  else { job.capture_status = 'fail'; job.error = run; await save(job.result, job); }
  await save(capturesPath, jobs);
  console.log(job.id, job.view, job.capture_status);
}
await save(join(out, `summary${suffix}.json`), { protocol_status: 'unassessed', reason: 'Human trait inspection required; see REPORT.md. Missing/failed images never pass.', partial: !!option('--case'), numeric: cases.map(c => ({ id: c.id, status: c.numeric.numeric_status ?? 'unassessed' })), captures: jobs.map(j => ({ id: j.id, view: j.view, status: j.capture_status, path: j.png })), owner_feedback: null });
process.exitCode = 1;
