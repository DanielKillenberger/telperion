struct Config {
    rings:u32, runs:u32, segments:u32, maximum:u32,
    ring_offset:u32, angle_offset:u32,
    // The section's lobes and their depth (`angular.rs`, `profile`).
    lobes:u32, depth:f32
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
        // The lobed outline at the ring's twist phase (`binormal.w`).
        let profile=1.0+cfg.depth*cos(f32(cfg.lobes)*(angle.x+binormal.w));
        let width=centre.w*select(profile,1.0,cfg.lobes==0u);
        put(offsets.x+k,centre.xyz+(normal.xyz*angle.y+binormal.xyz*angle.z)*width);
    }
    if lane==0u {
        if offsets.y!=0xffffffffu { put(offsets.y,centre.xyz); }
        if offsets.z!=0xffffffffu { put(offsets.z,centre.xyz); }
    }
}
