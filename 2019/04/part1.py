"""Day 4: Secure Container

You arrive at the Venus fuel depot only to discover it's protected by a
password. The Elves had written the password on a sticky note, but someone
threw it out.

However, they do remember a few key facts about the password:

*   It is a six-digit number.
*   The value is within the range given in your puzzle input.
*   Two adjacent digits are the same (like `22` in `122345`).
*   Going from left to right, the digits never decrease; they only ever increase
    or stay the same (like `111123` or `135679`).

Other than the range rule, the following are true:

*   111111 meets these criteria (double 11, never decreases).
*   223450 does not meet these criteria (decreasing pair of digits 50).
*   123789 does not meet these criteria (no double).

How many different passwords within the range given in your puzzle input meet these criteria?

"""


def valid(password):
    password = str(password)
    return (
        # It is a six-digit number.
        len(password) == 6 and password.isdigit()
        # Two adjacent digits are the same (like `22` in `122345`).
        and any(password[i] == password[i + 1] for i in range(5))
        # Going from left to right, the digits never decrease
        and not any(password[i] > password[i + 1] for i in range(5))
    )


assert valid(111111)
assert valid(223450)
assert valid(123789)


def how_many_in_range(first, last):
    return sum(1 for password in range(first, last + 1) if valid(password))


with open("puzzle-input.txt") as f:
    first, last = f.read().strip().split('-')
    print(how_many_in_range(int(first), int(last))
