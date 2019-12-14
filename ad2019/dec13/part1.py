"""Day 13: Care Package

As you ponder the solitude of space and the ever-increasing three-hour
roundtrip for messages between you and Earth, you notice that the Space Mail
Indicator Light is blinking. To help keep you sane, the Elves have sent you a
care package.

It's a new game for the ship's arcade cabinet! Unfortunately, the arcade is all
the way on the other end of the ship. Surely, it won't be hard to build your
own - the care package even comes with schematics.

The arcade cabinet runs Intcode software like the game the Elves sent (your
puzzle input). It has a primitive screen capable of drawing square tiles on a
grid. The software draws tiles to the screen with output instructions: every
three output instructions specify the x position (distance from the left), y
position (distance from the top), and tile id. The tile id is interpreted as
follows:

-   0 is an empty tile. No game object appears in this tile.
-   1 is a wall tile. Walls are indestructible barriers.
-   2 is a block tile. Blocks can be broken by the ball.
-   3 is a horizontal paddle tile. The paddle is indestructible.
-   4 is a ball tile. The ball moves diagonally and bounces off objects.

For example, a sequence of output values like 1,2,3,6,5,4 would draw a
horizontal paddle tile (1 tile from the left and 2 tiles from the top) and a
ball tile (6 tiles from the left and 5 tiles from the top).

Start the game. How many block tiles are on the screen when the game exits?

"""

from lib.advent import *
from ad2019.intcode.interpreter import parse, IntcodeVM

EMPTY = 0
WALL = 1
BLOCK = 2
PADDLE = 3
BALL = 4

def main():
    program = parse(puzzle_input())

    WIDTH = 100
    HEIGHT = 100
    screen = [[0] * WIDTH for _ in range(HEIGHT)]

    def output_consumer():
        while True:
            x = yield
            y = yield
            tile_id = yield
            screen[y][x] = tile_id

    outputter = output_consumer()
    next(outputter)

    vm = IntcodeVM(program, output=outputter.send)
    vm.run_some()
    assert vm.state == 'halt'
    print(sum(row.count(BLOCK) for row in screen))


if __name__ == '__main__':
    main()
