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

def distance(p, q):
    return sum(abs(a - b) for a, b in zip(p, q))
            
def constellation_count(text):
    points = []
    for line in text.strip().splitlines():
        points.append(tuple(map(int, line.split(','))))
    c = EqRelation()
    for i, p in enumerate(points):
        c.add_element(p)
        for q in points[:i]:
            if distance(p, q) <= 3:
                c.add_edge(p, q)
    return c.count

def test():
    sample1 = '''\
     0,0,0,0
     3,0,0,0
     0,3,0,0
     0,0,3,0
     0,0,0,3
     0,0,0,6
     9,0,0,0
    12,0,0,0
    '''
    assert constellation_count(sample1) == 2

    sample2 = '''\
    -1,2,2,0
    0,0,2,-2
    0,0,0,-2
    -1,2,0,0
    -2,-2,-2,2
    3,0,2,-1
    -1,3,2,2
    -1,0,-1,0
    0,2,1,-2
    3,0,0,0
    '''
    assert constellation_count(sample2) == 4

    sample3 = '''\
    1,-1,0,1
    2,0,-1,0
    3,2,-1,0
    0,0,3,1
    0,0,-1,-1
    2,3,-2,0
    -2,2,0,0
    2,-2,0,-1
    1,-1,0,-1
    3,2,0,2
    '''
    assert constellation_count(sample3) == 3

    sample4 = '''\
    1,-1,-1,-2
    -2,-2,0,1
    0,2,1,3
    -2,3,-2,1
    0,2,3,-2
    -1,-1,1,-2
    0,-2,-1,0
    -2,2,3,-1
    1,2,2,0
    -1,-2,0,-2
    '''
    assert constellation_count(sample4) == 8

test()


with open("puzzle-input.txt") as f:
    print(constellation_count(f.read()))
