def replace(mylist, index, value):
    x = list(mylist)
    x[index] = value
    return x

opcodes = {
    'addr': (lambda regs, a, b: regs[a] + regs[b]),
    'addi': (lambda regs, a, b: regs[a] + b),
    'mulr': (lambda regs, a, b: regs[a] * regs[b]),
    'muli': (lambda regs, a, b: regs[a] * b),
    'banr': (lambda regs, a, b: regs[a] & regs[b]),
    'bani': (lambda regs, a, b: regs[a] & b),
    'borr': (lambda regs, a, b: regs[a] | regs[b]),
    'bori': (lambda regs, a, b: regs[a] | b),
    'setr': (lambda regs, a, b: regs[a]),
    'seti': (lambda regs, a, b: a),
    'gtrr': (lambda regs, a, b: regs[a] > regs[b]),
    'gtir': (lambda regs, a, b: a > regs[b]),
    'gtri': (lambda regs, a, b: regs[a] > b),
    'eqrr': (lambda regs, a, b: regs[a] == regs[b]),
    'eqir': (lambda regs, a, b: a == regs[b]),
    'eqri': (lambda regs, a, b: regs[a] == b),
}

example_program = '''\
#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5
'''

def parse_insn(line):
    fields = line.split()
    return tuple(fields[:1] + list(map(int, fields[1:])))
    
def parse_program(text):
    lines = text.splitlines()
    ip_line = lines.pop(0)
    [token, ip_reg] = ip_line.split()
    assert token == '#ip'
    return (int(ip_reg), [parse_insn(line) for line in lines])

NREGS = 6

def run(bound_reg, program):
    regs = [0] * NREGS
    regs[0] = 0
    ip = 0

    while 0 <= ip < len(program):
        #print("ip={} {} {} {} {} {}".format(ip, repr(regs), *program[ip]), end='')
        regs[bound_reg] = ip
        op, a, b, c = program[ip]
        regs[c] = opcodes[op](regs, a, b)
        #print(" " + repr(regs))
        ip = regs[bound_reg]
        ip += 1
    return regs[0]

assert run(*parse_program(example_program)) == 6


with open("puzzle-input.txt") as f:
    text = f.read()

print(run(*parse_program(text)))
