import { execFileSync } from 'node:child_process';
import { createServer } from 'vite';

/* The binding suite: build the core wasm module, serve the harness, and run the
 * contract the page holds the binding to. Nothing here draws, so nothing here
 * needs an adapter - the renderer has its own suite. BROWSER_URL can target an
 * already-built harness instead. */
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
