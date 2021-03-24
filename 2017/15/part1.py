import itertools

def gen(state, factor):
    while True:
        state = (state * factor) % 2147483647
        yield state

A = 16807
B = 48271

assert list(zip(itertools.islice(gen(65, A), 5), gen(8921, B))) == [
    (1092455, 430625591),
    (1181022009, 1233683848),
    (245556042, 1431495498),
    (1744312007, 137874439),
    (1352636452, 285222916),
]

def judge(a, b, n):
    return sum(1
               for x, y in zip(itertools.islice(a, n), b)
               if (x & ((1 << 16) - 1)) == (y & ((1 << 16) - 1)))

assert judge(gen(65, A), gen(8921, B), 5) == 1
#assert judge(gen(65, A), gen(8921, B), 40_000_000) == 588

with open('puzzle-input.txt') as f:
    a0 = int(f.readline().split()[-1])
    b0 = int(f.readline().split()[-1])
    print(judge(gen(a0, A), gen(b0, B), 40_000_000))
        
