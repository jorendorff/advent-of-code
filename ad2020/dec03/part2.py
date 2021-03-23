


with open("input.txt") as f:
    lines = [line.rstrip("\n") for line in f.readlines()]

def trees_encountered(lines, dx, dy):
    assert len(lines) > 0
    width = len(lines[0])
    assert all(len(line) == width for line in lines)
    assert all(line.strip(".#") == "" for line in lines)

    trees = sum(1 for i, line in enumerate(lines[::dy]) if line[dx * i % width] == '#')
    return trees

SLOPES = [
    (1, 1),
    (3, 1),
    (5, 1),
    (7, 1),
    (1, 2),
]

def solve(lines):
    product = 1
    for slope in SLOPES:
        product *= trees_encountered(lines, *slope)
    return product

example = [
    "..##.......",
    "#...#...#..",
    ".#....#..#.",
    "..#.#...#.#",
    ".#...##..#.",
    "..#.##.....",
    ".#.#.#....#",
    ".#........#",
    "#.##...#...",
    "#...##....#",
    ".#..#...#.#",
]

assert solve(example) == 336

print(solve(lines))
