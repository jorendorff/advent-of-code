from lib.advent import *


def solve(nums):
    nums = sorted([int(s) for s in nums.split()])
    assert not any(nums[i] == nums[i + 1] for i in range(len(nums) - 1)), \
        NotImplementedError("no interchangeable adapters please")

    nums.insert(0, 0)
    nums.append(nums[-1] + 3)
    counts = [1]
    for j in range(1, len(nums)):
        i = j
        while i > 0 and nums[i - 1] + 3 >= nums[j]:
            i -= 1
        counts.append(sum(counts[i:]))
    return counts[-1]


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

assert solve(example1) == 8


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

assert solve(example2) == 19208


if __name__ == '__main__':
    print(solve(puzzle_input()))
