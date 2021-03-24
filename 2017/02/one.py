import sys

total = 0
for line in sys.stdin:
    nums = list(map(int, line.split()))
    total += max(nums) - min(nums)

print(total)
