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
        out[m.group(1)] = parse_list(m.group(2))
    return out

def parse_list(bags):
    if bags == 'no other bags':
        return []
    out = {}
    for phrase in bags.split(", "):
        m = PHRASE_RE.match(phrase)
        quantity = int(m.group(1))
        color = m.group(2)
        if color in out:
            raise ValueError(f"rule talks about {color} bags more than once: {bags}")
        out[color] = quantity
    return out

def containers(rules_text, target='shiny gold'):
    rules = parse_rules(rules_text)
    contained_by = defaultdict(set)
    for outer_color in rules:
        for inner_color in rules[outer_color]:
            contained_by[inner_color].add(outer_color)

    todo = [target]
    seen = set(todo)
    while todo:
        color = todo.pop()
        for wrapper in contained_by[color]:
            # this code doesn't detect all cycles, but we can at least rule out
            # the target bag containing itself
            assert wrapper != target, ValueError("rules have target bag containing itself")
            if wrapper not in seen:
                seen.add(wrapper)
                todo.append(wrapper)
    return seen - {target}

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

assert containers(example) == {
    'bright white',
    'muted yellow',
    'dark orange',
    'light red',
}

assert len(containers(example)) == 4

with open("input.txt") as f:
    puzzle_input = f.read()

print(len(containers(puzzle_input)))
