""" Library of useful bits and bobs for Advent of Code. """

from collections import deque, defaultdict, namedtuple, Counter
import itertools
import math
import sys


# Loading the puzzle input

def puzzle_input():
    """Load puzzle-input.txt as text."""
    import inspect
    import pathlib
    caller_script = inspect.currentframe().f_back.f_globals['__file__']
    filename = pathlib.Path(caller_script).parent / "puzzle-input.txt"
    with open(filename) as f:
        return f.read()


# Pure math

def round_up(n, d):
    return (n + d - 1) // d

assert round_up(33, 10) == 4

def lcm(*numbers):
    a = 1
    for n in numbers:
        a = a * n // math.gcd(a, n)
    return a


# Functions on iterables (what I think of as sequences)

def flatten(iterable):
    """Smoosh a list of lists down to a list."""
    return itertools.chain.from_iterable(iterable)

def flat_map(f, iterable):
    """Like JS flatMap or Haskell concatMap. Return flatten(map(f, iterable))."""
    return itertools.chain.from_iterable(itertools.imap(f, iterable))

def first_repeated_value(iterable):
    """Return the first value that iterable yields a second time. (The values must be hashable.)"""
    seen = set()
    for x in iterable:
        if x in seen:
            return x
        seen.add(x)



# About functions

def fn_exp(f, n):
    """Function exponentiation."""
    def g(x):
        for i in range(n):
            x = f(x)
        return x
    return g


def fn_iter(f, start):
    """Yield the sequence `start, f(start), f(f(start)), ...`"""
    x = start
    while True:
        yield x
        x = f(x)


def fix(f, start):
    """Find a fix point of `f` by iteration, starting at `start`.

    If `fn_iter(f, start) never reaches a fixed point, this never returns.
    """
    it = fn_iter(f, start)
    prev = next(it)
    for v in it:
        if v == prev:
            return v
        prev = v


class Cycle:
    """Information about a sequence (of hashable values) that eventually cycles."""
    def __init__(self, iterable):
        i = 0
        seen = {}
        seq = []
        for x in iterable:
            if x in seen:
                break
            seen[x] = i
            seq.append(x)
            i += 1

        c0 = seen[x]  # cycle start index
        self.prefix = seq[:c0]
        self.cycle = seq[c0:]

    def __getitem__(self, k):
        """Return the element at index `k`, i.e. fn_exp(f, k)(start)."""
        if not isinstance(k, int):
            raise ValueError("index must be an integer")
        if k < 0:
            raise ValueError("index must be nonnegative")
        if k < len(self.prefix):
            return self.prefix[k]
        else:
            return self.cycle[(k - len(self.prefix)) % len(self.cycle)]


# Euclidean geometry

class Vec2(tuple):
    def __new__(cls, x, y):
        return tuple.__new__(cls, (x, y))

    def __str__(self):
        return f"({self[0]}, {self[1]})"

    def __repr__(self):
        return f"{self.__class__.__name__}({self[0]}, {self[1]})"

    @property
    def x(self):
        return self[0]

    @property
    def y(self):
        return self[1]

    def __add__(self, other):
        return Vec2(self.x + other.x, self.y + other.y)

    def __sub__(self, other):
        return Vec2(self.x - other.x, self.y - other.y)

    def __neg__(self):
        return Vec2(-self.x, -self.y)

    def rotate_left(self):
        return Vec2(-self.y, self.x)

    def rotate_right(self):
        return Vec2(self.y, -self.x)



# Rectangles

def ranges_overlap(r1, r2):
    """True if the integer ranges r1 and r2 share any elements."""
    assert r1.step == 1 and r2.step == 1
    assert type(r1.start) == type(r1.stop) == type(r2.start) == type(r2.stop) == int
    return r1 and r2 and not (r1.stop <= r2.start) and not (r2.stop <= r1.start)

class Rectangle:
    def __init__(self, x0, y0, x1, y1):
        self.x0 = x0
        self.y0 = y0
        self.x1 = x1
        self.y1 = y1

    def rx(self):
        assert isinstance(self.x0, int)
        assert isinstance(self.x1, int)
        return range(self.x0, self.x1)

    def ry(self):
        assert isinstance(self.y0, int)
        assert isinstance(self.y1, int)
        return range(self.y0, self.y1)

    @property
    def width(self):
        return self.x1 - self.x0

    @property
    def height(self):
        return self.y1 - self.y0

    def overlaps(self, other):
        return ranges_overlap(self.rx(), other.rx()) and ranges_overlap(self.ry(), other.ry())


# Graphs

class Graph:
    """ Non-directed graph. """
    def __init__(self, edges=()):
        self.nodes = defaultdict(set)
        for a, b in edges:
            self.add(a, b)

    def add(self, a, b):
        self.nodes[a].add(b)
        self.nodes[b].add(a)


