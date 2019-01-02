import re
from collections import defaultdict


sample_input = '''\
initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #
'''

def parse_input(text):
    """Break up the text and store the patterns in a table."""
    initial_line, pattern_lines = text.strip().split("\n\n")
    m = re.match(r'^initial state: ([#.]*)$', initial_line)
    if m is None:
        raise ValueError("unexpected input")
    initial_state = m.group(1)

    patterns = defaultdict(lambda: '.')
    for line in pattern_lines.splitlines():
        m = re.match(r'^([#.]{5}) => ([#.])$', line)
        left = m.group(1)
        right = m.group(2)
        patterns[left] = right

    return (0, initial_state), patterns

def step(state, patterns):
    """Advance 1 generation."""
    start_index, s = state    #   0, '#..#.#..##......###...###'
    s = '....' + s + '....'   #  '....#..#.#..##......###...###....'
    new_row = ''
    for i in range(0, len(s) - 4):
        slice = s[i:i+5]
        result = patterns[slice]
        new_row += result

    start_index -= 2
    while new_row.startswith('.'):
        new_row = new_row[1:]
        start_index += 1
    new_row = new_row.rstrip('.')
    return start_index, new_row

def advance(state, patterns, n):
    """Advance n generations."""
    for i in range(n):
        state = step(state, patterns)
    return state

def plant_score(state):
    start_index, s = state
    total = 0
    for i, c in enumerate(s):
        if c == '#':
            total += i + start_index
    return total

sample_state, sample_patterns = parse_input(sample_input)
assert step(sample_state, sample_patterns) == (0, '#...#....#.....#..#..#..#')
assert advance(sample_state, sample_patterns, 3) == (-1, '#.#...#..#.#....#..#..#...#')
assert advance(sample_state, sample_patterns, 11) == (0, '#...##...#.#...#.#...#...#...#')
assert advance(sample_state, sample_patterns, 20) == (-2, '#....##....#####...#######....#.#..##')

assert plant_score(advance(sample_state, sample_patterns, 20)) == 325

with open('puzzle-input.txt') as f:
    state, patterns = parse_input(f.read())
    print(plant_score(advance(state, patterns, 20)))




