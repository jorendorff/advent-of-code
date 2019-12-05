"""Part Two

The air conditioner comes online! Its cold air feels good for a while, but then
the TEST alarms start to go off. Since the air conditioner can't vent its heat
anywhere but back into the spacecraft, it's actually making the air inside the
ship *warmer*.

Instead, you'll need to use the TEST to extend the thermal radiators.
Fortunately, the diagnostic program (your puzzle input) is already equipped for
this. Unfortunately, your Intcode computer is not.

Your computer is only missing a few opcodes:

*   Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the
    instruction pointer to the value from the second parameter. Otherwise, it
    does nothing.

*   Opcode 6 is jump-if-false: if the first parameter is zero, it sets the
    instruction pointer to the value from the second parameter. Otherwise, it
    does nothing.

*   Opcode 7 is less than: if the first parameter is less than the second
    parameter, it stores 1 in the position given by the third
    parameter. Otherwise, it stores 0.

*   Opcode 8 is equals: if the first parameter is equal to the second
    parameter, it stores 1 in the position given by the third
    parameter. Otherwise, it stores 0.

Like all instructions, these instructions need to support parameter modes as
described above.

Normally, after an instruction is finished, the instruction pointer increases
by the number of values in that instruction. However, if the instruction
modifies the instruction pointer, that value is used and the instruction
pointer is not automatically increased.

For example, here are several programs that take one input, compare it to the
value 8, and then produce one output:

*   `3,9,8,9,10,9,4,9,99,-1,8` - Using position mode, consider whether the input
    is equal to 8; output 1 (if it is) or 0 (if it is not).

*   `3,9,7,9,10,9,4,9,99,-1,8` - Using position mode, consider whether the input
    is less than 8; output 1 (if it is) or 0 (if it is not).

*   `3,3,1108,-1,8,3,4,3,99` - Using immediate mode, consider whether the input
    is equal to 8; output 1 (if it is) or 0 (if it is not).

*   `3,3,1107,-1,8,3,4,3,99` - Using immediate mode, consider whether the input
    is less than 8; output 1 (if it is) or 0 (if it is not).

Here are some jump tests that take an input, then output 0 if the input was
zero or 1 if the input was non-zero:

*   `3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9` (using position mode)

*   `3,3,1105,-1,9,1101,0,0,12,4,12,99,1` (using immediate mode)

Here's a larger example:

```
3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99
```

The above example program uses an input instruction to ask for a single
number. The program will then output 999 if the input value is below 8, output
1000 if the input value is equal to 8, or output 1001 if the input value is
greater than 8.

This time, when the TEST diagnostic program runs its input instruction to get
the ID of the system to test, provide it `5`, the ID for the ship's thermal
radiator controller. This diagnostic test suite only outputs one number, the
diagnostic code.

What is the diagnostic code for system ID 5?
"""


def vm(memory, inputs):
    """Run the whole program, modifying `memory` in place. Return `memory`."""

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


def run_diagnostic_program(memory, input_values):
    [output] = vm(memory, input_values)
    return output


with open("puzzle-input.txt") as f:
    memory = [int(word) for word in f.read().strip().split(',')]
    print(run_diagnostic_program(memory, [5]))
