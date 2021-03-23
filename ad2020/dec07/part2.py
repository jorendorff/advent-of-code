import re
from collections import defaultdict

LINE_RE = re.compile(r'^(\w+ \w+) bags contain (.*)\.$')
PHRASE_RE = re.compile(r'^(\d+) (\w+ \w+) bags?$')

def parse_rules(rules):
    out = {}
    for line in rules.splitlines():
        m = LINE_RE.match(line)
        if m is None:
            raise ValueError(f"can't parse line: {line}")
        color = m.group(1)
        if color in out:
            raise ValueError(f"more than one rule for {color} bags")
        out[m.group(1)] = parse_contents(m.group(2))
    return out

def parse_contents(bags):
    if bags == 'no other bags':
        return {}
    out = {}
    for phrase in bags.split(", "):
        m = PHRASE_RE.match(phrase)
        quantity = int(m.group(1))
        color = m.group(2)
        if color in out:
            raise ValueError(f"rule talks about {color} bags more than once: {bags}")
        out[color] = quantity
    return out

def count_bags_required_inside(rules, color):
    if isinstance(rules, str):
        rules = parse_rules(rules)
    return sum(
        quantity * (1 + count_bags_required_inside(rules, inner_color))
        for inner_color, quantity in rules[color].items()
    )

example = """\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"""

assert count_bags_required_inside(example, 'shiny gold') == 32

example2 = """\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
"""

assert count_bags_required_inside(example2, 'shiny gold') == 126

with open("input.txt") as f:
    puzzle_input = f.read()

print(count_bags_required_inside(puzzle_input, 'shiny gold'))
