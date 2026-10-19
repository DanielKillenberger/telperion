import { execFileSync } from 'node:child_process';
import { createServer } from 'vite';

/* The binding suite alone: build the core wasm module, serve the source, and
 * run tests/browser/bindings.mjs in a plain headless Chromium. Nothing here
 * draws, so nothing here needs WebGPU; CI runs this entry on every run.
 * tests/browser/field.mjs needs WebGPU in a worker and stays out of it
 * (scripts/test-wasm.mjs, docs/gpu-tests.md). BROWSER_URL can target an
 * already-served tree instead. */
let server;
try {
  if (!process.env.BROWSER_URL) {
    execFileSync(process.execPath, ['scripts/build-wasm.mjs'], { stdio: 'inherit', timeout: 600_000 });
    server = await createServer({ server: { host: '127.0.0.1', port: 0 } });
    await server.listen();
    process.env.BROWSER_URL = `http://127.0.0.1:${server.httpServer.address().port}`;
  }
  await import('../tests/browser/bindings.mjs');
} finally {
  await server?.close();
}
