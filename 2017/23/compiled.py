from math import sqrt

h = 0
for b in range(105700, 122701, 17):
    if b % 2 == 0 or any(b % k == 0 for k in range(3, int(sqrt(b)) + 3, 2)):
        h += 1
print(h)