# Equivalence relations

class EqRelation:
    def __init__(self, edges=()):
        self.vertices = {}
        self.count = 0
        for a, b in edges:
            self.add_edge(a, b)

    def _query(self, key):
        if key not in self.vertices:
            self.vertices[key] = key
            self.count += 1
            return key
        k = key
        while True:
            j = self.vertices[k]
            if j == k:
                break
            k = j
        self.vertices[key] = k
        return k

    def add_element(self, e):
        self._query(e)
    
    def add_edge(self, a, b):
        a = self._query(a)
        b = self._query(b)
        if a != b:
            self.vertices[b] = a
            self.count -= 1
        assert self.count == len(set(self._query(k) for k in list(self.vertices)))


# Labeled digraphs

class LabeledDigraph:
    def __init__(self, triples=()):
        self.data = {}
        for v, label, w in triples:
            self.add_edge(v, label, w)

    def add_vertex(self, value):
        if value not in self.data:
            self.data[value] = {}

    def add_edge(self, v1, label, v2):
        self.add_vertex(v1)
        self.add_vertex(v2)
        self.data[v1][label] = v2

    def shortest_path(self, v1, v2):
        """Return a list of labels of edges forming any shortest path from v1 to v2
        or None if no such path exists."""
        # Breadth-first search; entries in visited are `w: (label, v)` where
        # `self.data[v][label] == w`.
        visited = {v1: None}
        todo = deque([v1])
        while todo:
            v = todo.popleft()
            for label, w in self.data[v].items():
                if w not in visited:
                    visited[w] = (label, v)
                    todo.append(w)
                    if w == v2:
                        break
            if v2 in visited:
                break
        else:
            return None

        path = []
        back_edge = visited[v2]
        while back_edge is not None:
            label, prev = back_edge
            path.append(label)
            back_edge = visited[prev]
        return path[::-1]

assert LabeledDigraph([(0, 'W', 1)]).shortest_path(0, 1) == ['W']
assert LabeledDigraph([(0, 'W', 1)]).shortest_path(0, 0) == []
assert LabeledDigraph([(0, 'W', 1)]).shortest_path(1, 0) is None
assert LabeledDigraph([(0, 'W', 1), (1, 'S', 2)]).shortest_path(0, 2) == ['W', 'S']
assert LabeledDigraph([(1, 'S', 2), (0, 'W', 1)]).shortest_path(0, 2) == ['W', 'S']


# Parsing

def tokenize(pattern, str, whitespace=r'\s'):
    import re
    token_re = re.compile(pattern)
    whitespace_re = re.compile(rf'(?:{whitespace})*')

    i = 0
    while i < len(str):
        i = whitespace_re.match(str, i).end()
        if i == len(str):
            break
        m = token_re.match(str, i)
        if m is None:
            raise ValueError(f"elves could not make sense of:\n    {str}\n    {'':{i}}^")
        yield m.group()
        i = m.end()

assert list(tokenize(r'\w+|\d+|[()[\]+]', '21 + roger[vector 3]')) == ['21', '+', 'roger', '[', 'vector', '3', ']']


# Algorithms

def bisect(predicate, lo=0, hi=None):
    """Return the least value i in range(lo, hi+1) for which predicate(i) is true,
    assuming predicate is monotonic.
    """
    if hi is None:
        hi = 4
        while not predicate(hi):
            hi *= 2
    else:
        assert predicate(hi)

    while lo < hi:
        mid = (lo + hi) // 2
        if predicate(mid):
            hi = mid
        else:
            lo = mid + 1
    return hi


class IntervalSet:
    def __init__(self, ranges):
        out = []
        for start, stop in sorted(ranges):
            if len(out) == 0 or start > out[-1][1]:
                out.append((start, stop))
            else:
                out[-1] = (out[-1][0], max(out[-1][1], stop))
        self.spans = out

    def __contains__(self, v):
        n = len(self.spans)
        i = bisect(lambda j: j == n or v < self.spans[j][1], 0, n)
        return i < n and self.spans[i][0] <= v

# Diagnostics

def print_grid(f, p0, p1, cmap=None):
    x0, y0 = p0
    x1, y1 = p1

    if not (x0 < x1 and y0 < y1):
        print("print_grid: empty range", p0, p1)
        return

    sys.stdout.write(" " * 8)
    for x in range(x0, x1):
        sys.stdout.write(str(x % 10))
    print()

    for y in range(y0, y1):
        sys.stdout.write("%7d " % y)
        for x in range(x0, x1):
            result = f(x, y)
            if cmap is None:
                if isinstance(result, bool):
                    c = '#' if result else '.'
                else:
                    c = str(result % 10)
            else:
                if 0 <= result < len(cmap):
                    c = cmap[result]
                else:
                    c = '?'
            sys.stdout.write(c)
        print()

