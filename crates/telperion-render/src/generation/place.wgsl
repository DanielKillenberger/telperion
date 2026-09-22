const FAR:f32=3.402823466e38;
struct Config {
    counts:vec4<u32>, // stations, segments, element vertices, ring vertices per ring
    random:vec4<u32>, // seed, stations per internode, workgroups, profile points
    box_min:vec4<f32>, box_extent:vec4<f32>,
    shape:vec4<f32>, // internode, divergence radians, contact, scatter radians
    lean:vec4<f32>, // forward, rise, outward, upward
    size:vec4<f32>, // size, variation, shell, unused
    envelope:vec4<f32>, // height, crown base, max radius, fullness
    curve:vec4<f32>, // shoulder
}
struct Segment {
    a:vec4<f32>, b:vec4<f32>, tangent:vec4<f32>, normal:vec4<f32>, binormal:vec4<f32>,
    distance:vec4<f32>, range:vec4<u32>, edge:vec4<u32>,
}
@group(0) @binding(0) var<uniform> config:Config;
@group(0) @binding(1) var<storage,read> segments:array<Segment>;
@group(0) @binding(2) var<storage,read> rings:array<f32>;
@group(0) @binding(3) var<storage,read> geometry:array<vec4<f32>>;
@group(0) @binding(4) var<storage,read_write> raw:array<u32>;
@group(0) @binding(5) var<storage,read_write> ranks:array<u32>;
@group(0) @binding(6) var<storage,read_write> summary:array<atomic<u32>>;
fn finite(v:vec3<f32>) -> bool { return all(abs(v)<=vec3<f32>(FAR)); }
fn radius_at(y:f32) -> f32 {
    let base=config.envelope.x*config.envelope.y; let span=config.envelope.x-base;
    if span<=0.0 { return 0.0; }
    let t=(y-base)/span; if t<=0.0 || t>=1.0 { return 0.0; }
    var p=(t-config.envelope.w)/(1.0-config.envelope.w);
    if t<config.envelope.w { p=1.0-t/config.envelope.w; }
    return config.envelope.z*pow(max(1.0-pow(p,config.curve.x),0.0),1.0/config.curve.x);
}
fn in_shell(p:vec3<f32>) -> bool {
    let r=length(p.xz); let shell=config.size.z;
    if radius_at(p.y)-r<=shell { return true; }
    let at=vec2<f32>(r,p.y);
    for(var i=1u;i<config.random.w;i++) {
        let a=geometry[config.counts.z+i-1u].xy; let b=geometry[config.counts.z+i].xy;
        let delta=b-a; let norm=dot(delta,delta); var t=0.0;
        if norm>0.0 { t=clamp(dot(at-a,delta)/norm,0.0,1.0); }
        if distance(at,a+delta*t)<=shell { return true; }
    }
    return false;
}
var<workgroup> scan:array<u32,256>;
var<workgroup> bounds:array<f32,3072>;
@compute @workgroup_size(256)
fn place(@builtin(global_invocation_id) gid:vec3<u32>, @builtin(local_invocation_index) lane:u32, @builtin(workgroup_id) group:vec3<u32>, @builtin(num_workgroups) grid:vec3<u32>) {
    let group_index=group.x+group.y*grid.x;
    if group_index>=config.random.z { return; }
    let index=group_index*256u+lane; var alive=false; var words=vec3<u32>(0u);
    var crown_min=vec3<f32>(FAR); var crown_max=vec3<f32>(-FAR);
    var full_min=vec3<f32>(FAR); var full_max=vec3<f32>(-FAR);
    if index<config.counts.x {
        words=station(index);
        let matrix=leaf_transform(words,config.box_min.xyz,config.box_extent.xyz);
        for(var v=0u;v<config.counts.z;v++) {
            let p=(matrix*vec4<f32>(geometry[v].xyz,1.0)).xyz;
            if !finite(p) { atomicOr(&summary[1],2u); }
            full_min=min(full_min,p);full_max=max(full_max,p);
            // Bounds need all vertices even after one vertex retains the leaf.
            if !alive { alive=in_shell(p); }
        }
        if alive { crown_min=matrix[3].xyz;crown_max=crown_min; }
    }
    if !alive { full_min=vec3<f32>(FAR);full_max=vec3<f32>(-FAR); }
    for(var j=0u;j<3u;j++) {
        bounds[lane*12u+j]=crown_min[j];bounds[lane*12u+3u+j]=crown_max[j];
        bounds[lane*12u+6u+j]=full_min[j];bounds[lane*12u+9u+j]=full_max[j];
    }
    scan[lane]=select(0u,1u,alive); workgroupBarrier();
    for(var offset=1u;offset<256u;offset*=2u) {
        var add=0u; if lane>=offset { add=scan[lane-offset]; }
        workgroupBarrier();scan[lane]+=add;workgroupBarrier();
    }
    if index<config.counts.x {
        ranks[index]=select(0xffffffffu,scan[lane]-1u,alive);
        raw[index*3u]=words.x;raw[index*3u+1u]=words.y;raw[index*3u+2u]=words.z;
    }
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
        atomicStore(&summary[16u+group_index],scan[255]);
        for(var j=0u;j<12u;j++) {
            if (j/3u)%2u==0u { atomicMin(&summary[2u+j],ordered(bounds[j])); }
            else { atomicMax(&summary[2u+j],ordered(bounds[j])); }
        }
    }
}
@compute @workgroup_size(1)
fn prefix() {
    var total=0u;
    for(var group=0u;group<config.random.z;group++) {
        let count=atomicLoad(&summary[16u+group]);atomicStore(&summary[16u+group],total);total+=count;
    }
    atomicStore(&summary[0],total);
}
