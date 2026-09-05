import { execFileSync } from 'node:child_process';
import { createServer } from 'vite';

// BROWSER_URL can target an already-built harness. Otherwise build and serve it here.
let server;
try {
  if (!process.env.BROWSER_URL) {
    execFileSync(process.execPath, ['scripts/build-wasm.mjs'], { stdio: 'inherit', timeout: 600_000 });
    server = await createServer({ server: { host: '127.0.0.1', port: 0 } });
    await server.listen();
    process.env.BROWSER_URL = `http://127.0.0.1:${server.httpServer.address().port}`;
  }
  await import('../tests/browser/integration.mjs');
} finally {
  await server?.close();
}
