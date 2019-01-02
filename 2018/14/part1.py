puzzle_input = 409551

def step(recipes, positions):
    combo = sum(recipes[i] for i in positions)
    recipes += [int(c) for c in str(combo)]
    positions = [(i + 1 + recipes[i]) % len(recipes) for i in positions]
    return recipes, positions

assert step([3, 7], [0, 1]) == ([3, 7, 1, 0], [0, 1])
assert step([3, 7, 1, 0], [0, 1]) == ([3, 7, 1, 0, 1, 0], [4, 3])


def f(n):
    recipes = [3, 7]
    positions = [0, 1]
    for i in range(n + 6):
        recipes, positions = step(recipes, positions)
    assert len(recipes) >= n + 10
    return ''.join(str(score) for score in recipes[n:n + 10])

assert f(5) == '0124515891'
assert f(9) == '5158916779'
assert f(2018) == '5941429882'

print(f(puzzle_input))

