def swap(state, pos, length):
    state = state[pos:] + state[:pos]
    state = state[:length][::-1] + state[length:]
    state = state[len(state)-pos:] + state[:len(state)-pos]
    return state

assert swap([0, 1, 2, 3, 4], 0, 3) == [2, 1, 0, 3, 4]
assert swap([2, 1, 0, 3, 4], 3, 4) == [4, 3, 0, 1, 2]
assert swap([4, 3, 0, 1, 2], 3, 1) == [4, 3, 0, 1, 2]
assert swap([4, 3, 0, 1, 2], 1, 5) == [3, 4, 2, 1, 0]

def bad_hash(n, lengths):
    state = list(range(n))
    pos = 0
    skip_size = 0
    for length in lengths:
        state = swap(state, pos, length)
        pos = (pos + length + skip_size) % n
        skip_size += 1
    return state[0] * state[1]

assert bad_hash(5, [3, 4, 1, 5]) == 12

with open("puzzle-input.txt") as f:
    lengths = [int(s) for s in f.read().strip().split(',')]
    print(bad_hash(256, lengths))
