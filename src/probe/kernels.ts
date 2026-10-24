/// <reference types="vite/client" />
/* fn-256's cycle kernels, the same text the spike compiles: each module is
 * the contract prelude followed by one kernel file, joined exactly as
 * `examples/gpucycle/main.rs` (`shader`) and `chain.rs` join them. The
 * files are imported as raw text from the spike, never copied. */
import wide from "../../crates/telperion-render/src/generation/cycle/u64.wgsl?raw";
import lineage from "../../crates/telperion-render/src/generation/cycle/keys.wgsl?raw";
import contract from "../../crates/telperion-render/src/generation/cycle/decide.wgsl?raw";
import arith from "../../crates/telperion-render/src/generation/cycle/arith.wgsl?raw";
import common from "../../crates/telperion-render/examples/gpucycle/common.wgsl?raw";
import sum from "../../crates/telperion-render/examples/gpucycle/sum.wgsl?raw";
import shed from "../../crates/telperion-render/examples/gpucycle/shed.wgsl?raw";
import flat from "../../crates/telperion-render/examples/gpucycle/flat.wgsl?raw";
import split from "../../crates/telperion-render/examples/gpucycle/split.wgsl?raw";
import light from "../../crates/telperion-render/examples/gpucycle/light.wgsl?raw";
import fate from "../../crates/telperion-render/examples/gpucycle/fate.wgsl?raw";
import chainBody from "../../crates/telperion-render/examples/gpucycle/chain.wgsl?raw";

/** One module and its entry points; binding 0 is the dynamic-offset
 * uniform and bindings 1..=buffers are read-write storage, as `Gpu::kit`
 * lays them out. */
export interface Kit {
  readonly name: string;
  readonly source: string;
  readonly entries: readonly string[];
  readonly buffers: number;
}

const shader = (body: string): string => [wide, lineage, contract, arith, common, body].join("\n");

export const CYCLE: readonly Kit[] = [
  { name: "sum", source: shader(sum), entries: ["walk", "unpack", "block"], buffers: 15 },
  { name: "shed", source: shader(shed), entries: ["local", "walk", "block"], buffers: 18 },
  { name: "flat", source: shader(flat), entries: ["own", "lit_of", "group_sums", "size_of"], buffers: 17 },
  { name: "split", source: shader(split), entries: ["factor", "walk", "block"], buffers: 15 },
  {
    name: "light",
    source: shader(light),
    entries: ["span", "deposit", "densities", "sweep_wg", "sweep_layer", "combine", "read"],
    buffers: 12,
  },
  { name: "fate", source: shader(fate), entries: ["fate", "scan_blocks", "scan_sums", "scan_add"], buffers: 11 },
];

export const CHAIN: Kit = {
  name: "chain",
  source: [wide, lineage, common, chainBody].join("\n"),
  entries: ["step", "noop"],
  buffers: 2,
};
