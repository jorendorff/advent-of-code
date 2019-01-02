from collections import defaultdict, deque

def parse_re(regexp):
    assert regexp.startswith('^')
    assert regexp.endswith('$')

    def parse_alt(regexp, i):
        opts = []
        while i < len(regexp) and regexp[i] not in ')$':
            i, s = parse_seq(regexp, i)
            opts.append(s)
            if regexp[i:i+1] != '|':
                break
            i += 1
        return i, ('alt', opts)

    def parse_seq(regexp, i):
        seq = []
        while i < len(regexp) and regexp[i] not in '|)$':
            i, atom = parse_atom(regexp, i)
            seq.append(atom)
        return i, seq

    def parse_atom(regexp, i):
        c = regexp[i]
        if c in 'NEWS':
            return i + 1, c
        elif c == '(':
            i += 1
            i, result = parse_alt(regexp, i)
            if regexp[i] != ')':
                raise ValueError("expected ) at offset {}".format(i))
            i += 1
            return i, result
        else:
            raise ValueError("unexpected {} at offset {}".format(repr(c), i))

    i, result = parse_alt(regexp, 1)
    if i >= len(regexp):
        raise ValueError("internal error")
    if i != len(regexp) - 1:
        raise ValueError("unexpected {} at offset {}".format(repr(regexp[i]), i))
    return result

DIRS = {
    'N': (0, 1),
    'S': (0, -1),
    'W': (-1, 0),
    'E': (1, 0)
}

def update(doormap1, doormap2):
    for room, doors in doormap2.items():
        doormap1[room] |= doors

def all_doors(re_ast):
    doors = defaultdict(set)
    def add_door(x, y, dx, dy):
        p = (x, y)
        q = (x + dx, y + dy)
        doors[p].add(q)
        doors[q].add(p)
        return q

    def eval_re(re_ast, starts):
        if isinstance(re_ast, list):
            for node in re_ast:
                starts = eval_re(node, starts)
            return starts
        elif isinstance(re_ast, str):
            for c in re_ast:
                dx, dy = DIRS[c]
                starts = [add_door(x, y, dx, dy) for x, y in starts]
            return set(starts)
        elif isinstance(re_ast, tuple):
            kind, nodes = re_ast
            if kind != 'alt':
                raise ValueError("unexpected regexp: " + repr(re_ast))
            finishes = set()
            for node in nodes:
                finishes |= set(eval_re(node, starts))
            return finishes
        else:
            raise ValueError("unexpected regexp: " + repr(re_ast))

    finishes = eval_re(re_ast, set([(0, 0)]))
    return doors, finishes

assert all_doors(parse_re('^WNE$')) == (
    defaultdict(set, {
        (0, 0): set([(-1, 0)]),
        (-1, 0): set([(0, 0), (-1, 1)]),
        (-1, 1): set([(-1, 0), (0, 1)]),
        (0, 1): set([(-1, 1)])
    }),
    set([(0, 1)])
)

def breadth_first_traverse(graph, start):
    seen = set(start)
    todo = deque([(start, 0)])
    while todo:
        node, distance = todo.popleft()
        yield node, distance
        for room in graph[node]:
            if room not in seen:
                seen.add(room)
                todo.append((room, distance + 1))

def distant_rooms(re_ast):
    graph, ignored_rooms = all_doors(re_ast)
    return sum(1 for room, distance in breadth_first_traverse(graph, (0, 0))
               if distance >= 1000)

with open('puzzle-input.txt') as f:
    regex = f.read().strip()
print(distant_rooms(parse_re(regex)))
