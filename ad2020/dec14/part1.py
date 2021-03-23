from lib.advent import *
import re

MASK_RE = re.compile(r'^\s*mask\s*=\s*([X01]{36})\s*$')
ASSIGN_RE = re.compile(r'^\s*mem\s*\[\s*(0|[1-9]\d*)\s*\]\s*=\s*(0|[1-9]\d*)\s*$')

def parse_program(text):
    for line in text.splitlines():
        m = MASK_RE.match(line)
        if m is not None:
            mask = m.group(1)
            mask_x = int(mask.replace('1', '0').replace('X', '1'), 2)
            mask_1 = int(mask.replace('X', '0'), 2)
            yield ('mask', mask_x, mask_1)
            continue

        m = ASSIGN_RE.match(line)
        if m is not None:
            yield ('assign', int(m.group(1)), int(m.group(2)))
            continue

        raise ValueError(f"invalid line in program:\n{line}")


def run_program(code):
    if isinstance(code, str):
        code = parse_program(code)

    mem = {}
    mask_x = (1 << 36) - 1
    mask_1 = 0
    for op, a, b in code:
        if op == 'mask':
            mask_x, mask_1 = a, b
        elif op == 'assign':
            mem[a] = b & mask_x | mask_1
        else:
            raise ValueError(f"unrecognized op: {op}")

    return sum(mem.values())


example = """\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
"""

assert run_program(example) == 165


if __name__ == '__main__':
    print(run_program(puzzle_input()))
