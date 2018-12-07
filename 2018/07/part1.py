from collections import defaultdict

def psort(lines):
    blockers = defaultdict(set)

    for line in lines:
        [_, a, _, _, _, _, _, b, _, _] = line.split()
        blockers[b].add(a)
        blockers[a]  # Also create an entry for a if there isn't one

    log = ''
    while blockers:
        ready = set(b for b, stops in blockers.items() if not stops)
        if not ready:
            raise ValueError("bad")
        choice = min(ready)
        log += choice
        del blockers[choice]
        for stops in blockers.values():
            if choice in stops:
                stops.remove(choice)
    return log


sample_input = '''\
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
'''

assert psort(sample_input.splitlines()) == 'CABDFE'

if __name__ == '__main__':
    with open('puzzle-input.txt') as f:
        print(psort(f))
