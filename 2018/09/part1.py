
def play(nplayers, nmarbles):
    scores = [0] * nplayers
    state = [0]
    pos = 0
    current_player = 0

    for i in range(1, nmarbles + 1):
        if i % 23 == 0:
            scores[current_player] += i
            pos = (pos - 7) % len(state)
            scores[current_player] += state[pos]
            del state[pos]
            pos = pos % len(state)
        else:
            pos = (pos + 2) % len(state)
            state.insert(pos, i)
        current_player = (current_player + 1) % nplayers

    return max(scores)

assert play(9, 25) == 32

assert play(10, 1618) == 8317
assert play(13, 7999) == 146373
assert play(17, 1104) == 2764
assert play(21, 6111) == 54718
assert play(30, 5807) == 37305

import re
with open('puzzle-input.txt') as f:
    m = re.match(r'(\d+) players; last marble is worth (\d+) points$', f.read().strip())
    nplayers = int(m.group(1))
    nmarbles = int(m.group(2))
    print(play(nplayers, nmarbles))
    print(play(nplayers, nmarbles * 100))
