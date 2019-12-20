"""Day 20: Donut Maze

You notice a strange pattern on the surface of Pluto and land nearby to get a closer look. Upon closer inspection, you realize you've come across one of the famous space-warping mazes of the long-lost Pluto civilization!

Because there isn't much space on Pluto, the civilization that used to live here thrived by inventing a method for folding spacetime. Although the technology is no longer understood, mazes like this one provide a small glimpse into the daily life of an ancient Pluto citizen.

This maze is shaped like a donut. Portals along the inner and outer edge of the donut can instantly teleport you from one side to the other. For example:

             A           
             A           
      #######.#########  
      #######.........#  
      #######.#######.#  
      #######.#######.#  
      #######.#######.#  
      #####  B    ###.#  
    BC...##  C    ###.#  
      ##.##       ###.#  
      ##...DE  F  ###.#  
      #####    G  ###.#  
      #########.#####.#  
    DE..#######...###.#  
      #.#########.###.#  
    FG..#########.....#  
      ###########.#####  
                 Z       
                 Z       

This map of the maze shows solid walls (#) and open passages (.). Every maze on Pluto has a start (the open tile next to AA) and an end (the open tile next to ZZ). Mazes on Pluto also have portals; this maze has three pairs of portals: BC, DE, and FG. When on an open tile next to one of these labels, a single step can take you to the other tile with the same label. (You can only walk on . tiles; labels and empty space are not traversable.)

One path through the maze doesn't require any portals. Starting at AA, you could go down 1, right 8, down 12, left 4, and down 1 to reach ZZ, a total of 26 steps.

However, there is a shorter path: You could walk from AA to the inner BC portal (4 steps), warp to the outer BC portal (1 step), walk to the inner DE (6 steps), warp to the outer DE (1 step), walk to the outer FG (4 steps), warp to the inner FG (1 step), and finally walk to ZZ (6 steps). In total, this is only 23 steps.

Here is a larger example:

                       A               
                       A               
      #################.#############  
      #.#...#...................#.#.#  
      #.#.#.###.###.###.#########.#.#  
      #.#.#.......#...#.....#.#.#...#  
      #.#########.###.#####.#.#.###.#  
      #.............#.#.....#.......#  
      ###.###########.###.#####.#.#.#  
      #.....#        A   C    #.#.#.#  
      #######        S   P    #####.#  
      #.#...#                 #......VT
      #.#.#.#                 #.#####  
      #...#.#               YN....#.#  
      #.###.#                 #####.#  
    DI....#.#                 #.....#  
      #####.#                 #.###.#  
    ZZ......#               QG....#..AS
      ###.###                 #######  
    JO..#.#.#                 #.....#  
      #.#.#.#                 ###.#.#  
      #...#..DI             BU....#..LF
      #####.#                 #.#####  
    YN......#               VT..#....QG
      #.###.#                 #.###.#  
      #.#...#                 #.....#  
      ###.###    J L     J    #.#.###  
      #.....#    O F     P    #.#...#  
      #.###.#####.#.#####.#####.###.#  
      #...#.#.#...#.....#.....#.#...#  
      #.#####.###.###.#.#.#########.#  
      #...#.#.....#...#.#.#.#.....#.#  
      #.###.#####.###.###.#.#.#######  
      #.#.........#...#.............#  
      #########.###.###.#############  
               B   J   C               
               U   P   P               

Here, AA has no direct path to ZZ, but it does connect to AS and CP. By passing through AS, QG, BU, and JO, you can reach ZZ in 58 steps.

In your maze, how many steps does it take to get from the open tile marked AA to the open tile marked ZZ?
"""

from lib.advent import *


DIRS = [(0, 1), (1, 0), (0, -1), (-1, 0)]


def parse_maze(text):
    labeled_points = defaultdict(list)
    graph = LabeledDigraph()

    grid = text.splitlines()
    height = len(grid)
    width = len(grid[0])
    for y in range(2, height - 2):
        for x in range(2, width - 2):
            p = x, y
            if grid[y][x] == '.':
                for dx, dy in DIRS:
                    q = y + dy, x + dx
                    c = grid[y+dy][x+dx]
                    if c == '.':
                        graph.add_edge(p, None, q)
                    elif c.isalpha():
                        d = grid[y + dy + dy][x + dx + dx]
                        if not d.isalpha():
                            raise ValueError("expected two letters, not one, adjacent to", p)
                        if dy < 0 or dx < 0:
                            c, d = d, c  # unreverse!
                        key = c + d
                        labeled_points[key].add(p)

    aa = labeled_points['AA']
    del labeled_points['AA']
    zz = labeled_points['ZZ']
    del labeled_points['ZZ']

    for key, pts in labeled_points.items():
        if len(pts) != 2:
            raise ValueError("key {} is next to these points: {}".format(key, repr(pts))
                             + "\n(expected each tag to label exactly 2 points)")
        p, q = pts
        graph.add_edge(p, key, q)
        graph.add_edge(q, key, p)

    return graph, aa, zz


def solve(text):
    graph, aa, zz = parse_maze(text)
    return len(graph.shortest_path(aa, zz))


X1 = """\
         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       
"""

assert solve(X1) == 23

X2 = """\
                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               
"""

assert solve(X2) == 58

def main():
    text = puzzle_input()
    print(solve(text))

if __name__ == '__main__':
    main()
