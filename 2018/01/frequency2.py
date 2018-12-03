import sys
import itertools
nums = map(int, sys.stdin.read().split())

seen = set()
x = 0
for n in itertools.cycle(nums):
    if x in seen:
        print(x)
        break
    seen.add(x)
    x += n

