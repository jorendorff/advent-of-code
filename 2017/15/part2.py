import itertools

def gen(state, factor, threshold):
    assert threshold != 0
    assert threshold & (threshold - 1) == 0
    mask = threshold - 1
    while True:
        state = (state * factor) % 2147483647
        if state & mask == 0:
            yield state

A = 16807
B = 48271

THR_A = 4
THR_B = 8

assert list(zip(itertools.islice(gen(65, A, THR_A), 5), gen(8921, B, THR_B))) == [
    (1352636452, 1233683848),
    (1992081072, 862516352),
    (530830436, 1159784568),
    (1980017072, 1616057672),
    (740335192, 412269392),
]

def judge(a, b, n):
    return sum(1
               for x, y in zip(itertools.islice(a, n), b)
               if (x & ((1 << 16) - 1)) == (y & ((1 << 16) - 1)))

assert judge(gen(65, A, THR_A), gen(8921, B, THR_B), 5) == 0
assert judge(gen(65, A, THR_A), gen(8921, B, THR_B), 1055) == 0
assert judge(gen(65, A, THR_A), gen(8921, B, THR_B), 1056) == 1
assert judge(gen(65, A, THR_A), gen(8921, B, THR_B), 5_000_000) == 309

with open('puzzle-input.txt') as f:
    a0 = int(f.readline().split()[-1])
    b0 = int(f.readline().split()[-1])
    print(judge(gen(a0, A, THR_A), gen(b0, B, THR_B), 5_000_000))

