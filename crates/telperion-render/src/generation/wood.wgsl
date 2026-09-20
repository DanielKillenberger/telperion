struct Config { runs: u32, segments: u32, row: u32, ring_offset: u32, angle_offset: u32, _a: u32, _b: u32, _c: u32 }
@group(0) @binding(0) var<uniform> cfg: Config;
@group(0) @binding(1) var<storage, read> positions: array<f32>;
@group(0) @binding(2) var<storage, read> metadata: array<u32>;
@group(0) @binding(3) var<storage, read_write> normals: array<f32>;
@group(0) @binding(4) var<storage, read_write> indices: array<u32>;
@group(0) @binding(5) var<storage, read_write> coords: array<f32>;
@group(0) @binding(6) var<storage, read_write> radii: array<f32>;
@group(0) @binding(7) var<storage, read_write> status: atomic<u32>;
fn triangle(base: u32, rings: u32, face: u32) -> vec3u {
    let s = cfg.segments;
    let strip = (rings - 1u) * s * 2u;
    if face < strip {
        let pair = face / 2u;
        let lower = base + pair / s * s;
        let k = pair % s;
        let next = (k + 1u) % s;
        if face % 2u == 0u { return vec3u(lower + k, lower + next, lower + s + k); }
        return vec3u(lower + next, lower + s + next, lower + s + k);
    }
    let cap = face - strip;
    let k = cap / 2u;
    let next = (k + 1u) % s;
    let bottom = base + rings * s;
    if cap % 2u == 0u { return vec3u(bottom, base + next, base + k); }
    let top = bottom - s;
    return vec3u(bottom + 1u, top + k, top + next);
}
fn point(i: u32) -> vec3f { return vec3f(positions[i*3u], positions[i*3u+1u], positions[i*3u+2u]); }
fn add_face(sum: vec3f, base: u32, rings: u32, face: u32, vertex: u32) -> vec3f {
    let t = triangle(base, rings, face);
    if any(t == vec3u(vertex)) { return sum + cross(point(t.z)-point(t.y), point(t.x)-point(t.y)); }
    return sum;
}
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
        var sum = vec3f(0.0);
        var ring = min(local/s, rings-1u);
        var angle = 0.0;
        var radius = 0.0;
        if local < ring_vertices {
            let k = local%s;
            let prev = (k+s-1u)%s;
            let low = min(k,prev);
            let high = max(k,prev);
            angle = bitcast<f32>(metadata[cfg.angle_offset+k]);
            radius = bitcast<f32>(metadata[cfg.ring_offset+(ring_start+ring)*2u+1u]);
            for(var strip = select(0u, ring-1u, ring>0u); strip <= ring && strip+1u<rings; strip += 1u) {
                let start = strip*s*2u;
                sum = add_face(sum,base,rings,start+low*2u,vertex);
                sum = add_face(sum,base,rings,start+low*2u+1u,vertex);
                sum = add_face(sum,base,rings,start+high*2u,vertex);
                sum = add_face(sum,base,rings,start+high*2u+1u,vertex);
            }
            let start = (rings-1u)*s*2u;
            if ring==0u { sum=add_face(sum,base,rings,start+low*2u,vertex); }
            if ring==rings-1u { sum=add_face(sum,base,rings,start+low*2u+1u,vertex); }
            if ring==0u { sum=add_face(sum,base,rings,start+high*2u,vertex); }
            if ring==rings-1u { sum=add_face(sum,base,rings,start+high*2u+1u,vertex); }
        } else {
            let cap = local-ring_vertices;
            ring = select(0u,rings-1u,cap==1u);
            for(var k=0u;k<s;k+=1u) { sum=add_face(sum,base,rings,(rings-1u)*s*2u+k*2u+cap,vertex); }
        }
        let scale = max(max(abs(sum.x),abs(sum.y)),abs(sum.z));
        var n = vec3f(0.0);
        if scale > 1.17549435e-38 && scale < 3.402823e38 {
            n = normalize(sum/scale);
        }
        if !all(abs(n)<=vec3f(1.0)) || dot(n,n)<0.999 { atomicOr(&status,1u); }
        normals[vertex*3u]=n.x; normals[vertex*3u+1u]=n.y; normals[vertex*3u+2u]=n.z;
        coords[vertex*2u]=bitcast<f32>(metadata[cfg.ring_offset+(ring_start+ring)*2u]);
        coords[vertex*2u+1u]=angle;
        radii[vertex]=radius;
    }
}
