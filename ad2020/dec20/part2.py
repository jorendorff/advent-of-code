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
        width = len(rows[0])
        height = len(rows)
        return Tile(self.id, [[rows[width - x - 1][y] for x in range(width)] for y in range(height)])

    def turn_anti(self):
        """Return a new row-list, rotating `rows` by 90° anticlockwise."""
        return self.turn().turn().turn()  # herpsum derpem

    def flip(self):
        """Flip the given list-of-lists along the main diagonal."""
        rows = self.rows
        width = len(rows[0])
        height = len(rows)
        # cursed line of code:
        return Tile(self.id, [[rows[x][y] for x in range(height)] for y in range(width)])

    def orientations(self):
        for start in [self, self.flip()]:
            tile = start
            for _ in range(4):
                yield tile
                tile = tile.turn()

    def dump(self):
        print(f"Tile {self.id}:")
        for i, row in enumerate(self.rows):
            print(f"{i:3}: {''.join(row)}")
        print()

    def bits(self):
        return [
            int(''.join(row).replace('.', '0').replace('#', '1'), 2)
            for row in self.rows
        ]

    def aligns_vert(self, other):
        """True if `other` can be placed directly below `self`."""
        return (len(self.rows[0]) == len(other.rows[0])
                and self.rows[-1] == other.rows[0])

    def aligns_horiz(self, other):
        """True if `other` can be placed directly to the right of `self`."""
        return (len(self.rows) == len(other.rows)
                and all(self.rows[i][-1] == other.rows[i][0]
                        for i in range(len(self.rows))))

    @classmethod
    def stitch(cls, tile_matrix):
        return cls(0, [
            [
                tile.rows[i][j]
                for tile in tile_row
                for j in range(1, SIZE - 1)
            ]
            for tile_row in tile_matrix
            for i in range(1, SIZE - 1)
        ])

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


def find_corner_tile_ids(tiles, tiles_by_edge_code):
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

    if len(result) != 4:
        raise ValueError("puzzle has more than 4 corners or something")
    return result


def arrange_tiles(tiles):
    n = int(math.sqrt(len(tiles)))
    if n * n != len(tiles):
        raise ValueError(f"can't make a square of {len(tiles)} tiles (it's not a square number)")
    if n < 2:
        raise NotImplementedError("can't solve trivial puzzle")

    tiles_by_edge_code = defaultdict(list)
    for tile in tiles.values():
        for oriented_tile in tile.orientations():
            tiles_by_edge_code[oriented_tile.edge_code].append(oriented_tile)

    for code, tile_list in tiles_by_edge_code.items():
        if len(tile_list) not in (1, 2):
            raise ValueError("can't solve this one")

    def is_edge_up(tile):
        """True if the top edge of `tile` doesn't match with anything."""
        return len(tiles_by_edge_code[tile.edge_code]) == 1

    def find_mate(tile):
        """Return the other tile with the same edge code as `tile`."""
        candidates = [c for c in tiles_by_edge_code[tile.edge_code] if c.id != tile.id]
        if len(candidates) == 0:
            raise ValueError("match not found")
        if len(candidates) > 1:
            raise ValueError("multiple tiles could fit")
        return candidates[0]

    grid = [[None] * n for _ in range(n)]
    for r in range(n):
        for c in range(n):
            if r == 0 and c == 0:
                # Place the initial corner tile.
                any_corner_id = find_corner_tile_ids(tiles, tiles_by_edge_code)[0]
                tile = tiles[any_corner_id]

                # Now make sure it's oriented right. No need to flip as the
                # program can solve a jigsaw puzzle just as well face-up or
                # face-down...
                if is_edge_up(tile) and is_edge_up(tile.turn().turn()):
                    raise ValueError("have a tile that won't fit anywhere")
                while not (is_edge_up(tile) and is_edge_up(tile.turn())):
                    tile = tile.turn()
            elif c == 0:
                # Line up vertically
                tile = find_mate(grid[r - 1][c].turn().turn())
                tile = tile.flip().turn()
            else:
                # Line up horizontally
                tile = find_mate(grid[r][c - 1].turn_anti())
                tile = tile.flip()

            if r > 0:
                assert grid[r - 1][c].aligns_vert(tile)
            if c > 0:
                assert grid[r][c - 1].aligns_horiz(tile)
            grid[r][c] = tile
    return grid


SEA_MONSTER = """\
..................#.
#....##....##....###
.#..#..#..#..#..#...
"""


def roughness(map_tile):
    monster_tile = Tile('SEA_MONSTER', SEA_MONSTER.split())
    monster_bits = monster_tile.bits()
    monster_height = len(monster_bits)
    monster_width = len(monster_tile.rows[0])
    
    for orientation in map_tile.orientations():
        map_height = len(orientation.rows)
        map_width = len(orientation.rows[0])
        map_bits = orientation.bits()
        match_bits = [0 for _row in map_bits]
        match_count = 0
        for r in range(0, map_height - monster_height):
            for c in range(0, map_width - monster_width):
                if all(
                        (map_bits[r + y] & (monster_bits[y] << c)) == (monster_bits[y] << c)
                        for y in range(monster_height)
                ):
                    match_count += 1
                    for y in range(monster_height):
                        match_bits[r + y] |= monster_bits[y] << c
                        # check that no bits set in match are unset in the map
                        assert (match_bits[r + y] & ~map_bits[r + y]) == 0
        if match_count:
            break
    else:
        raise ValueError("found no monsters in map, in any orientation")
    
    # dump the grid for fun
    print("searched this map for sea monsters")
    orientation.dump()
    print(f"{match_count} monster(s) found")
    finished_map = Tile(0, [
        [
            'O' if (match_bits[y] >> x) & 1
            else '#' if (map_bits[y] >> x) & 1
            else '.'
            for x in range(0, map_width)
        ]
        for y in range(0, map_height)
    ])
    finished_map.dump()

    return bits_set(map_bits) - bits_set(match_bits)


def count_ones(i):
    """Population count - number of bits set in integer i."""
    count = 0
    while i > 0:
        count += i & 1
        i >>= 1
    return count


def bits_set(bits):
    """Total number of bits set in a bitmap."""
    return sum(count_ones(row) for row in bits)


def solve(text):
    tiles = parse_input(text)
    tile_matrix = arrange_tiles(tiles)
    stitched_map = Tile.stitch(tile_matrix)
    return roughness(stitched_map)


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

assert solve(example) == 273


if __name__ == '__main__':
    print(solve(puzzle_input()))
