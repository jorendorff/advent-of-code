"""Part Two

You aren't sure how large Santa's ship is. You aren't even sure if you'll need
to use this thing on Santa's ship, but it doesn't hurt to be prepared. You
figure Santa's ship might fit in a 100x100 square.

The beam gets wider as it travels away from the emitter; you'll need to be a
minimum distance away to fit a square of that size into the beam fully. (Don't
rotate the square; it should be aligned to the same axes as the drone grid.)

For example, suppose you have the following tractor beam readings:

    #.......................................
    .#......................................
    ..##....................................
    ...###..................................
    ....###.................................
    .....####...............................
    ......#####.............................
    ......######............................
    .......#######..........................
    ........########........................
    .........#########......................
    ..........#########.....................
    ...........##########...................
    ...........############.................
    ............############................
    .............#############..............
    ..............##############............
    ...............###############..........
    ................###############.........
    ................#################.......
    .................########OOOOOOOOOO.....
    ..................#######OOOOOOOOOO#....
    ...................######OOOOOOOOOO###..
    ....................#####OOOOOOOOOO#####
    .....................####OOOOOOOOOO#####
    .....................####OOOOOOOOOO#####
    ......................###OOOOOOOOOO#####
    .......................##OOOOOOOOOO#####
    ........................#OOOOOOOOOO#####
    .........................OOOOOOOOOO#####
    ..........................##############
    ..........................##############
    ...........................#############
    ............................############
    .............................###########

In this example, the 10x10 square closest to the emitter that fits entirely
within the tractor beam has been marked O. Within it, the point closest to the
emitter (the only highlighted O) is at X=25, Y=20.

Find the 100x100 square closest to the emitter that fits entirely within the
tractor beam; within that square, find the point closest to the emitter. What
value do you get if you take that point's X coordinate, multiply it by 10000,
then add the point's Y coordinate? (In the example above, this would be
250020.)
"""

from lib.advent import *
from ad2019.intcode.interpreter import IntcodeVM, parse


def get_program_output(program, x, y):
    vm = IntcodeVM(program, input=[x, y])
    vm.run_some()
    assert vm.state == 'output'
    assert vm.last_output_value in [0, 1]
    return vm.last_output_value == 1

def bisect(lo, hi, predicate):
    # find smallest value for which predicate is true
    assert not predicate(lo)
    assert predicate(hi)

    while lo + 1 < hi:
        mid = (lo + hi) // 2
        if predicate(mid):
            hi = mid
        else:
            lo = mid
    return hi


import sys
def main():
    program = parse(puzzle_input())

    LIMIT = 1520
    ranges = [None] * LIMIT

    for y in range(1, LIMIT):
        start = bisect(0, y * 2, lambda x: get_program_output(program, x, y) == 1)
        stop = bisect(y*2, y*2 + 8, lambda x: get_program_output(program, x, y) == 0)
        ranges[y] = start, stop

    for y in range(1, LIMIT):
        start, stop = ranges[y]
        start2, stop2 = ranges[y + 100 - 1]
        if stop - start2 >= 100:
            x = start2
            print(10000 * x + y)
            break

if __name__ == '__main__':
    main()
