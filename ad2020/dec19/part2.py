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


def parse_input(text):
    [rules, messages] = text.split("\n\n")
    rules += "\n"
    rules = dict(parse_rules(rules))
    rules[8] = [[42], [42, 8]]
    rules[11] = [[42, 31], [42, 11, 31]]
    return rules, [line.rstrip() for line in messages.splitlines()]


def rule_matches(rules, rule_id, message):
    rule = rules[rule_id]
    if isinstance(rule, str):
        if message.startswith(rule):
            yield message[len(rule):]
    else:
        for plist in rule:
            yield from plist_matches(rules, plist, message)


def plist_matches(rules, plist, message):
    if plist == []:
        yield message
    else:
        first, rest = plist[0], plist[1:]
        for message_rest in rule_matches(rules, first, message):
            yield from plist_matches(rules, rest, message_rest)


def is_match(rules, message):
    return any(m == '' for m in rule_matches(rules, 0, message))


def solve(text):
    rules, cases = parse_input(text)
    return sum(1 for case in cases if is_match(rules, case))


example = """\
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
"""

assert solve(example) == 12


if __name__ == '__main__':
    print(solve(puzzle_input()))
