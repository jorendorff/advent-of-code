from lib.advent import *


DIRS = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
]


def follow_instructions(text):
    x, y = 0, 0
    h = 0
    for s in text.split():
        action = s[0]
        value = int(s[1:])
        if action == 'N':
            y += value
        elif action == 'S':
            y -= value
        elif action == 'E':
            x += value
        elif action == 'W':
            x -= value
        elif action == 'L':
            assert value % 90 == 0
            h += value // 90
        elif action == 'R':
            assert value % 90 == 0
            h -= value // 90
        elif action == 'F':
            dx, dy = DIRS[h % 4]
            x += value * dx
            y += value * dy
        else:
            raise ValueError(f"unrecognized action in {s}")
    return x, y


def solve(text):
    x, y = follow_instructions(text)
    return abs(x) + abs(y)


example = "F10 N3 F7 R90 F11"
assert solve(example) == 25


if __name__ == '__main__':
    print(solve(puzzle_input()))
