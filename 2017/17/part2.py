
def get_value_after_zero(nrounds, nsteps):
    print("storm({}, {})".format(nrounds, nsteps))
    pos = 0
    zero_pos = 0
    value_after_zero = 0
    for i in range(1, nrounds + 1):
        pos = (pos + nsteps) % i
        pos += 1
        if pos == zero_pos + 1:
            value_after_zero = i
        if pos <= zero_pos:
            zero_pos += 1
    return value_after_zero

assert get_value_after_zero(1, 3) == 1
assert get_value_after_zero(2, 3) == 2
assert get_value_after_zero(9, 3) == 9

with open("puzzle-input.txt") as f:
    nsteps = int(f.read().strip())
MANY_ROUNDS = 50_000_000
print(get_value_after_zero(MANY_ROUNDS, nsteps))
