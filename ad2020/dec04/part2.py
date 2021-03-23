import string

FIELDS = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid', 'cid']

EYE_COLORS = {'amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'}

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
    if not all(field in passport or field == 'cid'
               for field in FIELDS):
        return False

    byr, iyr, eyr, hgt, hcl, ecl, pid = [passport[field] for field in FIELDS[:-1]]
    return (
        len(byr) == 4 and byr.isdigit() and 1920 <= int(byr) <= 2002
        and len(iyr) == 4 and iyr.isdigit() and 2010 <= int(iyr) <= 2020
        and len(eyr) == 4 and eyr.isdigit() and 2020 <= int(eyr) <= 2030
        and ((hgt[-2:] == 'cm' and hgt[:-2].isdigit() and 150 <= int(hgt[:-2]) <= 193)
             or (hgt[-2:] == 'in' and hgt[:-2].isdigit() and 59 <= int(hgt[:-2]) <= 76))
        and len(hcl) == 7 and hcl.startswith('#') and all(c in string.hexdigits for c in hcl[1:])
        and ecl in EYE_COLORS
        and len(pid) == 9 and pid.isdigit()
    )

def count_valid(text):
    passports = [parse_chunk(chunk) for chunk in text.split("\n\n")]
    return sum(1 for p in passports if is_valid(p))

bad_example = """\
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
"""

assert count_valid(bad_example) == 0

good_example = """\
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"""

assert count_valid(good_example) == 4


with open('input.txt') as f:
    text = f.read()

print(count_valid(text))
