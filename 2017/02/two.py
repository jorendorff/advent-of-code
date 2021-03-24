import sys

total = 0
for line in sys.stdin:
    nums = list(map(int, line.split()))
    hits = []
    for a in nums:
        for b in nums:
            if a > b and a % b == 0:
                hits.append(a // b)
    assert len(hits) == 1
    total += hits[0]

print(total)
