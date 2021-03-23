import re, collections

Entry = collections.namedtuple('Entry', 'min max letter password')

def parse_line(lineno, line):
    m = re.match(r'^(0|[1-9]\d*)-(0|[1-9]\d*) ([a-z]): ([a-z]+)\n?$', line)
    if m is None:
        raise ValueError(f"can't read line {lineno}: {line}")
    min, max = int(m.group(1)), int(m.group(2))
    if min > max:
        raise ValueError(f"min > max on line {lineno}: {line}")
    return Entry(min, max, m.group(3), m.group(4))

def is_valid(e):
    return e.min <= e.password.count(e.letter) <= e.max

assert is_valid(parse_line(1, '1-3 a: abcde'))
assert not is_valid(parse_line(2, '1-3 b: cdefg'))
assert is_valid(parse_line(3, '2-9 c: ccccccccc'))

with open("input.txt") as f:
    lines = f.readlines()

entries = [parse_line(i, line) for i, line in enumerate(lines)]

print(sum(1 for e in entries if is_valid(e)))
