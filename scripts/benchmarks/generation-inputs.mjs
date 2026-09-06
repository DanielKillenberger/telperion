// Canonical f64 benchmark inputs and independent traversal of exported CPU BVHs.
export function snapshotArrays(s) {
  return { wood: s.wood, woodBounds: s.woodIndex.bounds, woodTopology: s.woodIndex.topology,
    leafBounds: s.leaves.bounds, leafTopology: s.leaves.topology };
}
export function querySnapshot(s, cells) {
  if (!(cells instanceof Float64Array) || cells.length % 4) throw Error('invalid packed cells');
  const flags = new Uint8Array(cells.length / 4), limit = Math.sqrt(Number.MAX_VALUE / 64);
  for (let q = 0; q < flags.length; q++) {
    const [x, y, z, h] = cells.subarray(q * 4, q * 4 + 4), p = [x,y,z];
    if (![x,y,z,h].every(Number.isFinite) || h < 0) throw Error('invalid field query');
    const inflation = h * Math.sqrt(3);
    if (p.some(v => Math.abs(v - inflation) > limit || Math.abs(v + inflation) > limit)) throw Error('field coordinate overflow');
    const overlap = (bounds, i, radius) => {
      for (let a = 0; a < 3; a++) if (bounds[i*6+a] > p[a]+radius || bounds[i*6+a+3] < p[a]-radius) return false;
      return true;
    };
    const contains = id => {
      const i = id * 8, w = s.wood;
      const dx = w[i+3]-w[i], dy = w[i+4]-w[i+1], dz = w[i+5]-w[i+2];
      const qx = x-w[i], qy = y-w[i+1], qz = z-w[i+2];
      const r = w[i+6]+inflation, dr = w[i+7]-w[i+6];
      const a = (dx*dx+dy*dy+dz*dz)-dr*dr, b = (qx*dx+qy*dy+qz*dz)+r*dr;
      const t = a > 0 ? Math.min(1,Math.max(0,b/a)) : b > a*0.5 ? 1 : 0;
      const ex = qx-dx*t, ey = qy-dy*t, ez = qz-dz*t, radius = r+dr*t;
      return ex*ex+ey*ey+ez*ez <= radius*radius;
    };
    const any = (index, radius, wood) => {
      if (!index.nodeCount) return false;
      const stack = [0], t = index.topology;
      while (stack.length) {
        const n = stack.pop();
        if (!overlap(index.bounds,n,radius)) continue;
        if (t[n*4+2] !== 0xffffffff) { stack.push(t[n*4+3],t[n*4+2]); continue; }
        for (let i = t[n*4]; i < t[n*4+1]; i++) {
          if (overlap(index.bounds,index.nodeCount+i,radius) && (!wood || contains(t[index.nodeCount*4+i]))) return true;
        }
      }
      return false;
    };
    flags[q] = Number(any(s.woodIndex,inflation,true)) | Number(any(s.leaves,h,false))*2;
  }
  return flags;
}
export function gridCells(bounds, resolution) {
  const step = Math.max(...bounds.max.map((v,i) => v-bounds.min[i])) / resolution;
  const cells = new Float64Array(resolution**3*4);
  let i = 0;
  for (let x=0;x<resolution;x++) for(let y=0;y<resolution;y++) for(let z=0;z<resolution;z++)
    cells.set([bounds.min[0]+(x+.5)*step,bounds.min[1]+(y+.5)*step,bounds.min[2]+(z+.5)*step,step/2],i++*4);
  return cells;
}
export function boundaryCells(s) {
  const cells = [0,0,0,0, 1e10,1e10,1e10,1, 1e100,1e100,1e100,0];
  // Fixed deterministic stride through tapered wood endpoints and leaf corners;
  // contacts and one-ULP-scale neighbours challenge the f32 candidate later.
  for (let n=0;n<Math.min(32,s.wood.length/8);n++) {
    const i = Math.floor(n*(s.wood.length/8)/32)*8;
    for (const endpoint of [0,3]) for (const sign of [-1,1]) for (const delta of [-1e-10,0,1e-10])
      cells.push(s.wood[i+endpoint]+sign*s.wood[i+(endpoint===0?6:7)]+delta,s.wood[i+endpoint+1],s.wood[i+endpoint+2],0);
  }
  const index = s.leaves, count = index.bounds.length/6-index.nodeCount;
  for (let n=0;n<Math.min(32,count);n++) {
    const i = (index.nodeCount+Math.floor(n*count/32))*6;
    for(const end of [0,3]) for(const delta of [-1e-10,0,1e-10])
      cells.push(index.bounds[i+end]+delta,index.bounds[i+end+1],index.bounds[i+end+2],0);
  }
  return new Float64Array(cells);
}
export async function hashArray(array) {
  const digest = await crypto.subtle.digest('SHA-256',array);
  return Array.from(new Uint8Array(digest),v=>v.toString(16).padStart(2,'0')).join('');
}
