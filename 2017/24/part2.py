# the naive way: depth-first search

from collections import defaultdict

def find_best_road(edges, start=0, road_so_far=(), score_so_far=(0, 0)):
    best_road = road_so_far
    best_score = score_so_far
    for node, count in edges[start].items():
        edge = frozenset([start, node])
        if road_so_far.count(edge) < count:
            extended = road_so_far + (edge,)
            extended_score = (score_so_far[0] + 1, score_so_far[1] + start + node)

            road, score = find_best_road(edges, node, extended, extended_score)
            if score > best_score:
                best_road = road
                best_score = score
    return best_road, best_score

def parse_input(lines):
    q = defaultdict(lambda: defaultdict(int))
    for line in lines:
        src, dst = map(int, line.strip().split('/', 1))
        q[src][dst] += 1
        if src != dst:
            q[dst][src] += 1
    return q

sample_input = '''\
0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10
'''.splitlines()

assert find_best_road(parse_input(sample_input)) == ((frozenset([0, 2]),
                                                      frozenset([2, 2]),
                                                      frozenset([2, 3]),
                                                      frozenset([3, 5])), (4, 19))

with open('puzzle-input.txt') as f:
    path, (length, strength) = find_best_road(parse_input(f))
    print(strength)


