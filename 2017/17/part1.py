
def storm(nrounds, nsteps):
    state = [0]
    pos = 0
    for i in range(1, nrounds + 1):
        pos = (pos + nsteps) % len(state)
        pos += 1
        state.insert(pos, i)
    return state

assert storm(1, 3) == [0, 1]
assert storm(2, 3) == [0, 2, 1]
assert storm(9, 3) == [0, 9, 5, 7, 2, 4, 3, 8, 6, 1]
example = storm(2017, 3)
example_hit = example.index(2017)
assert example[example_hit - 3:example_hit + 4] == [1512, 1134, 151, 2017, 638, 1513, 851]

def after(nrounds, nsteps):
    result = storm(nrounds, nsteps)
    i = result.index(nrounds)
    return result[(i + 1) % len(result)]

assert after(2017, 3) == 638

with open("puzzle-input.txt") as f:
    nsteps = int(f.read().strip())
print(after(2017, nsteps))
