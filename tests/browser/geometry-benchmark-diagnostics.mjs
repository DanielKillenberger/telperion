// fn19-v1 projected coverage diagnostics. No statistic implies botanical approval.
export function validateMatch(expected, actual) {
  for (const key of ['protocol_sha256','source_sha256','camera_sha256','roi_sha256','width','height']) {
    if (!expected[key] || expected[key] !== actual?.[key]) throw Error(`${key.includes('source')?'source-mismatch':'roi-mismatch'}: ${key}`);
  }
}
export function targetSupport(species, registry) {
  const entry=registry.find(e=>e.id===species.preset && e.profile_id===species.profile_id);
  const missing=species.required_capabilities.filter(c=>!entry?.capabilities.includes(c));
  return {status:entry && !missing.length?'supported':'unsupported-anatomy',missing_capabilities:missing};
}
function array(values,n,unit=false) {
  if (!values || values.length!==n) throw Error('missing-asset: incomplete array');
  for(const v of values)if(!Number.isFinite(v) || unit&&(v<0||v>1))throw Error('nonfinite: invalid linear sample');
}
export function convergence(beauty64,beauty128,coverage64,coverage128,width,height) {
  if(!Number.isSafeInteger(width)||!Number.isSafeInteger(height)||width<=0||height<=0)throw Error('roi-mismatch: dimensions');
  const n=width*height;array(beauty64,n*3,true);array(beauty128,n*3,true);array(coverage64,n,true);array(coverage128,n,true);
  let sq=0,tileMax=0;
  for(let i=0;i<n*3;i++)sq+=(beauty64[i]-beauty128[i])**2;
  for(let y=0;y<height;y+=16)for(let x=0;x<width;x+=16){let sum=0,count=0;
    for(let yy=y;yy<Math.min(y+16,height);yy++)for(let xx=x;xx<Math.min(x+16,width);xx++){const i=yy*width+xx;sum+=coverage64[i]-coverage128[i];count++;}
    tileMax=Math.max(tileMax,Math.abs(sum/count));
  }
  const rmse=Math.sqrt(sq/(n*3)),pass=rmse<=.005&&tileMax<=.015;
  return {status:pass?'measured':'failed',samples:[64,128],beauty_rmse:rmse,coverage_tile_max:tileMax,reason:pass?null:'unconverged'};
}
export function convexHull(points) {
  const sorted=points.map(p=>[p[0],p[1]]).sort((a,b)=>a[0]-b[0]||a[1]-b[1]);
  const unique=sorted.filter((p,i)=>!i||p[0]!==sorted[i-1][0]||p[1]!==sorted[i-1][1]);
  const cross=(a,b,c)=>(b[0]-a[0])*(c[1]-a[1])-(b[1]-a[1])*(c[0]-a[0]);
  const half=ps=>{const h=[];for(const p of ps){while(h.length>1&&cross(h.at(-2),h.at(-1),p)<=0)h.pop();h.push(p);}return h;};
  return [...half(unique).slice(0,-1),...half([...unique].reverse()).slice(0,-1)];
}
export function rasterizeHull(points,width,height) {
  if(!points.length||points.some(p=>p.length!==2||!p.every(Number.isFinite)))throw Error('empty-crown: invalid projected vertices');
  if(points.some(([x,y])=>x<=0||y<=0||x>=width||y>=height))throw Error('clipped-subject: crown touches viewport');
  const polygon=convexHull(points);if(polygon.length<3)throw Error('empty-crown: collinear hull');
  const raster=new Uint8Array(width*height);
  for(let y=0;y<height;y++)for(let x=0;x<width;x++) {
    if(polygon.every((a,i)=>{const b=polygon[(i+1)%polygon.length];return (b[0]-a[0])*(y+.5-a[1])-(b[1]-a[1])*(x+.5-a[0])>=-1e-9;}))raster[y*width+x]=1;
  }
  if(!raster.some(Boolean))throw Error('empty-crown: zero ROI area');
  return {polygon,raster};
}
export function analyzeGaps({coverage,roi,width,height,metresPerPixel}) {
  const n=width*height;array(coverage,n,true);array(roi,n,true);
  if(!Number.isFinite(metresPerPixel)||metresPerPixel<=0)throw Error('roi-mismatch: scale');
  let area=0,minY=height,maxY=-1;
  for(let i=0;i<n;i++)if(roi[i]){
    if(roi[i]!==1)throw Error('roi-mismatch: nonbinary ROI');
    const x=i%width,y=Math.floor(i/width);if(x===0||y===0||x===width-1||y===height-1)throw Error('clipped-subject: ROI edge');
    area++;minY=Math.min(minY,y);maxY=Math.max(maxY,y);
  }
  if(!area||!coverage.some(v=>v>0))throw Error('empty-crown');
  const neighbors=i=>{const x=i%width,y=Math.floor(i/width);return [x?i-1:-1,x<width-1?i+1:-1,y?i-width:-1,y<height-1?i+width:-1];};
  const at=threshold=>{
    const seen=new Uint8Array(n),bands=Array.from({length:4},()=>({roi_pixels:0,occupied_pixels:0,exterior_pixels:0,enclosed_pixels:0}));
    let occupied=0,outside=0,total=0,exterior=0,enclosed=0,largest=0;const holes=[];
    for(let i=0;i<n;i++){
      if(coverage[i]>=threshold){total++;if(roi[i])occupied++;else outside++;}
      if(roi[i]){const y=Math.floor(i/width),b=Math.min(3,Math.floor(4*(maxY-y)/Math.max(1,maxY-minY)));bands[b].roi_pixels++;if(coverage[i]>=threshold)bands[b].occupied_pixels++;}
      if(!roi[i]||coverage[i]>=threshold||seen[i])continue;
      const queue=[i];seen[i]=1;let open=false;
      for(let j=0;j<queue.length;j++)for(const k of neighbors(queue[j])){
        if(k<0||!roi[k])open=true;
        else if(!seen[k]&&coverage[k]<threshold){seen[k]=1;queue.push(k);}
      }
      if(open)exterior+=queue.length;else{enclosed+=queue.length;holes.push(queue.length);largest=Math.max(largest,queue.length);}
      for(const k of queue){const y=Math.floor(k/width),band=Math.min(3,Math.floor(4*(maxY-y)/Math.max(1,maxY-minY)));bands[band][open?'exterior_pixels':'enclosed_pixels']++;}
    }
    const edges=[1,4,16,64,256,1024],hist=edges.map((e,i)=>holes.filter(a=>a>=e&&(i===edges.length-1||a<edges[i+1])).length);
    return {threshold,roi_pixels:area,occupied_pixels:occupied,occupied_ratio:occupied/area,exterior_pixels:exterior,exterior_ratio:exterior/area,enclosed_pixels:enclosed,enclosed_ratio:enclosed/area,hole_count:holes.length,largest_hole_pixels:largest,largest_hole_m2:Number((largest*metresPerPixel**2).toPrecision(15)),hole_area_histogram:{edges_px:[...edges,null],counts:hist},outside_roi_pixels:outside,outside_roi_fraction:total?outside/total:null,height_bands:bands.map(b=>({...b,occupied_ratio:b.roi_pixels?b.occupied_pixels/b.roi_pixels:null,exterior_ratio:b.roi_pixels?b.exterior_pixels/b.roi_pixels:null,enclosed_ratio:b.roi_pixels?b.enclosed_pixels/b.roi_pixels:null}))};
  };
  const primary=at(.5),sensitivity=[.25,.75].map(t=>{const r=at(t);return {...r,difference_from_primary:{occupied_pixels:r.occupied_pixels-primary.occupied_pixels,hole_count:r.hole_count-primary.hole_count,enclosed_pixels:r.enclosed_pixels-primary.enclosed_pixels,exterior_pixels:r.exterior_pixels-primary.exterior_pixels}};});
  return {definition:'projected-gaps-v1',status:'measured',units:{area:'pixels; projected m2',coverage:'linear MSAA fraction',ratio:'ROI area fraction'},background_connectivity:4,foreground_connectivity:8,metres_per_pixel:metresPerPixel,primary,sensitivity,biological_assessment:'unassessed',favorable_direction:null};
}
