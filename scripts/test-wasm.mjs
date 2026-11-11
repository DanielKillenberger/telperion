import { execFileSync } from 'node:child_process';
import { createServer } from 'vite';

/* The field package in a browser: build the core's module, which the field
 * entry loads (src/field/index.ts grows through TreeEngine and reads
 * telperion.wasm; it reads nothing of the renderer's module), serve the
 * source, and run tests/browser/field.mjs. It grows its trees on the CPU in
 * a worker and needs no WebGPU (fn-353). The binding suite is
 * scripts/test-bindings.mjs. BROWSER_URL can target an already-served tree
 * instead. */
let server;
try {
  if (!process.env.BROWSER_URL) {
    execFileSync(process.execPath, ['scripts/build-wasm.mjs'], { stdio: 'inherit', timeout: 900_000 });
    server = await createServer({ server: { host: '127.0.0.1', port: 0 } });
    await server.listen();
    process.env.BROWSER_URL = `http://127.0.0.1:${server.httpServer.address().port}`;
  }
  await import('../tests/browser/field.mjs');
} finally {
  await server?.close();
}
