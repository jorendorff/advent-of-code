from collections import deque

insns = {
    'set': 'r*',
    'mul': 'r*',
    'jnz': '*i',
    'sub': 'r*',
}

REGS = set('abcdefgh')

def parse_program(s):
    program = []
    for line in s.splitlines():
        insn = line.split()
        if insn[0] not in insns:
            raise ValueError('unrecognized instruction: ' + insn[0])
        operands = insn[1:]
        expected = insns[insn[0]]
        if len(operands) != len(expected):
            raise ValueError("wrong number of operands for " + insn[0])
        for i, (operand, expected_type) in enumerate(zip(operands, expected), start=1):
            if operand in REGS:
                if expected_type not in ('*', 'r'):
                    raise ValueError("found register, expected integer")
            elif expected_type == 'r':
                raise ValueError("expected register for operand {} of {}".format(i, insn[0]))
            else:
                assert expected_type in ('i', '*')
                insn[i] = int(insn[i])
        program.append(tuple(insn))
    return program

def run_program(code):
    pc = 0
    regs = {k: 0 for k in REGS}

    def evaluate(operand):
        if operand in REGS:
            return regs[operand]
        else:
            return int(operand)

    multiplies = 0
        
    while 0 <= pc < len(code):
        insn = code[pc]
        i = insn[0]
        if i == 'set':
            regs[insn[1]] = evaluate(insn[2])
        elif i == 'sub':
            regs[insn[1]] -= evaluate(insn[2])
        elif i == 'mul':
            regs[insn[1]] *= evaluate(insn[2])
            multiplies += 1
        else:
            assert i == 'jnz'
            if evaluate(insn[1]) != 0:
                pc += evaluate(insn[2])
                continue  # skip pc increment
        pc += 1

    return multiplies

sample_program = '''\
set a 8
set b 1
mul b 2
sub a 1
jnz a -2
'''

assert run_program(parse_program(sample_program)) == 8

with open('puzzle-input.txt') as f:
    program = parse_program(f.read())
print(run_program(program))
