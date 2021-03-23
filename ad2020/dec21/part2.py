# This puzzle seems a little off to me, for two reasons:
#
# 1.  We have to assume each allergen is marked at least once.
#
# 2.  I think the use of "can't possibly" in the puzzle is problematic...
#     but maybe not.

from lib.advent import *
import re


LINE_RE = re.compile(r'^([a-z ]+) \(contains ([a-z, ]*)\)$')


def parse_data(text):
    data = []
    for line in text.splitlines():
        match = LINE_RE.match(line.rstrip())
        if match is None:
            raise ValueError("can't parse line: {line.rstrip()!r}")
        data.append((set(match.group(1).split()), set(match.group(2).split(", "))))
    return data


def solve(text):
    data = parse_data(text)

    ingredients = set()
    allergens = set()
    for box_ingredients, box_allergens in data:
        ingredients |= box_ingredients
        allergens |= box_allergens

    # This doesn't logically eliminate every possible ingredient, is the
    # problem I see.
    all_suspects = set()
    suspects_by_allergen = {}
    for a in allergens:
        suspects = ingredients.copy()
        for bi, ba in data:
            if a in ba:
                suspects &= bi
        suspects_by_allergen[a] = suspects
        all_suspects |= suspects

    if len(all_suspects) != len(allergens):
        raise NotImplementedError("did not think about this case")

    unsolved_allergens = set(allergens)
    solved = {}
    while unsolved_allergens:
        for a in unsolved_allergens:
            remaining = suspects_by_allergen[a] - set(solved.keys())
            if len(remaining) == 1:
                [i] = remaining
                solved[i] = a
                unsolved_allergens.remove(a)
                break
        else:
            raise ValueError("can't solve, maybe no unique solution?")

    pairs = sorted(solved.items(), key=lambda pair: pair[1])
    for i, a in pairs:
        print(f"  - {i} contains {a}.")
    print()

    return ','.join(i for i, a in pairs)


example = """\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
"""

assert solve(example) == 'mxmxvkd,sqjhc,fvjkl'


if __name__ == '__main__':
    print(solve(puzzle_input()))
