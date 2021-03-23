def decode(s):
    assert len(s) == 10
    assert all(c in 'BF' for c in s[:7])
    assert all(c in 'LR' for c in s[7:])

    row = int(s[:7].replace('F', '0').replace('B', '1'), 2)
    col = int(s[7:].replace('L', '0').replace('R', '1'), 2)
    return row, col

assert decode('BFFFBBFRRR') == (70, 7)
assert decode('FFFBBBFRRR') == (14, 7)
assert decode('BBFFBBFRLL') == (102, 4)


def seat_id(coords):
    row, col = coords
    return row * 8 + col

assert seat_id(decode('BFFFBBFRRR')) == 567
assert seat_id(decode('FFFBBBFRRR')) == 119
assert seat_id(decode('BBFFBBFRLL')) == 820


with open('input.txt') as f:
    codes = f.read().split()

points = {seat_id(decode(c)) for c in codes}
for i in range(min(points), max(points)):
    if i not in points and (i + 1) in points and (i - 1) in points:
        print(i)





