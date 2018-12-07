from collections import defaultdict

def how_long_must_we_toil(nworkers, base_duration, lines):
    blockers = defaultdict(set)

    for line in lines:
        [_, a, _, _, _, _, _, b, _, _] = line.split()
        blockers[b].add(a)
        blockers[a]  # also create an entry for a if there isn't one!

    idle_count = nworkers
    active = {}
    t = 0
    while blockers or active:
        for task in list(active):
            active[task] -= 1
            if active[task] == 0:
                del active[task]
                for stops in blockers.values():
                    if task in stops:
                        stops.remove(task)
                idle_count += 1

        if not blockers and not active:
            break

        while idle_count > 0:
            ready = set(b for b, stops in blockers.items() if not stops)
            if not ready:
                break # can't work, sorry buddy
            choice = min(ready)
            del blockers[choice]
            active[choice] = base_duration + (ord(choice) - ord('A') + 1)
            idle_count -= 1

        t += 1
    return t


sample_input = '''\
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
'''

assert how_long_must_we_toil(2, 0, sample_input.splitlines()) == 15

if __name__ == '__main__':
    with open('puzzle-input.txt') as f:
        print(how_long_must_we_toil(5, 60, f))
