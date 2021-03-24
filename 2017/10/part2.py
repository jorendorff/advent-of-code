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

def bad_hash_rounds(state, lengths, nrounds):
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

def bad_hash(data):
    lengths = [ord(c) for c in data]
    sparse_hash = bad_hash_rounds(list(range(256)), lengths, nrounds=64)
    block_size = 16
    dense_hash = [reduce(lambda a, b: a ^ b, sparse_hash[i:i+block_size], 0)
                  for i in range(0, len(sparse_hash), block_size)]
    return ''.join('%02x' % byte for byte in dense_hash)

assert bad_hash("") == "a2582a3a0e66e6e86e3812dcb672a272"
assert bad_hash("AoC 2017") == "33efeb34ea91902bb2f59c9920caa6cd"
assert bad_hash("1,2,3") == "3efbe78a8d82f29979031a4aa0b16a9d"
assert bad_hash("1,2,4") == "63960835bcdc130f0b66d7ff4f6a5a8e"

with open("puzzle-input.txt") as f:
    data = f.read().strip()
print(bad_hash(data))
