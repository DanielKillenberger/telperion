"""fn-211 task 9 (host decision 2026-10-07): atan2 as a table-reduced
polynomial. Prints A_j = round(atan(j/32) / 2pi 2^62), j = 0..32, in
turns, and the series' coefficients C_k = round((-1)^k / ((2k + 1) 2pi)
2^62), k = 0..3, at 100 digits."""
from decimal import Decimal as D, getcontext, ROUND_HALF_EVEN
getcontext().prec = 100
Q = D(2) ** 62


def atan(x):
    x, k = D(x), 0
    while abs(x) > D("0.01"):
        x, k = x / (1 + (1 + x * x).sqrt()), k + 1
    s, term, n = D(0), x, 1
    while abs(term) > D(10) ** -90:
        s, term, n = s + term / n, term * -x * x, n + 2
    return s * 2 ** k


TAU = 8 * atan(1)


def nearest(v):
    return int(v.to_integral_value(rounding=ROUND_HALF_EVEN))


print("A =", [hex(nearest(atan(D(j) / 32) / TAU * Q)) for j in range(33)])
print("C =", [nearest(D((-1) ** k) / ((2 * k + 1) * TAU) * Q) for k in range(4)])
