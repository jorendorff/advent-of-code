from lib.advent import *


SIZE = 10


class Tile:
    """A single orientation of a tile, including its id and bitmap."""

    def __init__(self, id, rows):
        self.id = id
        self.rows = rows
        self.edge_code = int(''.join(rows[0]).replace('.', '0').replace('#', '1'), 2)

    def turn(self):
        """Return a new row-list, rotating `rows` by 90° clockwise."""
        rows = self.rows
        return Tile(self.id, [[rows[SIZE - x - 1][y] for x in range(SIZE)] for y in range(SIZE)])

    def flip(self):
        """Flip the given list-of-lists along the main diagonal."""
        rows = self.rows
        return Tile(self.id, [[rows[x][y] for x in range(SIZE)] for y in range(SIZE)])

    def orientations(self):
        for start in [self, self.flip()]:
            tile = start
            for _ in range(4):
                yield tile
                tile = tile.turn()


def parse_input(text):
    tiles_text = text.strip().replace("\n\n", "\n----\n").split("----\n")
    tiles = {}
    for tile in tiles_text:
        lines = tile.splitlines()
        first = lines.pop(0).rstrip()
        if not (first.startswith("Tile ") and first.endswith(":") and first[5:-1].isdigit()):
            raise ValueError("can't parse line: " + first.rstrip())
        id = int(first[5:-1])
        if id in tiles:
            raise ValueError(f"more than one tile {id} found in input")
        rows = [row.rstrip() for row in lines]
        if len(rows) != SIZE or not all(len(row) == SIZE for row in rows):
            raise ValueError(f"tile {id} is the wrong size")
        if not all(c in '#.' for row in rows for c in row):
            raise ValueError(f"unexpected character in tile {id}")
        tiles[id] = Tile(id, rows)
    return tiles


def find_corner_tile_ids(tiles):
    n = int(math.sqrt(len(tiles)))
    if n * n != len(tiles):
        raise ValueError(f"can't make a square of {len(tiles)} tiles (it's not a square number)")

    tiles_by_edge_code = defaultdict(list)
    for tile in tiles.values():
        for oriented_tile in tile.orientations():
            tiles_by_edge_code[oriented_tile.edge_code].append(oriented_tile)

    for code, tile_list in tiles_by_edge_code.items():
        if len(tile_list) not in (1, 2):
            raise ValueError("can't solve this one")

    result = [
        tile.id
        for tile in tiles.values()
        # "if the number of oriented-edges of this tile that don't line up with anything is 4"
        if len([
                1
                for otile in tile.orientations()
                if len(tiles_by_edge_code[otile.edge_code]) == 1
        ]) == 4
    ]

    print(result)
    if len(result) != 4:
        raise ValueError("puzzle has more than 4 corners or something")
    return result


def solve(text):
    tiles = parse_input(text)
    ids = find_corner_tile_ids(tiles)

    product = 1
    for id in ids:
        product *= id
    return product


example = """\
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...
"""

assert solve(example) == 20899048083289


if __name__ == '__main__':
    print(solve(puzzle_input()))
