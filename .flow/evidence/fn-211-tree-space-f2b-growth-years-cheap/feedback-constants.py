"""fn-211 task 8: the feedback contract's literal constants (CONTRACT-A.md
2.3), in exact integer arithmetic at 2^400: 2 pi in Q60, 1 / (2 pi) in
Q64, the CORDIC table atan(2^-i) in turns at 2^62 a turn, and its gain.
Each is the nearest integer, ties away from zero (none is a tie)."""
S = 400
ONE = 1 << S


def atan_inv(n):
    """atan(1 / n) at scale ONE, by its series."""
    total, term, k, sign = 0, ONE // n, 1, 1
    while term:
        total += sign * (term // k)
        term //= n * n
        k += 2
        sign = -sign
    return total


def atan_pow2(i):
    """atan(2^-i) at scale ONE."""
    if i == 0:
        return PI // 4
    x = ONE >> i
    total, power, k, sign = 0, x, 1, 1
    while power:
        total += sign * (power // k)
        power = power * x * x >> (2 * S)
        k += 2
        sign = -sign
    return total


def isqrt(n):
    x = 1 << ((n.bit_length() + 1) // 2)
    while True:
        y = (x + n // x) // 2
        if y >= x:
            return x
        x = y


def nearest(num, shift_from):
    """num / 2^shift_from to the nearest integer."""
    return (num + (1 << (shift_from - 1))) >> shift_from


PI = 4 * (4 * atan_inv(5) - atan_inv(239))
TAU = 2 * PI
N = 40
print(f"TAU60 = {nearest(TAU, S - 60):#x}")
print(f"INV_TAU64 = {nearest((ONE << S) // TAU, S - 64):#x}")
print("ATAN = [")
for i in range(N):
    turns = atan_pow2(i) * ONE // TAU
    print(f"    {nearest(turns, S - 62):#x},")
print("]")
k = ONE
for i in range(N):
    k = k * ONE // isqrt((ONE + (ONE >> (2 * i))) * ONE)
print(f"K = {nearest(k, S - 62):#x}")
