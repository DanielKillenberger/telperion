struct Config { runs: u32, segments: u32, row: u32, ring_offset: u32, angle_offset: u32, _a: u32, _b: u32, _c: u32 }
@group(0) @binding(0) var<uniform> cfg: Config;
@group(0) @binding(1) var<storage, read> positions: array<f32>;
@group(0) @binding(2) var<storage, read> metadata: array<u32>;
@group(0) @binding(3) var<storage, read_write> normals: array<f32>;
@group(0) @binding(4) var<storage, read_write> indices: array<u32>;
@group(0) @binding(5) var<storage, read_write> coords: array<f32>;
@group(0) @binding(6) var<storage, read_write> radii: array<f32>;
@group(0) @binding(7) var<storage, read_write> status: atomic<u32>;
@compute @workgroup_size(64)
fn expand(@builtin(workgroup_id) group: vec3u, @builtin(local_invocation_index) lane: u32) {
    let run = group.x + group.y * cfg.row;
    if run >= cfg.runs { return; }
    let base = metadata[run*4u];
    let first = metadata[run*4u+1u];
    let ring_start = metadata[run*4u+2u];
    let rings = metadata[run*4u+3u];
    let s = cfg.segments;
    let ring_vertices = rings*s;
    for(var face = lane; face < rings*s*2u; face += 64u) {
        let t = triangle(base, rings, face);
        indices[first+face*3u] = t.x;
        indices[first+face*3u+1u] = t.y;
        indices[first+face*3u+2u] = t.z;
    }
    for(var local = lane; local < ring_vertices+2u; local += 64u) {
        let vertex = base+local;
        var ring = min(local/s, rings-1u);
        var angle = 0.0;
        var radius = 0.0;
        if local < ring_vertices {
            angle = bitcast<f32>(metadata[cfg.angle_offset+local%s]);
            radius = bitcast<f32>(metadata[cfg.ring_offset+(ring_start+ring)*2u+1u]);
        } else {
            ring = select(0u,rings-1u,local-ring_vertices==1u);
        }
        let n = vertex_normal(base,rings,local);
        if !usable_normal(n) { atomicOr(&status,1u); }
        normals[vertex*3u]=n.x; normals[vertex*3u+1u]=n.y; normals[vertex*3u+2u]=n.z;
        coords[vertex*2u]=bitcast<f32>(metadata[cfg.ring_offset+(ring_start+ring)*2u]);
        coords[vertex*2u+1u]=angle;
        radii[vertex]=radius;
    }
}
