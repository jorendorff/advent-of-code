
from . import part1
from .part1 import phase, crack
from lib.advent import *

def partial_sums(arr):
    psums = [0]
    t = 0
    for e in arr:
        t += e
        psums.append(t)
    return psums

def phase(arr):
    N = len(arr)
    psums = partial_sums(arr)
    def sumrange(start, stop):
        if stop > N:
            stop = N
        return psums[stop] - psums[start]

    def result_at(w):
        start = w - 1
        polarity = 1
        total = 0
        while start < N:
            total += polarity * sumrange(start, start + w)
            start += w + w
            polarity = -polarity
        return abs(total) % 10

    return [result_at(i) for i in range(1, N + 1)]

assert phase(crack('12345678')) == crack('48226158')

assert phase(crack('48226158')) == crack('34040438')

assert fn_exp(phase, 4)(crack('12345678')) == crack('01029498')


fft100 = fn_exp(phase, 100)

assert fft100(crack('80871224585914546619083218645595'))[:8] == crack('24176176')
assert fft100(crack('19617804207202209144916044189917'))[:8] == crack('73745418')
assert fft100(crack('69317163492948606335995924319873'))[:8] == crack('52432133')

def main():
    text = puzzle_input().strip()
    arr = crack(text) * 10000
    message_offset = int(text[:7])
    print("message offset:", message_offset)

    for i in range(100):
        print(i)
        arr = phase(arr)

    eight = arr[message_offset:message_offset + 8]
    print(''.join(str(digit) for digit in eight))

if __name__ == '__main__':
    main()
