import unittest

INSTRUCTION_SET = {
    1: ("add", 2, 1),
    2: ("mul", 2, 1),
    3: ("input", 0, 1),
    4: ("output", 1, 0),
    5: ("jump-if-true", 2, 0),
    6: ("jump-if-false", 2, 0),
    7: ("lt", 2, 1),
    8: ("eq", 2, 1),
    9: ("arb", 1, 0),
    99: ("halt", 0, 0),
}


class Instruction:
    def __init__(self, addr, name, operands):
        self.addr = addr
        self.name = name
        self.operands = operands

    def __str__(self):
        ostr = ""
        for i, op in enumerate(self.operands):
            if i == 0 or op.startswith('->'):
                ostr += " "
            else:
                ostr += ", "
            ostr += op
        return f"{self.addr:6d}  {self.name}{ostr}"


def decode_instruction(memory, ip):
    """Disassemble the instruction at ip.

    Returns a pair (instruction, next_ip).

    If the instruction can't be decoded, this returns some sort of harmless
    pair of values rather than throwing.
    """

    readp = ip

    insn = memory[readp]
    readp += 1
    modes, opcode = divmod(insn, 100)
    if opcode not in INSTRUCTION_SET:
        return Instruction(ip, "???" + str(insn), ()), ip + 1

    insn_name, in_count, dest_count = INSTRUCTION_SET[opcode]

    # Decode input operands.
    operands = []
    for _ in range(in_count):
        if readp >= len(memory):
            operands.append("???<error>")
            continue
        n = memory[readp]
        readp += 1
        modes, mode = divmod(modes, 10)
        if mode == 0:
            operands.append(f"[{n}]")
        elif mode == 1:
            operands.append(str(n))
        elif mode == 2:
            operands.append(f"rel[{n}]")
        else:
            operands.append(f"???mode{mode}({n})")

    # Decode the output operand, if any.
    for _ in range(dest_count):
        if readp >= len(memory):
            operands.append("-> ???<error>")
            continue
        n = memory[readp]
        readp += 1
        modes, mode = divmod(modes, 10)
        # Note: mode 1 (immediate mode) not allowed here
        if mode == 0:
            operands.append(f"-> [{n}]")
        elif mode == 2:
            operands.append(f"-> rel[{n}]")
        else:
            operands.append(f"???mode{mode}({n})")

    # There shouldn't be any modes left over.
    if modes:
        return Instruction(ip, "???" + str(insn), ()), ip + 1


    return Instruction(ip, insn_name, operands), readp


def disassemble(program):
    ip = 0
    while ip < len(program):
        insn, next_ip = decode_instruction(program, ip)
        yield insn
        ip = next_ip


def dis(program):
    for insn in disassemble(program):
        print(insn)


class TestDisassembler(unittest.TestCase):
    def dis(self, program):
        return [str(insn) for insn in disassemble(program)]

    def test_d7p2_1(self):
        # One of the example programs from 2019 day 7 part 2.
        p1 = [3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
              27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5]
        self.assertEqual(self.dis(p1), [
            '     0  input -> 26',
            '     2  add [26], -4 -> 26',
            '     6  input -> 27',
            '     8  mul [27], 2 -> 27',
            '    12  add [27], [26] -> 27',
            '    16  output [27]',
            '    18  add [28], -1 -> 28',
            '    22  jump-if-true [28], 6',
            '    25  halt',
            # The rest of this image is not instructions,
            # but failing gracefully is part of the test.
            '    26  ???0',
            '    27  ???0',
            '    28  jump-if-true ???<error>, ???<error>'
        ])

    def test_d7p2_2(self):
        # The other example program.
        p2 = [3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
              -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
              53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10]
        self.assertEqual(self.dis(p2[:52]), [
            '     0  input -> 52',
            '     2  add [52], -5 -> 52',
            '     6  input -> 53',
            '     8  add [52], [56] -> 54',
            '    12  lt [54], 5 -> 55',
            '    16  jump-if-true [55], 26',
            '    19  add [54], -5 -> 54',
            '    23  jump-if-true 1, 12',
            '    26  add [53], [54] -> 53',
            '    30  eq [54], 0 -> 55',
            '    34  add [55], 1 -> 55',
            '    38  mul [53], [55] -> 53',
            '    42  output [53]',
            '    44  add [56], -1 -> 56',
            '    48  jump-if-true [56], 6',
            '    51  halt'
        ])

if __name__ == '__main__':
    unittest.main()
