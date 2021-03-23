PUBLIC_KEYS = [8987316, 14681524]

MODULUS = 20201227

def is_prime(n):
    return [i for i in range(2, n + 1) if n % i == 0] == [n]


def transform(subject, loop_size):
    return pow(subject, loop_size, MODULUS)


def to_loop_size(pk):
    assert 0 < pk < MODULUS
    n = 1
    for i in range(MODULUS - 1):
        if n == pk:
            assert transform(7, i) == pk
            return i
        n = (n * 7) % MODULUS
    assert False

loop_sizes = [to_loop_size(k) for k in PUBLIC_KEYS]
print(loop_sizes)
print(transform(PUBLIC_KEYS[0], loop_sizes[1]))
print(transform(PUBLIC_KEYS[1], loop_sizes[0]))
