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

def draw_corner(key):
    h = list(hashes(key))
    s = ''
    for row in h[:8]:
        byte = row[0]
        s += ''.join('.#'[(byte & (1 << bit)) >> bit] for bit in reversed(range(8))) + '\n'
    return s

def population_count(key):
    return sum(1
               for dense_hash in hashes(key)
               for byte in dense_hash
               for bit in range(8)
               if (1 << bit) & byte)

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
assert population_count(example_key) == 8108

with open("puzzle-input.txt") as f:
    key = f.read().strip()
    print(population_count(key))
