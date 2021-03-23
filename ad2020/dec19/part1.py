from lib.advent import *
import re

UINT_RE = r'(?:0|[1-9]\d*)'
LIST_RE = rf'{UINT_RE}(?:\s+{UINT_RE})*'
line_re = re.compile(rf'^({UINT_RE})\s*:\s*("(.)"|{LIST_RE}(?:\s*\|\s*{LIST_RE})*)\s*$')

def parse_rules(text):
    for line in text.splitlines():
        m = line_re.match(line)
        if m is None:
            raise ValueError("error on line: " + line.strip())
        id = int(m.group(1))
        pattern = m.group(2)
        single = m.group(3)
        if single is not None:
            yield id, single
        else:
            yield id, [
                [int(s) for s in plist.split()]
                for plist in pattern.split("|")
            ]


def rules_to_re(rules):
    cache = {}
    def get(n):
        if n not in cache:
            rule = rules[n]
            if isinstance(rule, str):
                cache[n] = re.escape(rule)
            else:
                alt = '|'.join(
                    ''.join(get(j) for j in plist)
                    for plist in rule
                )
                cache[n] = rf'(?:{alt})'
        return cache[n]

    return re.compile('^' + get(0) + '$')


def parse_input(text):
    [rules, messages] = text.split("\n\n")
    rules += "\n"
    return dict(parse_rules(rules)), [line.rstrip() for line in messages.splitlines()]


def solve(text):
    rules, cases = parse_input(text)
    zero_re = rules_to_re(rules)
    return sum(1 for case in cases if zero_re.match(case) is not None)


example = """\
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb
"""

assert solve(example) == 2


if __name__ == '__main__':
    print(solve(puzzle_input()))
