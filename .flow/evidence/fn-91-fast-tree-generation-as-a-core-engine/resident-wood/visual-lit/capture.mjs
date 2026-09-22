import { createServer } from "vite";
import { chromium } from "playwright";
import { writeFile, readFile } from "node:fs/promises";
import { createHash } from "node:crypto";
const directory = ".flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/resident-wood/visual-lit";
const server = await createServer({ server: { host: "127.0.0.1", port: 0 } });
await server.listen();
const url = `http://127.0.0.1:${server.httpServer.address().port}`;
const flags = ["--no-sandbox", "--enable-unsafe-webgpu", "--enable-features=Vulkan", "--use-angle=vulkan", "--disable-vulkan-surface", "--ignore-gpu-blocklist"];
const browser = await chromium.launch({ executablePath: "/usr/bin/chromium", headless: false, args: flags });
const result = { date: new Date().toISOString(), browser: browser.version(), flags, viewport: { width: 1280, height: 720 }, seed: 1, view: "bare", framing: "exterior hero", wasmSha256: createHash("sha256").update(await readFile("src/browser/render/telperion_render_bg.wasm")).digest("hex"), captures: [] };
try {
  for (const preset of ["oregon-white-oak", "norway-spruce"]) {
    let pose;
    for (const mode of ["cpu", "gpu"]) {
      const page = await browser.newPage({ viewport: result.viewport });
      await page.route(url + "/", route => route.fulfill({ contentType: "text/html", body: '<style>html,body{margin:0}canvas{width:1280px;height:720px;display:block}</style><canvas></canvas>' }));
      await page.goto(url);
      const capture = await page.evaluate(async ({ preset, mode, pose }) => {
        const { createRenderer } = await import("/src/browser/render.ts");
        const { presetById } = await import("/src/browser/core.ts");
        const { familyJson, presetToParams } = await import("/harness/family.ts");
        const family = familyJson({ ...presetToParams(presetById(preset)), seed: 1 });
        const original = GPUAdapter.prototype.requestDevice;
        let device;
        GPUAdapter.prototype.requestDevice = async function (...args) { device = await original.apply(this, args); return device; };
        let renderer;
        try { renderer = await createRenderer(document.querySelector("canvas")); }
        finally { GPUAdapter.prototype.requestDevice = original; }
        await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)));
        const submitted = mode === "gpu" ? await renderer.setTreeGpu(family) : renderer.setTree(family);
        if (mode === "gpu" && (submitted.backend !== "Gpu" || submitted.stages.woodBackend !== "Gpu")) throw Error("unexpected CPU fallback");
        const hero = renderer.hero();
        if (!pose) pose = hero;
        renderer.setCamera(pose);
        renderer.setView("bare");
        const delta = pose.position.map((v, i) => v - pose.target[i]);
        const azimuth = (Math.atan2(delta[0], delta[2]) * 180 / Math.PI + 360) % 360;
        renderer.setScene({ ...renderer.scene(), sunAzimuth: azimuth, sunElevation: 25,
          sunRed: 5, sunGreen: 5, sunBlue: 5,
          skyZenithRed: 1.5, skyZenithGreen: 1.5, skyZenithBlue: 1.5,
          skyHorizonRed: 1.5, skyHorizonGreen: 1.5, skyHorizonBlue: 1.5,
          groundRed: 0.45, groundGreen: 0.45, groundBlue: 0.45 });
        renderer.frame();
        await device.queue.onSubmittedWorkDone();
        await new Promise(resolve => requestAnimationFrame(resolve));
        window.captureRenderer = renderer;
        return { preset, mode, pose, hero, view: "bare", scene: renderer.scene(), submitted, stats: renderer.stats() };
      }, { preset, mode, pose });
      pose = capture.pose;
      capture.image = `wood-detail-${preset}-${mode}.png`;
      await page.locator("canvas").screenshot({ path: `${directory}/${capture.image}` });
      result.captures.push(capture);
      await writeFile(`${directory}/wood-detail.json`, JSON.stringify(result, null, 2) + "\n");
      console.log(preset, mode, capture.submitted.foliageInstances, capture.image);
      await page.evaluate(() => window.captureRenderer.dispose());
      await page.close();
    }
  }
} catch (error) { result.error = String(error); throw error; }
finally {
  await writeFile(`${directory}/wood-detail.json`, JSON.stringify(result, null, 2) + "\n");
  await browser.close();
  await server.close();
}
