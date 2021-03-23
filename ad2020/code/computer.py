""" AoC 2020 intcode interpreter """

OPS = {
    'acc',
    'jmp',
    'nop',
}

def parse_program(s):
    out = []
    for i, line in enumerate(s.strip().splitlines()):
        if '#' in line:
            line = line[:line.index('#')]
        line = line.strip()
        if line == '':
            continue
        fields = line.split()
        if len(fields) != 2:
            raise ValueError(f"error parsing line {i + 1}: expected 2 fields, got {len(fields)}\n{line}")
        op, arg = fields
        if not op.isalpha():
            raise ValueError(f"error parsing line {i + 1}: opcode expected \n{line}")
        if op not in OPS:
            raise ValueError(f"error parsing line {i + 1}: unrecognized opcode '{op}' \n{line}")
        if arg[:1] not in ('+', '-'):
            raise ValueError(f"error parsing line {i + 1}: expected + or -\n{line}")
        out.append((op, int(arg.lstrip('+'))))
    return out


def to_program(program):
    if isinstance(program, str):
        return parse_program(program)
    return list(program)


class Computer:
    def __init__(self, program):
        program = to_program(program)
        self.program = program
        self.visit_count = len(program) * [0]
        self.break_on_revisit = False
        self.point = 0
        self.acc = 0

    def run_insn(self):
        op, arg = self.program[self.point]
        self.visit_count[self.point] += 1

        jumped = False
        if op == 'acc':
            self.acc += arg
        elif op == 'jmp':
            self.point += arg
            jumped = True
        elif op == 'nop':
            pass
        else:
            raise ValueError(f"unrecognized opcode at {self.point}: {op} {arg:+d}")

        if not jumped:
            self.point += 1

    def run(self):
        while 0 <= self.point < len(self.program):
            if self.break_on_revisit and self.visit_count[self.point] > 0:
                return
            self.run_insn()

    def dump_profile(self):
        for i, ((op, arg), count) in enumerate(zip(self.program, self.visit_count)):
            if self.point == i:
                caret = "> "
            else:
                caret = "  "
            print(f"{caret}{op} {arg:+4} | {count}")
        print()

