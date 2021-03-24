from collections import deque

insns = {
    'set': 'r*',
    'mul': 'r*',
    'jnz': '*i',
    'sub': 'r*',
}

REGS = tuple('abcdefgh')

def parse_program(s):
    program = []
    for line in s.splitlines():
        insn = line.split()
        if insn[0] not in insns:
            raise ValueError('unrecognized instruction: ' + insn[0])
        operands = insn[1:]
        expected = insns[insn[0]]
        if len(operands) != len(expected):
            raise ValueError("wrong number of operands for " + insn[0])
        for i, (operand, expected_type) in enumerate(zip(operands, expected), start=1):
            if operand in REGS:
                if expected_type not in ('*', 'r'):
                    raise ValueError("found register, expected integer")
            elif expected_type == 'r':
                raise ValueError("expected register for operand {} of {}".format(i, insn[0]))
            else:
                assert expected_type in ('i', '*')
                insn[i] = int(insn[i])
        program.append(tuple(insn))
    return program

def optimize_program(code):
    break_indices = set()
    halt_index = len(code)
    for i, insn in enumerate(code):
        if insn[0] == 'jnz':
            target = i + insn[2]
            if not (0 <= target < len(code)):
                target = halt_index
            code[i] = ('jnza', insn[1], target, i + 1)
            break_indices.add(target)
            break_indices.add(i + 1)
    break_indices.add(halt_index)
    code.append(('halt',))

    break_indices.add(0)
    break_indices.add(len(code))
    breaks = sorted(break_indices)
    basic_blocks = [(a, code[a:b]) for a, b in zip(breaks, breaks[1:])]
    assert sum([b for i, b in basic_blocks], []) == code

    # simplify jnza to goto
    for i, block in basic_blocks:
        if block[-1][0] == 'jnza' and type(block[-1][1]) == int:
            block[-1] = ('goto', block[-1][2 if block[-1][1] != 0 else 3])

    # make sure every block ends with a control instruction
    for i, block in basic_blocks:
        if block[-1][0] not in ('jnza', 'halt', 'goto'):
            # we happen to know the index of the next block...
            block.append(('goto', i + len(block)))

    # eliminate goto-to-goto and jnza-to-goto
    blocks = dict(basic_blocks)
    done = False
    def opt_jump_target(k):
        nonlocal done
        target_block = blocks[block[-1][k]]
        if len(target_block) == 1:
            if target_block[0][0] == 'goto':
                done = False
                changed_insn = list(block[-1])
                changed_insn[k] = target_block[0][1]
                print("changing", block[-1], "to", tuple(changed_insn))
                block[-1] = tuple(changed_insn)
    while not done:
        done = True
        for i, block in basic_blocks:
            if block[-1][0] == 'goto':
                opt_jump_target(1)
            elif block[-1][0] == 'jnza':
                opt_jump_target(2)
                opt_jump_target(3)

    # In what follows, it seems like it'd be more natural to treat individual
    # instructions as steps, rather than blocks... then we can identify dead
    # instructions...

    # which registers are live at which labels?
    def liveness_analysis():
        reverse_jumps = {i: [] for i in blocks}
        for i, block in blocks.items():
            insn = block[-1]
            if insn[0] == 'goto':
                reverse_jumps[insn[1]].append(i)
            elif insn[0] == 'jnza':
                reverse_jumps[insn[2]].append(i)
                reverse_jumps[insn[3]].append(i)

        # which registers are live on entry to each block
        live_on_entry = {i: set() for i in blocks}
        live_on_exit = {i: set() for i in blocks}
        live_on_exit[halt_index].add('h')
        queue = deque([halt_index])
        while queue:
            i = queue.popleft()
            live = live_on_exit[i].copy()
            insn = blocks[i][-1]
            if insn[0] == 'jnza':
                live.add(insn[1])
            for insn in reversed(blocks[i][:-1]):
                verb, r, op = insn
                if r in live:
                    if verb == 'set':
                        live.remove(r)
                    if op in REGS:
                        live.add(op)
            if live - live_on_entry[i]:
                live |= live_on_entry[i]
                live_on_entry[i] = live
                for prev_i in reverse_jumps[i]:
                    if live - live_on_exit[prev_i]:
                        live_on_exit[prev_i] |= live
                        if prev_i not in queue:
                            queue.append(prev_i)
        return live_on_entry

    live_on_entry = liveness_analysis()

    ALL = 'all'
    def pretty_state(s):
        return "{" + ' '.join("{}={}".format(r, pretty_aval(v)) for r, v in sorted(s.items())) + "}"

    def pretty_aval(v):
        if v == ALL:
            return '*'
        elif len(v) == 0:
            return '?'
        else:
            return '/'.join(map(str, sorted(v)))

    def abstract_interpret():
        # hey let's abstract-interpret this thing
        # values are sets of size 0, 1, or 2; and ALL
        EMPTY = frozenset()
        ZERO = frozenset([0])

        def contains_zero(v):
            return v is ALL or 0 in v

        def contains_nonzero(v):
            return v is ALL or any(x != 0 for x in v)

        def remove_zero(v):
            if v is ALL:
                return ALL
            return v - ZERO

        def remove_nonzero(v):
            if contains_zero(v):
                return ZERO
            else:
                return EMPTY

        def union(a, b):
            if a is ALL or b is ALL:
                return ALL
            c = a | b
            if len(c) > 2:
                return ALL  # round up
            return c

        def mul(a, b):
            if a == EMPTY or b == EMPTY:
                return EMPTY
            if a == ZERO or b == ZERO:
                return ZERO
            if a == ALL or b == ALL:
                return ALL
            c = frozenset([x * y for x in a for y in b])
            if len(c) > 2:
                return ALL
            return c

        def sub(a, b):
            if a == EMPTY or b == EMPTY:
                return EMPTY
            if b == ZERO:
                return a
            if a == ALL or b == ALL:
                return ALL
            c = frozenset([x - y for x in a for y in b])
            if len(c) > 2:
                return ALL
            return c

        possible_values = {i: {r: frozenset() for r in REGS} for i in blocks.keys()}
        def add_possible_values(i, vals):
            known = possible_values[i]
            for r in known.keys():
                combined = union(known[r], vals[r])
                if combined != known[r]:
                    if i not in todo:
                        print("will have to visit block {}".format(i))
                        todo.append(i)
                    known[r] = combined

        def abstract_evaluate(operand):
            if type(operand) == int:
                return frozenset([operand])
            else:
                return abstract_state[operand]

        values_on_entry = {r: frozenset([0]) for r in REGS}
        values_on_entry['a'] = frozenset([1])
        possible_values[0] = values_on_entry
        todo = deque([0])
        while todo:
            i = todo.popleft()
            block = blocks[i]
            abstract_state = possible_values[i].copy()
            #print("starting block {} with values {}".format(i, pretty_state(abstract_state)))
            for insn in block[:-1]:
                lhs_reg = insn[1]
                rhs = abstract_evaluate(insn[2])
                if insn[0] == 'set':
                    abstract_state[lhs_reg] = rhs
                elif insn[0] == 'mul':
                    abstract_state[lhs_reg] = mul(abstract_state[lhs_reg], rhs)
                elif insn[0] == 'sub':
                    abstract_state[lhs_reg] = sub(abstract_state[lhs_reg], rhs)
            #print("finishing block {} with values {}".format(i, pretty_state(abstract_state)))
            if block[-1][0] == 'goto':
                add_possible_values(block[-1][1], abstract_state)
            elif block[-1][0] == 'jnza':
                r = block[-1][1]
                if contains_nonzero(abstract_state[r]):
                    there = abstract_state.copy()
                    there[r] = remove_zero(there[r])
                    add_possible_values(block[-1][2], there)
                if contains_zero(abstract_state[r]):
                    there = abstract_state.copy()
                    there[r] = remove_nonzero(there[r])
                    add_possible_values(block[-1][3], there)

        return possible_values

    possible_values = abstract_interpret()

    # dump the program
    print("----")
    for i, block in basic_blocks:
        if any(possible_values[i].values()):
            live_regs = live_on_entry[i]
            print("L{}:\t\t;; {}".format(i, pretty_state({k:v for k, v in possible_values[i].items()
                                                          if k in live_regs})))
        else:
            print("L{}:\t\t;; unreachable".format(i))
        for insn in block:
            print("  {}".format(' '.join(str(x) for x in insn)))
        print()

    return dict(basic_blocks)

def run_program(code):
    blocks = optimize_program(code)

    regs = {k: 0 for k in REGS}

    def evaluate(operand):
        if operand in REGS:
            return regs[operand]
        else:
            return int(operand)

    multiplies = 0

    block = blocks[0]
    while block is not None:
        for insn in block[:-1]:
            i = insn[0]
            if i == 'set':
                regs[insn[1]] = evaluate(insn[2])
            elif i == 'sub':
                regs[insn[1]] -= evaluate(insn[2])
            else:
                assert i == 'mul'
                regs[insn[1]] *= evaluate(insn[2])
                multiplies += 1

        # last instruction of a basic block tells us which block to run next
        insn = block[-1]
        i = insn[0]
        if i == 'halt':
            block = None
        elif i == 'goto':
            block = blocks[insn[1]]
        else:
            assert i == 'jnza'
            if evaluate(insn[1]) != 0:
                block = blocks[insn[2]]
            else:
                block = blocks[insn[3]]

        
    return multiplies

sample_program = '''\
set a 8
set b 1
mul b 2
sub a 1
jnz a -2
'''

assert run_program(parse_program(sample_program)) == 8

with open('puzzle-input.txt') as f:
    program = parse_program(f.read())
print(run_program(program))
