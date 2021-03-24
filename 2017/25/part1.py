import re
from collections import deque

def parse_action(lines):
    m = re.match(r'^  If the current value is ([01]):$', lines[0])
    current = int(m.group(1))
    m = re.match(r'^    - Write the value ([01])\.$', lines[1])
    write = int(m.group(1))
    m = re.match(r'^    - Move one slot to the (right|left)\.$', lines[2])
    dir = +1 if m.group(1) == 'right' else -1
    m = re.match(r'^    - Continue with state (\w+)\.$', lines[3])
    result_state = m.group(1)
    return (current, (write, dir, result_state))

def parse_state(text):
    lines = text.splitlines()
    m = re.match(r'^In state (\w+):$', lines.pop(0))
    name = m.group(1)
    assert len(lines) % 4 == 0
    return (name, dict(parse_action(lines[k:k + 4]) for k in range(0, len(lines), 4)))

def parse_blueprint(text):
    grafs = text.split("\n\n")

    first = grafs.pop(0)
    [begin_line, checksum_line] = first.splitlines()
    m = re.match(r'^Begin in state (\w+)\.$', begin_line)
    start_state = m.group(1)
    m = re.match(r'Perform a diagnostic checksum after (\d+) steps\.$', checksum_line)
    nsteps = int(m.group(1))

    rules = dict(parse_state(g) for g in grafs)

    return (rules, start_state, nsteps)

class Machine:
    def __init__(self, rules, start_state):
        self.rules = rules
        self.state = start_state
        self.tape_deque = deque([0])
        self.pos = 0

    def step(self):
        self.tape_deque[self.pos], d, self.state = self.rules[self.state][self.tape_deque[self.pos]]
        self.pos += d
        while self.pos < 0:
            self.tape_deque.appendleft(0)
            self.pos += 1
        while self.pos >= len(self.tape_deque):
            self.tape_deque.append(0)

    def snapshot(self):
        return ''.join(map(str, self.tape_deque)).strip('0')

    def checksum(self):
        return sum(self.tape_deque)

sample_blueprint = '''\
Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
'''

snaps = ['1', '11', '1', '101', '1101', '1101']

rules, start_state, nsteps = parse_blueprint(sample_blueprint)
assert nsteps == len(snaps)
m = Machine(rules, start_state)
for expected in snaps:
    m.step()
    assert m.snapshot() == expected
assert m.checksum() == 3


with open('puzzle-input.txt') as f:
    text = f.read()
rules, start_state, nsteps = parse_blueprint(text)
m = Machine(rules, start_state)
for i in range(nsteps):
    m.step()
print(m.checksum())
