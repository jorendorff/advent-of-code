def count(surveys):
    questions = ''.join(surveys)
    assert questions.isalpha()
    return len(set(questions))

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

assert counts(example) == [3, 3, 3, 1, 1]
assert sum(counts(example)) == 11

with open("input.txt") as f:
    data = f.read()
print(sum(counts(data)))
