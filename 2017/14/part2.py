from functools import reduce

def swap(state, pos, length):
    state = state[pos:] + state[:pos]
    state = state[:length][::-1] + state[length:]
    state = state[len(state)-pos:] + state[:len(state)-pos]
    return state

assert swap([0, 1, 2, 3, 4], 0, 3) == [2, 1, 0, 3, 4]
assert swap([2, 1, 0, 3, 4], 3, 4) == [4, 3, 0, 1, 2]
assert swap([4, 3, 0, 1, 2], 3, 1) == [4, 3, 0, 1, 2]
assert swap([4, 3, 0, 1, 2], 1, 5) == [3, 4, 2, 1, 0]

def knot_hash_rounds(state, lengths, nrounds):
    lengths = list(lengths) + [17, 31, 73, 47, 23]
    n = len(state)
    pos = 0
    skip_size = 0
    for i in range(nrounds):
        for length in lengths:
            state = swap(state, pos, length)
            pos = (pos + length + skip_size) % n
            skip_size += 1
    return state

def knot_hash(data):
    lengths = [ord(c) for c in data]
    sparse_hash = knot_hash_rounds(list(range(256)), lengths, nrounds=64)
    block_size = 16
    dense_hash = [reduce(lambda a, b: a ^ b, sparse_hash[i:i+block_size], 0)
                  for i in range(0, len(sparse_hash), block_size)]
    return dense_hash

def hashes(key):
    for i in range(128):
        yield knot_hash(key + '-' + str(i))

def grid(key):
    return [[(byte & (1 << bit)) >> bit
             for byte in dense_hash
             for bit in reversed(range(8))]
            for dense_hash in hashes(key)]

def draw_corner(key):
    h = grid(key)
    s = ''
    for row in h[:8]:
        s += ''.join('.#'[bit] for bit in row[:8]) + '\n'
    return s

example_key = 'flqrgnkx'

example_output = '''\
##.#.#..
.#.#.#.#
....#.#.
#.#.##.#
.##.#...
##..#..#
.#...#..
##.#.##.
'''

assert draw_corner(example_key) == example_output

def adjacent_pairs(map):
    h = len(map)
    w = len(map[0])
    for y in range(h):
        row = map[y]
        for x in range(w - 1):
            if row[x] and row[x + 1]:
                yield ((x, y), (x + 1, y))
    for x in range(w):
        for y in range(h - 1):
            if map[y][x] and map[y + 1][x]:
                yield ((x, y), (x, y + 1))

class EquivalenceClasses:
    def __init__(self, edges=[]):
        self.groups = {}
        self.count = 0
        for a, b in edges:
            self.add_edge(a, b)

    def get(self, a):
        if a not in self.groups:
            self.groups[a] = set([a])
            self.count += 1
        return self.groups[a]

    def add_edge(self, a, b):
        g1 = self.get(a)
        if b not in g1:
            g2 = self.get(b)
            assert g1 & g2 == set()
            for e in g2:
                g1.add(e)
                self.groups[e] = g1
            self.count -= 1

def regions(key):
    g = grid(key)
    ec = EquivalenceClasses(adjacent_pairs(g))

    # Add every point. Necessary because regions that consist of only one point
    # are still not present.
    for y in range(len(g)):
        for x in range(len(g[y])):
            if g[y][x]:
                ec.get((x, y))

    return ec

def explain(key):
    g = grid(key)
    ec = regions(key)
    for y in range(len(g)):
        for x in range(len(g[y])):
            if g[y][x] == 1:
                g[y][x] = '#'
            else:
                g[y][x] = '.'
    next = 1
    for y in range(len(g)):
        for x in range(len(g[y])):
            if g[y][x] == '#':
                for xi, yi in ec.groups[x, y]:
                    g[yi][xi] = str(next)
                next += 1
    for row in g:
        print(' '.join('%3s' % v for v in row))


#print(how_many_regions(example_key))
#explain(example_key)

assert regions(example_key).count == 1242

with open("puzzle-input.txt") as f:
    key = f.read().strip()
    print(regions(key).count)
