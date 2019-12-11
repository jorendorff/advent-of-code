class IntcodeVM:
    def __init__(self, program, input=None, output=None):
        """Copy `program` into the new VM.

        input - Determines the behavior of the input instruction. Must be one of:
            iterable: the input instruction takes values from this iterator.
            callable: the input instruction gets its value by calling this value with no arguments.
            None: the input instruction suspends the VM.

        output - Determines the behavior of the output instruction.
            callable: the output instruction calls it, passing an integer value.
            None: the output instruction suspends the VM.
        """

        self.memory = list(program)
        self.ip = 0
        self.relative_base = 0
        self.state = 'start'
        if input is None:
            self.input = None
        elif callable(input):
            self.input = input
        else:
            self.input = iter(input).__next__
        self.output = output

    def _load(self, addr):
        if 0 <= addr < len(self.memory):
            return self.memory[addr]
        else:
            return 0

    def _store(self, addr, value):
        if addr < 0:
            raise ValueError(f"store to negative address {addr}")
        if addr >= len(self.memory):
            self.memory += [0] * (addr - len(self.memory) + 1)
        self.memory[addr] = value

    def _get(self, operand_index):
        """Get an operand for the current instruction."""
        ip = self.ip
        modes = self.memory[ip] // 100
        operand = self.memory[ip + operand_index]
        mode = modes // (10 ** (operand_index - 1)) % 10
        if mode == 0:
            # position mode
            return self._load(operand)
        elif mode == 1:
            # immediate mode
            return operand
        elif mode == 2:
            # relative mode
            return self._load(operand + self.relative_base)
        else:
            raise ValueError("invalid mode {} for operand {} of instruction at ip={}"
                             .format(mode, operand_index, ip))

    def _get_addr(self, operand_index):
        """Get the target address for an output operand."""
        ip = self.ip
        modes = self.memory[ip] // 100
        operand = self.memory[ip + operand_index]
        mode = modes // (10 ** (operand_index - 1)) % 10
        if mode == 0:
            return operand
        elif mode == 2:
            return operand + self.relative_base
        else:
            raise ValueError("invalid mode {} for output operand {} of instruction at ip={}"
                             .format(mode, operand_index, ip))

    def trace(self, message, *args):
        #print("* " + message.format(*args))
        pass

    def run_some(self):
        """Run until the next input, output, or halt instruction."""
        assert self.state != 'input'
        while True:
            self.trace("running instruction at {}", self.ip)
            insn = self.memory[self.ip]
            if insn < 0:
                raise ValueError("negative instruction at ip={}, seems fishy"
                                 .format(self.ip))
            opcode = insn % 100
            if opcode in (1, 2):
                # add/mul v1, v2 -> addr
                a, b, out_addr = self._get(1), self._get(2), self._get_addr(3)
                if opcode == 1:
                    c = a + b
                else:
                    c = a * b
                self._store(out_addr, c)
                self.ip += 4
            elif opcode == 3:
                # input -> addr
                if self.input is None:
                    self.trace("suspending to wait for input")
                    self.state = 'input'
                    return
                addr = self._get_addr(1)
                self._store(addr, self.input())
                self.ip += 2
            elif opcode == 4:
                # output v1
                value = self._get(1)
                if self.output is None:
                    # Suspend for output.
                    self.trace("suspending to output {}", value)
                    self.last_output_value = value
                    self.state = 'output'
                    self.ip += 2
                    return
                self.output(value)
                self.ip += 2
            elif opcode == 5:
                # jump-if-true v, dest
                cond, target = self._get(1), self._get(2)
                self.ip = target if cond != 0 else self.ip + 3
            elif opcode == 6:
                # jump-if-false v, dest
                cond, target = self._get(1), self._get(2)
                self.ip = target if cond == 0 else self.ip + 3
            elif opcode == 7:
                # lt v1, v2 -> addr
                a, b, out_addr = self._get(1), self._get(2), self._get_addr(3)
                self._store(out_addr, int(a < b))
                self.ip += 4
            elif opcode == 8:
                # eq v1, v2 -> addr
                a, b, out_addr = self._get(1), self._get(2), self._get_addr(3)
                self._store(out_addr, int(a == b))
                self.ip += 4
            elif opcode == 9:
                # arb v
                self.relative_base += self._get(1)
                self.ip += 2
            elif opcode == 99:
                # halt
                self.trace("halt")
                self.state = 'halt'
                return
            else:
                raise ValueError("unrecognized opcode {} at ip={}"
                                 .format(opcode, self.ip))

    def send(self, input_value):
        """Resume at an input instruction."""
        assert self.state == 'input'
        assert self.memory[self.ip] % 100 == 3  # input

        self.trace("received input {}", input_value)

        # second half of input instruction
        out_addr = self.memory[self.ip + 1]
        self._store(out_addr, input_value)
        self.ip += 2

        self.state = None
        return self.run_some()


def vm(memory, input=()):
    """Run a program, modifying `memory` in place.

    This generator yields the program's outputs and returns `memory`.
    """

    vm = IntcodeVM([], input=input)
    vm.memory = memory  # modify this list directly

    vm.run_some()
    while vm.state != 'halt':
        assert vm.state == 'output'
        yield vm.last_output_value
        vm.run_some()

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

def run(program, input=[]):
    """Run an intcode program and return the list of its outputs.

    This does not modify the program; it makes a copy.
    """
    log = []
    vm = IntcodeVM(program, input=input, output=log.append)
    vm.run_some()
    return log



def test_day_9():
    assert run([109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]) \
                 == [109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]

    [result] = run([1102,34915192,34915192,7,4,7,99,0])
    assert len(str(result)) == 16

    assert run([104,1125899906842624,99]) == [1125899906842624]


test_day_9()


def load(filename="puzzle-input.txt"):
    with open(filename) as f:
        return parse(f.read())


def parse(text):
    return [int(word.strip()) for word in text.split(',')]
