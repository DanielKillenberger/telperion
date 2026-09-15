import type { ChangeRecord, NodeIdentity, PlacementIdentity, Run, Placement } from './specimen';
/** The fixed little-endian record ABI, not a view into retained wasm memory. */
export class SpecimenWire {
  private offset = 0;
  private view: DataView;
  constructor(bytes: Uint8Array) { this.view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength); }
  private u8(): number { return this.view.getUint8(this.offset++); }
  private u32(): number { const n = this.view.getUint32(this.offset, true); this.offset += 4; return n; }
  private u64(): number {
    const n = this.view.getBigUint64(this.offset, true); this.offset += 8;
    const value = Number(n);
    if (!Number.isSafeInteger(value)) throw Error('Specimen identity exceeds JavaScript integer range');
    return value;
  }
  private f64(): number { const n = this.view.getFloat64(this.offset, true); this.offset += 8; return n; }
  private f32(): number { const n = this.view.getFloat32(this.offset, true); this.offset += 4; return n; }
  private vector<T>(read: () => T): T[] { return Array.from({ length: this.u64() }, read); }
  private node = (): NodeIdentity => ({ birth: this.u64(), key: { idx: this.u32(), version: this.u32() } });
  private identity = (): PlacementIdentity => ({ shoot: this.node(), station: this.u32() });
  private placement = (): Placement => ({ identity: this.identity(), transform: Array.from({ length: 16 }, () => this.f32()) });
  private run = (): Run => ({ identity: this.node(), nodes: this.vector(() => ({
    identity: this.node(), parent: this.u8() ? this.node() : null,
    position: { x: this.f64(), y: this.f64(), z: this.f64() },
    radii: [this.f64(), this.f64(), this.f64()], kind: ['Structural', 'Branch', 'Twig'][this.u32()],
    stem: this.u8() !== 0,
  })) });
  identities() { return { nodes: this.vector(this.node), placements: this.vector(this.identity), shed: this.vector(this.node) }; }
  changes(): ChangeRecord {
    return { born_runs: this.vector(this.run), resized_runs: this.vector(this.run), shed_runs: this.vector(this.node),
      born_placements: this.vector(this.placement), moved_placements: this.vector(this.placement), shed_placements: this.vector(this.identity) };
  }
}
