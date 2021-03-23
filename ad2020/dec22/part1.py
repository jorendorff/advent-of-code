from lib.advent import *


def parse_deck(text):
    return deque([int(line.strip()) for line in text.splitlines()[1:]])


def parse_input(text):
    p1, p2 = text.split("\n\n")
    return parse_deck(p1), parse_deck(p2)


def play_game(d1, d2):
    while d1 and d2:
        c1 = d1.popleft()
        c2 = d2.popleft()
        if c1 < c2:
            d2.append(c2)
            d2.append(c1)
        else:
            assert c1 > c2
            d1.append(c1)
            d1.append(c2)

    # return winning deck
    return d1 or d2


def score(deck):
    n = len(deck)
    total = 0
    for i, c in enumerate(deck):
        total += c * (n - i)
    return total


def solve(text):
    d1, d2 = parse_input(text)
    dw = play_game(d1, d2)
    return score(dw)



example = """\
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
"""

assert solve(example) == 306


if __name__ == '__main__':
    print(solve(puzzle_input()))
