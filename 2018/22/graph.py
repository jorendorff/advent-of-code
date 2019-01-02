import heapq

class Path:
    def __init__(self, start, edges=()):
        self.edges = list(edges)
        self.vertices = [start]
        for e in self.edges:
            self.vertices.append(e.destination)

    @classmethod
    def unravel(cls, start, rlist):
        redges = []
        while rlist is not None:
            rlist, edge = rlist
            redges.append(edge)
        return cls(start, redges[::-1])

    @property
    def cost(self):
        return sum(getattr(e, 'cost', 1) for e in self.edges)

    @property
    def origin(self):
        return self.vertices[0]

    @property
    def destination(self):
        return self.vertices[-1]

class _WorkItem:
    def __init__(self, cost_so_far, vertex, path_so_far):
        self.cost_so_far = cost_so_far
        self.vertex = vertex
        self.path_so_far = path_so_far

    def __lt__(self, other):
        return self.cost_so_far < other.cost_so_far


def cheapest_path(start, stop, exits):
    """Return a path from start to stop arbitrarily selected from those with
    minimum cost.

    exits(node) returns an iterable of edges, where each edge has a
    .destination attribute and an optional .cost attribute (defaults to 1).
    """

    todo = [_WorkItem(0, start, None)]
    heapq.heapify(todo)
    visited = set()
    while todo:
        item = heapq.heappop(todo)
        here = item.vertex
        if here == stop:
            return Path.unravel(start, item.path_so_far)
        if here in visited:
            continue
        visited.add(here)
        for edge in exits(here):
            there = edge.destination
            if there in visited:
                continue
            dt = getattr(edge, 'cost', 1)
            heapq.heappush(todo, _WorkItem(item.cost_so_far + dt, there, (item.path_so_far, edge)))
    return None
