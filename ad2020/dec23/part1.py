from lib.advent import *

MIN_CUP = 1
MAX_CUP = 9

class Game:
    def __init__(self, cups):
        self.cups = [int(ch) for ch in cups]

    def move(self):
        cups = self.cups

        # Each move, the crab does the following actions:
        #
        # -   The crab picks up the three cups that are immediately clockwise of
        #     the current cup. They are removed from the circle; cup spacing is
        #     adjusted as necessary to maintain the circle.
        held = cups[1:4]
        del cups[1:4]

        # -   The crab selects a destination cup: the cup with a label equal to
        #     the current cup's label minus one. If this would select one of the
        #     cups that was just picked up, the crab will keep subtracting one
        #     until it finds a cup that wasn't just picked up. If at any point in
        #     this process the value goes below the lowest value on any cup's
        #     label, it wraps around to the highest value on any cup's label
        #     instead.
        dest = cups[0]
        while True:
            dest -= 1
            if dest < MIN_CUP:
                dest = MAX_CUP
            if dest in cups:
                break

        # -   The crab places the cups it just picked up so that they are
        #     immediately clockwise of the destination cup. They keep the same
        #     order as when they were picked up.
        i = cups.index(dest) + 1
        cups[i:i] = held

        # -   The crab selects a new current cup: the cup which is immediately
        #     clockwise of the current cup.
        cups.append(cups.pop(0))

    def labels_after_cup_1(self):
        cups = self.cups
        while cups[0] != 1:
            cups.append(cups.pop(0))
        return ''.join(str(n) for n in cups[1:])



def solve(s, moves=100):
    game = Game(s)
    for _ in range(moves):
        game.move()
    return game.labels_after_cup_1()


assert solve('389125467', moves=10) == '92658374'
assert solve('389125467') == '67384529'


if __name__ == '__main__':
    print(solve('158937462'))
