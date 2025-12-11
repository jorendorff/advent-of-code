import sys, time
from collections import defaultdict

def main(args):
    if len(args) != 1:
        print("usage: day11.py puzzle-input.txt")
        sys.exit(1)
    [filename] = args
    with open(filename) as f:
        lines = f.readlines()

    devices = set()
    rev = defaultdict(list)
    for line in lines:
        src, colon, dsts = line.partition(': ')
        devices.add(src)
        assert colon
        for dst in dsts.split():
            devices.add(dst)
            rev[dst].append(src)

    t0 = time.time()
            
    # assign each device a tier, using slowest possible algorithm
    # `out` is in tier 0; all of each node's predecessors are in higher tiers.
    # if the dag is cyclic, this simply doesn't terminate lol
    tier_map = {k: 0 for k in devices}
    done = False
    while not done:
        done = True
        for dst, srcs in rev.items():
            for src in srcs:
                if tier_map[src] <= tier_map[dst]:
                    tier_map[src] = tier_map[dst] + 1
                    done = False

    # Invert the tier map, producing the list of tiers
    tiers = [[] for _ in range(max(tier_map.values()) + 1)]
    for dev, t in tier_map.items():
        tiers[t].append(dev)

    def count_paths(route_src, route_dst):    
        num_paths = defaultdict(int)
        num_paths[route_dst] = 1
        r0 = tier_map[route_dst]
        r1 = tier_map[route_src]
        for tier in tiers[r0:r1]:
            for dst in tier:
                for src in rev[dst]:
                    num_paths[src] += num_paths[dst]
        return num_paths[route_src]

    print("part 1:", count_paths('you', 'out'))

    part2 = (count_paths('svr', 'fft') * count_paths('fft', 'dac') * count_paths('dac', 'out')
            + count_paths('svr', 'dac') * count_paths('dac', 'fft') * count_paths('fft', 'out'))
    print("part 2:", part2)
    print("done in", time.time() - t0, "seconds")

main(sys.argv[1:])
