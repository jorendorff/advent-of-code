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


def find_fields(text):
    fields, your_ticket, nearby_tickets = parse_input(text)
    n = len(fields)
    if len(your_ticket) != n:
        raise ValueError(f"expected {n} fields, your ticket has {len(your_ticket)}")
    for ticket in nearby_tickets:
        if len(ticket) != n:
            raise ValueError(f"expected {n} fields, a nearby ticket has {len(ticket)}")

    valid = IntervalSet((lo, hi + 1) for name, ranges in fields for lo, hi in ranges)
    valid_tickets = [ticket for ticket in nearby_tickets if all(v in valid for v in ticket)]

    fields = [
        (name, IntervalSet((lo, hi + 1) for lo, hi in ranges))
        for name, ranges in fields
    ]
    values_by_column = [[ticket[i] for ticket in valid_tickets] for i in range(n)]

    # At a cost of O(n^2 * m), build a table of which fields could qualify for
    # which columns.
    possible_fields_by_column = [
        [
            j
            for j in range(n)
            if all(v in fields[j][1] for v in values_by_column[col])
        ]
        for col in range(n)
    ]

    # At this point it's a matter of finding a permutation S of range(n)
    # such that S[col] in possible_fields_by_column[col].
    #
    # This could be phrased as an exact cover problem, and then Knuth's
    # Algorithm X ("Dancing Links") could be brought to bear.

    # Glorious brute force from here, O(2^n), ameliorated using `column_order`,
    # a hack to try to slot in sure bets first. (It turns out to be a huge deal
    # in practice.)
    solution = [None] * n
    column_order = sorted(range(n), key=lambda col: len(possible_fields_by_column[col]))
    used = [False] * n
    def try_solve(progress):
        if progress == n:
            yield solution[:]
        else:
            col = column_order[progress]
            for j in possible_fields_by_column[col]:
                if not used[j]:
                    solution[col] = j
                    used[j] = True
                    for answer in try_solve(progress + 1):
                        yield answer
                    print("backtracking...")
                    used[j] = False
                    solution[col] = None

    for answer in try_solve(0):
        return [(fields[answer[col]][0], value) for col, value in enumerate(your_ticket)]
    else:
        raise ValueError("no solutions")


def solve(text):
    count = 0
    product = 1
    for name, value in find_fields(text):
        if name.split()[0] == "departure":
            count += 1
            product *= value
    if count != 6:
        raise ValueError(f"expected 6 fields to start with the word 'departure', got {count}")
    return product


example = """\
class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9
"""

assert find_fields(example) == [('row', 11), ('class', 12), ('seat', 13)]


if __name__ == '__main__':
    print(solve(puzzle_input()))

