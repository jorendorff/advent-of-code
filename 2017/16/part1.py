
def dance(start, moves):
    state = list(start)
    for move in moves:
        if move[0] == 's':
            x = int(move[1:].strip())
            j = len(state) - x
            state = state[j:] + state[:j]
        elif move[0] == 'x':
            a, b = move[1:].strip().split("/")
            a = int(a)
            b = int(b)
            state[a], state[b] = state[b], state[a]
        elif move[0] == 'p':
            a, b = move[1:].strip().split("/")
            ai = state.index(a)
            bi = state.index(b)
            state[ai], state[bi] = b, a
        else:
            raise ValueError("unrecognized move: " + repr(move.strip()))
    return ''.join(state)

assert dance('abcde', ['s1', 'x3/4', 'pe/b']) == 'baedc'

with open('puzzle-input.txt') as f:
    text = f.read()
print(dance('abcdefghijklmnop', text.strip().split(',')))

    
