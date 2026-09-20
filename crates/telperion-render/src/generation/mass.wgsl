struct Mass { count:u32, reach:u32, pad:vec2<u32>, minimum:vec4<f32>, extent:vec4<f32>, grid_min:vec4<f32>, dims:vec4<u32> }
@group(0) @binding(0) var<uniform> config:Mass;
@group(0) @binding(1) var<storage,read> leaves:array<u32>;
@group(0) @binding(2) var<storage,read_write> counts:array<atomic<u32>>;
@group(0) @binding(3) var<storage,read_write> masses:array<f32>;
fn index(p:vec3<u32>) -> u32 { return p.x+config.dims.x*(p.y+config.dims.y*p.z); }
@compute @workgroup_size(256)
fn occupancy(@builtin(global_invocation_id) gid:vec3<u32>, @builtin(num_workgroups) grid:vec3<u32>) {
    let i=gid.x+gid.y*grid.x*256u;if i>=config.count { return; }
    let words=vec3<u32>(leaves[i*3u],leaves[i*3u+1u],leaves[i*3u+2u]);
    let p=leaf_position(words,config.minimum.xyz,config.extent.xyz);
    let cell=vec3<u32>(clamp(floor((p-config.grid_min.xyz)/config.grid_min.w),vec3<f32>(0.0),vec3<f32>(config.dims.xyz-vec3<u32>(1u))));
    let old=atomicAdd(&counts[index(cell)+1u],1u);if old==0u { atomicAdd(&counts[0],1u); }
}
@compute @workgroup_size(256)
fn depth(@builtin(global_invocation_id) gid:vec3<u32>, @builtin(num_workgroups) grid:vec3<u32>) {
    let i=gid.x+gid.y*grid.x*256u;let total=config.dims.x*config.dims.y*config.dims.z;if i>=total { return; }
    if i==0u {
        masses[0]=config.grid_min.x;masses[1]=config.grid_min.y;masses[2]=config.grid_min.z;masses[3]=config.grid_min.w;
        masses[4]=f32(config.dims.x);masses[5]=f32(config.dims.y);masses[6]=f32(config.dims.z);masses[7]=0.0;
    }
    var over=0.0;
    if atomicLoad(&counts[i+1u])>0u {
        let mean=f32(config.count)/f32(max(atomicLoad(&counts[0]),1u));
        let x=i%config.dims.x;let y=(i/config.dims.x)%config.dims.y;let z=i/(config.dims.x*config.dims.y);
        for(var step=1u;step<=config.reach;step++) {
            if y+step>=config.dims.y { break; }
            let count=atomicLoad(&counts[index(vec3<u32>(x,y+step,z))+1u]);if count==0u { break; }
            over+=min(f32(count)/mean,1.0);
        }
    }
    masses[8u+i]=over/f32(config.reach);
}
