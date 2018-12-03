import re
import collections

Claim = collections.namedtuple('Claim', 'id left top width height')

class ClaimMap:
    def __init__(self, claims):
        self.rows = [[]]
        self.contested_count = 0
        for c in claims:
            self.add_claim(c)

    def add(self, x, y):
        while len(self.rows[0]) <= x:
            for row in self.rows:
                row.append(0)
        while len(self.rows) <= y:
            self.rows.append([0] * len(self.rows[0]))
        if self.rows[y][x] == 0:
            self.rows[y][x] = 1
        elif self.rows[y][x] == 1:
            self.rows[y][x] = 2
            self.contested_count += 1

    def add_claim(self, claim):
        for y in range(claim.top, claim.top + claim.height):
            for x in range(claim.left, claim.left + claim.width):
                self.add(x, y)

    def is_claim_uncontested(self, claim):
        return all(self.rows[y][x] == 1
                   for y in range(claim.top, claim.top + claim.height)
                   for x in range(claim.left, claim.left + claim.width))

def contested_area(claims):
    return ClaimMap(claims).contested_count

def uncontested_claim(claims):
    claims = list(claims)
    map = ClaimMap(claims)
    ok_claims = [c for c in claims if map.is_claim_uncontested(c)]
    if len(ok_claims) != 1:
        raise ValueError("{} uncontested claims found".format(len(ok_claims)))
    return ok_claims[0].id

def parse(lines):
    for line in lines:
        # line format:
        #     #1 @ 53,238: 26x24
        match = re.match(r'^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)\n?$', line)
        if match is None:
            raise ValueError("could not parse line: " + line)
        yield Claim(*map(int, match.groups()))

testcase1 = [
    '#1 @ 1,3: 4x4',
    '#2 @ 3,1: 4x4',
    '#3 @ 5,5: 2x2',
]
assert contested_area(parse(testcase1)) == 4  # part 1
assert uncontested_claim(parse(testcase1)) == 3  # part 2

if __name__ == '__main__':
    with open("input.txt") as f:
        claims = list(parse(f))
    print(contested_area(claims))  # part 1
    print(uncontested_claim(claims))  # part 2
