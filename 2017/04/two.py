import sys

valid = 0
for line in sys.stdin:
    words = line.split()
    if len(words) == len(set([''.join(sorted(w)) for w in words])):
        valid += 1
print(valid)
