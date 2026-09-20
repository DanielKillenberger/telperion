struct Config { rings:u32, runs:u32, segments:u32, maximum:u32, lobes:u32, depth:f32, pad:vec2u }
@group(0) @binding(0) var<uniform> cfg:Config;
@group(0) @binding(1) var<storage,read> rings:array<vec4u>;
@group(0) @binding(2) var<storage,read> angular:array<vec4f>;
@group(0) @binding(3) var<storage,read> runs:array<vec4u>;
@group(0) @binding(4) var<storage,read_write> positions:array<f32>;
@group(0) @binding(5) var<storage,read_write> partial:array<u32>;
@group(0) @binding(6) var<storage,read_write> result:array<u32>;
fn put(at:u32,p:vec3f) { positions[at*3]=p.x; positions[at*3+1]=p.y; positions[at*3+2]=p.z; }
fn point(at:u32)->vec3f { return vec3f(positions[at*3],positions[at*3+1],positions[at*3+2]); }
fn finite(p:vec3f)->bool { return all((bitcast<vec3u>(p)&vec3u(0x7f800000u))!=vec3u(0x7f800000u)); }
@compute @workgroup_size(64)
fn emit(@builtin(workgroup_id) group:vec3u,@builtin(local_invocation_index) lane:u32) {
 let id=group.x+group.y*cfg.maximum; if(id>=cfg.rings){return;}
 let c=bitcast<vec4f>(rings[id*4]); let n=bitcast<vec4f>(rings[id*4+1]);
 let b=bitcast<vec4f>(rings[id*4+2]); let offsets=rings[id*4+3];
 for(var k=lane;k<cfg.segments;k+=64u){
  let a=angular[k];
  let profile=1.0+cfg.depth*cos(f32(cfg.lobes)*(a.x+b.w));
  let width=c.w*select(profile,1.0,cfg.lobes==0u);
  put(offsets.x+k,c.xyz+(n.xyz*a.y+b.xyz*a.z)*width);
 }
 if(lane==0u){ if(offsets.y!=0xffffffffu){put(offsets.y,c.xyz);} if(offsets.z!=0xffffffffu){put(offsets.z,c.xyz);} }
}
fn triangle(r:vec4u,face:u32)->vec3u {
 let strip=(r.w-1u)*cfg.segments*2u;
 if(face<strip){let pair=face/2u;let lower=r.x+pair/cfg.segments*cfg.segments;let k=pair%cfg.segments;let next=(k+1u)%cfg.segments;
 if(face%2u==0u){return vec3u(lower+k,lower+next,lower+cfg.segments+k);}
 return vec3u(lower+next,lower+cfg.segments+next,lower+cfg.segments+k);}
 let cap=face-strip;let k=cap/2u;let next=(k+1u)%cfg.segments;let bottom=r.x+r.w*cfg.segments;
 if(cap%2u==0u){return vec3u(bottom,r.x+next,r.x+k);} return vec3u(bottom+1u,bottom-cfg.segments+k,bottom-cfg.segments+next);
}
var<workgroup> lo:array<vec3f,64>;
var<workgroup> hi:array<vec3f,64>;
var<workgroup> counts:array<vec2u,64>;
fn reduce(lane:u32) {
 workgroupBarrier();
 for(var stride=32u;stride>0u;stride/=2u){
  if(lane<stride){lo[lane]=min(lo[lane],lo[lane+stride]);hi[lane]=max(hi[lane],hi[lane+stride]);counts[lane]+=counts[lane+stride];}
  workgroupBarrier();
 }
}
@compute @workgroup_size(64)
fn validate(@builtin(workgroup_id) group:vec3u,@builtin(local_invocation_index) lane:u32){
 let id=group.x+group.y*cfg.maximum;if(id>=cfg.runs){return;}
 let r=runs[id];var low=vec3f(3.402823e38);var high=vec3f(-3.402823e38);var bad=0u;var collapsed=0u;
 for(var v=lane;v<r.w*cfg.segments+2u;v+=64u){let p=point(r.x+v);if(!finite(p)){bad+=1u;} low=min(low,p);high=max(high,p);}
 for(var f=lane;f<r.w*cfg.segments*2u;f+=64u){let t=triangle(r,f);let a=point(t.x);let b=point(t.y);let c=point(t.z);
 // |point| <= 2^58 implies |edge| <= 2^59, hence each cross component
 // is at most 2 * (2^59)^2 = 2^119, below f32::MAX / 256 (~2^120).
 if(!finite(a)||!finite(b)||!finite(c)||any(abs(a)>vec3f(288230376151711744.0))||any(abs(b)>vec3f(288230376151711744.0))||any(abs(c)>vec3f(288230376151711744.0))){bad+=1u;continue;}
 let u=c-b;let v=a-b;
 if(!finite(u)||!finite(v)||any(abs(u)>vec3f(576460752303423488.0))||any(abs(v)>vec3f(576460752303423488.0))){bad+=1u;continue;}
 let n=cross(u,v);let scale=max(abs(n.x),max(abs(n.y),abs(n.z)));
 if(!finite(n)){bad+=1u;} else if(scale<1.17549435e-38 || scale>=1.3292279e36){collapsed+=1u;}}
 lo[lane]=low;hi[lane]=high;counts[lane]=vec2u(bad,collapsed);reduce(lane);
 if(lane==0u){let at=id*8u;partial[at]=bitcast<u32>(lo[0].x);partial[at+1u]=bitcast<u32>(lo[0].y);partial[at+2u]=bitcast<u32>(lo[0].z);partial[at+3u]=counts[0].x;partial[at+4u]=bitcast<u32>(hi[0].x);partial[at+5u]=bitcast<u32>(hi[0].y);partial[at+6u]=bitcast<u32>(hi[0].z);partial[at+7u]=counts[0].y;}
}
@compute @workgroup_size(64)
fn finish(@builtin(local_invocation_index) lane:u32){
 var low=vec3f(3.402823e38);var high=vec3f(-3.402823e38);var count=vec2u(0u);
 for(var i=lane;i<cfg.runs;i+=64u){let at=i*8u;low=min(low,bitcast<vec3f>(vec3u(partial[at],partial[at+1u],partial[at+2u])));high=max(high,bitcast<vec3f>(vec3u(partial[at+4u],partial[at+5u],partial[at+6u])));count+=vec2u(partial[at+3u],partial[at+7u]);}
 lo[lane]=low;hi[lane]=high;counts[lane]=count;reduce(lane);
 if(lane==0u){result[0]=bitcast<u32>(lo[0].x);result[1]=bitcast<u32>(lo[0].y);result[2]=bitcast<u32>(lo[0].z);result[3]=counts[0].x;result[4]=bitcast<u32>(hi[0].x);result[5]=bitcast<u32>(hi[0].y);result[6]=bitcast<u32>(hi[0].z);result[7]=counts[0].y;}
}
