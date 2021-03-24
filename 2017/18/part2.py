from collections import deque

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

class Process:
    def __init__(self, id):
        self.regs = {r: 0 for r in regs}
        self.regs['p'] = id
        self.pc = 0
        self.mailbox = deque()
        self.send_count = 0
        self.status = 'ready'

    def eval(self, operand):
        if isinstance(operand, str):
            return self.regs[operand]
        else:
            assert isinstance(operand, int)
            return operand
        
def run_program(code):
    p0 = Process(0)
    p1 = Process(1)

    p = p0
    q = p1

    while True:
        if not 0 <= p.pc < len(code):
            p.status = 'terminated'
            if q.status != 'ready':
                print("both terminated")
                return p1.send_count
            p, q = q, p

        insn = code[p.pc]
        i = insn[0]
        if i == 'set':
            p.regs[insn[1]] = p.eval(insn[2])
        elif i == 'add':
            p.regs[insn[1]] += p.eval(insn[2])
        elif i == 'mul':
            p.regs[insn[1]] *= p.eval(insn[2])
        elif i == 'mod':
            p.regs[insn[1]] %= p.eval(insn[2])
        elif i == 'snd':
            q.mailbox.append(p.eval(insn[1]))
            if q.status == 'blocked':
                q.status = 'ready'
            p.send_count += 1
        elif i == 'rcv':
            if p.mailbox:
                p.regs[insn[1]] = p.mailbox.popleft()
            else:
                p.status = 'blocked'
                if q.status != 'ready':
                    # deadlock
                    return p1.send_count
                p, q = q, p
                continue  # skip pc increment, so that on resuming, we'll re-run the rcv insn.
        elif i == 'jgz':
            if p.eval(insn[1]) > 0:
                p.pc += p.eval(insn[2])
                continue  # skip pc increment
        p.pc += 1

sample_program = '''\
snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d
'''

assert run_program(parse_program(sample_program)) == 3

with open('puzzle-input.txt') as f:
    program = parse_program(f.read())
print(run_program(program))
