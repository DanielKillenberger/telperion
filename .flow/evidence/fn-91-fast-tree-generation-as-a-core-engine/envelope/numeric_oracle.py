import decimal, json
from pathlib import Path
ctx=decimal.getcontext(); ctx.prec=60
D=decimal.Decimal.from_float
ev=Path(__file__).resolve().parent
max_inner=decimal.Decimal(0); max_shape=decimal.Decimal(0); count=0
for line in (ev/'numeric.csv').read_text().splitlines():
 s,p,a,r=map(float,line.split(',')); count+=1
 exact_inner=ctx.power(D(p),D(s))
 exact_shape=ctx.power(1-exact_inner, 1/D(s))
 max_inner=max(max_inner,abs(D(a)-exact_inner))
 max_shape=max(max_shape,abs(D(r)-exact_shape))
assert count==8192
assert max_inner < decimal.Decimal('1e-12') and max_shape < decimal.Decimal('1e-9')
result={'samples':count,'oracle':'Python Decimal 60-digit power; exact binary64 input conversion; mathematical reciprocal','max_inner_absolute_error':str(max_inner),'max_normalized_shape_absolute_error':str(max_shape),'normalized_margin':'1e-6','limitation':'Empirical pinned-implementation qualification; no exhaustive or formal libm error guarantee.'}
(ev/'numeric-summary.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result))
