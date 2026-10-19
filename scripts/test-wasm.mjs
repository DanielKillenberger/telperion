import { execFileSync } from 'node:child_process';
import { createServer } from 'vite';

/* The field package in a browser: build the core wasm module, serve the
 * source, and run tests/browser/field.mjs. It grows its trees on a WebGPU
 * device in a worker, so it needs a browser with WebGPU and runs only where
 * one is (docs/gpu-tests.md); no CI job runs it. The binding suite, which
 * needs no GPU, is scripts/test-bindings.mjs. BROWSER_URL can target an
 * already-served tree instead. */
let server;
try {
  if (!process.env.BROWSER_URL) {
    execFileSync(process.execPath, ['scripts/build-wasm.mjs'], { stdio: 'inherit', timeout: 600_000 });
    server = await createServer({ server: { host: '127.0.0.1', port: 0 } });
    await server.listen();
    process.env.BROWSER_URL = `http://127.0.0.1:${server.httpServer.address().port}`;
  }
  await import('../tests/browser/field.mjs');
} finally {
  await server?.close();
}
