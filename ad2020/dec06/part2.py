def count(surveys):
    questions = ''.join(surveys)
    assert questions.isalpha()
    qset = set(questions)
    return sum(1 for q in qset if all(q in survey for survey in surveys))

def counts(text):
    chunks = text.split("\n\n")
    return [count(chunk.split()) for chunk in chunks]

example = """\
abc

a
b
c

ab
ac

a
a
a
a

b
"""

assert counts(example) == [3, 0, 1, 1, 1]
assert sum(counts(example)) == 6

with open("input.txt") as f:
    data = f.read()
print(sum(counts(data)))
