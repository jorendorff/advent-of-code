grid = []
steps = []

with open('solution.txt') as f:
    for line in f:
        if line.strip() == '':
            break
        grid.append(line.rstrip('\n'))
    for line in f:
        t, rest = line.rstrip('\n').split(' ', 1)
        steps.append((int(t), eval(rest)))

# check first and last step
assert steps[0][0] == 0
assert steps[0][1] == (0, 0, 'torch')
assert steps[-1][1] == (9, 751, 'torch')

pt, (px, py, pq) = steps[0]
for step in steps[1:]:
    t, (x, y, q) = step
    print(step)
    assert 0 <= y < len(grid)
    if x < 0:
        print("FAIL")
    if x >= len(grid[y]):
        print(step)
    assert 0 <= x < len(grid[y])
    # check that q is valid for (x, y)
    # rocky as ., wet as =, narrow as |
    region = grid[y][x]
    if region in ('.', 'M', 'T'):
        assert q == 'gear' or q == 'torch'
    elif region == '=':
        assert q == 'gear' or q == 'neither'
    else:
        assert region == '|'
        assert q == 'torch' or q == 'neither'

    if q == pq:
        dx = x - px
        dy = y - py
        assert abs(dx) + abs(dy) == 1
        assert t == pt + 1
    else:
        assert x == px
        assert y == py
        assert t == pt + 7
    pt, (px, py, pq) = step

print("it is ok")
