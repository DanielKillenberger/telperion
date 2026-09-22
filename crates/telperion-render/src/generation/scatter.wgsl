struct Compact { count:u32, groups:u32, pad:vec2<u32> }
@group(0) @binding(0) var<uniform> config:Compact;
@group(0) @binding(1) var<storage,read> raw:array<u32>;
@group(0) @binding(2) var<storage,read> ranks:array<u32>;
@group(0) @binding(3) var<storage,read> summary:array<u32>;
@group(0) @binding(4) var<storage,read_write> compact:array<u32>;
@compute @workgroup_size(256)
fn scatter(@builtin(global_invocation_id) gid:vec3<u32>, @builtin(num_workgroups) grid:vec3<u32>) {
    let i=gid.x+gid.y*grid.x*256u; if i>=config.count || ranks[i]==0xffffffffu { return; }
    let destination=summary[16u+i/256u]+ranks[i];
    for(var j=0u;j<3u;j++) { compact[destination*3u+j]=raw[i*3u+j]; }
}
