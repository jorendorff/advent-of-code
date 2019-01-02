def parse(nums):
    nkids, nmeta = nums[:2]
    rest = nums[2:]
    kids = []
    for i in range(nkids):
        kid, rest = parse(rest)
        kids.append(kid)
    meta = rest[:nmeta]
    rest = rest[nmeta:]
    return (kids, meta), rest

test_nums = [2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2]
assert parse(test_nums) == (([([], [10, 11, 12]), ([([], [99])], [2])], [1, 1, 2]), [])

def sum_meta(node):
    kids, meta = node
    if kids:
        return sum((sum_meta(kids[i-1]) if 0 <= i-1 < len(kids) else 0) for i in meta)
    else:
        return sum(meta)

assert sum_meta(parse(test_nums)[0]) == 66

with open('puzzle-input.txt') as f:
    nums = [int(x) for x in f.read().split()]
    outermost, rest = parse(nums)
    assert rest == []
    print(sum_meta(outermost))
