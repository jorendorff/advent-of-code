from lib.advent import *

MIN_CUP = 1

class Game:
    def __init__(self, cups, max_cup = 9):
        assert max_cup >= MIN_CUP
        cups = [int(ch) for ch in cups]
        assert len(cups) == len(set(cups))
        max_given = max(cups) if cups else 0
        assert max_given <= max_cup

        self.next = [None] * (max_cup + 1)
        prev = cups[-1] if max_given == max_cup else max_cup
        if cups:
            for c in cups:
                self.next[prev] = c
                prev = c
        for c in range(max_given + 1, max_cup + 1):
            self.next[prev] = c
            prev = c

        self.current_cup = cups[0] if cups else 1
        self.max_cup = max_cup

    def move(self):
        next = self.next

        # Each move, the crab does the following actions:
        #
        # -   The crab picks up the three cups that are immediately clockwise of
        #     the current cup. They are removed from the circle; cup spacing is
        #     adjusted as necessary to maintain the circle.
        held1 = next[self.current_cup]
        held2 = next[held1]
        held3 = next[held2]
        unheld = next[held3]
        next[self.current_cup] = unheld

        # -   The crab selects a destination cup: the cup with a label equal to
        #     the current cup's label minus one. If this would select one of the
        #     cups that was just picked up, the crab will keep subtracting one
        #     until it finds a cup that wasn't just picked up. If at any point in
        #     this process the value goes below the lowest value on any cup's
        #     label, it wraps around to the highest value on any cup's label
        #     instead.
        dest = (self.current_cup - 1) or self.max_cup
        while dest in (held1, held2, held3):
            dest = (dest - 1) or self.max_cup

        # -   The crab places the cups it just picked up so that they are
        #     immediately clockwise of the destination cup. They keep the same
        #     order as when they were picked up.
        dest_right = next[dest]
        next[dest] = held1
        next[held3] = dest_right

        # -   The crab selects a new current cup: the cup which is immediately
        #     clockwise of the current cup.
        self.current_cup = next[self.current_cup]

    def labels_after_cup_1(self):
        next = self.next

        answer = ''
        c = next[1]
        while c != 1:
            answer += str(c)
            c = next[c]
        return answer


def solve1(s, max_cup=9, moves=100):
    game = Game(s)
    for _ in range(moves):
        game.move()
    return game.labels_after_cup_1()


def solve(s):
    CUPS = 1_000_000
    MOVES = 10_000_000
    game = Game(s, max_cup=CUPS)
    for _ in range(MOVES):
        game.move()
    c1 = game.next[1]
    c2 = game.next[c1]
    return c1 * c2


PUZZLE_INPUT = '158937462'

assert solve1('389125467', moves=10) == '92658374'
assert solve1('389125467') == '67384529'
assert solve1(PUZZLE_INPUT) == '69473825'
assert solve('389125467') == 149245887792


if __name__ == '__main__':
    print(solve(PUZZLE_INPUT))
