from collections import defaultdict

def distance(p1, p2):
    x1, y1 = p1
    x2, y2 = p2
    return abs(x2 - x1) + abs(y2 - y1)

def nearest_neighbor(points, p):
    nearest = points[0]
    d = distance(p, nearest)
    for q in points[1:]:
        dq = distance(p, q)
        if dq < d:
            nearest = q
            d = dq
        elif dq == d:
            nearest = None
    return nearest

letters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789'

def area_around_least_dangerous_point(points):
    left = min(x for x, y in points)
    right = max(x for x, y in points)
    top = min(y for x, y in points)
    bottom = max(y for x, y in points)

    horiz = range(left, right + 1)
    vert = range(top, bottom + 1)
    
    border = ([(x, top) for x in horiz] +
              [(right, y) for y in vert] +
              [(x, bottom) for x in horiz] +
              [(left, y) for y in vert])

    disqualified = set()
    for p in border:
        q = nearest_neighbor(points, p)
        if q is not None:
            disqualified.add(q)

    areas = defaultdict(int)
    for y in vert:
        row = []
        for x in horiz:
            p = (x, y)
            q = nearest_neighbor(points, p)
            row.append('.' if q is None else '*' if p in points else letters[points.index(q)])
            if q is not None and q not in disqualified:
                areas[q] += 1
        #print(''.join(row))
    return max(areas.values())

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

assert area_around_least_dangerous_point(parse_input(sample_input)) == 17
for n in [1, 2, 3, 50]:
    assert area_around_least_dangerous_point([(0, 0), (2*n, 0), (0, 2*n), (2*n, 2*n), (n, n)]) == 2*(n-1)*n + 1


if __name__ == '__main__':
    with open('input.txt') as f:
        points = parse_input(f)
    print(area_around_least_dangerous_point(points))
