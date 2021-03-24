def run_step(bins):
    bins = list(bins)
    blocks = max(bins)
    i = bins.index(blocks)
    bins[i] = 0
    for j in range(blocks):
        bins[(i + 1 + j) % len(bins)] += 1
    return tuple(bins)

def how_long_must_we_toil(bins):
    seen = {bins: 0}
    steps = 0
    while 1:
        bins = run_step(bins)
        steps += 1
        if bins in seen:
            return steps - seen[bins]
        seen[bins] = steps

assert run_step((0, 2, 7, 0)) == (2, 4, 1, 2)
assert run_step((2, 4, 1, 2)) == (3, 1, 2, 3)
assert run_step((3, 1, 2, 3)) == (0, 2, 3, 4)
assert run_step((0, 2, 3, 4)) == (1, 3, 4, 1)
assert run_step((1, 3, 4, 1)) == (2, 4, 1, 2)
assert how_long_must_we_toil((0, 2, 7, 0)) == 4

if __name__ == '__main__':
    with open('puzzle-input.txt') as f:
        bins = tuple(int(x) for x in f.read().split())
    print(how_long_must_we_toil(bins))
