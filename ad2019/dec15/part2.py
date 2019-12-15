"""Part Two

You quickly repair the oxygen system; oxygen gradually fills the area.

Oxygen starts in the location containing the repaired oxygen system. It takes one minute for oxygen to spread to all open locations that are adjacent to a location that already contains oxygen. Diagonal locations are not adjacent.

In the example above, suppose you've used the droid to explore the area fully and have the following map (where locations that currently contain oxygen are marked O):

 ##
#..##
#.#..#
#.O.#
 ###

Initially, the only location which contains oxygen is the location of the repaired oxygen system. However, after one minute, the oxygen spreads to all open (.) locations that are adjacent to a location containing oxygen:

 ##
#..##
#.#..#
#OOO#
 ###

After a total of two minutes, the map looks like this:

 ##
#..##
#O#O.#
#OOO#
 ###

After a total of three minutes:

 ##
#O.##
#O#OO#
#OOO#
 ###

And finally, the whole region is full of oxygen after a total of four minutes:

 ##
#OO##
#O#OO#
#OOO#
 ###

So, in this example, all locations contain oxygen after 4 minutes.

Use the repair droid to get a complete map of the area. How many minutes will it take to fill with oxygen?
"""

# The answer is just the height of the tree made by a breadth-first search from
# the destination.

from . import part1
from ad2019.intcode.interpreter import parse
from lib.advent import *

def main():
    program = part1.parse(puzzle_input())
    robot = part1.Robot(program)
    while robot.unexplored:
        robot.explore()

    u = robot.known_universe
    oxygen = robot.target
    print(max(len(u.shortest_path(oxygen, point))
              for point in u.data))

if __name__ == '__main__':
    main()
