
class RuleMap:
    def __init__(self, lines):
        self._table = {}
        for line in lines:
            left, right = line.strip().split(" => ")
            left = left.split("/")
            right = right.split("/")
            leftT = [''.join(left[y][x] for y in range(len(left))) for x in range(len(left))]

            keys = set()
            for dx in 1, -1:
                for dy in 1, -1:
                    for src in left, leftT:
                        keys.add("/".join(row[::dx] for row in src[::dy]))

            for key in keys:
                assert key not in self._table
                self._table[key] = right

    def __getitem__(self, tile):
        return self._table[tile]

class Art:
    def __init__(self, lines):
        self.lines = [line.rstrip('\n') for line in lines]

        size = len(self.lines)
        for line in self.lines:
            if len(line) != size:
                raise ValueError("bad line length: " + line)

    def tile(self, x, y, size):
        return '/'.join(row[x:x+size] for row in self.lines[y:y+size])

    def refine(self, rule_map):
        size = len(self.lines)
        assert all(len(line) == size for line in self.lines)
        if size % 2 == 0:
            n = 2
        else:
            assert size % 3 == 0
            n = 3

        output_tiles = [[rule_map[self.tile(x, y, n)]
                         for x in range(0, len(self.lines[0]), n)]
                        for y in range(0, len(self.lines), n)]
        return Art([
            ''.join(tile[subrow] for tile in output_tile_row)
            for output_tile_row in output_tiles
            for subrow in range(n + 1)])

    def population(self):
        return sum(row.count('#') for row in self.lines)

sample_rules = '''\
../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#
'''
sample_rule_map = RuleMap(sample_rules.splitlines())

start = '''\
.#.
..#
###
'''.splitlines()
assert Art(start).population() == 5
assert Art(start).refine(sample_rule_map).refine(sample_rule_map).population() == 12

with open("puzzle-input.txt") as f:
    rule_map = RuleMap(f)
    art = Art(start)
    for i in range(18):
        art = art.refine(rule_map)
    print(art.population())
