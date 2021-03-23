import re, collections

Entry = collections.namedtuple('Entry', 'i1 i2 letter password')

def parse_line(lineno, line):
    m = re.match(r'^(0|[1-9]\d*)-(0|[1-9]\d*) ([a-z]): ([a-z]+)\n?$', line)
    if m is None:
        raise ValueError(f"can't read line {lineno}: {line}")
    i1, i2 = int(m.group(1)), int(m.group(2))
    return Entry(i1, i2, m.group(3), m.group(4))

def is_valid(e):
    return (
        1 <= e.i1 <= len(e.password)
        and 1 <= e.i2 <= len(e.password)
        and (e.password[e.i1 - 1] + e.password[e.i2 - 1]).count(e.letter) == 1
    )

assert is_valid(parse_line(1, '1-3 a: abcde'))
assert not is_valid(parse_line(2, '1-3 b: cdefg'))
assert not is_valid(parse_line(3, '2-9 c: ccccccccc'))

with open("input.txt") as f:
    lines = f.readlines()

entries = [parse_line(i, line) for i, line in enumerate(lines)]

print(sum(1 for e in entries if is_valid(e)))
