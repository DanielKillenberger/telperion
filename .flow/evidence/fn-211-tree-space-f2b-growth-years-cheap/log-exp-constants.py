"""fn-211 task 9 (host decision 2026-10-07): the decision contract's log2
and exp2_neg as table-reduced Q62 polynomials. Prints the literal
constants: 64 reciprocals r_j = round(2^62 / c_j), c_j = 1 + (j + 1/2)/64,
their logs L_j = round(-log2(r_j / 2^62) 2^62), the log polynomial's
coefficients (-1)^(k+1) / (k ln 2) for k = 1..6, the 64 powers
E_j = round(2^(-j/64) 2^62), the exponential's (-1)^k / k! for k = 0..6,
and ln 2 in Q62. Each the nearest integer, at 120 digits."""
from decimal import Decimal as D, getcontext, ROUND_HALF_EVEN
getcontext().prec = 120
Q = D(2) ** 62
LN2 = D(2).ln()


def nearest(x):
    return int(x.to_integral_value(rounding=ROUND_HALF_EVEN))


recip, logs, powers = [], [], []
for j in range(64):
    c = 1 + (D(j) + D(1) / 2) / 64
    r = nearest(Q / c)
    recip.append(r)
    logs.append(nearest(-(D(r) / Q).ln() / LN2 * Q))
    powers.append(nearest((-(D(j) / 64) * LN2).exp() * Q))
log_poly = [nearest(D((-1) ** (k + 1)) / (k * LN2) * Q) for k in range(1, 7)]
fact = 1
exp_poly = []
for k in range(7):
    if k:
        fact *= k
    exp_poly.append(nearest(D((-1) ** k) / fact * Q))


def show(name, xs):
    print(f"{name} = [")
    for x in xs:
        print(f"    {'-' if x < 0 else ''}{abs(x):#x},")
    print("]")


show("RECIP", recip)
show("LOGS", logs)
show("LOG_POLY", log_poly)
show("POWERS", powers)
show("EXP_POLY", exp_poly)
print(f"LN2_62 = {nearest(LN2 * Q):#x}")
