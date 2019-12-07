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
            # Input.
            out_addr = memory[ip + 1]
            memory[out_addr] = next(inputs)
            ip += 2
        elif opcode == 4:
            # Output.
            yield get(1)
            ip += 2
        elif opcode == 99:
            break
        else:
            raise ValueError("unrecognized opcode {} at ip={}"
                             .format(opcode, ip))
    return memory


assert list(vm([3,0,4,0,99], [12345, 54321])) == [12345]


def run(program, inputs=[]):
    """Run an intcode program and return the list of its outputs.

    This does not modify the program; it makes a copy.
    """
    memory = list(program)
    it = vm(memory, inputs)
    return list(it)


assert compute([1,9,10,3,2,3,11,0,99,30,40,50]) == [3500,9,10,70,2,3,11,0,99,30,40,50]

assert compute([1,0,0,0,99]) == [2,0,0,0,99]
assert compute([2,3,0,3,99]) == [2,3,0,6,99]
assert compute([2,4,4,5,99,0]) == [2,4,4,5,99,9801]
assert compute([1,1,1,4,99,5,6,0,99]) == [30,1,1,4,2,5,6,0,99]

assert compute([1002,4,3,4,33]) == [1002,4,3,4,99]
assert compute([1101,100,-1,4,0]) == [1101,100,-1,4,99]

def load(filename="puzzle-input.txt"):
    with open(filename) as f:
        return parse(f.read())

def parse(text):
    return [int(word.strip()) for word in text.split(',')]
