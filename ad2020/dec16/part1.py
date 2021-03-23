from lib.advent import *


def parse_range(r):
    [lo, hi] = r.split("-")
    lo = int(lo)
    hi = int(hi)
    assert lo < hi
    return lo, hi


def parse_ticket(line):
    return [int(f.strip()) for f in line.split(",")]

def parse_input(text):
    [fields_text, yours_text, nearby_text] = text.split("\n\n")
    fields = []
    for line in fields_text.splitlines():
        name, ranges = line.split(": ")
        fields.append((name, [parse_range(r) for r in ranges.split(" or ")]))

    lines = yours_text.splitlines()
    if lines.pop(0) != 'your ticket:':
        raise ValueError("expected 'your ticket:'")
    if len(lines) != 1:
        raise ValueError("'your ticket' should be a single line")
    your_ticket = parse_ticket(lines[0])

    lines = nearby_text.splitlines()
    if lines.pop(0) != 'nearby tickets:':
        raise ValueError("expected 'nearby tickets:'")
    nearby_tickets = [parse_ticket(line) for line in lines]

    return fields, your_ticket, nearby_tickets


def solve(text):
    fields, _your_ticket, nearby_tickets = parse_input(text)
    valid = IntervalSet((lo, hi + 1) for name, ranges in fields for lo, hi in ranges)
    print(valid.spans)
    for ticket in nearby_tickets:
        for v in ticket:
            if v not in valid:
                print(v)
    print("----")
    return sum(v for ticket in nearby_tickets for v in ticket if v not in valid)


example = """\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
"""

assert solve(example) == 71


if __name__ == '__main__':
    print(solve(puzzle_input()))

