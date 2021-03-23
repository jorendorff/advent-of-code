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
    for a in allergens:
        suspects = ingredients.copy()
        for bi, ba in data:
            if a in ba:
                suspects &= bi
        all_suspects |= suspects

    ## print(f"hmm. we have {len(all_suspects)} suspects:")
    ## print("   ", sorted(all_suspects))
    ## print(f"for {len(allergens)} allergens:")
    ## print("   ", sorted(allergens))
    if len(all_suspects) != len(allergens):
        raise NotImplementedError("can't be sure of solution, perhaps more ingredients can be cleared")

    clean_ingredients = ingredients - all_suspects
    return sum(
        1
        for bi, ba in data
        for i in bi
        if i in clean_ingredients
    )


example = """\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
"""

assert solve(example) == 5


if __name__ == '__main__':
    print(solve(puzzle_input()))
