"""Day 2: 1202 Program Alarm - Part Two

"Good, the new computer seems to be working correctly! Keep it nearby during
this mission - you'll probably use it again. Real Intcode computers support
many more features than your new one, but we'll let you know what they are as
you need them."

"However, your current priority should be to complete your gravity assist
around the Moon. For this mission to succeed, we should settle on some
terminology for the parts you've already built."

Intcode programs are given as a list of integers; these values are used as the
initial state for the computer's memory. When you run an Intcode program, make
sure to start by initializing memory to the program's values. A position in
memory is called an address (for example, the first value in memory is at
"address 0").

Opcodes (like 1, 2, or 99) mark the beginning of an instruction. The values
used immediately after an opcode, if any, are called the instruction's
parameters. For example, in the instruction 1,2,3,4, 1 is the opcode; 2, 3, and
4 are the parameters. The instruction 99 contains only an opcode and has no
parameters.

The address of the current instruction is called the instruction pointer; it
starts at 0. After an instruction finishes, the instruction pointer increases
by the number of values in the instruction; until you add more instructions to
the computer, this is always 4 (1 opcode + 3 parameters) for the add and
multiply instructions. (The halt instruction would increase the instruction
pointer by 1, but it halts the program instead.)

"With terminology out of the way, we're ready to proceed. To complete the
gravity assist, you need to determine what pair of inputs produces the output
19690720."

The inputs should still be provided to the program by replacing the values at
addresses 1 and 2, just like before. In this program, the value placed in
address 1 is called the noun, and the value placed in address 2 is called the
verb. Each of the two input values will be between 0 and 99, inclusive.

Once the program has halted, its output is available at address 0, also just
like before. Each time you try a pair of inputs, make sure you first reset the
computer's memory to the values in the program (your puzzle input) - in other
words, don't reuse memory from a previous attempt.

Find the input noun and verb that cause the program to produce the output
19690720. What is 100 * noun + verb? (For example, if noun=12 and verb=2, the
answer would be 1202.)
"""


import itertools, functools, operator
from lib.advent import *


# The brute solution would be to run the interpreter forward, in a nested loop.
#
# Another easy solution would be to examine the puzzle input and figure it out
# from that.
#
# I'm going to try analysis.

class Value:
    def __add__(self, other):
        return simplify_add([self, other])

    def __mul__(self, other):
        return simplify_mul([self, other])

    def _key(self):
        return (self.__class__.__name__, tuple(getattr(self, attr) for attr in self.__slots__))

    def __eq__(self, other):
        return self._key() == other._key()

    def __hash__(self):
        return hash(self._key())


class Number(Value):
    __slots__ = ['value']

    def __init__(self, value):
        self.value = value

    def __repr__(self):
        return str(self.value)

    def eval(self, env):
        return self.value


class Variable(Value):
    __slots__ = ['name']

    def __init__(self, name):
        self.name = name

    def __repr__(self):
        return self.name

    def eval(self, env):
        return env[self.name]


def flatten(inputs, cls, identity, f):
    for v in inputs:
        if type(v) is cls:
            values += v.values
        else:
            values.append(v)
    return values


def simplify_add(values):
    def flatten(values):
        for v in values:
            if type(v) is Add:
                yield from flatten(v.values)
                if v.constant != 0:
                    yield Number(v.constant)
            else:
                yield v

    constant = 0
    abstract = []
    for v in flatten(values):
        if type(v) is Number:
            constant += v.value
        else:
            abstract.append(v)

    if len(abstract) == 0:
        return Number(constant)
    elif len(abstract) == 1 and constant == 0:
        return abstract[0]
    else:
        abstract.sort(key=repr)
        # Should combine like terms here, but it turns out not to be necessary.
        return Add(abstract, constant)


class Add(Value):
    __slots__ = ['values', 'constant']

    def __init__(self, values, constant=0):
        assert len(values) != 0
        self.values = values
        self.constant = constant

    def all_values(self):
        yield from self.values
        if self.constant != 0:
            yield Number(self.constant)

    def __repr__(self):
        return '(' + ' + '.join(repr(v) for v in self.all_values()) + ')'

    def eval(self, env):
        return sum(v.eval(env) for v in self.values) + self.constant

