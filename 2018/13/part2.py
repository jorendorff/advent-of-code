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



def last_cart(text):
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
        carts.sort(key=move_order)
        i = 0
        while i < len(carts):
            cart = carts[i]
            cart.advance()
            c = track_map[cart.y][cart.x]
            if c in '/\\+':
                cart.curve(c)
            else:
                assert c in '-|'

            for j, cart2 in enumerate(carts):
                if j != i:
                    if (cart.x, cart.y) == (cart2.x, cart2.y):
                        del carts[j]
                        if j < i:
                            i -= 1
                        assert carts[i] is cart
                        del carts[i]
                        break
            else:
                i += 1

        if len(carts) == 1:
            return carts[0].x, carts[0].y


sample_input = r'''
/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/
'''.lstrip('\n')

assert last_cart(sample_input) == (6, 4)


with open('puzzle-input.txt') as f:
    print(last_cart(f.read()))

