from lib.advent import *


class Evaluator:
    def __init__(self, line):
        self.tokens = list(tokenize(r'\d+|[+*()]', line))
        self.i = 0

    def at_end(self):
        return self.i == len(self.tokens)

    def peek(self):
        if self.at_end():
            return None
        else:
            return self.tokens[self.i]

    def take(self):
        if self.at_end():
            raise ValueError("more expected")
        token = self.tokens[self.i]
        self.i += 1
        return token

    def require(self, t):
        x = self.take()
        if x != t:
            raise ValueError(f"expected '{t}', got '{x}'")

    def parse_prim(self):
        t = self.take()
        if t.isdigit():
            return int(t)
        elif t == '(':
            v = self.parse_expr()
            self.require(')')
            return v
        else:
            raise ValueError(f"expected number or '(', got {t}")

    def parse_add_expr(self):
        n = self.parse_prim()
        while self.peek() == '+':
            self.take()
            arg = self.parse_prim()
            n += arg
        return n

    def parse_expr(self):
        n = self.parse_add_expr()
        while self.peek() == '*':
            self.take()
            arg = self.parse_add_expr()
            n *= arg
        return n


def eval(s):
    ev = Evaluator(s)
    result = ev.parse_expr()
    if not ev.at_end():
        raise ValueError("expected end of line, got {ev.peek()}")
    return result

assert eval('1 + 2 * 3 + 4 * 5 + 6') == 231
assert eval('1 + (2 * 3) + (4 * (5 + 6))') == 51
assert eval('2 * 3 + (4 * 5)') == 46
assert eval('5 + (8 * 3 + 9 + 3 * 4 * 3)') == 1445
assert eval('5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))') == 669060
assert eval('((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2') == 23340


if __name__ == '__main__':
    print(sum(eval(line) for line in puzzle_input().splitlines()))
