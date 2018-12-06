import itertools

def safe_region(points, limit):
    N = len(points)
    DIMS = range(len(points[0]))
    distance_lists_by_dim = []
    for d in DIMS:
        proj = [p[d] for p in points]
        start = min(proj) - (limit + N - 1) / N
        assert abs(start - min(proj)) * N >= limit
        stop = max(proj) + limit / N + 1 + 1
        assert ((stop - 1) - max(proj)) * N >= limit
        dx = []
        for x in range(start, stop):
            dx.append(sum(abs(x - px) for px in proj))
        distance_lists_by_dim.append(dx)
    count = 0
    for distances in itertools.product(*distance_lists_by_dim):
        if sum(distances) < limit:
            count += 1
    return count

def parse_input(lines):
    points = [tuple(int(field.strip()) for field in line.split(","))
            for line in lines]
    assert all(len(p) == 2 for p in points)
    assert len(set(points)) == len(points)
    return points

sample_input = '''\
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9
'''.splitlines()

sample_points = parse_input(sample_input)

assert safe_region(sample_points, 32) == 16

if __name__ == '__main__':
    with open('input.txt') as f:
        points = parse_input(f)
    print(safe_region(points, 10000))