assert Number(3) + Number(0) == Number(3)
assert Number(3) + Variable('Fred') == Add([Variable('Fred')], 3)


class Mul(Value):
    __slots__ = ['constant', 'values']

    def __init__(self, constant, factors):
        assert len(factors) != 0
        self.constant = constant
        self.values = factors

    def all_values(self):
        if self.constant != 1:
            yield Number(self.constant)
        yield from self.values

    def __repr__(self):
        return '(' + ' * '.join(repr(v) for v in self.all_values()) + ')'

    def eval(self, env):
        return self.constant * functools.reduce(operator.mul, [v.eval(env) for v in self.values])


def simplify_mul(values):
    def flatten(values):
        for v in values:
            if type(v) is Mul:
                if v.constant != 1:
                    yield Number(v.constant)
                yield from flatten(v.values)
            else:
                yield v

    constant = 1
    factors = []
    sum_factors = []
    for v in flatten(values):
        if type(v) is Number:
            constant *= v.value
        elif type(v) is Add:
            sum_factors.append(v)
        else:
            factors.append(v)

    if constant == 0:
        start = Number(0)
    elif len(factors) == 0:
        start = Number(constant)
    elif len(factors) == 1 and constant == 1:
        start = factors[0]
    else:
        factors.sort(key=repr)
        start = Mul(constant, factors)

    if sum_factors:
        # Distribute multiplication across addition.
        results = [simplify_mul([start, *summands])
                   for summands in itertools.product(*[f.all_values() for f in sum_factors])]
        return simplify_add(results)
    else:
        return start


assert (simplify_mul([Number(2) * Variable('x') + Number(3),
                      Number(2)])
        == Number(4) * Variable('x') + Number(6))


class Lookup(Value):
    def __init__(self, memory, addr):
        self.memory = list(memory)
        self.addr = addr

    def __repr__(self):
        return f'Lookup(..., {repr(self.addr)})'

    def eval(self, env):
        return self.memory[self.addr.eval(env)].eval(env)


def abstract_interpret(memory):
    def get_concrete(addr):
        abstract_value = memory[addr]
        if type(abstract_value) is not Number:
            raise ValueError("can't compute: variable")
        return abstract_value.value

    def lookup(addr):
        if type(addr) is Number:
            return memory[addr.value]
        else:
            return Lookup(memory, addr)

    def put_concrete(addr, abstract_value):
        if type(addr) is not Number:
            raise ValueError("can't compute: variable destination address")
        memory[addr.value] = abstract_value

    ip = 0  # instruction pointer
    while True:
        insn = get_concrete(ip)
        if insn in (1, 2):
            a_addr = memory[ip + 1]
            b_addr = memory[ip + 2]
            out_addr = memory[ip + 3]

            a = lookup(a_addr)
            b = lookup(b_addr)

            if insn == 1:
                result = a + b
            else:
                result = a * b

            put_concrete(out_addr, result)
            ip += 4
        elif insn == 99:
            break
        else:
            raise ValueError(f"bad instruction {insn} at ip={ip}")

    return memory


NOUN = Variable('noun')
VERB = Variable('verb')

assert NOUN + VERB + Number(1) == Add([NOUN, VERB], 1)


def process(source, output_addr=0):
    memory = [Number(i) for i in source]
    memory[1] = NOUN
    memory[2] = VERB
    return abstract_interpret(memory)[output_addr]


assert process([1,0,0,3, 1,1,2,3, 1,3,4,3, 99], 3) == NOUN + VERB + Number(1)
assert process([1,0,0,3, 1,1,2,3, 1,3,4,3, 1,5,0,3, 99], 3) == Number(2)


def main():
    TARGET = 19690720

    source = [int(word) for word in puzzle_input().strip().split(',')]

    size = len(source)
    print(size, "ints read")

    abstract_result = process(source)
    print("this program computes:", abstract_result)

    # OK, wussed out at the end. Rather than solve for noun and verb, just try
    # all combinations. I could have saved a lot of time and done this up
    # front. :)
    for noun in range(size):
        for verb in range(size):
            if abstract_result.eval(locals()) == TARGET:
                print(noun, verb, "==>", 100 * noun + verb)


if __name__ == '__main__':
    main()
