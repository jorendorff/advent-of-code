import re

def parse(text):
    bots = []
    for line in text.strip().splitlines():
        m = re.match(r'^pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)$', line)
        x, y, z, r = map(int, m.groups())
        bots.append(((x, y, z), r))
    return bots

def distance(p, q):
    return sum(abs(a - b) for a, b in zip(p, q))

def count(bots):
    apos, ar = max(bots, key=lambda pair: pair[1])
    return sum(1
               for pos, r in bots
               if distance(apos, pos) <= ar)

sample_input = '''\
pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1
'''

assert count(parse(sample_input)) == 7

with open('puzzle-input.txt') as f:
    text = f.read()
bots = parse(text)
print(count(bots))
