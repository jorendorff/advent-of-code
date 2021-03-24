def score(s):
    depth = 0
    score = 0
    i = 0
    while i < len(s):
        if s[i] == '!':
            i += 2
        elif s[i] == '<':
            while s[i] != '>':
                if s[i] == '!':
                    i += 2
                else:
                    i += 1
                if i >= len(s):
                    raise ValueError("unmatched <")
            i += 1
        elif s[i] == '}':
            if depth == 0:
                raise ValueError("unmatched }")
            depth -= 1
            i += 1
        elif s[i] == '{':
            depth += 1
            score += depth
            i += 1
        elif s[i] == ',':
            i += 1
        else:
            raise ValueError("unrecognized character %r at %d" % (s[i], i))
    return score

assert score('{}') == 1
assert score('{{{}}}') == 6
assert score('{{},{}}') == 5
assert score('{{{},{},{{}}}}') == 16
assert score('{<a>,<a>,<a>,<a>}') == 1
assert score('{{<ab>},{<ab>},{<ab>},{<ab>}}') == 9
assert score('{{<!!>},{<!!>},{<!!>},{<!!>}}') == 9
assert score('{{<a!>},{<a!>},{<a!>},{<ab>}}') == 3

with open('puzzle-input.txt') as f:
    print(score(f.read().strip()))
