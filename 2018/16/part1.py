import re

def replace(mylist, index, value):
    x = list(mylist)
    x[index] = value
    return x

opcodes = [
    ('addr', lambda regs, a, b: regs[a] + regs[b]),
    ('addi', lambda regs, a, b: regs[a] + b),
    ('mulr', lambda regs, a, b: regs[a] * regs[b]),
    ('muli', lambda regs, a, b: regs[a] * b),
    ('banr', lambda regs, a, b: regs[a] & regs[b]),
    ('bani', lambda regs, a, b: regs[a] & b),
    ('borr', lambda regs, a, b: regs[a] | regs[b]),
    ('bori', lambda regs, a, b: regs[a] | b),
    ('setr', lambda regs, a, b: regs[a]),
    ('seti', lambda regs, a, b: a),
    ('gtrr', lambda regs, a, b: regs[a] > regs[b]),
    ('gtir', lambda regs, a, b: a > regs[b]),
    ('gtri', lambda regs, a, b: regs[a] > b),
    ('eqrr', lambda regs, a, b: regs[a] == regs[b]),
    ('eqir', lambda regs, a, b: a == regs[b]),
    ('eqri', lambda regs, a, b: regs[a] == b),
]


def how_many_opcodes(s):
    m = re.match(r'''(?mx)
        ^\s*
           Before: \s* (\[   (?:\d,[ ]){3}\d   \])\n
           (\d+) \s+ (\d) \s+ (\d) \s+ (\d) \n
           After: \s* (\[   (?:\d,[ ]){3}\d   \])
          \s*$''', s)

    if m is None:
        print(s)
    
    before, op, a, b, c, after = map(eval, m.groups())

    total = 0
    for name, f in opcodes:
        out = replace(before, c, f(before, a, b))
        if out == after:
            total += 1

    return total
    
    



    
    
example = '''\
Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]
'''

assert how_many_opcodes(example) == 3





with open('puzzle-input.txt') as f:
    text = f.read()

samples, program = text.split("\n\n\n\n")

total = 0
for sample in samples.split("\n\n"):
    if how_many_opcodes(sample) >= 3:
        total += 1
print(total)
