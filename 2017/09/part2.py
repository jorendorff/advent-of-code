def garbage_count(s):
    depth = 0
    count = 0
    i = 0
    while i < len(s):
        if s[i] == '!':
            i += 2
        elif s[i] == '<':
            i += 1
            while i < len(s) and s[i] != '>':
                if s[i] == '!':
                    i += 2
                else:
                    count += 1
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
            i += 1
        elif s[i] == ',':
            i += 1
        else:
            raise ValueError("unrecognized character %r at %d" % (s[i], i))
    return count

assert garbage_count('<>') == 0
assert garbage_count('<random characters>') == 17
assert garbage_count('<<<<>') == 3
assert garbage_count('<{!>}>') == 2
assert garbage_count('<!!>') == 0
assert garbage_count('<!!!>>') == 0
assert garbage_count('<{o"i!a,<{i<a>') == 10


with open('puzzle-input.txt') as f:
    print(garbage_count(f.read().strip()))
