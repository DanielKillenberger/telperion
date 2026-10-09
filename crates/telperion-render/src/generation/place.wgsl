const FAR:f32=3.402823466e38;
struct Config {
    counts:vec4<u32>, // stations, segments, element vertices, ring vertices per ring
    random:vec4<u32>, // seed, stations per internode, workgroups, unused
    box_min:vec4<f32>, box_extent:vec4<f32>,
    shape:vec4<f32>, // internode, divergence radians, contact, scatter radians
    lean:vec4<f32>, // forward, rise, outward, upward
    size:vec4<f32>, // size, variation, internode in whole micrometres (bits), unused
}
struct Segment {
    a:vec4<f32>, b:vec4<f32>, tangent:vec4<f32>, normal:vec4<f32>, binormal:vec4<f32>,
    distance:vec4<f32>, range:vec4<u32>, edge:vec4<u32>,
}
@group(0) @binding(0) var<uniform> config:Config;
@group(0) @binding(1) var<storage,read> segments:array<Segment>;
@group(0) @binding(2) var<storage,read> rings:array<f32>;
@group(0) @binding(3) var<storage,read> geometry:array<vec4<f32>>;
@group(0) @binding(4) var<storage,read_write> leaves:array<u32>;
@group(0) @binding(5) var<storage,read_write> summary:array<atomic<u32>>;
fn finite(v:vec3<f32>) -> bool { return all(abs(v)<=vec3<f32>(FAR)); }
// Every placed leaf is drawn, in its station's order: the shell's
// rejection is gone (fn-198 R1, fn-212 R2).
var<workgroup> bounds:array<f32,3072>;
@compute @workgroup_size(256)
fn place(@builtin(local_invocation_index) lane:u32, @builtin(workgroup_id) group:vec3<u32>, @builtin(num_workgroups) grid:vec3<u32>) {
    let group_index=group.x+group.y*grid.x;
    if group_index>=config.random.z { return; }
    let index=group_index*256u+lane;
    var words=vec3<u32>(0u);
    let live=index<config.counts.x;
    if live { words=station(index); }
    written(index,words,live,lane);
}
// The device's florets (fn-229), in the same submission as its leaves:
// `segments` holds one record a bloom (`stations.wgsl`'s `blooms`), and
// each floret is drawn as `bloom.wgsl` draws it and packed as a leaf is.
@compute @workgroup_size(256)
fn bloom(@builtin(local_invocation_index) lane:u32, @builtin(workgroup_id) group:vec3<u32>, @builtin(num_workgroups) grid:vec3<u32>) {
    let group_index=group.x+group.y*grid.x;
    if group_index>=config.random.z { return; }
    let index=group_index*256u+lane;
    var words=vec3<u32>(0u);
    let live=index<config.counts.x;
    if live { words=floret_words(index); }
    written(index,words,live,lane);
}
// A placed instance written at its place, and the workgroup's bounds
// folded: the crown's from their origins, the whole tree's from every
// vertex.
fn written(index:u32, words:vec3<u32>, live:bool, lane:u32) {
    var crown_min=vec3<f32>(FAR); var crown_max=vec3<f32>(-FAR);
    var full_min=vec3<f32>(FAR); var full_max=vec3<f32>(-FAR);
    if live {
        let matrix=leaf_transform(words,config.box_min.xyz,config.box_extent.xyz);
        for(var v=0u;v<config.counts.z;v++) {
            let p=(matrix*vec4<f32>(geometry[v].xyz,1.0)).xyz;
            if !finite(p) { atomicOr(&summary[1],2u); }
            full_min=min(full_min,p);full_max=max(full_max,p);
        }
        crown_min=matrix[3].xyz;crown_max=crown_min;
        let at=index+config.random.w;
        leaves[at*3u]=words.x;leaves[at*3u+1u]=words.y;leaves[at*3u+2u]=words.z;
    }
    for(var j=0u;j<3u;j++) {
        bounds[lane*12u+j]=crown_min[j];bounds[lane*12u+3u+j]=crown_max[j];
        bounds[lane*12u+6u+j]=full_min[j];bounds[lane*12u+9u+j]=full_max[j];
    }
    workgroupBarrier();
    for(var step=128u;step>0u;step/=2u) {
        if lane<step {
            for(var j=0u;j<12u;j++) {
                let a=bounds[lane*12u+j];let b=bounds[(lane+step)*12u+j];
                bounds[lane*12u+j]=select(min(a,b),max(a,b),(j/3u)%2u==1u);
            }
        }
        workgroupBarrier();
    }
    if lane==0u {
        for(var j=0u;j<12u;j++) {
            if (j/3u)%2u==0u { atomicMin(&summary[2u+j],ordered(bounds[j])); }
            else { atomicMax(&summary[2u+j],ordered(bounds[j])); }
        }
    }
}
// A floret's three words: its bloom's record found by its first floret
// (the node and its bloom's growth, its parent, its bud's side, and its
// first floret, count and key), its index among the bloom's kept florets,
// and its transform at its size, packed against the box.
fn floret_words(index:u32) -> vec3<u32> {
    var lo=0u; var hi=config.counts.y;
    // cap: each step halves hi - lo, a u32, so 32 steps at most.
    while lo+1u<hi { let mid=lo+(hi-lo)/2u; if segments[mid].range.x<=index { lo=mid; } else { hi=mid; } }
    let r=segments[lo];
    let f=Flower(config.shape,config.lean,config.size);
    let key=r.range.zw;
    let ordinal=index-r.range.x;
    var i=0u; var seen=0u; var size=0.0;
    // cap: the bloom's florets; past them the record is wrong, named.
    let cap=u32(ceil(f.a.x));
    loop {
        if i>=cap { atomicOr(&summary[1],2u); break; }
        size=floret_size(f,r.a.w,key,i);
        if floret_kept(size) { if seen==ordinal { break; } seen++; }
        i++;
    }
    let fl=floret(f,r.a.xyz,r.b.xyz,r.tangent.xyz,key,i);
    let side=cross(fl.stalk,fl.face);
    if !finite(fl.point) || !finite(fl.stalk) || !finite(fl.face) || !(size>=0.0 && size<=65504.0) {
        atomicOr(&summary[1],2u);
    }
    if any(fl.point<config.box_min.xyz) || any(fl.point>config.box_min.xyz+config.box_extent.xyz) { atomicOr(&summary[1],4u); }
    let norm=clamp((fl.point-config.box_min.xyz)/max(config.box_extent.xyz,vec3<f32>(1e-30)),vec3<f32>(0.0),vec3<f32>(1.0));
    let codes=vec3<u32>(floor(norm*65535.0+vec3<f32>(0.5)));
    return vec3<u32>(rotation_word(side,fl.stalk,fl.face),codes.x|(codes.y<<16u),codes.z|(pack2x16float(vec2<f32>(0.0,size))&0xffff0000u));
}
