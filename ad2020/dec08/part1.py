from lib.advent import *
from ad2020.code import Computer

def solve(program):
    c = Computer(program)
    c.break_on_revisit = True
    c.run()
    c.dump_profile()
    return c.acc


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

assert solve(example) == 5


if __name__ == '__main__':
    print(solve(puzzle_input()))
