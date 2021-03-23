from lib.advent import *


DIRS = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
]


def follow_instructions(text):
    x, y = 0, 0
    wx, wy = 10, 1  # coordinates of waypoint relative to (x, y)
    for s in text.split():
        action = s[0]
        value = int(s[1:])
        if action == 'N':
            wy += value
        elif action == 'S':
            wy -= value
        elif action == 'E':
            wx += value
        elif action == 'W':
            wx -= value
        elif action == 'L':
            assert value % 90 == 0
            for _ in range(value // 90 % 4):
                wx, wy = -wy, wx
        elif action == 'R':
            assert value % 90 == 0
            for _ in range(value // 90 % 4):
                wx, wy = wy, -wx
        elif action == 'F':
            x += value * wx
            y += value * wy
        else:
            raise ValueError(f"unrecognized action in {s}")
    return x, y


def solve(text):
    x, y = follow_instructions(text)
    return abs(x) + abs(y)


example = "F10 N3 F7 R90 F11"
assert solve(example) == 286


if __name__ == '__main__':
    print(solve(puzzle_input()))
