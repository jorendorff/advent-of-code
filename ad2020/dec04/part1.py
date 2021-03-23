
FIELDS = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid', 'cid']

def parse_chunk(chunk):
    data = {}
    for word in chunk.split():
        key, sep, value = word.partition(":")
        assert sep == ":"
        assert key in FIELDS, f"{key} is not a valid field"
        assert key not in data
        data[key] = value
    return data

def is_valid(passport):
    return all(field in passport or field == 'cid'
               for field in FIELDS)

def count_valid(text):
    passports = [parse_chunk(chunk) for chunk in text.split("\n\n")]
    return sum(1 for p in passports if is_valid(p))

example = """\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"""

assert count_valid(example) == 2

with open('input.txt') as f:
    text = f.read()

print(count_valid(text))
