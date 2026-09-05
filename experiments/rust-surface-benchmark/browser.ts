import { buildSurface as reference } from "./frozen/src/mesh/surface";
import { buildSurface as optimized } from "./optimized";
import { pack, unpack, summary } from "./shared";
type Mesh = { positions: Float32Array; indices: Uint32Array };
function assert(ok: boolean, message: string) {
  if (!ok) throw Error(message);
}
async function hash(a: ArrayBufferView) {
  return [
    ...new Uint8Array(
      await crypto.subtle.digest(
        "SHA-256",
        a.buffer.slice(a.byteOffset, a.byteOffset + a.byteLength),
      ),
    ),
  ]
    .map((x) => x.toString(16).padStart(2, "0"))
    .join("");
}
async function meshHash(m: Mesh) {
  const bytes = new Uint8Array(m.positions.byteLength + m.indices.byteLength);
  bytes.set(
    new Uint8Array(
      m.positions.buffer,
      m.positions.byteOffset,
      m.positions.byteLength,
    ),
  );
  bytes.set(
    new Uint8Array(
      m.indices.buffer,
      m.indices.byteOffset,
      m.indices.byteLength,
    ),
    m.positions.byteLength,
  );
  return hash(bytes);
}
function compare(ref: Mesh, m: Mesh, scale: number, label: string) {
  assert(
    m.positions.length === ref.positions.length &&
      m.indices.length === ref.indices.length,
    label + " counts",
  );
  // Selected before running: 8 float32 epsilons at the input coordinate scale.
  const tolerance = 8 * 2 ** -23 * Math.max(1, scale);
  let maxError = 0;
  for (let i = 0; i < m.positions.length; i++) {
    assert(Number.isFinite(m.positions[i]), label + " nonfinite " + i);
    maxError = Math.max(maxError, Math.abs(m.positions[i] - ref.positions[i]));
  }
  assert(
    maxError <= tolerance,
    label + " position error " + maxError + " > " + tolerance,
  );
  for (let i = 0; i < m.indices.length; i++)
    assert(
      m.indices[i] < m.positions.length / 3 && m.indices[i] === ref.indices[i],
      label + " index/winding " + i,
    );
  return { maxError, tolerance };
}
function closed(m: Mesh) {
  const edges = new Map<string, { count: number; balance: number }>();
  for (let i = 0; i < m.indices.length; i += 3)
    for (let k = 0; k < 3; k++) {
      const a = m.indices[i + k],
        b = m.indices[i + ((k + 1) % 3)];
      const key = a < b ? a + "," + b : b + "," + a;
      const edge = edges.get(key) ?? { count: 0, balance: 0 };
      edge.count++;
      edge.balance += a < b ? 1 : -1;
      edges.set(key, edge);
    }
  assert(
    [...edges.values()].every((x) => x.count === 2 && x.balance === 0),
    "oriented boundary",
  );
}
function nativeMesh(b: ArrayBuffer): Mesh {
  const h = new Uint32Array(b, 0, 2);
  return {
    positions: new Float32Array(b, 8, h[0]),
    indices: new Uint32Array(b, 8 + h[0] * 4, h[1]),
  };
}
async function main() {
  const mode = new URLSearchParams(location.search).get("mode") ?? "test";
  const manifest = await (await fetch("fixtures/manifest.json")).json();
  const coldStart = performance.now();
  const wasm = await WebAssembly.instantiateStreaming(
    fetch("surface.wasm"),
    {},
  );
  const coldMs = performance.now() - coldStart;
  const ex: any = wasm.instance.exports;
  const load = (a: Float64Array) => {
    const ptr = ex.input_resize(a.length);
    new Float64Array(ex.memory.buffer, ptr, a.length).set(a);
  };
  const output = (): Mesh => ({
    positions: new Float32Array(
      ex.memory.buffer,
      ex.positions_ptr(),
      ex.positions_len(),
    ).slice(),
    indices: new Uint32Array(
      ex.memory.buffer,
      ex.indices_ptr(),
      ex.indices_len(),
    ).slice(),
  });
  const results = [];
  let sink = 0;
  for (const meta of manifest) {
    const binary = await (
      await fetch("fixtures/" + meta.name + ".bin")
    ).arrayBuffer();
    assert(
      (await hash(new Uint8Array(binary))) === meta.inputSha256,
      "input hash",
    );
    const a = new Float64Array(binary);
    const c = unpack(a);
    const call = (f: any) => f(c.skeleton, c.field, c.envelope, c.params);
    const ref = call(reference);
    const opt = call(optimized);
    load(a);
    ex.compute();
    const rust = output();
    const native = nativeMesh(
      await (await fetch("native/" + meta.name + ".bin")).arrayBuffer(),
    );
    let scale = 1;
    for (const node of c.skeleton.nodes)
      scale = Math.max(
        scale,
        Math.abs(node.position.x),
        Math.abs(node.position.y),
        Math.abs(node.position.z),
      );
    const correctness = {
      optimized: compare(ref, opt, scale, "optimized " + meta.name),
      wasm: compare(ref, rust, scale, "wasm " + meta.name),
      native: compare(ref, native, scale, "native " + meta.name),
    };
    if (meta.nodes < 100) closed(ref);
    const hashes = {
      reference: await meshHash(ref),
      optimized: await meshHash(opt),
      wasm: await meshHash(rust),
      native: await meshHash(native),
    };
    assert(hashes.reference === hashes.optimized, "TS byte identity");
    assert(
      (await meshHash(call(reference))) === hashes.reference,
      "reference determinism",
    );
    assert(
      (await meshHash(call(optimized))) === hashes.optimized,
      "optimized determinism",
    );
    load(a);
    ex.compute();
    assert((await meshHash(output())) === hashes.wasm, "Wasm determinism");
    const times: any = {};
    let jsHeapObservedMax: number | null = null;
    let linearMemoryObservedMax = ex.memory.buffer.byteLength;
    if (mode === "bench" && ["telperion", "laurelin"].includes(meta.name)) {
      const impls = [
        {
          name: "reference",
          compute: () => call(reference),
          caller: () => call(reference),
        },
        {
          name: "optimized",
          compute: () => call(optimized),
          caller: () => call(optimized),
        },
        {
          name: "wasm",
          compute: () => {
            ex.compute();
            return null;
          },
          caller: () => {
            load(pack(c));
            ex.compute();
            return output();
          },
        },
      ];
      for (const impl of impls) {
        load(a);
        for (let j = 0; j < 3; j++) {
          impl.compute();
          impl.caller();
        }
        times[impl.name] = { compute: [], caller: [] };
      }
      // Rotate order each repetition to reduce fixed order / thermal bias.
      for (let j = 0; j < 10; j++)
        for (let k = 0; k < impls.length; k++) {
          const impl = impls[(j + k) % impls.length];
          load(a);
          for (const kind of ["compute", "caller"] as const) {
            const t = performance.now();
            const m = impl[kind]();
            const elapsed = performance.now() - t;
            times[impl.name][kind].push(elapsed);
            sink += m
              ? m.positions.length + m.indices.length
              : ex.positions_len();
          }
          const heap = (performance as any).memory?.usedJSHeapSize;
          if (heap !== undefined)
            jsHeapObservedMax = Math.max(jsHeapObservedMax ?? 0, heap);
          linearMemoryObservedMax = Math.max(
            linearMemoryObservedMax,
            ex.memory.buffer.byteLength,
          );
        }
      for (const impl of Object.keys(times))
        for (const kind of ["compute", "caller"])
          times[impl][kind] = summary(times[impl][kind]);
    }
    results.push({
      ...meta,
      vertices: ref.positions.length / 3,
      triangles: ref.indices.length / 3,
      outputBytes: ref.positions.byteLength + ref.indices.byteLength,
      correctness,
      hashes,
      times,
      memory: {
        jsHeapObservedMax,
        wasmLinearMemoryObservedMax: linearMemoryObservedMax,
        totalBrowserPeak: null,
      },
    });
    (document.querySelector("#status") as HTMLElement).textContent =
      "Finished " + meta.name;
  }
  return {
    mode,
    environment: {
      userAgent: navigator.userAgent,
      hardwareConcurrency: navigator.hardwareConcurrency,
      crossOriginIsolated,
      wasmColdFetchCompileInstantiateMs: coldMs,
    },
    warmups: 3,
    repetitions: 10,
    results,
    sink,
  };
}
main()
  .then(async (result) => {
    await fetch("/result", { method: "POST", body: JSON.stringify(result) });
  })
  .catch(async (error) => {
    await fetch("/result", {
      method: "POST",
      body: JSON.stringify({ error: String(error), stack: error.stack }),
    });
  });
