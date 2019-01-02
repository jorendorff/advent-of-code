puzzle_input = 409551

def step(recipes, positions):
    combo = sum(recipes[i] for i in positions)
    recipes += [int(c) for c in str(combo)]
    positions = [(i + 1 + recipes[i]) % len(recipes) for i in positions]
    return recipes, positions

assert step([3, 7], [0, 1]) == ([3, 7, 1, 0], [0, 1])
assert step([3, 7, 1, 0], [0, 1]) == ([3, 7, 1, 0, 1, 0], [4, 3])


def f(target):
    target = [int(d) for d in target]
    n = len(target)
    recipes = [3, 7]
    positions = [0, 1]
    while True:
        recipes, positions = step(recipes, positions)
        if recipes[-(n+1):-1] == target:
            return len(recipes) - (n + 1)
        if recipes[-n:] == target:
            return len(recipes) - n

assert f('51589') == 9
assert f('01245') == 5
assert f('92510') == 18
assert f('59414') == 2018

print(f(str(puzzle_input)))

