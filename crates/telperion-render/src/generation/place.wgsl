const FAR:f32=3.402823466e38;
struct Config {
    counts:vec4<u32>, // stations, segments, element vertices, ring vertices per ring
    random:vec4<u32>, // seed, stations per internode, workgroups, unused
    box_min:vec4<f32>, box_extent:vec4<f32>,
    shape:vec4<f32>, // internode, divergence radians, contact, scatter radians
    lean:vec4<f32>, // forward, rise, outward, upward
    size:vec4<f32>, // size, variation, unused, unused
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
// rejection is gone (fn-198 R1, fn-212 R2). Each workgroup folds its
// leaves' bounds: the crown's from their origins, the whole tree's from
// every vertex.
var<workgroup> bounds:array<f32,3072>;
@compute @workgroup_size(256)
fn place(@builtin(global_invocation_id) gid:vec3<u32>, @builtin(local_invocation_index) lane:u32, @builtin(workgroup_id) group:vec3<u32>, @builtin(num_workgroups) grid:vec3<u32>) {
    let group_index=group.x+group.y*grid.x;
    if group_index>=config.random.z { return; }
    let index=group_index*256u+lane;
    var crown_min=vec3<f32>(FAR); var crown_max=vec3<f32>(-FAR);
    var full_min=vec3<f32>(FAR); var full_max=vec3<f32>(-FAR);
    if index<config.counts.x {
        let words=station(index);
        let matrix=leaf_transform(words,config.box_min.xyz,config.box_extent.xyz);
        for(var v=0u;v<config.counts.z;v++) {
            let p=(matrix*vec4<f32>(geometry[v].xyz,1.0)).xyz;
            if !finite(p) { atomicOr(&summary[1],2u); }
            full_min=min(full_min,p);full_max=max(full_max,p);
        }
        crown_min=matrix[3].xyz;crown_max=crown_min;
        leaves[index*3u]=words.x;leaves[index*3u+1u]=words.y;leaves[index*3u+2u]=words.z;
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
