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


def bits_to_combos(mask_x):
    bits = [1 << i for i in range(36) if mask_x & (1 << i)]
    n = len(bits)
    return [
        sum(bits[k] for k in range(n) if j & (1 << k))
        for j in range(1 << len(bits))
    ]

assert bits_to_combos(0b10010) == [0, 2, 16, 18]
assert bits_to_combos(0) == [0]


def run_program(code):
    if isinstance(code, str):
        code = parse_program(code)

    mem = {}
    mask_x = 0
    mask_combos = [0]
    mask_1 = 0
    for op, a, b in code:
        if op == 'mask':
            mask_combos = bits_to_combos(a)
            mask_x = a
            mask_1 = b
        elif op == 'assign':
            base = (a | mask_1) & ~mask_x
            for offset in mask_combos:
                mem[base + offset] = b
        else:
            raise ValueError(f"unrecognized op: {op}")

    return sum(mem.values())


example = """\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
"""

assert run_program(example) == 208


if __name__ == '__main__':
    print(run_program(puzzle_input()))
