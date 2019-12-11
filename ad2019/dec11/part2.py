"""Part Two

You're not sure what it's trying to paint, but it's definitely not a
registration identifier. The Space Police are getting impatient.

Checking your external ship cameras again, you notice a white panel marked
"emergency hull painting robot starting panel". The rest of the panels are
still black, but it looks like the robot was expecting to start on a white
panel, not a black one.

Based on the Space Law Space Brochure that the Space Police attached to one of
your windows, a valid registration identifier is always eight capital
letters. After starting the robot on a single white panel instead, what
registration identifier does it paint on your hull?
"""

from lib.advent import *
from ad2019.intcode.interpreter import parse, IntcodeVM
from .part1 import Robot


def show_hull(white_panels):
    y0 = int(max(p.imag for p in white_panels))
    y1 = int(min(p.imag for p in white_panels))
    x0 = int(min(p.real for p in white_panels))
    x1 = int(max(p.real for p in white_panels))

    for y in reversed(range(y1, y0 + 1)):
        line = ''
        for x in range(x0, x1 + 1):
            line += '#' if (x + y * 1j) in white_panels else '.'
        print(line)


def main():
    robot = Robot()
    robot.white_panels.add(robot.loc)
    program = parse(puzzle_input())
    vm = IntcodeVM(program,
                   input=robot.access_camera,
                   output=robot.command)

    vm.run_some()
    if vm.state != 'halt':
        raise ValueError(f"program did not finish; state is {vm.state!r}")
    show_hull(robot.white_panels)


if __name__ == '__main__':
    main()
