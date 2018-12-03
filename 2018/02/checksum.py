from collections import Counter

def checksum(box_ids):
    twos = 0
    threes = 0
    for box_id in box_ids:
        hist = Counter(box_id)
        if 2 in hist.values():
            twos += 1
        if 3 in hist.values():
            threes += 1
    return twos * threes

assert checksum(['abcdef', 'bababc', 'abbcde', 'abcccd', 'aabcdd', 'abcdee', 'ababab']) == 12


if __name__ == '__main__':
    with open("input.txt") as f:
        puzzle_input = f.read().splitlines()
    print(checksum(puzzle_input))

