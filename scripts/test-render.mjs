import { execFileSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { createServer } from 'vite';

/* The browser conformance run: build the two wasm modules the page loads,
 * serve the harness on a port nobody else is on, and hand the test its URL.
 * BROWSER_URL targets an already-served harness instead.
 *
 * The renderer needs a real GPU, and on Linux a real GPU needs a display: a
 * headless Chromium here is offered SwiftShader and nothing else, which the
 * renderer refuses by design. So an X display is named when the session has
 * one and the caller's environment does not - the test skips rather than
 * fails when that display turns out to offer no adapter either. */
if (!process.env.DISPLAY && !process.env.WAYLAND_DISPLAY && existsSync('/tmp/.X11-unix/X0')) {
  process.env.DISPLAY = ':0';
  console.log('No DISPLAY set; using :0, where this session keeps its X server.');
}

let server;
try {
  if (!process.env.BROWSER_URL) {
    for (const build of ['scripts/build-wasm.mjs', 'scripts/build-render.mjs']) {
      execFileSync(process.execPath, [build], { stdio: 'inherit', timeout: 900_000 });
    }
    server = await createServer({ server: { host: '127.0.0.1', port: 0 } });
    await server.listen();
    process.env.BROWSER_URL = `http://127.0.0.1:${server.httpServer.address().port}`;
  }
  await import('../tests/browser/render.mjs');
} finally {
  await server?.close();
}
