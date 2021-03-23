from lib.advent import *


def parse_deck(text):
    return deque([int(line.strip()) for line in text.splitlines()[1:]])


def parse_input(text):
    p1, p2 = text.split("\n\n")
    return parse_deck(p1), parse_deck(p2)


def play_game(d1, d2):
    cache = set()
    while d1 and d2:
        k = tuple(d1), tuple(d2)
        if k in cache:
            return 1, d1
        cache.add(k)

        c1 = d1.popleft()
        c2 = d2.popleft()
        if c1 <= len(d1) and c2 <= len(d2):
            winner = play_game(deque(list(d1)[:c1]), deque(list(d2)[:c2]))[0]
        elif c1 > c2:
            winner = 1
        else:
            assert c1 < c2
            winner = 2

        if winner == 1:
            d1.append(c1)
            d1.append(c2)
        else:
            d2.append(c2)
            d2.append(c1)

    # return winning deck
    return winner, d1 or d2


def score(deck):
    n = len(deck)
    total = 0
    for i, c in enumerate(deck):
        total += c * (n - i)
    return total


def solve(text):
    d1, d2 = parse_input(text)
    _winner = dw = play_game(d1, d2)
    return score(dw)


def solve(text):
    d1, d2 = parse_input(text)
    _winner, dw = play_game(d1, d2)
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

assert solve(example) == 291


example2 = """\
Player 1:
43
19

Player 2:
2
29
14
"""

# 43 19 / 2 29 14
# 19 43 2 / 29 14
# 43 2 / 14 29 19
# 2 43 14 / 29 19
# 43 14 / 19 29 2
# 14 43 19 / 29 2
# 43 19 / 2 29 14 - player 1 wins by loop

assert solve(example2) == 43 * 2 + 19 * 1


if __name__ == '__main__':
    print(solve(puzzle_input()))
