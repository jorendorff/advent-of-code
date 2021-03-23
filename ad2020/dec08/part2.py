from lib.advent import *
from ad2020.code import parse_program, Computer

def solve(program):
    # brute force is best
    program = parse_program(program)
    for i in range(len(program)):
        save_op, arg = program[i]
        if save_op == 'nop':
            program[i] = ('jmp', arg)
        elif save_op == 'jmp':
            program[i] = ('nop', arg)
        else:
            continue

        try:
            c = Computer(program)
            c.break_on_revisit = True
            c.run()
            if c.point == len(program):
                return c.acc
        finally:
            program[i] = (save_op, arg)


example = """
    nop +0
    acc +1
    jmp +4
    acc +3
    jmp -3
    acc -99
    acc +1
    jmp -4
    acc +6
"""

assert solve(example) == 8


if __name__ == '__main__':
    print(solve(puzzle_input()))
