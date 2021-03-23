from lib.advent import *

def dist(nums):
    prev = 0
    dist = [0, 0, 0, 0]
    for num in nums:
        assert 0 <= num - prev <= 3
        dist[num - prev] += 1
        prev = num
    dist[3] += 1  # to your device's built-in adapter
    return dist


def solve(nums):
    nums = sorted([int(s) for s in nums.split()])

    d = dist(nums)
    return d[1] * d[3]


example1 = """\
16
10
15
5
1
11
7
19
6
12
4
"""

assert solve(example1) == 7 * 5


example2 = """\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
"""

assert solve(example2) == 22 * 10


if __name__ == '__main__':
    print(solve(puzzle_input()))
