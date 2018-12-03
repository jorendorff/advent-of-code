def find_neighbors(box_ids):
    for i, a in enumerate(box_ids):
        for b in box_ids[i+1:]:
            if len(a) != len(b):
                raise ValueError("input contains box ids of different lengths: {} and {}".format(a, b))
            common_chars = [ca for ca, cb in zip(a, b) if ca == cb]
            if len(common_chars) == len(a) - 1:
                yield ''.join(common_chars)

assert list(find_neighbors('abcde fghij klmno pqrst fguij axcye wvxyz'.split())) == ['fgij']

if __name__ == '__main__':
    with open("input.txt") as f:
        puzzle_input = f.read().splitlines()
    for solution in find_neighbors(puzzle_input):
        print(solution)
