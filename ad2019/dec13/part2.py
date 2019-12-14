"""Part Two

The game didn't run because you didn't put in any quarters. Unfortunately, you
did not bring any quarters. Memory address 0 represents the number of quarters
that have been inserted; set it to 2 to play for free.

The arcade cabinet has a joystick that can move left and right. The software
reads the position of the joystick with input instructions:

-   If the joystick is in the neutral position, provide 0.
-   If the joystick is tilted to the left, provide -1.
-   If the joystick is tilted to the right, provide 1.

The arcade cabinet also has a segment display capable of showing a single
number that represents the player's current score. When three output
instructions specify X=-1, Y=0, the third output instruction is not a tile; the
value instead specifies the new score to show in the segment display. For
example, a sequence of output values like -1,0,12345 would show 12345 as the
player's current score.

Beat the game by breaking all the blocks. What is your score after the last
block is broken?
"""

from lib.advent import *
from ad2019.intcode.interpreter import parse, IntcodeVM
from ad2019.intcode import disassembler
from time import sleep

EMPTY = 0
WALL = 1
BLOCK = 2
PADDLE = 3
BALL = 4

pictures = " #O=*"

import sys, tty, termios
def getch():
    """https://stackoverflow.com/questions/510357/python-read-a-single-character-from-the-user"""
    fd = sys.stdin.fileno()
    old_settings = termios.tcgetattr(fd)
    try:
        tty.setraw(sys.stdin.fileno())
        ch = sys.stdin.read(1)
    finally:
        termios.tcsetattr(fd, termios.TCSADRAIN, old_settings)
    return ch


def main():
    program = parse(puzzle_input())

    WIDTH = 36
    HEIGHT = 24
    screen = [[0] * WIDTH for _ in range(HEIGHT)]
    score = 0

    def locate(object):
        for row in screen:
            if object in row:
                return row.index(object)
        return None

    def prompt():
        print("  %%%dd" % WIDTH % score)
        print('-' * (WIDTH + 2))
        for row in screen:
            print('|' + ''.join(pictures[tile_id] for tile_id in row) + '|')
        print('-' * (WIDTH + 2))
        b = locate(BALL)
        p = locate(PADDLE)
        sleep(0.01)
        if b < p:
            return -1
        elif b == p:
            return 0
        else:
            return 1
##
##        c = getch()
##        print(f"You typed: {c!r}")
##        if c == '\x03' or c in 'Qq':
##            raise KeyboardInterrupt
##        elif c in 'AHah':
##            return -1
##        elif c in 'DLdl':
##            return 1
##        else:
##            return 0
##        

    def output_consumer():
        nonlocal score
        while True:
            x = yield
            y = yield
            value = yield
            if x == -1 and y == 0:
                score = value
            else:
                screen[y][x] = value

    outputter = output_consumer()
    next(outputter)

    program[0] = 2
    vm = IntcodeVM(program, input=prompt, output=outputter.send)
    vm.run_some()
    assert vm.state == 'halt'
    prompt()
    print(score)


if __name__ == '__main__':
    main()
