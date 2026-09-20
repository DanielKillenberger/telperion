fn contact_point(i:u32) -> vec3<f32> { return vec3<f32>(rings[i*3u], rings[i*3u+1u], rings[i*3u+2u]); }
fn closest(p: vec3<f32>, a: vec3<f32>, b: vec3<f32>, c: vec3<f32>) -> vec3<f32> {
    let ab=b-a; let ac=c-a; let ap=p-a;
    let d1=dot(ab,ap); let d2=dot(ac,ap);
    if d1<=0.0 && d2<=0.0 { return a; }
    let bp=p-b; let d3=dot(ab,bp); let d4=dot(ac,bp);
    if d3>=0.0 && d4<=d3 { return b; }
    let vc=d1*d4-d3*d2;
    if vc<=0.0 && d1>=0.0 && d3<=0.0 { return a+ab*(d1/(d1-d3)); }
    let cp=p-c; let d5=dot(ab,cp); let d6=dot(ac,cp);
    if d6>=0.0 && d5<=d6 { return c; }
    let vb=d5*d2-d1*d6;
    if vb<=0.0 && d2>=0.0 && d6<=0.0 { return a+ac*(d2/(d2-d6)); }
    let va=d3*d6-d5*d4;
    if va<=0.0 && d4-d3>=0.0 && d5-d6>=0.0 { return b+(c-b)*((d4-d3)/((d4-d3)+(d5-d6))); }
    let inv=1.0/(va+vb+vc);
    return a+ab*(vb*inv)+ac*(vc*inv);
}
fn ray_triangle(origin:vec3<f32>, direction:vec3<f32>, a:vec3<f32>, b:vec3<f32>, c:vec3<f32>) -> f32 {
    let ab=b-a; let ac=c-a; let h=cross(direction,ac); let det=dot(ab,h);
    if abs(det)<1e-18 { return FAR; }
    let inv=1.0/det; let s=origin-a; let u=inv*dot(s,h);
    if u < -1e-7 || u > 1.0000001 { return FAR; }
    let q=cross(s,ab); let v=inv*dot(direction,q);
    if v < -1e-7 || u+v>1.0000001 { return FAR; }
    let t=inv*dot(ac,q);
    return select(FAR,t,t>=0.0);
}
fn contact(edge:vec4<u32>, origin:vec3<f32>, radial:vec3<f32>, radius:f32) -> vec3<f32> {
    let n=config.counts.w;
    var starts=array<u32,3>(edge.x,edge.x,edge.y);
    var ends=array<u32,3>(edge.y,edge.x,edge.y);
    if edge.x>edge.z { starts[1]=edge.x-n; ends[1]=edge.x; }
    if edge.y<edge.w { starts[2]=edge.y; ends[2]=edge.y+n; }
    var best=FAR;
    for(var s=0u;s<3u;s++) {
        if starts[s]==ends[s] { continue; }
        for(var k=0u;k<n;k++) {
            let next=(k+1u)%n;
            let a=contact_point(starts[s]+k); let b=contact_point(starts[s]+next);
            let c=contact_point(ends[s]+k); let d=contact_point(ends[s]+next);
            best=min(best,ray_triangle(origin,radial,a,b,c));
            best=min(best,ray_triangle(origin,radial,b,d,c));
        }
    }
    if best<FAR { return origin+radial*best; }
    let seat_target=origin+radial*radius;
    var point=seat_target; var distance=FAR;
    for(var s=0u;s<3u;s++) {
        if starts[s]==ends[s] { continue; }
        for(var k=0u;k<n;k++) {
            let next=(k+1u)%n;
            let a=contact_point(starts[s]+k); let b=contact_point(starts[s]+next);
            let c=contact_point(ends[s]+k); let d=contact_point(ends[s]+next);
            let p=closest(seat_target,a,b,c); let q=closest(seat_target,b,d,c);
            let dp=dot(p-seat_target,p-seat_target); let dq=dot(q-seat_target,q-seat_target);
            if dp<distance { distance=dp; point=p; }
            if dq<distance { distance=dq; point=q; }
        }
    }
    if distance==FAR { atomicOr(&summary[1],1u); }
    return point;
}
