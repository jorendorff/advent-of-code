from .advent import Cycle

DIRS = [
    ( 0,  1),
    ( 1,  1),
    ( 1,  0),
    ( 1, -1),
    ( 0, -1),
    (-1, -1),
    (-1,  0),
    (-1,  1),
]

class Grid:
    """ A grid of values, typically ints or characters, as in the Game of Life. """

    def __init__(self, rows):
        if isinstance(rows, str):
            if rows.endswith('\n'):
                rows = rows[:-1]
            rows = rows.split('\n')
        self.rows = [list(row) for row in rows]
        if self.rows:
            width = len(self.rows[0])
            assert all(len(row) == width for row in self.rows)

    def __getitem__(self, point):
        x, y = point
        return self.rows[y][x]

    def count_adjacent(self, point, value):
        """Return the number of cells adjacent to `point` (x, y) that contain the given `value`.

        Includes diagonally adjacent cells. Result will be from 0 to 8, inclusive.
        """
        width = len(self.rows[0])
        height = len(self.rows)
        x, y = point
        count = 0
        for dx, dy in DIRS:
            nx = x + dx
            ny = y + dy
            if 0 <= nx < width and 0 <= ny < height and self[nx, ny] == value:
                count += 1
        return count

    def count_visible(self, point, value, is_transparent=lambda c: c == '.'):
        """Return the number of cells "visible" from `point` that contain `value`.

        A cell `p2` is visible from `point` if `point` and `p2` lie on the same
        line in one of the eight DIRS and all values in cells between `point`
        and p2` on that line are transparent according to the predicate
        `is_transparent`.

        `point` is not visible from itself.

        Result will be from 0 to 8, inclusive.
        """
        width = len(self.rows[0])
        height = len(self.rows)
        x, y = point
        count = 0
        for dx, dy in DIRS:
            nx = x + dx
            ny = y + dy
            while 0 <= nx < width and 0 <= ny < height:
                nv = self[nx, ny]
                if nv == value:
                    count += 1
                    break
                if not is_transparent(nv):
                    break
                nx += dx
                ny += dy
        return count

    def step(self, transition):

        """ Iterate using a transition function.

        This doesn't change the size of the grid.
        """
        self.rows = [[transition((x, y), c) for x, c in enumerate(row)]
                     for y, row in enumerate(self.rows)]

    def to_str(self):
        """ Current state as a string. """
        return ''.join(''.join(row) + '\n' for row in self.rows)

    def cycle(self, transition):
        """ Return a Cycle of the results of repeatedly calling self.step(transition). """
        def states():
            while True:
                yield self.to_str()
                self.step(transition)

        return Cycle(states())

        
