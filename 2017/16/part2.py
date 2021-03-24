
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

def exp_fn(f, n, x0):
    what = [x0]  # maps timestamps to states
    when = {x0: 0}  # maps states to timestamps
    xi = x0
    i = 1
    while i <= n:
        xi = f(xi)
        if xi in when:
            break
        what.append(xi)
        when[xi] = i
        i += 1
    if i > n:
        assert xi == what[n]
        return xi
    loop_start = when[xi]
    loop_len = i - when[xi]
    return what[loop_start + (n - loop_start) % loop_len]

with open('puzzle-input.txt') as f:
    text = f.read()
moves = text.strip().split(',')
s0 = 'abcdefghijklmnop'
MANY = 1_000_000_000
print(exp_fn(lambda state: dance(state, moves), MANY, s0))

    
