insns = {
    'set': 'r*',
    'mul': 'r*',
    'jgz': '**',
    'add': 'r*',
    'mod': 'r*',
    'snd': '*',
    'rcv': 'r',
}

regs = set('abcdefghijklmnopqrstuvwxyz')

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
            if operand in regs:
                assert expected_type in ('*', 'r')
            elif expected_type == 'r':
                raise ValueError("expected register for operand {} of {}".format(i, insn[0]))
            else:
                assert expected_type == '*'
                insn[i] = int(insn[i])
        program.append(tuple(insn))
    return program

def run_program(code):
    reg = {r: 0 for r in regs}
    def evaluate(operand):
        if isinstance(operand, str):
            return reg[operand]
        else:
            assert isinstance(operand, int)
            return operand

    last_freq = None
    pc = 0
    while True:
        insn = code[pc]
        pc += 1
        i = insn[0]
        if i == 'set':
            reg[insn[1]] = evaluate(insn[2])
        elif i == 'add':
            reg[insn[1]] += evaluate(insn[2])
        elif i == 'mul':
            reg[insn[1]] *= evaluate(insn[2])
        elif i == 'mod':
            reg[insn[1]] %= evaluate(insn[2])
        elif i == 'snd':
            last_freq = evaluate(insn[1])
        elif i == 'rcv':
            return last_freq
        elif i == 'jgz':
            if evaluate(insn[1]) > 0:
                pc -= 1 # undo the normal increment which we already did
                pc += evaluate(insn[2])

sample_program = '''\
set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2
'''

assert run_program(parse_program(sample_program)) == 4

with open('puzzle-input.txt') as f:
    program = parse_program(f.read())
print(run_program(program))
