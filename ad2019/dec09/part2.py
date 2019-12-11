"""Part Two

You now have a complete Intcode computer.

Finally, you can lock on to the Ceres distress signal! You just need to boost
your sensors using the BOOST program.

The program runs in sensor boost mode by providing the input instruction the
value 2. Once run, it will boost the sensors automatically, but it might take a
few seconds to complete the operation on slower hardware. In sensor boost mode,
the program will output a single value: the coordinates of the distress signal.

Run the BOOST program in sensor boost mode. What are the coordinates of the
distress signal?
"""

import sys; sys.path.append("..")
from intcode import interpreter, disassembler


def main():
    program = interpreter.load()
    disassembler.dis(program)
    [result] = interpreter.run(program, input=[2])
    print(result)


if __name__ == '__main__':
    main()
