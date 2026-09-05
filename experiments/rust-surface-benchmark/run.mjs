import { createServer } from "node:http";
import { spawn, execFileSync } from "node:child_process";
import {
  readFileSync,
  writeFileSync,
  mkdirSync,
  existsSync,
  mkdtempSync,
} from "node:fs";
import { fileURLToPath } from "node:url";
import { join, resolve, extname } from "node:path";
import { tmpdir, cpus, platform, release, totalmem } from "node:os";
import { createHash } from "node:crypto";
import { createRequire } from "node:module";
const require = createRequire(import.meta.url);
process.chdir(fileURLToPath(new URL(".", import.meta.url)));
const mode = process.argv[2];
if (!["test", "bench"].includes(mode))
  throw Error("Usage: node run.mjs test|bench; first run node build.mjs");
if (!existsSync("build/fixtures/manifest.json"))
  throw Error("Missing fixtures; run node build.mjs first.");
const manifest = JSON.parse(readFileSync("build/fixtures/manifest.json"));
mkdirSync("build/native", { recursive: true });
const nativeResults = [];
for (const c of manifest) {
  const args = [
    "build/fixtures/" + c.name + ".bin",
    "build/native/" + c.name + ".bin",
  ];
  const r = JSON.parse(
    execFileSync("target/release/surface-native", args, { encoding: "utf8" }),
  );
  const first = readFileSync(args[1]);
  if (mode === "test") {
    execFileSync("target/release/surface-native", args);
    if (!first.equals(readFileSync(args[1])))
      throw Error("Native nondeterminism " + c.name);
  }
  nativeResults.push({
    name: c.name,
    ...r,
    serializedOutputSha256: createHash("sha256").update(first).digest("hex"),
  });
}
const summarize = (samples) => {
  const s = [...samples].sort((a, b) => a - b);
  return { samples, median: (s[4] + s[5]) / 2, p95: s[9] };
};
for (const r of nativeResults) {
  r.compute = summarize(r.computeMs);
  r.caller = summarize(r.callerMs);
  delete r.computeMs;
  delete r.callerMs;
}
const profile = mkdtempSync(join(tmpdir(), "surface-chromium-"));
let browser;
let timeout;
const result = await new Promise((resolveResult, reject) => {
  const server = createServer((req, res) => {
    if (req.method === "POST" && req.url === "/result") {
      let data = "";
      req.on("data", (part) => (data += part));
      req.on("end", () => {
        res.end("ok");
        server.close();
        try {
          resolveResult(JSON.parse(data));
        } catch (e) {
          reject(e);
        }
      });
      return;
    }
    const rel =
      decodeURIComponent(new URL(req.url, "http://localhost").pathname).slice(
        1,
      ) || "index.html";
    const path = resolve("build", rel);
    if (!path.startsWith(resolve("build") + "/")) {
      res.writeHead(403).end();
      return;
    }
    try {
      const data = readFileSync(path);
      res.setHeader(
        "Content-Type",
        extname(path) === ".wasm"
          ? "application/wasm"
          : extname(path) === ".js"
            ? "text/javascript"
            : extname(path) === ".html"
              ? "text/html"
              : "application/octet-stream",
      );
      res.setHeader("Cross-Origin-Opener-Policy", "same-origin");
      res.setHeader("Cross-Origin-Embedder-Policy", "require-corp");
      res.end(data);
    } catch {
      res.writeHead(404).end();
    }
  });
  server.listen(0, "127.0.0.1", () => {
    const port = server.address().port;
    const executable = process.env.CHROMIUM ?? "/usr/bin/chromium";
    browser = spawn(
      executable,
      [
        "--headless",
        "--no-sandbox",
        "--disable-gpu",
        "--disable-dev-shm-usage",
        "--no-first-run",
        "--disable-background-networking",
        "--enable-precise-memory-info",
        "--user-data-dir=" + profile,
        "http://127.0.0.1:" + port + "/?mode=" + mode,
      ],
      { stdio: ["ignore", "ignore", "pipe"] },
    );
    let errors = "";
    browser.stderr.on("data", (d) => (errors += d));
    browser.on("error", reject);
    browser.on("exit", (code) => {
      if (code)
        reject(Error("Chromium exited " + code + ": " + errors.slice(-4000)));
    });
  });
  timeout = setTimeout(() => {
    server.close();
    reject(Error("Browser timeout after 8 minutes"));
  }, 480000);
}).finally(() => {
  clearTimeout(timeout);
  browser?.kill();
});
if (result.error) throw Error(result.error + "\n" + result.stack);
const version = (cmd) =>
  execFileSync(cmd[0], cmd.slice(1), { encoding: "utf8" }).trim();
const complete = {
  ...result,
  native: nativeResults,
  host: {
    platform: platform(),
    release: release(),
    cpu: cpus()[0]?.model,
    logicalCpus: cpus().length,
    ramBytes: totalmem(),
    node: process.version,
    rustc: version(["rustc", "--version"]),
    cargo: version(["cargo", "--version"]),
    chromium: version([
      process.env.CHROMIUM ?? "/usr/bin/chromium",
      "--version",
    ]),
    three: JSON.parse(
      readFileSync(resolve(require.resolve("three"), "../../package.json")),
    ).version,
  },
  build: {
    profile: "release",
    rustflags: "-D warnings",
    lto: true,
    codegenUnits: 1,
    wasmSha256: createHash("sha256")
      .update(readFileSync("build/surface.wasm"))
      .digest("hex"),
    sourceSha256: Object.fromEntries(
      [
        "rust/lib.rs",
        "rust/main.rs",
        "optimized.ts",
        "browser.ts",
        "shared.ts",
        "fixtures.ts",
        "build.mjs",
        "run.mjs",
        "Cargo.toml",
        "Cargo.lock",
      ].map((p) => [
        p,
        createHash("sha256").update(readFileSync(p)).digest("hex"),
      ]),
    ),
  },
  timestamp: new Date().toISOString(),
};
// Root installation is optional: resolve dependency version through Node's resolver.
mkdirSync(mode === "bench" ? "results" : "build", { recursive: true });
const out =
  mode === "bench" ? "results/measurements.json" : "build/correctness.json";
writeFileSync(out, JSON.stringify(complete, null, 2) + "\n");
console.log(
  JSON.stringify(
    {
      out,
      cases: result.results.length,
      representative: result.results
        .filter((c) => ["telperion", "laurelin"].includes(c.name))
        .map((c) => ({
          name: c.name,
          nodes: c.nodes,
          vertices: c.vertices,
          times: c.times,
        })),
    },
    null,
    2,
  ),
);
// Chromium may still be closing files; temporary profile is OS-temporary and never an input.
