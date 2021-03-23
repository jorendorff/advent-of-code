from lib.advent import *


class Deck:
    pass


class EmptyDeck(Deck):
    def __len__(self):
        return 0

    def __hash__(self):
        return 0

    def __eq__(self, other):
        return self is other

    def __iter__(self):
        if False:
            yield 0

    def __add__(self, other):
        return other

    def pop_top(self):
        raise ValueError("can't pop empty deck")


Empty = EmptyDeck()


class SingleCard(Deck):
    def __init__(self, value):
        self.value = value
        self.hash = hash(self.value)

    def __len__(self):
        return 1

    def __hash__(self):
        return self.hash

    def __eq__(self, other):
        return isinstance(other, SingleCard) and self.value == other.value

    def __iter__(self):
        yield self.value

    def __add__(self, other):
        if other is Empty:
            return self
        else:
            # unbalanced if other is a Pile but that's ok for us
            return Pile(self, other)

    def pop_top(self):
        return (self, Empty)

    def slice(self, n):
        if n == 1:
            return self
        elif n == 0:
            return Empty
        else:
            raise ValueError("not enough cards in deck")


def rotate_i64_left(n, k):
    """Rotate signed 64-bit integer n left by k bits."""
    mask = (1 << 64) - 1
    result = ((n << (k % 64)) & mask) | ((n & mask) >> ((64 - k) % 64))
    if result > mask:
        result -= 1 << 64
    return result


class Pile(Deck):
    def __init__(self, above, below):
        assert above is not Empty
        assert below is not Empty
        self.above = above
        self.below = below
        self.len = len(above) + len(below)

        # hash is fancy to ensure that decks with the same cards in the same
        # order have the same hash, regardless of tree structure.
        ha = hash(above)
        hb = hash(below)
        self.hash = rotate_i64_left(ha, 3 * len(below)) ^ hb

    def __len__(self):
        return self.len

    def __hash__(self):
        return self.hash

    def __eq__(self, other):
        return len(self) == len(other) and list(self) == list(other)

    def __iter__(self):
        yield from self.above
        yield from self.below

    def __add__(self, other):
        # Balanced as long as we only add single cards at the bottom
        if len(self.above) >= len(self.below) + len(other):
            return Pile(self.above, self.below + other)
        else:
            return Pile(self, other)

    def slice(self, count):
        """Return a deck containing the top `count` cards of this deck."""
        def cards(deck):
            if isinstance(deck, SingleCard):
                yield deck
            else:
                yield from cards(deck.above)
                yield from cards(deck.below)
        assert count <= len(self)
        c = list(itertools.islice(cards(self), 0, count))
        return sum(c, start=Empty)

    def pop_top(self):
        card, rest = self.above.pop_top()
        return card, rest + self.below


def parse_deck(text):
    return sum(
        [SingleCard(int(line.strip())) for line in text.splitlines()[1:]],
        start=Empty
    )


def parse_input(text):
    p1, p2 = text.split("\n\n")
    return parse_deck(p1), parse_deck(p2)


def play_game(d1, d2):
    game_counter = 0

    def play_subgame(d1, d2):
        assert isinstance(d1, Deck)
        assert isinstance(d2, Deck)

        nonlocal game_counter
        game_counter += 1
        game_id = game_counter
        #print(f"=== Game {game_id} ===")
        #print()

        round = 0
        cache = set()
        while d1 and d2:
            round += 1
            #print(f"-- Round {round} (Game {game_id}) --")
            #print(f"Player 1's deck: {list(d1)!r}")
            #print(f"Player 2's deck: {list(d2)!r}")

            if (d1, d2) in cache:
                #print("repeated game state found, bailing out")
                result = 1, d1
                return result
            cache.add((d1, d2))

            c1, d1 = d1.pop_top()
            #print("Player 1 plays:", c1.value)
            c2, d2 = d2.pop_top()
            #print("Player 2 plays:", c2.value)
            if c1.value <= len(d1) and c2.value <= len(d2):
                #print("Playing a sub-game to determine the winner...")
                #print()
                sd1 = d1.slice(c1.value)
                sd2 = d2.slice(c2.value)
                winner = play_subgame(sd1, sd2)[0]
            elif c1.value > c2.value:
                winner = 1
            else:
                winner = 2

            #print(f"Player {winner} wins round {round} of game {game_id}!")
            if winner == 1:
                d1 += c1
                d1 += c2
            else:
                d2 += c2
                d2 += c1
            #print()

        result = 1 if d1 else 2, d1 or d2
        return result

    return play_subgame(d1, d2)

def score(deck):
    n = len(deck)
    total = 0
    for i, c in enumerate(deck):
        total += c * (n - i)
    return total


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
