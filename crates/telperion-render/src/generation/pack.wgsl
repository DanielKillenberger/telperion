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
fn random(ordinal:u32) -> f32 {
    var z=(config.random.x ^ 0x2c9e1a7fu)+(ordinal+1u)*0x9e3779b9u;
    z=(z^(z>>16u))*0x21f0aaadu; z=(z^(z>>15u))*0x735a2d97u;
    return f32(z^(z>>15u))*(1.0/4294967296.0);
}
fn rotate(v:vec3<f32>, axis:vec3<f32>, angle:f32) -> vec3<f32> {
    let c=cos(angle); let s=sin(angle);
    return v*c+cross(axis,v)*s+axis*(dot(axis,v)*(1.0-c));
}
fn station(index:u32) -> vec3<u32> {
    var lo=0u; var hi=config.counts.y;
    while lo+1u<hi { let mid=lo+(hi-lo)/2u; if segments[mid].range.x<=index { lo=mid; } else { hi=mid; } }
    let seg=segments[lo];
    let k=seg.range.z+index-seg.range.x;
    let internode=k/config.random.y;
    let distance=f32(internode)*config.shape.x;
    var t=0.0; if seg.distance.y>1e-12 { t=(distance-seg.distance.x)/seg.distance.y; }
    let centre=seg.a.xyz*(1.0-t)+seg.b.xyz*t;
    let radius=seg.a.w*(1.0-t)+seg.b.w*t;
    let turn=f32(internode-seg.range.z/config.random.y)*config.shape.y+f32(k%config.random.y)*6.283185307179586/f32(config.random.y);
    let sin_turn=seg.distance.z*cos(turn)+seg.distance.w*sin(turn);
    let cos_turn=seg.distance.w*cos(turn)-seg.distance.z*sin(turn);
    let radial=seg.normal.xyz*cos_turn+seg.binormal.xyz*sin_turn;
    var point=centre+radial*radius;
    if config.shape.z>0.0 { point=point*(1.0-config.shape.z)+contact(seg.edge,centre,radial,radius)*config.shape.z; }
    var axis=radial+seg.tangent.xyz*(config.lean.x+config.lean.y*max(radial.y,0.0));
    let outward=vec3<f32>(point.x,0.0,point.z);
    if dot(outward,outward)>1e-12 { axis+=normalize(outward)*config.lean.z; }
    axis.y+=config.lean.w;
    if dot(axis,axis)<=1e-12 { axis=radial; }
    axis=normalize(axis);
    var face=vec3<f32>(0.0,1.0,0.0)-axis*axis.y;
    if dot(face,face)<=1e-12 { face=seg.tangent.xyz-axis*dot(seg.tangent.xyz,axis); }
    if dot(face,face)<=1e-12 { face=seg.normal.xyz-axis*dot(seg.normal.xyz,axis); }
    face=normalize(face); var side=normalize(cross(axis,face));
    var draw=index*select(1u,4u,config.shape.w>0.0);
    if config.shape.w>0.0 {
        let z=random(draw)*2.0-1.0; let phi=random(draw+1u)*6.283185307179586;
        let ring=sqrt(max(1.0-z*z,0.0)); let jitter=vec3<f32>(ring*cos(phi),z,ring*sin(phi));
        let angle=config.shape.w*random(draw+2u);
        axis=rotate(axis,jitter,angle);face=rotate(face,jitter,angle);side=rotate(side,jitter,angle); draw+=3u;
    }
    let scale=config.size.x*(1.0+config.size.y*(random(draw)*2.0-1.0));
    if !finite(point) || !finite(axis) || !finite(face) || !finite(side) || !(scale>=0.0 && scale<=65504.0) {
        atomicOr(&summary[1],2u);
    }
    if any(point<config.box_min.xyz) || any(point>config.box_min.xyz+config.box_extent.xyz) { atomicOr(&summary[1],4u); }
    let norm=clamp((point-config.box_min.xyz)/max(config.box_extent.xyz,vec3<f32>(1e-30)),vec3<f32>(0.0),vec3<f32>(1.0));
    let codes=vec3<u32>(floor(norm*65535.0+vec3<f32>(0.5)));
    return vec3<u32>(rotation_word(side,axis,face),codes.x|(codes.y<<16u),codes.z|(pack2x16float(vec2<f32>(0.0,scale))&0xffff0000u));
}
