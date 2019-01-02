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

opcode_table = dict(opcodes)

def parse_sample(s):
    m = re.match(r'''(?mx)
        ^\s*
           Before: \s* (\[   (?:\d,[ ]){3}\d   \])\n
           (\d+) \s+ (\d) \s+ (\d) \s+ (\d) \n
           After: \s* (\[   (?:\d,[ ]){3}\d   \])
          \s*$''', s)

    if m is None:
        print(s)
    
    return tuple(map(eval, m.groups()))


def which_opcodes(sample):
    before, op, a, b, c, after = sample
    for name, f in opcodes:
        out = replace(before, c, f(before, a, b))
        if out == after:
            yield name
    
   
    
example = '''\
Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]
'''

assert list(which_opcodes(parse_sample(example))) == ['addi', 'mulr', 'seti']






with open('puzzle-input.txt') as f:
    text = f.read()

samples, program = text.split("\n\n\n\n")

total = 0
all_opcode_names = set(opcode_table.keys())
possibles = [all_opcode_names.copy() for i in range(16)]
for sample in samples.split("\n\n"):
    sample = parse_sample(sample)
    op = sample[1]
    possibles[op] &= set(which_opcodes(sample))

while not all(len(s) == 1 for s in possibles):
    for i, working_set in enumerate(possibles):
        if len(working_set) == 1:
            [item] = working_set
            for j, sj in enumerate(possibles):
                if j != i and item in sj:
                    sj.remove(item)

actual_opcodes = [next(iter(s)) for s in possibles]
    

regs = [0, 0, 0, 0]
for line in program.strip().splitlines():
    op, a, b, c = map(int, line.split())
    name = actual_opcodes[op]
    f = opcode_table[name]
    regs = replace(regs, c, f(regs, a, b))

print(regs[0])
