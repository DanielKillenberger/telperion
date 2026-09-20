struct Config {
    rings:u32, runs:u32, segments:u32, maximum:u32,
    ring_offset:u32, angle_offset:u32, pad:vec2u
}
@group(0) @binding(0) var<uniform> cfg:Config;
@group(0) @binding(1) var<storage,read> descriptors:array<vec4u>;
@group(0) @binding(2) var<storage,read> angular:array<vec4f>;
@group(0) @binding(3) var<storage,read_write> metadata:array<u32>;
@group(0) @binding(4) var<storage,read_write> positions:array<f32>;
@group(0) @binding(5) var<storage,read_write> partial:array<u32>;
@group(0) @binding(6) var<storage,read_write> result:array<u32>;
fn put(at:u32,p:vec3f) {
    positions[at*3]=p.x; positions[at*3+1]=p.y; positions[at*3+2]=p.z;
}
fn finite(p:vec3f)->bool {
    return all((bitcast<vec3u>(p)&vec3u(0x7f800000u))!=vec3u(0x7f800000u));
}
@compute @workgroup_size(64)
fn emit(@builtin(workgroup_id) group:vec3u,@builtin(local_invocation_index) lane:u32) {
    let id=group.x+group.y*cfg.maximum;
    if id>=cfg.rings { return; }
    let centre=bitcast<vec4f>(descriptors[id*4]);
    let normal=bitcast<vec4f>(descriptors[id*4+1]);
    let binormal=bitcast<vec4f>(descriptors[id*4+2]);
    let offsets=descriptors[id*4+3];
    for(var k=lane;k<cfg.segments;k+=64u) {
        let angle=angular[k];
        put(offsets.x+k,centre.xyz+(normal.xyz*angle.y+binormal.xyz*angle.z)*centre.w);
    }
    if lane==0u {
        if offsets.y!=0xffffffffu { put(offsets.y,centre.xyz); }
        if offsets.z!=0xffffffffu { put(offsets.z,centre.xyz); }
    }
}
var<workgroup> low:array<vec3f,64>;
var<workgroup> high:array<vec3f,64>;
var<workgroup> invalid:array<u32,64>;
fn reduce(lane:u32) {
    workgroupBarrier();
    for(var stride=32u;stride>0u;stride/=2u) {
        if lane<stride {
            low[lane]=min(low[lane],low[lane+stride]);
            high[lane]=max(high[lane],high[lane+stride]);
            invalid[lane]|=invalid[lane+stride];
        }
        workgroupBarrier();
    }
}
@compute @workgroup_size(64)
fn admit(@builtin(workgroup_id) group:vec3u,@builtin(local_invocation_index) lane:u32) {
    let id=group.x+group.y*cfg.maximum;
    if id>=cfg.runs { return; }
    let base=metadata[id*4]; let rings=metadata[id*4+3];
    var lo=vec3f(3.402823e38); var hi=vec3f(-3.402823e38); var bad=0u;
    for(var v=lane;v<rings*cfg.segments+2u;v+=64u) {
        let p=point(base+v);
        if !finite(p)||any(abs(p)>vec3f(288230376151711744.0)) { bad=1u; }
        lo=min(lo,p); hi=max(hi,p);
    }
    low[lane]=lo; high[lane]=hi; invalid[lane]=bad; reduce(lane);
    // Point <= 2^58 bounds subtraction by 2^59 and each cross by 2^119.
    // All lanes stop before any multiplication if the point guard fails.
    let points_safe=invalid[0]==0u;
    workgroupBarrier();
    if points_safe {
        for(var f=lane;f<rings*cfg.segments*2u;f+=64u) {
            let t=triangle(base,rings,f);
            let u=point(t.z)-point(t.y); let v=point(t.x)-point(t.y);
            if !finite(u)||!finite(v)||any(abs(u)>vec3f(576460752303423488.0))||any(abs(v)>vec3f(576460752303423488.0)) {
                bad=1u; continue;
            }
            let n=cross(u,v); let scale=max(abs(n.x),max(abs(n.y),abs(n.z)));
            if !finite(n)||scale<1.17549435e-38||scale>=1.3292279e36 { bad=1u; }
        }
    }
    invalid[lane]|=bad; reduce(lane);
    let faces_safe=invalid[0]==0u;
    workgroupBarrier();
    if faces_safe {
            for(var v=lane;v<rings*cfg.segments+2u;v+=64u) {
                if !usable_normal(vertex_normal(base,rings,v)) { bad=1u; }
            }
    }
    invalid[lane]|=bad; reduce(lane);
    if lane==0u {
        let at=id*8u;
        partial[at]=bitcast<u32>(low[0].x); partial[at+1]=bitcast<u32>(low[0].y);
        partial[at+2]=bitcast<u32>(low[0].z); partial[at+3]=invalid[0];
        partial[at+4]=bitcast<u32>(high[0].x); partial[at+5]=bitcast<u32>(high[0].y);
        partial[at+6]=bitcast<u32>(high[0].z);
    }
}
@compute @workgroup_size(64)
fn radii(@builtin(workgroup_id) group:vec3u,@builtin(local_invocation_index) lane:u32) {
    let id=group.x+group.y*cfg.maximum;
    if id>=cfg.rings||lane!=0u||result[3]!=0u { return; }
    let base=descriptors[id*4+3].x; let origin=point(base);
    var centre=vec3f(0.0);
    for(var k=0u;k<cfg.segments;k+=1u) { centre+=point(base+k)-origin; }
    centre/=f32(cfg.segments);
    var radius=0.0;
    for(var k=0u;k<cfg.segments;k+=1u) { radius+=length((point(base+k)-origin)-centre); }
    metadata[cfg.ring_offset+id*2u]=descriptors[id*4+1].w;
    metadata[cfg.ring_offset+id*2u+1u]=bitcast<u32>(radius/f32(cfg.segments));
}
@compute @workgroup_size(64)
fn finish(@builtin(local_invocation_index) lane:u32) {
    var lo=vec3f(3.402823e38); var hi=vec3f(-3.402823e38); var bad=0u;
    for(var i=lane;i<cfg.runs;i+=64u) {
        let at=i*8u;
        lo=min(lo,bitcast<vec3f>(vec3u(partial[at],partial[at+1],partial[at+2])));
        hi=max(hi,bitcast<vec3f>(vec3u(partial[at+4],partial[at+5],partial[at+6])));
        bad|=partial[at+3];
    }
    low[lane]=lo; high[lane]=hi; invalid[lane]=bad; reduce(lane);
    if lane==0u {
        result[0]=bitcast<u32>(low[0].x); result[1]=bitcast<u32>(low[0].y);
        result[2]=bitcast<u32>(low[0].z); result[3]=invalid[0];
        result[4]=bitcast<u32>(high[0].x); result[5]=bitcast<u32>(high[0].y);
        result[6]=bitcast<u32>(high[0].z);
    }
}
