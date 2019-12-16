
from lib.advent import *


def upattern(w):
    while True:
        for e in [0, 1, 0, -1]:
            for i in range(w):
                yield e

def pattern(w):
    it = upattern(w)
    next(it)
    return it

def phase(arr):
    return [abs(sum(i * j for i, j in zip(arr, pattern(w)))) % 10
            for w in range(1, len(arr) + 1)]

def crack(s):
    assert s.isdigit()
    return [int(digit) for digit in s]

assert phase(crack('12345678')) == crack('48226158')

assert phase(crack('48226158')) == crack('34040438')

assert fn_exp(phase, 4)(crack('12345678')) == crack('01029498')


fft100 = fn_exp(phase, 100)

assert fft100(crack('80871224585914546619083218645595'))[:8] == crack('24176176')
assert fft100(crack('19617804207202209144916044189917'))[:8] == crack('73745418')
assert fft100(crack('69317163492948606335995924319873'))[:8] == crack('52432133')


def main():
    s = fft100(crack(puzzle_input().strip()))
    eight = s[:8]
    print(''.join(str(digit) for digit in eight))


if __name__ == '__main__':
    main()
