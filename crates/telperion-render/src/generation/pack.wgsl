fn rotation_word(x:vec3<f32>, y:vec3<f32>, z:vec3<f32>) -> u32 {
    let trace=x.x+y.y+z.z;
    var q:vec4<f32>;
    if trace>0.0 {
        let s=sqrt(trace+1.0)*2.0; q=vec4<f32>(0.25*s,(y.z-z.y)/s,(z.x-x.z)/s,(x.y-y.x)/s);
    } else if x.x>y.y && x.x>z.z {
        let s=sqrt(1.0+x.x-y.y-z.z)*2.0; q=vec4<f32>((y.z-z.y)/s,0.25*s,(y.x+x.y)/s,(z.x+x.z)/s);
    } else if y.y>z.z {
        let s=sqrt(1.0+y.y-x.x-z.z)*2.0; q=vec4<f32>((z.x-x.z)/s,(y.x+x.y)/s,0.25*s,(z.y+y.z)/s);
    } else {
        let s=sqrt(1.0+z.z-x.x-y.y)*2.0; q=vec4<f32>((x.y-y.x)/s,(z.x+x.z)/s,(z.y+y.z)/s,0.25*s);
    }
    q=normalize(q);
    var largest=0u;
    for(var i=1u;i<4u;i++) { if abs(q[i])>abs(q[largest]) { largest=i; } }
    q*=select(1.0,-1.0,q[largest]<0.0);
    var kept=vec3<f32>(0.0); var slot=0u;
    for(var i=0u;i<4u;i++) { if i!=largest { kept[slot]=q[i]; slot++; } }
    let exact=clamp((kept+vec3<f32>(LEAF_RANGE))/(2.0*LEAF_RANGE),vec3<f32>(0.0),vec3<f32>(1.0))*1023.0;
    var best=-FAR; var word=0u;
    for(var corner=0u;corner<8u;corner++) {
        let up=vec3<f32>(f32(corner&1u),f32((corner>>1u)&1u),f32((corner>>2u)&1u));
        let codes=clamp(floor(exact)+up,vec3<f32>(0.0),vec3<f32>(1023.0));
        let v=codes/1023.0*(2.0*LEAF_RANGE)-vec3<f32>(LEAF_RANGE);
        let dropped=sqrt(max(1.0-dot(v,v),0.0));
        let alignment=dot(v,kept)+dropped*q[largest];
        if alignment>best { best=alignment; word=(largest<<30u)|u32(codes.x)|(u32(codes.y)<<10u)|(u32(codes.z)<<20u); }
    }
    return word;
}
fn mix32(v:u32) -> u32 {
    var z=v; z=(z^(z>>16u))*0x21f0aaadu; z=(z^(z>>15u))*0x735a2d97u;
    return z^(z>>15u);
}
// The `ordinal`th draw of a run's stream: the family's seed and its first
// node's key (`tree::leaf_stream`, fn-212), whatever order runs are walked.
fn random(key:u32, ordinal:u32) -> f32 {
    let stream=mix32(config.random.x ^ 0x2c9e1a7fu ^ key);
    return f32(mix32(stream+(ordinal+1u)*0x9e3779b9u))*(1.0/4294967296.0);
}
fn rotate(v:vec3<f32>, axis:vec3<f32>, angle:f32) -> vec3<f32> {
    let c=cos(angle); let s=sin(angle);
    return v*c+cross(axis,v)*s+axis*(dot(axis,v)*(1.0-c));
}
// Leaf `j` of a cluster of `n` (fn-223, `cluster::radial`): the level
// bearing of the tip's heading, +X where it stands vertical, turned about
// +Y by the spread either way, evenly.
fn fanned(heading:vec3<f32>, n:u32, spread:f32, j:u32) -> vec3<f32> {
    var bearing=vec3<f32>(1.0,0.0,0.0);
    let flat=vec3<f32>(heading.x,0.0,heading.z);
    if dot(flat,flat)>1e-12 { bearing=normalize(flat); }
    var across=0.0;
    if n>1u { across=spread*(2.0*f32(j)/f32(n-1u)-1.0); }
    return rotate(bearing,vec3<f32>(0.0,1.0,0.0),across);
}
// Whether station `k` of a segment that may shed keeps its leaf
// (`tree::keeps`): the retention of its wood, before the segment's
// boundary or from it, against its draw on the segment's stream.
fn retained(seg:Segment, k:u32) -> bool {
    return kept_by(seg,bitcast<u32>(seg.normal.w),k);
}
// Whether station `k` keeps its leaf under the retention word `keep`.
fn kept_by(seg:Segment, keep:u32, k:u32) -> bool {
    let unit=bitcast<u32>(config.size.z);
    var q=keep>>16u;
    if (k/config.random.y)*unit>=bitcast<u32>(seg.tangent.w) { q=keep&0xffffu; }
    if q==0xffffu { return true; }
    let stream=bitcast<u32>(seg.binormal.w);
    return (mix32(stream+(k+1u)*0x9e3779b9u)>>16u)<q;
}
fn station(index:u32) -> vec3<u32> {
    var lo=0u; var hi=config.counts.y;
    // cap: each step halves hi - lo, a u32, so 32 steps at most.
    while lo+1u<hi { let mid=lo+(hi-lo)/2u; if segments[mid].range.x<=index { lo=mid; } else { hi=mid; } }
    let seg=segments[lo];
    var k=seg.range.z+index-seg.range.x;
    // A rosette's frond: its stations are its leaflets (fn-256).
    if seg.distance.y < -1.5 { return leaflet(seg,k); }
    // A segment some of whose stations shed their leaves (fn-248): the
    // leaf's station is the kept one its place among them names.
    if bitcast<u32>(seg.normal.w)!=0xffffffffu {
        var left=index-seg.range.x; k=seg.range.z;
        // cap: a tile holds 256 stations, so 256 steps at most.
        loop { if retained(seg,k) { if left==0u { break; } left-=1u; } k+=1u; }
    }
    var radial:vec3<f32>; var point:vec3<f32>; let tangent=seg.tangent.xyz; var normal=seg.normal.xyz;
    if seg.distance.y<0.0 {
        // A cluster at its tip, on the tip's own stream.
        radial=fanned(tangent,seg.range.y,seg.distance.z,k);
        point=seg.a.xyz; normal=radial;
    } else {
        let internode=k/config.random.y;
        let distance=f32(internode)*config.shape.x;
        var t=0.0; if seg.distance.y>1e-12 { t=(distance-seg.distance.x)/seg.distance.y; }
        let centre=seg.a.xyz*(1.0-t)+seg.b.xyz*t;
        let radius=seg.a.w*(1.0-t)+seg.b.w*t;
        let turn=f32(internode-seg.range.z/config.random.y)*config.shape.y+f32(k%config.random.y)*6.283185307179586/f32(config.random.y);
        let sin_turn=seg.distance.z*cos(turn)+seg.distance.w*sin(turn);
        let cos_turn=seg.distance.w*cos(turn)-seg.distance.z*sin(turn);
        radial=seg.normal.xyz*cos_turn+seg.binormal.xyz*sin_turn;
        point=centre+radial*radius;
        if config.shape.z>0.0 { point=point*(1.0-config.shape.z)+contact(seg.edge,centre,radial,radius)*config.shape.z; }
    }
    var axis=radial+tangent*(config.lean.x+config.lean.y*max(radial.y,0.0));
    let outward=vec3<f32>(point.x,0.0,point.z);
    if dot(outward,outward)>1e-12 { axis+=normalize(outward)*config.lean.z; }
    axis.y+=config.lean.w;
    if dot(axis,axis)<=1e-12 { axis=radial; }
    axis=normalize(axis);
    var words=packed(point,axis,tangent,normal,seg.range.w,k,leaf_share(seg,k));
    // A colouring leaf is drawn as one withering (fn-270 R5c).
    if falls(seg,k) && seg.season.x>0.0 { words.z|=LEAF_WITHERED; }
    return words;
}
// Whether station `k`'s leaf falls this year: past its lifespan, or one
// its wood keeps no longer.
fn falls(seg:Segment, k:u32) -> bool {
    let now=bitcast<u32>(seg.unfolded.y);
    let kept=now==0xffffffffu || kept_by(seg,now,k);
    let life=bitcast<u32>(seg.season.y);
    let passing=life!=0u && (life==0xffffu || (mix32(bitcast<u32>(seg.season.z)+(k+1u)*0x9e3779b9u)>>16u)<life);
    return !kept || passing;
}
// The share of its whole size station `k`'s leaf is drawn at within the
// year (`tree::Unfolded::leaf`, fn-270 R5, R5c): one past its lifespan,
// or one its wood keeps no longer, stays whole until its own abscission
// day and is gone from then, and where kept grows again from its bud;
// any other stands at its wood's share.
fn leaf_share(seg:Segment, k:u32) -> f32 {
    let now=bitcast<u32>(seg.unfolded.y);
    let kept=now==0xffffffffu || kept_by(seg,now,k);
    if !falls(seg,k) { return seg.unfolded.x; }
    let gone=u32(floor(seg.unfolded.z*65535.0+0.5));
    let fallen=gone!=0u && (gone==0xffffu || (mix32(bitcast<u32>(seg.season.w)+(k+1u)*0x9e3779b9u)>>16u)<gone);
    return select(1.0,0.0,fallen)+select(0.0,seg.unfolded.w,kept);
}
// Leaflet `j` of a frond's segment (`rosette.rs`'s `fan`,
// `leaflet::leaflets`): on the rachis that leaves the frond's point on
// its heading, bent by its arch toward the frame's lift, at the leaflet
// pitch to alternate sides, the last closed by the terminal share and a
// spine at its own pitch and share; a lone placement on the frond's
// heading itself. A skirt frond's leaflets wither.
fn leaflet(seg:Segment, j:u32) -> vec3<u32> {
    let count=seg.range.y; let heading=seg.tangent.xyz; let length=seg.a.w;
    var point=seg.a.xyz; var axis=heading; var tangent=seg.b.xyz; var normal=seg.normal.xyz;
    var share=seg.b.w;
    if count>1u {
        var lift=vec3<f32>(0.0,1.0,0.0)-heading*heading.y;
        if dot(lift,lift)>1e-12 { lift=normalize(lift); } else { lift=square_to(heading); }
        let arch=seg.binormal.x;
        let t=f32(j+1u)/f32(count);
        point=seg.a.xyz+heading*(length*t)+lift*(arch*length*t*t);
        let run=normalize(heading+lift*(2.0*arch*t));
        var closing=1.0; if j+1u==count { closing=1.0-seg.binormal.z; }
        let hand=select(-1.0,1.0,j%2u==0u);
        var pitch=seg.binormal.y;
        if j<u32(seg.distance.z) && seg.tangent.w>0.0 { pitch=seg.binormal.w; share*=seg.tangent.w; }
        axis=rotate(run,lift,hand*pitch*closing);
        tangent=run; normal=cross(heading,lift);
    }
    var words=packed(point,axis,tangent,normal,seg.range.w,j,share);
    if seg.distance.x>0.5 { words.z|=LEAF_WITHERED; }
    return words;
}
// A unit vector square to `v` (`Vec3::perpendicular`).
fn square_to(v:vec3<f32>) -> vec3<f32> {
    let a=abs(v); var reference=vec3<f32>(0.0,0.0,1.0);
    if a.x<=a.y && a.x<=a.z { reference=vec3<f32>(1.0,0.0,0.0); } else if a.y<=a.z { reference=vec3<f32>(0.0,1.0,0.0); }
    return normalize(cross(reference,v));
}
// A leaf at `point` along `axis`, framed by `tangent` and `normal`, its
// draws on `key` from its ordinal `k`, its size times `share`, as the
// leaf words placement writes.
fn packed(point:vec3<f32>, axis_in:vec3<f32>, tangent:vec3<f32>, normal:vec3<f32>, key:u32, k:u32, share:f32) -> vec3<u32> {
    var axis=axis_in;
    var face=vec3<f32>(0.0,1.0,0.0)-axis*axis.y;
    if dot(face,face)<=1e-12 { face=tangent-axis*dot(tangent,axis); }
    if dot(face,face)<=1e-12 { face=normal-axis*dot(normal,axis); }
    face=normalize(face); var side=normalize(cross(axis,face));
    var draw=k*select(1u,4u,config.shape.w>0.0);
    if config.shape.w>0.0 {
        let z=random(key,draw)*2.0-1.0; let phi=random(key,draw+1u)*6.283185307179586;
        let ring=sqrt(max(1.0-z*z,0.0)); let jitter=vec3<f32>(ring*cos(phi),z,ring*sin(phi));
        let angle=config.shape.w*random(key,draw+2u);
        axis=rotate(axis,jitter,angle);face=rotate(face,jitter,angle);side=rotate(side,jitter,angle); draw+=3u;
    }
    let scale=config.size.x*(1.0+config.size.y*(random(key,draw)*2.0-1.0))*share;
    if !finite(point) || !finite(axis) || !finite(face) || !finite(side) || !(scale>=0.0 && scale<=65504.0) {
        atomicOr(&summary[1],2u);
    }
    if any(point<config.box_min.xyz) || any(point>config.box_min.xyz+config.box_extent.xyz) { atomicOr(&summary[1],4u); }
    let norm=clamp((point-config.box_min.xyz)/max(config.box_extent.xyz,vec3<f32>(1e-30)),vec3<f32>(0.0),vec3<f32>(1.0));
    let codes=vec3<u32>(floor(norm*65535.0+vec3<f32>(0.5)));
    return vec3<u32>(rotation_word(side,axis,face),codes.x|(codes.y<<16u),codes.z|(pack2x16float(vec2<f32>(0.0,scale))&0xffff0000u));
}
