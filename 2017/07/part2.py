import re
from collections import defaultdict

class Program:
    def __init__(self, name, wt, kids):
        self.name = name
        self.own_wt = wt
        self.kids = kids
        self.parent = None
        self.total_wt = wt
        self.balanced = None

    def compute_total_wt(self):
        self.total_wt = self.own_wt + sum(k.compute_total_wt() for k in self.kids)
        return self.total_wt


def corrected_weight(lines):
    program_directory = {}
    all_above = set()
    for line in lines:
        match = re.match(r'^(\w+) \((\d+)\)(?: -> (\w+(?:, \w+)*))?$', line)
        if match is None:
            raise ValueError("bad line: " + line)
        name = match.group(1)
        wt = int(match.group(2))
        above = match.group(3)
        if above is None:
            above = []
        else:
            above = above.split(", ")
        program_directory[name] = Program(name, wt, above)

    programs = list(program_directory.values())
    for p in programs:
        p.kids = [program_directory[kid] for kid in p.kids]

    for p in programs:
        for kid in p.kids:
            kid.parent = p

    for p in programs:
        p.compute_total_wt()

    for p in programs:
        assert p.total_wt == p.own_wt + sum(k.total_wt for k in p.kids)

    for p in programs:
        p.balanced = (len(set(k.total_wt for k in p.kids)) < 2)

    # now, find a program that is not balanced, but whose kids are all balanced.
    # one of those balanced kids is the problem child.
    solution = None
    for p in programs:
        if not p.balanced and all(k.balanced for k in p.kids):
            by_total_wt = defaultdict(list)
            for k in p.kids:
                by_total_wt[k.total_wt].append(k)
            if solution is not None:
                raise ValueError("more than one problem program")
            if len(by_total_wt) != 2:
                raise ValueError("more than one problem program on top of " + p.name)
            [k1, k2] = by_total_wt.keys()
            if len(by_total_wt[k1]) != 1:
                k1, k2 = k2, k1
            if len(by_total_wt[k1]) != 1:
                raise ValueError("ambiguous input or multiple problem programs")
            [node] = by_total_wt[k1]
            solution = node.own_wt + (k2 - k1)
    return solution

test_input = '''\
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)
'''

assert corrected_weight(test_input.splitlines()) == 60

if __name__ == '__main__':
    print(corrected_weight(open("puzzle-input.txt")))
