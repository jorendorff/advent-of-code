from collections import defaultdict

test_program = '''\
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
'''

def evaluate(expr, env):
    if 'a' <= expr <= 'z':
        return env[expr]
    return int(expr)

def run(lines):
    record = 0
    registers = defaultdict(int)

    for line in lines:
        [target, target_op, amount, _if, comp_left, comp_op, comp_right] = line.split()
        if target_op not in ('inc', 'dec'):
            raise ValueError('inc/dec expected')
        if _if != 'if':
            raise ValueError("if expected")
        if comp_op not in ('==', '!=', '<', '>', '<=', '>='):
            raise ValueError("unrecognized comparison operator")

        compare = eval('lambda a, b: a %s b' % comp_op)
        if compare(evaluate(comp_left, registers), evaluate(comp_right, registers)):
            if target_op == 'inc':
                registers[target] += int(amount)
            else:
                assert target_op == 'dec'
                registers[target] -= int(amount)
            record = max(record, registers[target])

    return record

assert run(test_program.splitlines()) == 10

if __name__ == '__main__':
    print(run(open('puzzle-input.txt')))
