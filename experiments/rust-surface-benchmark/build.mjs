import { build } from "esbuild";
import { mkdirSync, copyFileSync, writeFileSync, readFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { execFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";
process.chdir(fileURLToPath(new URL(".", import.meta.url)));
const provenance = JSON.parse(readFileSync("provenance.json"));
for (const [p, expected] of Object.entries(provenance.sha256)) {
  const actual = createHash("sha256")
    .update(readFileSync("frozen/" + p))
    .digest("hex");
  if (actual !== expected) throw Error("Frozen source changed: " + p);
}
mkdirSync("build", { recursive: true });
for (const target of [[], ["--target", "wasm32-unknown-unknown", "--lib"]])
  execFileSync(
    "cargo",
    ["build", "--release", "--manifest-path", "Cargo.toml", ...target],
    { stdio: "inherit", env: { ...process.env, RUSTFLAGS: "-D warnings" } },
  );
copyFileSync(
  "target/wasm32-unknown-unknown/release/surface_benchmark.wasm",
  "build/surface.wasm",
);
await build({
  absWorkingDir: process.cwd(),
  entryPoints: ["fixtures.ts"],
  bundle: true,
  platform: "node",
  format: "esm",
  outfile: "build/fixtures.mjs",
});
await build({
  absWorkingDir: process.cwd(),
  entryPoints: ["browser.ts"],
  bundle: true,
  platform: "browser",
  format: "esm",
  outfile: "build/browser.js",
});
writeFileSync(
  "build/index.html",
  '<!doctype html><meta charset="utf-8"><title>Surface benchmark</title><pre id="status">Running</pre><script type="module" src="browser.js"></script>',
);
execFileSync(process.execPath, ["build/fixtures.mjs"], { stdio: "inherit" });
