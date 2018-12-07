import re
from collections import namedtuple, Counter

Shift = namedtuple('Shift', 'guard_id naps')

Guard = namedtuple('Guard', 'id nap_minutes')

def parse(lines):
    shift = None
    asleep_since = None
    for line in sorted(lines):
        line = line.strip()
        match = re.match(r'^\[.*\] Guard #(\d+) begins shift$', line)
        if match:
            if asleep_since is not None:
                raise ValueError("previous shift ended while guard asleep")
            if shift is not None:
                yield shift
            shift = Shift(guard_id=int(match.group(1)), naps=[])
            continue

        match = re.match(r'^\[1518-\d\d-\d\d 00:(\d\d)\] (falls asleep|wakes up)', line)
        if match is None:
            raise ValueError("can't parse line: " + repr(line))
        if match.group(2) == 'falls asleep':
            if asleep_since is not None:
                raise ValueError("fell asleep twice in a row")
            asleep_since = int(match.group(1))
        else:
            if asleep_since is None:
                raise ValueError("woke up without first falling asleep")
            now = int(match.group(1))
            if now < asleep_since:
                raise ValueError("time travel while snoozing")
            shift.naps.append((asleep_since, now))
            asleep_since = None
    # end of input
    if asleep_since is not None:
        raise ValueError("end of input while guard asleep")
    if shift is not None:
        yield shift

def guard_data(shifts):
    guards = {}
    for shift in shifts:
        if shift.guard_id not in guards:
            guards[shift.guard_id] = Guard(shift.guard_id, [])
        guard = guards[shift.guard_id]
        for start, stop in shift.naps:
            guard.nap_minutes.extend(list(range(start, stop)))
    return guards

def part1(lines):
    shifts = list(parse(lines))
    guards = guard_data(shifts)
    sleepiest = max(guards.values(), key=lambda guard: len(guard.nap_minutes))
    [(minute, count)] = Counter(sleepiest.nap_minutes).most_common(1)
    return sleepiest.id * minute

def part2(lines):
    shifts = list(parse(lines))
    guards = guard_data(shifts)
    counts = Counter((shift.guard_id, minute)
                     for shift in shifts
                     for start, stop in shift.naps
                     for minute in range(start, stop))
    [((guard_id, minute), count)] = counts.most_common(1)
    return guard_id * minute

test_input = '''\
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
'''

assert part1(test_input.splitlines()) == 240
assert part2(test_input.splitlines()) == 4455

if __name__ == '__main__':
    with open('puzzle-input.txt') as f:
        lines = f.readlines()
    print(part1(lines))
    print(part2(lines))


