"""Part Two

An Elf just remembered one more important detail: the two adjacent matching
digits are not part of a larger group of matching digits.

Given this additional criterion, but still ignoring the range rule, the following are now true:

*   112233 meets these criteria because the digits never decrease and all
    repeated digits are exactly two digits long.

*   123444 no longer meets the criteria (the repeated 44 is part of a larger
    group of 444).

*   111122 meets the criteria (even though 1 is repeated more than twice, it
    still contains a double 22).

How many different passwords within the range given in your puzzle input meet
all of the criteria?
"""


def valid(password):
    password = str(password)
    return (
        # It is a six-digit number.
        len(password) == 6 and password.isdigit()
        # Two adjacent digits are the same (like `22` in `122345`).
        # The two adjacent matching digits are not part of
        # a larger group of matching digits.
        and any(password[i] == password[i + 1]
                and (i - 1 < 0 or password[i - 1] != password[i])
                and (i + 2 >= 6 or password[i + 2] != password[i])
                for i in range(5))
        # Going from left to right, the digits never decrease
        and not any(password[i] > password[i + 1] for i in range(5))
    )


assert not valid(111111)
assert not valid(223450)
assert not valid(123789)
assert valid(112233)
assert not valid(123444)
assert valid(111122)


def how_many_in_range(first, last):
    return sum(1 for password in range(first, last + 1) if valid(password))


with open("puzzle-input.txt") as f:
    first, last = f.read().strip().split('-')
    print(how_many_in_range(int(first), int(last)))
