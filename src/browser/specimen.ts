import { SpecimenWire } from './specimen-wire';
import type { Family } from './core';
import type { LeafReference, LeafWords } from './leaf';

export interface NodeIdentity { birth: number; key: { idx: number; version: number } }
export interface PlacementIdentity { shoot: NodeIdentity; station: number }
/** The three packed words of one leaf; `./leaf` decodes them. */
export interface Placement { identity: PlacementIdentity; leaf: LeafWords }
export interface Run {
  identity: NodeIdentity;
  nodes: { identity: NodeIdentity; parent: NodeIdentity | null;
    position: { x: number; y: number; z: number }; radii: number[]; kind: string; stem: boolean }[];
}
export interface ChangeRecord {
  born_runs: Run[]; resized_runs: Run[]; shed_runs: NodeIdentity[];
  born_placements: Placement[]; moved_placements: Placement[]; shed_placements: PlacementIdentity[];
}
export interface SpecimenRead {
  age: number; envelope: Family['skeleton']['envelope']; surfaceHeight: number;
  diagnostics: { node_capped: boolean; level_capped: boolean; attraction_capped: boolean };
  crossover: number; shed: NodeIdentity[]; nodes: NodeIdentity[]; placements: PlacementIdentity[];
  structure: { values: Float64Array; topology: Uint32Array };
  /** Three u32 words a leaf, in placement order, and the box they decode
   * against - one box for the family, so every age reads the same one. */
  leaves: Uint32Array; foliageReference: LeafReference;
}
/** Owned schema-3 little-endian chronicle and writer frontiers, without meshes.
 * Caller mutation never reaches a retained specimen. */
export interface SpecimenSnapshot { schema: 3; data: Uint8Array }
export interface SpecimenHandle {
  readonly frontier: number;
  readonly historyCap: number;
  read(age?: number): SpecimenRead;
  advance(years: number): { frontier: number; changes: ChangeRecord };
  changes(from: number, to: number): ChangeRecord;
  setNodeCeiling(limit: number): void;
  setHistoryCap(years: number): void;
  snapshot(): SpecimenSnapshot;
  release(): void;
}
export interface SpecimenExports {
  memory: WebAssembly.Memory;
  request_alloc(n: number): number; request_ptr(): number;
  buffer_ptr(slot: number): number; buffer_len(slot: number): number;
  specimen_node_ceiling(handle: number, limit: number): number;
  specimen_history_cap(handle: number, years: number): number;
  specimen_build(cap: number): number;
  specimen_frontier(handle: number): number;
  specimen_read(handle: number, age: number): number;
  specimen_advance(handle: number, years: number): number;
  specimen_changes(handle: number, from: number, to: number): number;
  specimen_snapshot(handle: number): number;
  specimen_snapshot_alloc(length: number): number;
  specimen_snapshot_release(): void;
  specimen_import(): number;
  specimen_release(handle: number): number;
}
export function specimenBinding(get: () => SpecimenExports, check: (code: number) => void, metadata: () => unknown) {
  const wire = (slot: number) => {
    const e = get();
    return new SpecimenWire(new Uint8Array(e.memory.buffer, e.buffer_ptr(slot), e.buffer_len(slot)));
  };
  const wrap = (handle: number): SpecimenHandle => ({
    get frontier() { check(get().specimen_frontier(handle)); return (metadata() as { frontier: number }).frontier; },
    get historyCap() { check(get().specimen_frontier(handle)); return (metadata() as { historyCap: number }).historyCap; },
    read(age) {
      const e = get();
      check(e.specimen_read(handle, age ?? this.frontier));
      return { ...(metadata() as Omit<SpecimenRead, 'structure' | 'leaves'>), ...wire(18).identities(),
        structure: {
          values: new Float64Array(e.memory.buffer, e.buffer_ptr(6), e.buffer_len(6)).slice(),
          topology: new Uint32Array(e.memory.buffer, e.buffer_ptr(7), e.buffer_len(7)).slice(),
        }, leaves: new Uint32Array(e.memory.buffer, e.buffer_ptr(5), e.buffer_len(5)).slice() };
    },
    advance(years) { check(get().specimen_advance(handle, years)); return { ...(metadata() as { frontier: number }), changes: wire(17).changes() }; },
    changes(from, to) { check(get().specimen_changes(handle, from, to)); return wire(17).changes(); },
    setNodeCeiling(limit) { check(get().specimen_node_ceiling(handle, limit)); },
    setHistoryCap(years) { check(get().specimen_history_cap(handle, years)); },
    snapshot() {
      const e = get();
      try {
        check(e.specimen_snapshot(handle));
        return { schema: 3, data: new Uint8Array(e.memory.buffer, e.buffer_ptr(16), e.buffer_len(16)).slice() };
      } finally { e.specimen_snapshot_release(); }
    },
    release() { check(get().specimen_release(handle)); },
  });
  return {
    build(family: Family | string, cap: number): SpecimenHandle {
      const clean = typeof family === 'string' ? family : Object.fromEntries(Object.entries(family).filter(([k]) => !['id', 'name', 'note'].includes(k)));
      const bytes = new TextEncoder().encode(JSON.stringify(clean, (key, value) => {
        if (typeof value === 'number' && !Number.isFinite(value)) throw Error(`Invalid ${key} = ${value}`);
        return value;
      }));
      const e = get();
      check(e.request_alloc(bytes.length));
      new Uint8Array(e.memory.buffer, e.request_ptr(), bytes.length).set(bytes);
      check(e.specimen_build(cap));
      return wrap((metadata() as { handle: number }).handle);
    },
    import(snapshot: SpecimenSnapshot): SpecimenHandle {
      if (snapshot.schema !== 3 || !(snapshot.data instanceof Uint8Array)) throw Error('Invalid specimen snapshot schema/data');
      const e = get();
      try {
        check(e.specimen_snapshot_alloc(snapshot.data.length));
        new Uint8Array(e.memory.buffer, e.buffer_ptr(16), e.buffer_len(16)).set(snapshot.data);
        check(e.specimen_import());
        return wrap((metadata() as { handle: number }).handle);
      } finally { e.specimen_snapshot_release(); }
    },
  };
}
