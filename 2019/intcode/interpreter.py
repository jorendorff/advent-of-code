def vm(memory, inputs):
    """Run a program, modifying `memory` in place.

    This generator yields the program's outputs and returns `memory`.
    """

    def get(operand_index):
        operand = memory[ip + operand_index]
        mode = modes // (10 ** (operand_index - 1)) % 10
        if mode == 0:
            # position mode
            return memory[operand]
        elif mode == 1:
            # immediate mode
            return operand
        else:
            raise ValueError("invalid mode {} for operand {} of instruction at ip={}"
                             .format(mode, operand_index, ip))

    inputs = iter(inputs)

    ip = 0  # instruction pointer
    while True:
        insn = memory[ip]
        if insn < 0:
            raise ValueError("negative instruction at ip={}, seems fishy"
                             .format(ip))
        modes, opcode = divmod(insn, 100)
        if opcode in (1, 2):
            a = get(1)
            b = get(2)
            out_addr = memory[ip + 3]
            if opcode == 1:
                c = a + b
            else:
                c = a * b
            memory[out_addr] = c
            ip += 4
        elif opcode == 3:
            # input
            out_addr = memory[ip + 1]
            memory[out_addr] = next(inputs)
            ip += 2
        elif opcode == 4:
            # output
            yield get(1)
            ip += 2
        elif opcode == 5:
            # jump-if-true
            cond, target = get(1), get(2)
            ip = target if cond != 0 else ip + 3
        elif opcode == 6:
            # jump-if-false
            cond, target = get(1), get(2)
            ip = target if cond == 0 else ip + 3
        elif opcode == 7:
            # less than
            a, b, out_addr = get(1), get(2), memory[ip + 3]
            memory[out_addr] = int(a < b)
            ip += 4
        elif opcode == 8:
            # equals
            a, b, out_addr = get(1), get(2), memory[ip + 3]
            memory[out_addr] = int(a == b)
            ip += 4
        elif opcode == 99:
            break
        else:
            raise ValueError("unrecognized opcode {} at ip={}"
                             .format(opcode, ip))
    return memory


assert list(vm([3,0,4,0,99], [12345, 54321])) == [12345]


def test_multi(code, trial_values, expected):
    """Test some code that takes a single input and produces a single output, 0 or 1."""
    for v in trial_values:
        memory = list(code)  # copy before running vm, which mutates memory
        actual = list(vm(memory, [v]))
        assert actual == [int(expected(v))]


test_multi([3,9,8,9,10,9,4,9,99,-1,8], [7, 8, 9], lambda x: x == 8)
test_multi([3,9,7,9,10,9,4,9,99,-1,8], [7, 8, 9], lambda x: x < 8)
test_multi([3,3,1108,-1,8,3,4,3,99], [7, 8, 9], lambda x: x == 8)
test_multi([3,3,1107,-1,8,3,4,3,99], [7, 8, 9], lambda x: x < 8)

test_multi([3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], [-1, 0, 1], lambda x: x != 0)
test_multi([3,3,1105,-1,9,1101,0,0,12,4,12,99,1], [-1, 0, 1], lambda x: x != 0)

LARGE_EXAMPLE = [
    3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
    1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
    999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99
]

test_multi(LARGE_EXAMPLE, [7, 8, 9],
           lambda x: 999 if x < 8 else 1000 if x == 8 else 1001)


def _compute(memory):
    """Run an intcode program that doesn't require any inputs. Ignore outputs. Return the modified memory."""
    for _ in vm(memory, []):
        pass
    return memory


assert _compute([1,9,10,3,2,3,11,0,99,30,40,50]) == [3500,9,10,70,2,3,11,0,99,30,40,50]

assert _compute([1,0,0,0,99]) == [2,0,0,0,99]
assert _compute([2,3,0,3,99]) == [2,3,0,6,99]
assert _compute([2,4,4,5,99,0]) == [2,4,4,5,99,9801]
assert _compute([1,1,1,4,99,5,6,0,99]) == [30,1,1,4,2,5,6,0,99]

assert _compute([1002,4,3,4,33]) == [1002,4,3,4,99]
assert _compute([1101,100,-1,4,0]) == [1101,100,-1,4,99]


# High-level interface to the vm

def run(program, inputs=[]):
    """Run an intcode program and return the list of its outputs.

    This does not modify the program; it makes a copy.
    """
    memory = list(program)
    it = vm(memory, inputs)
    return list(it)


def load(filename="puzzle-input.txt"):
    with open(filename) as f:
        return parse(f.read())


def parse(text):
    return [int(word.strip()) for word in text.split(',')]
