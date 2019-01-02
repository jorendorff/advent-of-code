class Cart:
    def __init__(self, x, y, dx, dy):
        self.x = x
        self.y = y
        self.dx = dx
        self.dy = dy
        self.next = 'left'

    def turn_left(self):
        self.dx, self.dy = self.dy, -self.dx

    def turn_right(self):
        self.dx, self.dy = -self.dy, self.dx

    def advance(self):
        self.x += self.dx
        self.y += self.dy

    def curve(self, c):
        if c == '/':
            self.dx, self.dy = -self.dy, -self.dx
        elif c == '\\':
            self.dx, self.dy = self.dy, self.dx
        else:
            assert c == '+'
            if self.next == 'left':
                self.turn_left()
                self.next = 'straight'
            elif self.next == 'straight':
                self.next = 'right'
            else:
                assert self.next == 'right'
                self.turn_right()
                self.next = 'left'


def move_order(cart):
    return (cart.y, cart.x)

DIRS = {
    '<': (-1, 0),
    '>': (+1, 0),
    '^': (0, -1),
    'v': (0, +1),
}



def first_crash(text):
    orig_map = text.splitlines()
    carts = []
    for y, line in enumerate(orig_map):
        for x, c in enumerate(line):
            if c in DIRS:
                carts.append(Cart(x, y, *DIRS[c]))

    plain = text.replace('>', '-').replace('<', '-') \
        .replace('^', '|').replace('v', '|')
    track_map = plain.splitlines()

    while True:
        for cart in sorted(carts, key=move_order):
            cart.advance()
            c = track_map[cart.y][cart.x]
            if c in '/\\+':
                cart.curve(c)
            else:
                assert c in '-|'
            if any((a.x, a.y) == (cart.x, cart.y)
                   for a in carts if a is not cart):
                return (cart.x, cart.y)


sample_input = r'''
/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   
'''.lstrip('\n')

assert first_crash(sample_input) == (7, 3)


with open('puzzle-input.txt') as f:
    print(first_crash(f.read()))

