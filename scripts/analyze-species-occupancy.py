"""Project occupancy_audit output; requires NumPy and Pillow in a local venv.

Usage: python scripts/analyze-species-occupancy.py /tmp/fn99-pass5-audit
The diagnostic masks are never substituted for full connected renderer views.
"""
import json,pathlib,sys,numpy as np
from PIL import Image,ImageDraw
src=pathlib.Path(sys.argv[1]); rows=[]
for p in sorted(src.glob('*-[0-9]*.json')):
 d=json.loads(p.read_text()); row={k:d[k] for k in ['preset','seed','nodes','crossover','upper_m']}
 if 'curtain' not in d:
  allp=np.array([v for s in d['supports'] for v in s['twig_endpoints']]); centre=allp.mean(0)
  row['upper_endpoint_centre']=centre.tolist();row['supports']=[]
  for s in d['supports']:
   pts=np.array(s['twig_endpoints']);base=np.array(s['position']);delta=pts-base
   row['supports'].append(dict(node=s['node'],parent=s['parent'],position=s['position'],twigs=len(pts),centroid=pts.mean(0).tolist(),toward_upper_centre_fraction=float(np.mean(delta@(centre-base)>0)),upward_fraction=float(np.mean(delta[:,1]>0)),bounds=[pts.min(0).tolist(),pts.max(0).tolist()]))
 else:
  c=d['curtain'];origin=np.array(c['origin']);radial=origin.copy();radial[1]=0;radial/=np.linalg.norm(radial);across=np.cross([0,1,0],radial)
  verts=np.array(c['prototype']);indices=np.array(c['needle_indices']).reshape(-1,3);projected=[];runs=[]
  for r in c['runs']:
   m=np.array(r['matrices']).reshape(-1,4,4).transpose(0,2,1);v=np.einsum('nij,vj->nvi',m[:,:3,:3],verts)+m[:,:3,3][:,None,:]-origin
   xy=np.stack([v@across,-v[:,:,1]],axis=-1); projected.append(xy)
   # Resolve each supporting run's tangent from its original station origins.
   tangent=m[-1,:3,3]-m[0,:3,3]
   single=None
   if np.linalg.norm(tangent)>1e-8:
    tangent/=np.linalg.norm(tangent); transverse=np.cross(tangent,radial)
    if np.linalg.norm(transverse)<1e-8:transverse=np.cross(tangent,[1,0,0])
    transverse/=np.linalg.norm(transverse)
    q=np.stack([v@transverse,v@tangent],axis=-1);low=q.min((0,1));high=q.max((0,1));step=.00025
    sz=np.ceil((high-low)/step).astype(int)+1;mask=Image.new('1',tuple(sz));pen=ImageDraw.Draw(mask)
    for needle in q:
     pp=(needle-low)/step
     for tri in indices:pen.polygon([tuple(pt) for pt in pp[tri]],fill=1)
    rr=np.array(mask);single=dict(pitch_m=step,bbox_coverage=float(rr.mean()),mean_covered_width_m=float(rr.sum(1).mean()*step),empty_longitudinal_fraction=float(np.mean(~rr.any(1))),transverse_span_m=float(high[0]-low[0]))
   runs.append(dict(own_tangent_coverage=single,nodes=r['nodes'],length_m=r['length_m'],first_instance=r['first_instance'],needles=len(m),transverse_span_m=float(np.ptp(xy[:,:,0])),longitudinal_span_m=float(np.ptp(xy[:,:,1]))))
  xy=np.concatenate(projected);lo=xy.min((0,1));hi=xy.max((0,1));pitch=.0005;size=np.ceil((hi-lo)/pitch).astype(int)+1
  im=Image.new('1',tuple(size));draw=ImageDraw.Draw(im)
  for needle in xy:
   pts=(needle-lo)/pitch
   for tri in indices:draw.polygon([tuple(v) for v in pts[tri]],fill=1)
  a=np.array(im);bins=[]
  wood=Image.new('1',tuple(size));wd=ImageDraw.Draw(wood)
  for triangle in c.get('wood_triangles',[]):
   v=np.array(triangle)-origin
   q=np.stack([v@across,-v[:,1]],axis=-1)
   wd.polygon([tuple(pt) for pt in (q-lo)/pitch],fill=1)
  w=np.array(wood)
  wood_diagnostic=dict(available='wood_triangles' in c,method='Original swept surface side triangles on the same selected connected system; end caps excluded; clipped to needle projection bounds.',projected_area_m2=float(w.sum()*pitch*pitch),needle_only_area_m2=float((a&~w).sum()*pitch*pitch),wood_only_area_m2=float((w&~a).sum()*pitch*pitch),overlap_area_m2=float((w&a).sum()*pitch*pitch))
  for start in range(0,len(a),100):
   band=a[start:start+100];bins.append(dict(depth_m=float(lo[1]+start*pitch),mean_covered_width_m=float(band.sum(1).mean()*pitch),fraction_covered=float(band.mean()),wood_fraction=float(w[start:start+100].mean()),needle_wood_overlap_fraction=float((band&w[start:start+100]).mean())))
  row['curtain']=dict(wood_projection=wood_diagnostic,root=c['root'],socket=c['socket'],total_instances=c['total_instances'],selected_instances=len(xy),pitch_m=pitch,bounds=[lo.tolist(),hi.tolist()],covered_area_m2=float(a.sum()*pitch*pitch),bbox_coverage=float(a.mean()),bins_50mm=bins,runs=runs)
  print(p.stem,'coverage',round(a.mean(),3),'size',np.round(hi-lo,3),'runs',len(runs),'needle',len(xy))
 rows.append(row)
(src/'summary.json').write_text(json.dumps(dict(method='Exact original instance IDs/matrices; projected needle triangles with separate original wood side-triangle masks where supplied, vertical tangent plane of selected exterior secondary. 0.5mm diagnostic raster; 50mm longitudinal bands. Not a botanical gate or rendered acceptance.',cases=rows),indent=2))
