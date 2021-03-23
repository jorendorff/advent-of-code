


with open("input.txt") as f:
    lines = [line.rstrip("\n") for line in f.readlines()]

def solve(lines):
    assert len(lines) > 0
    width = len(lines[0])
    assert all(len(line) == width for line in lines)
    assert all(line.strip(".#") == "" for line in lines)

    m = 3
    x = 0
    trees = sum(1 for i, line in enumerate(lines) if line[m * i % width] == '#')
    return trees

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

assert solve(example) == 7

print(solve(lines))
