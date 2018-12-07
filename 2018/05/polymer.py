import re

def react(s):
    result = []
    for c in s:
        complement = c.lower() if c.isupper() else c.upper()
        if result and result[-1] == complement:
            del result[-1]
        else:
            result.append(c)
    return ''.join(result)

assert react('aA') == ''
assert react('abBA') == ''
assert react('abAB') == 'abAB'
assert react('aabAAB') == 'aabAAB'

def shortest(source):
    results = []
    for i in range(26):
        lower = chr(ord('a') + i)
        upper = chr(ord('A') + i)
        start = source.replace(lower, '').replace(upper, '')
        results.append(len(react(start)))
    return min(results)

assert shortest('dabAcCaCBAcCcaDA') == 4

assert shortest('baddacabbaUABBACADDAB') == 0

if __name__ == '__main__':
    with open('puzzle-input.txt') as f:
        polymer = f.read().strip()
    if re.match(r'^[a-zA-Z]*$', polymer) is None:
        print("bad input")
    print(shortest(polymer))
    
