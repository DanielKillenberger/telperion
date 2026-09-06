from pathlib import Path
for id in ['oak','spruce']:
 out=['<svg xmlns="http://www.w3.org/2000/svg" width="900" height="900"><rect width="900" height="900" fill="#eeeeeb"/>']
 for line in Path('.flow/tmp/habits.txt').read_text().splitlines():
  a=line.split()
  if a[0]!=id:continue
  _,index,*v=a; x,y,z,xx,yy,zz,r,rr=map(float,v); scale=34
  x=450+(x*.85+z*.5)*scale;xx=450+(xx*.85+zz*.5)*scale;y=860-y*scale;yy=860-yy*scale
  out.append(f'<line x1="{x}" y1="{y}" x2="{xx}" y2="{yy}" stroke="#413c34" stroke-width="{max(.35,(r+rr)*scale)}" stroke-linecap="round"/>')
 out.append('</svg>');Path(f'.flow/tmp/{id}.svg').write_text('\n'.join(out))
