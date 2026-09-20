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
fn vertex_normal(base:u32, rings:u32, local:u32) -> vec3f {
 let s=cfg.segments; let ring_vertices=rings*s; let vertex=base+local;
        var sum = vec3f(0.0);
        let ring = min(local/s, rings-1u);
        if local < ring_vertices {
            let k = local%s;
            let prev = (k+s-1u)%s;
            let low = min(k,prev);
            let high = max(k,prev);
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
            for(var k=0u;k<s;k+=1u) { sum=add_face(sum,base,rings,(rings-1u)*s*2u+k*2u+cap,vertex); }
        }
        let scale = max(max(abs(sum.x),abs(sum.y)),abs(sum.z));
        var n = vec3f(0.0);
        if scale > 1.17549435e-38 && scale < 3.402823e38 {
            n = normalize(sum/scale);
        }
        return n;
}
fn usable_normal(n:vec3f)->bool { return all(abs(n)<=vec3f(1.0)) && dot(n,n)>=0.999; }
