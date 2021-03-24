import re

def bottom_program(lines):
    all_names = set()
    all_above = set()
    for line in lines:
        match = re.match(r'^(\w+) \(\d+\)(?: -> (\w+(?:, \w+)*))?$', line)
        if match is None:
            raise ValueError("bad line: " + line)
        name = match.group(1)
        all_names.add(name)
        above = match.group(2)
        if above is not None:
            all_above |= set(above.split(", "))
    unsupported = all_names - all_above
    if len(unsupported) != 1:
        raise ValueError("unsupported are: " + repr(unsupported))

    [x] = list(unsupported)
    return x

if __name__ == '__main__':
    print(bottom_program(open("puzzle-input.txt")))
